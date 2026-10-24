//! One running Red, Blue or Yellow: the overworld, the text box, fades between maps,
//! options and resuming where you left off.

use std::collections::HashMap;

use super::data::{self, Dir, SpriteFrame};
use super::layout::{self, Layout};
use super::render::{self, Art};
use super::text::{self, Names};
use super::textbox::{TextBox, TextInput};
use super::world::{Event, Input, Motion, Overworld};
use crate::gfx::{decode_2bpp, rgb, Frame, Palette, Tile};
use crate::input::Buttons;
use crate::options::{ColorMode, Options};
use crate::rom::{GameId, Rom};

const FADE_FRAMES: u8 = 10;

struct Fade {
    frame: u8,
    warp: (u8, u8),
}

pub struct Game {
    pub id: GameId,
    rom: Rom,
    lay: &'static Layout,
    pub ow: Overworld,
    textbox: Option<TextBox>,
    sprites: HashMap<u8, Vec<SpriteFrame>>,
    font: Vec<Tile>,
    box_tiles: Vec<Tile>,
    flowers: [Tile; 3],
    shown_flower: u8,
    pub frame: Frame,
    view: (usize, usize),
    pub options: Options,
    prev: Buttons,
    fade: Option<Fade>,
    player_name: Vec<u8>,
    rival_name: Vec<u8>,
}

impl Game {
    pub fn new(id: GameId, data: Vec<u8>) -> Result<Game, String> {
        let lay: &'static Layout = match id {
            GameId::Red => &layout::RED,
            GameId::Blue => &layout::BLUE,
            GameId::Yellow => &layout::YELLOW,
            _ => return Err(format!("{} isn't playable in Retold yet", id.name())),
        };
        let rom = Rom::new(data);
        let ow = Overworld::new(&rom, lay, lay.reds_house_2f, 3, 6, Dir::Up, lay.pallet_town)
            .ok_or("couldn't read the first map")?;
        let flower = |p| decode_2bpp(&rom.bytes(p, 16));
        let flowers = [flower(lay.flower_tile1), flower(lay.flower_tile2), flower(lay.flower_tile3)];
        let mut g = Game {
            id,
            font: data::font_tiles(&rom, lay),
            box_tiles: data::textbox_tiles(&rom, lay),
            flowers,
            shown_flower: 0,
            rom,
            lay,
            ow,
            textbox: None,
            sprites: HashMap::new(),
            frame: Frame::new(160, 144),
            view: (20, 18),
            options: Options::default(),
            prev: Buttons::empty(),
            fade: None,
            player_name: text::encode("RED"),
            rival_name: text::encode("BLUE"),
        };
        g.load_sprites();
        g.render();
        Ok(g)
    }

    /// Sets how many 8x8 tiles the view shows. The original is 20x18; wider fills the phone.
    pub fn set_view(&mut self, tiles_w: usize, tiles_h: usize) {
        let w = (tiles_w.clamp(20, 64) / 2) * 2;
        let h = (tiles_h.clamp(18, 40) / 2) * 2;
        self.view = (w, h);
        self.frame.resize(w * 8, h * 8);
        self.render();
    }

    pub fn map_id(&self) -> u8 {
        self.ow.map.header.id
    }

    pub fn map_name(&self) -> &'static str {
        self.lay.map_names.get(self.map_id() as usize).copied().unwrap_or("?")
    }

    fn load_sprites(&mut self) {
        let mut ids: Vec<u8> = self.ow.npcs.iter().map(|n| n.sprite).collect();
        ids.push(1);
        for id in ids {
            if !self.sprites.contains_key(&id) {
                let frames = data::sprite_frames(&self.rom, self.lay, id);
                self.sprites.insert(id, frames);
            }
        }
    }

    fn palette(&self) -> Palette {
        match self.options.colors {
            ColorMode::SuperGameBoy => {
                let h = &self.ow.map.header;
                let id = data::overworld_palette_id(self.lay, h.id, h.tileset, self.ow.last_map);
                data::sgb_palette(&self.rom, self.lay, id)
            }
            ColorMode::Classic => [rgb(0x9b, 0xbc, 0x0f), rgb(0x8b, 0xac, 0x0f), rgb(0x30, 0x62, 0x30), rgb(0x0f, 0x38, 0x0f)],
            ColorMode::Gray => [rgb(255, 255, 255), rgb(0xaa, 0xaa, 0xaa), rgb(0x55, 0x55, 0x55), rgb(0, 0, 0)],
        }
    }

    /// Advances one 1/60 s frame with the buttons currently held, and redraws.
    pub fn step(&mut self, held: Buttons) {
        let pressed = held & !self.prev;
        self.prev = held;

        if let Some(f) = &mut self.fade {
            f.frame += 1;
            let (frame, (map, warp)) = (f.frame, f.warp);
            if frame == FADE_FRAMES {
                self.ow.take_warp(&self.rom, self.lay, map, warp);
                self.load_sprites();
            }
            if frame >= FADE_FRAMES * 2 {
                self.fade = None;
            }
            self.render();
            return;
        }

        let talking = self.textbox.is_some();
        let input = Input {
            held_dir: if talking { None } else { held.direction() },
            a_pressed: !talking && pressed.contains(Buttons::A),
            b_held: held.contains(Buttons::B),
        };
        let motion = Motion {
            walk_speed: self.options.walk_speed,
            run_speed: self.options.run_speed,
            run_enabled: self.options.run_with_b,
            npcs_wander: true,
        };
        let ev = self.ow.frame(&self.rom, self.lay, &input, &motion, talking);

        if let Some(tb) = &mut self.textbox {
            tb.frame(&TextInput {
                pressed: pressed.intersects(Buttons::A | Buttons::B),
                held: held.intersects(Buttons::A | Buttons::B) && self.options.hold_to_speed_text,
                speed: self.options.text_speed,
            });
            if tb.closed {
                self.textbox = None;
            }
        }

        match ev {
            Event::Talk { text, .. } => self.open_text(text),
            Event::Warp { map, warp } => {
                if self.options.fades {
                    self.fade = Some(Fade { frame: 0, warp: (map, warp) });
                } else {
                    self.ow.take_warp(&self.rom, self.lay, map, warp);
                    self.load_sprites();
                }
            }
            _ => {}
        }
        if self.ow.flower_frame != self.shown_flower && self.ow.map.tileset.anim == 2 {
            self.shown_flower = self.ow.flower_frame;
            self.ow.map.tiles[3] = self.flowers[self.shown_flower as usize];
        }
        // People who walked into view on a new map need their pictures.
        if self.ow.npcs.iter().any(|n| !self.sprites.contains_key(&n.sprite)) {
            self.load_sprites();
        }
        self.render();
    }

    fn open_text(&mut self, text_id: u8) {
        let Some(p) = data::text_pointer(&self.rom, &self.ow.map.header, text_id) else { return };
        let names = Names { player: &self.player_name, rival: &self.rival_name };
        let ops = text::decode(&self.rom, p, &names);
        let tb = TextBox::new(ops);
        if tb.has_words() {
            self.textbox = Some(tb);
        }
    }

    pub fn render(&mut self) {
        let pal = self.palette();
        let (w, h) = self.view;
        self.frame.resize(w * 8, h * 8);
        let sprites = &self.sprites;
        let lookup = |id: u8| sprites.get(&id).map(|v| v.as_slice());
        let art = Art { sprite: &lookup, font: &self.font, box_tiles: &self.box_tiles };
        render::draw_overworld(&mut self.frame, &self.ow, &art, &pal, w, h);
        if let Some(tb) = &self.textbox {
            render::draw_textbox(&mut self.frame, tb, &art, &pal, w, h);
        }
        if let Some(f) = &self.fade {
            let t = if f.frame < FADE_FRAMES { f.frame } else { FADE_FRAMES * 2 - f.frame };
            let amount = (t as u32 * 255 / FADE_FRAMES as u32).min(255) as u8;
            self.frame.fade_toward(pal[0], amount);
        }
    }

    /// Where you are, so the app can bring you back to the same spot.
    pub fn save(&self) -> Vec<u8> {
        let p = &self.ow.player;
        let mut out = b"RTLD".to_vec();
        out.push(1);
        out.push(self.id as u8);
        out.push(self.map_id());
        out.extend_from_slice(&(p.x as i16).to_le_bytes());
        out.extend_from_slice(&(p.y as i16).to_le_bytes());
        out.push(p.facing as u8);
        out.push(self.ow.last_map);
        out
    }

    pub fn load(&mut self, data: &[u8]) -> bool {
        if data.len() < 12 || &data[0..4] != b"RTLD" || data[4] != 1 || data[5] != self.id as u8 {
            return false;
        }
        let map = data[6];
        let x = i16::from_le_bytes([data[7], data[8]]) as i32;
        let y = i16::from_le_bytes([data[9], data[10]]) as i32;
        let facing = Dir::from_u8(data[11]);
        let last = *data.get(12).unwrap_or(&self.lay.pallet_town);
        let Some(ow) = Overworld::new(&self.rom, self.lay, map, x, y, facing, last) else { return false };
        if !ow.map.inside(x, y) {
            return false;
        }
        self.ow = ow;
        self.textbox = None;
        self.fade = None;
        self.load_sprites();
        self.render();
        true
    }

    /// Whether a map text shows any words yet (for Kiro's desk tool).
    pub fn debug_text_has_words(&self, text_id: u8) -> bool {
        let Some(p) = data::text_pointer(&self.rom, &self.ow.map.header, text_id) else { return false };
        let names = Names { player: &self.player_name, rival: &self.rival_name };
        TextBox::new(text::decode(&self.rom, p, &names)).has_words()
    }

    /// Jumps straight to a map's first door (for Kiro's desk tool).
    pub fn debug_goto(&mut self, map: u8) -> bool {
        let Some(h) = data::map_header(&self.rom, self.lay, map) else { return false };
        let objs = data::map_objects(&self.rom, &h);
        let (x, y) = objs.warps.first().map(|w| (w.x as i32, w.y as i32)).unwrap_or((h.width as i32, h.height as i32));
        if !self.ow.enter_map(&self.rom, self.lay, map, x, y) {
            return false;
        }
        self.load_sprites();
        self.render();
        true
    }
}
