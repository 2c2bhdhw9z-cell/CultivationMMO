//! The overworld: the map you're on, its neighbours, you, and the people around you.
//! Movement, collisions, ledges, doors and map edges follow the original's rules.

use super::data::{self, Connection, Dir, MapHeader, MapObjects, ObjectKind, Tileset, STAY, WALK};
use super::layout::Layout;
use crate::gfx::{decode_2bpp, Tile};
use crate::rom::Rom;

/// A neighbouring map, kept just for drawing its edge and walking into it.
pub struct Neighbor {
    pub conn: Connection,
    pub header: MapHeader,
    pub blocks: Vec<u8>,
}

pub struct LoadedMap {
    pub header: MapHeader,
    pub blocks: Vec<u8>,
    pub objects: MapObjects,
    pub tileset: Tileset,
    pub tiles: Vec<Tile>,
    pub blockset: Vec<[u8; 16]>,
    pub passable: Vec<u8>,
    pub doors: Vec<u8>,
    pub warp_tiles: Vec<u8>,
    pub neighbors: Vec<Neighbor>,
    water_raw: [u8; 16],
    flower_frame: Option<u8>,
}

impl LoadedMap {
    pub fn load(rom: &Rom, lay: &Layout, id: u8) -> Option<LoadedMap> {
        let header = data::map_header(rom, lay, id)?;
        let blocks = data::map_blocks(rom, &header);
        let objects = data::map_objects(rom, &header);
        let tileset = data::tileset(rom, lay, header.tileset);
        let tiles = data::tileset_tiles(rom, &tileset);
        let blockset = (0..=255u8).map(|b| data::tileset_block(rom, &tileset, b)).collect();
        let passable = data::tile_list(rom, tileset.coll, 0xff);
        let doors = data::door_tiles(rom, lay, header.tileset);
        let warp_tiles = data::warp_tiles(rom, lay, header.tileset);
        let neighbors = header
            .connections
            .iter()
            .filter_map(|c| {
                let h = data::map_header(rom, lay, c.map)?;
                let b = data::map_blocks(rom, &h);
                Some(Neighbor { conn: *c, header: h, blocks: b })
            })
            .collect();
        let mut water_raw = [0u8; 16];
        water_raw.copy_from_slice(&rom.bytes(tileset.gfx.offset(0x14 * 16), 16));
        Some(LoadedMap {
            header,
            blocks,
            objects,
            tileset,
            tiles,
            blockset,
            passable,
            doors,
            warp_tiles,
            neighbors,
            water_raw,
            flower_frame: None,
        })
    }

    pub fn width_steps(&self) -> i32 {
        self.header.width as i32 * 2
    }

    pub fn height_steps(&self) -> i32 {
        self.header.height as i32 * 2
    }

    pub fn block_at(&self, bx: i32, by: i32) -> u8 {
        let w = self.header.width as i32;
        let h = self.header.height as i32;
        if (0..w).contains(&bx) && (0..h).contains(&by) {
            return self.blocks[(by * w + bx) as usize];
        }
        for n in &self.neighbors {
            let tw = n.header.width as i32;
            let th = n.header.height as i32;
            let (side, nbx, nby) = match n.conn.dir {
                Dir::Up => (by < 0, bx - n.conn.offset, by + th),
                Dir::Down => (by >= h, bx - n.conn.offset, by - h),
                Dir::Left => (bx < 0, bx + tw, by - n.conn.offset),
                Dir::Right => (bx >= w, bx - w, by - n.conn.offset),
            };
            if side && (0..tw).contains(&nbx) && (0..th).contains(&nby) {
                return n.blocks[(nby * tw + nbx) as usize];
            }
        }
        self.objects.border
    }

    /// The tile at a tile position (8x8 units, from the map's top-left corner).
    pub fn tile_at(&self, tx: i32, ty: i32) -> u8 {
        let block = self.block_at(tx.div_euclid(4), ty.div_euclid(4));
        self.blockset[block as usize][(ty.rem_euclid(4) * 4 + tx.rem_euclid(4)) as usize]
    }

    /// The tile the game checks for a step position: its bottom-left quarter.
    pub fn step_tile(&self, x: i32, y: i32) -> u8 {
        self.tile_at(x * 2, y * 2 + 1)
    }

    pub fn passable(&self, tile: u8) -> bool {
        self.passable.contains(&tile)
    }

    pub fn inside(&self, x: i32, y: i32) -> bool {
        (0..self.width_steps()).contains(&x) && (0..self.height_steps()).contains(&y)
    }

    /// Moves the water and flowers along, like the original does every frame.
    pub fn animate(&mut self, c1: &mut u8, c2: &mut u8) {
        let anim = self.tileset.anim;
        if anim == 0 {
            return;
        }
        *c1 += 1;
        if *c1 < 20 {
            return;
        }
        if *c1 == 20 {
            *c2 = (*c2 + 1) & 7;
            let left = *c2 & 4 != 0;
            for b in self.water_raw.iter_mut() {
                *b = if left { b.rotate_left(1) } else { b.rotate_right(1) };
            }
            self.tiles[0x14] = decode_2bpp(&self.water_raw);
            if anim & 1 == 1 {
                *c1 = 0;
            }
            return;
        }
        *c1 = 0;
        self.flower_frame = Some(match *c2 & 3 {
            0 | 1 => 0,
            2 => 1,
            _ => 2,
        });
    }
}

#[derive(Clone, Copy, Debug)]
pub struct Step {
    pub dir: Dir,
    /// Pixels moved so far.
    pub done: u16,
    /// Pixels to move in total (16 a step, 32 for a ledge jump).
    pub total: u16,
    pub jump: bool,
}

#[derive(Clone, Debug)]
pub struct Player {
    pub x: i32,
    pub y: i32,
    pub facing: Dir,
    pub step: Option<Step>,
    /// Pixels walked, for the walking animation.
    pub walked: u32,
}

impl Player {
    /// Pixel offset of the current step, for drawing and scrolling.
    pub fn pixel_offset(&self) -> (i32, i32) {
        match self.step {
            Some(s) => {
                let (dx, dy) = s.dir.delta();
                (dx * s.done as i32, dy * s.done as i32)
            }
            None => (0, 0),
        }
    }

    /// Height of a ledge hop above the ground, in pixels.
    pub fn hop(&self) -> i32 {
        match self.step {
            Some(s) if s.jump => {
                // The original's hop: up 8 pixels and back over the jump.
                const ARC: [i32; 16] = [4, 6, 8, 10, 11, 12, 12, 12, 11, 10, 9, 8, 6, 4, 0, 0];
                let i = (s.done as usize * 16 / s.total as usize).min(15);
                ARC[i]
            }
            _ => 0,
        }
    }

    /// Which animation frame (0-3) the walk is on.
    pub fn anim_frame(&self) -> u8 {
        ((self.walked / 8) % 4) as u8
    }
}

#[derive(Clone, Debug)]
pub struct Npc {
    pub index: u8,
    pub sprite: u8,
    pub x: i32,
    pub y: i32,
    pub facing: Dir,
    pub movement: u8,
    pub range: u8,
    pub text: u8,
    pub kind: ObjectKind,
    pub step: Option<Step>,
    pub walked: u32,
    delay: u16,
    disp_x: i32,
    disp_y: i32,
    half: bool,
}

impl Npc {
    pub fn pixel_offset(&self) -> (i32, i32) {
        match self.step {
            Some(s) => {
                let (dx, dy) = s.dir.delta();
                (dx * s.done as i32, dy * s.done as i32)
            }
            None => (0, 0),
        }
    }

    pub fn anim_frame(&self) -> u8 {
        if self.step.is_none() {
            return 0;
        }
        ((self.walked / 4) % 4) as u8
    }

    /// Where it is, or where it's stepping to.
    fn occupies(&self, x: i32, y: i32) -> bool {
        if self.x == x && self.y == y {
            return true;
        }
        if let Some(s) = self.step {
            let (dx, dy) = s.dir.delta();
            return self.x + dx == x && self.y + dy == y;
        }
        false
    }
}

/// What happened during an overworld frame that the game above needs to act on.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Event {
    None,
    /// Talk to someone or read a sign: the map's text number, and the person (if any).
    Talk { text: u8, npc: Option<usize> },
    /// Through a door, stairs or mat to another map.
    Warp { map: u8, warp: u8 },
    Bump,
}

/// Settings that change how the overworld moves.
#[derive(Clone, Copy, Debug)]
pub struct Motion {
    /// Pixels per frame when walking (1 = the original's speed).
    pub walk_speed: u8,
    /// Pixels per frame when running (holding B).
    pub run_speed: u8,
    pub run_enabled: bool,
    /// People wander like in the original.
    pub npcs_wander: bool,
}

pub struct Overworld {
    pub map: LoadedMap,
    pub player: Player,
    pub npcs: Vec<Npc>,
    pub last_map: u8,
    pub flower_frame: u8,
    anim_c1: u8,
    anim_c2: u8,
    rng: u32,
    ledges: Vec<(Dir, u8, u8)>,
    pairs_land: Vec<(u8, u8, u8)>,
    /// Step down out of the door you just came through.
    step_out: bool,
    /// Frames left in which buttons are ignored (half a second after a warp, as in the original).
    ignore_input: u8,
}

pub struct Input {
    pub held_dir: Option<Dir>,
    pub a_pressed: bool,
    pub b_held: bool,
}

impl Overworld {
    pub fn new(rom: &Rom, lay: &Layout, map: u8, x: i32, y: i32, facing: Dir, last_map: u8) -> Option<Overworld> {
        let loaded = LoadedMap::load(rom, lay, map)?;
        let mut ow = Overworld {
            map: loaded,
            player: Player { x, y, facing, step: None, walked: 0 },
            npcs: Vec::new(),
            last_map,
            flower_frame: 0,
            anim_c1: 0,
            anim_c2: 0,
            rng: 0x1234_5678 ^ (map as u32) << 8,
            ledges: data::ledges(rom, lay),
            pairs_land: data::tile_pair_collisions(rom, lay.tile_pair_collisions_land),
            step_out: false,
            ignore_input: 0,
        };
        ow.spawn_npcs(rom, lay);
        Some(ow)
    }

    fn random(&mut self) -> u32 {
        let mut x = self.rng;
        x ^= x << 13;
        x ^= x >> 17;
        x ^= x << 5;
        self.rng = x;
        x
    }

    fn spawn_npcs(&mut self, rom: &Rom, lay: &Layout) {
        let hidden = data::hidden_at_start(rom, lay, self.map.header.id);
        self.npcs = self
            .map
            .objects
            .objects
            .iter()
            .filter(|o| !hidden.contains(&o.index))
            .map(|o| {
                let facing = match (o.movement, o.range) {
                    (STAY, 0xd0) => Dir::Down,
                    (STAY, 0xd1) => Dir::Up,
                    (STAY, 0xd2) => Dir::Left,
                    (STAY, 0xd3) => Dir::Right,
                    _ => Dir::Down,
                };
                Npc {
                    index: o.index,
                    sprite: o.sprite,
                    x: o.x as i32,
                    y: o.y as i32,
                    facing,
                    movement: o.movement,
                    range: o.range,
                    text: o.text,
                    kind: o.kind,
                    step: None,
                    walked: 0,
                    delay: 0,
                    disp_x: 8,
                    disp_y: 8,
                    half: false,
                }
            })
            .collect();
    }

    /// Changes map, keeping the person's facing.
    pub fn enter_map(&mut self, rom: &Rom, lay: &Layout, map: u8, x: i32, y: i32) -> bool {
        let Some(loaded) = LoadedMap::load(rom, lay, map) else { return false };
        self.map = loaded;
        self.player.x = x;
        self.player.y = y;
        self.player.step = None;
        self.spawn_npcs(rom, lay);
        true
    }

    fn is_outside(&self, lay: &Layout) -> bool {
        self.map.header.tileset == lay.overworld || self.map.header.tileset == lay.plateau
    }

    /// Goes through a warp, the way WarpFound does.
    pub fn take_warp(&mut self, rom: &Rom, lay: &Layout, map: u8, warp: u8) -> bool {
        let dest = if map == lay.last_map { self.last_map } else { map };
        if self.is_outside(lay) {
            self.last_map = self.map.header.id;
        }
        let Some(h) = data::map_header(rom, lay, dest) else { return false };
        let objs = data::map_objects(rom, &h);
        let Some(w) = objs.warps.get(warp as usize).copied() else { return false };
        if !self.enter_map(rom, lay, dest, w.x as i32, w.y as i32) {
            return false;
        }
        // Coming out of a door: the original steps you down out of it.
        let tile = self.map.step_tile(self.player.x, self.player.y);
        self.step_out = self.map.doors.contains(&tile);
        self.ignore_input = 30;
        true
    }

    fn npc_at(&self, x: i32, y: i32) -> Option<usize> {
        self.npcs.iter().position(|n| n.occupies(x, y))
    }

    fn warp_at(&self, x: i32, y: i32) -> Option<data::Warp> {
        self.map.objects.warps.iter().copied().find(|w| w.x as i32 == x && w.y as i32 == y)
    }

    /// ExtraWarpCheck: can a blocked step or a held direction take this warp?
    fn extra_warp_check(&self, rom: &Rom, lay: &Layout) -> bool {
        let id = self.map.header.id;
        let ts = self.map.header.tileset;
        let edge = id == lay.ss_anne_3f
            || !(id == lay.rocket_hideout_b1f
                || id == lay.rocket_hideout_b2f
                || id == lay.rocket_hideout_b4f
                || id == lay.rock_tunnel_1f
                || ts == lay.overworld
                || ts == lay.ship
                || ts == lay.ship_port
                || ts == lay.plateau);
        let p = &self.player;
        if edge {
            match p.facing {
                Dir::Down => p.y == self.map.height_steps() - 1,
                Dir::Up => p.y == 0,
                Dir::Left => p.x == 0,
                Dir::Right => p.x == self.map.width_steps() - 1,
            }
        } else {
            let (dx, dy) = p.facing.delta();
            let front = self.map.step_tile(p.x + dx, p.y + dy);
            if id == lay.ss_anne_bow {
                return front == 0x15;
            }
            data::carpet_tiles(rom, lay, p.facing).contains(&front)
        }
    }

    /// One frame (1/60 s) of the overworld.
    pub fn frame(&mut self, rom: &Rom, lay: &Layout, input: &Input, motion: &Motion, frozen: bool) -> Event {
        let (mut c1, mut c2) = (self.anim_c1, self.anim_c2);
        self.map.animate(&mut c1, &mut c2);
        self.anim_c1 = c1;
        self.anim_c2 = c2;
        if let Some(f) = self.map.flower_frame.take() {
            self.flower_frame = f;
        }
        if frozen {
            return Event::None;
        }
        if motion.npcs_wander {
            self.update_npcs();
        }

        if let Some(mut s) = self.player.step {
            let speed = if s.jump {
                1
            } else if motion.run_enabled && input.b_held {
                motion.run_speed.max(1)
            } else {
                motion.walk_speed.max(1)
            } as u16;
            let adv = speed.min(s.total - s.done);
            s.done += adv;
            self.player.walked += adv as u32;
            if s.done < s.total {
                self.player.step = Some(s);
                return Event::None;
            }
            // Step finished.
            let (dx, dy) = s.dir.delta();
            let n = if s.jump { 2 } else { 1 };
            self.player.x += dx * n;
            self.player.y += dy * n;
            self.player.step = None;
            if let Some(ev) = self.after_step(rom, lay, input) {
                return ev;
            }
        }

        if self.ignore_input > 0 {
            self.ignore_input -= 1;
        }
        if self.step_out {
            self.step_out = false;
            self.player.facing = Dir::Down;
            self.player.step = Some(Step { dir: Dir::Down, done: 0, total: 16, jump: false });
            return Event::None;
        }

        if self.ignore_input > 0 {
            return Event::None;
        }
        if input.a_pressed {
            if let Some(ev) = self.interact() {
                return ev;
            }
        }

        if let Some(dir) = input.held_dir {
            self.player.facing = dir;
            return self.try_move(rom, lay, dir);
        }
        Event::None
    }

    fn after_step(&mut self, rom: &Rom, lay: &Layout, input: &Input) -> Option<Event> {
        // Over the edge into the next map?
        let (x, y) = (self.player.x, self.player.y);
        if !self.map.inside(x, y) {
            let dir = if y < 0 {
                Dir::Up
            } else if y >= self.map.height_steps() {
                Dir::Down
            } else if x < 0 {
                Dir::Left
            } else {
                Dir::Right
            };
            if let Some(n) = self.map.neighbors.iter().find(|n| n.conn.dir == dir) {
                let (map, off) = (n.conn.map, n.conn.offset);
                let (tw, th) = (n.header.width as i32 * 2, n.header.height as i32 * 2);
                let (nx, ny) = match dir {
                    Dir::Up => (x - 2 * off, th - 1),
                    Dir::Down => (x - 2 * off, 0),
                    Dir::Left => (tw - 1, y - 2 * off),
                    Dir::Right => (0, y - 2 * off),
                };
                self.enter_map(rom, lay, map, nx, ny);
            }
            return None;
        }
        // CheckWarpsNoCollision.
        if let Some(w) = self.warp_at(x, y) {
            let tile = self.map.step_tile(x, y);
            if self.map.doors.contains(&tile) || self.map.warp_tiles.contains(&tile) {
                return Some(Event::Warp { map: w.dest_map, warp: w.dest_warp });
            }
            if input.held_dir.is_some() && self.extra_warp_check(rom, lay) {
                return Some(Event::Warp { map: w.dest_map, warp: w.dest_warp });
            }
        }
        None
    }

    fn try_move(&mut self, rom: &Rom, lay: &Layout, dir: Dir) -> Event {
        let (x, y) = (self.player.x, self.player.y);
        let (dx, dy) = dir.delta();
        let here = self.map.step_tile(x, y);
        let front = self.map.step_tile(x + dx, y + dy);
        // A ledge you can hop down.
        if self.ledges.iter().any(|&(f, stand, ledge)| f == dir && stand == here && ledge == front) {
            self.player.step = Some(Step { dir, done: 0, total: 32, jump: true });
            return Event::None;
        }
        let ts = self.map.header.tileset;
        let pair_blocked = self
            .pairs_land
            .iter()
            .any(|&(t, a, b)| t == ts && ((a == here && b == front) || (b == here && a == front)));
        let blocked = !self.map.passable(front) || pair_blocked || self.npc_at(x + dx, y + dy).is_some();
        if !blocked {
            self.player.step = Some(Step { dir, done: 0, total: 16, jump: false });
            return Event::None;
        }
        // Blocked: pushing into the edge from a warp square takes the warp.
        if let Some(w) = self.warp_at(x, y) {
            if self.extra_warp_check(rom, lay) {
                return Event::Warp { map: w.dest_map, warp: w.dest_warp };
            }
        }
        Event::Bump
    }

    fn interact(&mut self) -> Option<Event> {
        let p = &self.player;
        let (dx, dy) = p.facing.delta();
        let (fx, fy) = (p.x + dx, p.y + dy);
        let front_tile = self.map.step_tile(fx, fy);
        // Counters: talk to whoever stands behind them.
        let mut targets = vec![(fx, fy)];
        if self.map.tileset.counters.contains(&front_tile) {
            targets.push((fx + dx, fy + dy));
        }
        for (tx, ty) in targets {
            if let Some(i) = self.npcs.iter().position(|n| n.x == tx && n.y == ty && n.step.is_none()) {
                // Everyone turns to face you when you talk to them, as in the original.
                self.npcs[i].facing = self.player.facing.opposite();
                let n = &self.npcs[i];
                return Some(Event::Talk { text: n.text, npc: Some(i) });
            }
        }
        // Signs are read facing them from below... or any side, as the original allows.
        if let Some(s) = self.map.objects.signs.iter().find(|s| s.x as i32 == fx && s.y as i32 == fy) {
            return Some(Event::Talk { text: s.text, npc: None });
        }
        None
    }

    fn update_npcs(&mut self) {
        let (px, py) = (self.player.x, self.player.y);
        for i in 0..self.npcs.len() {
            // People only move at half the player's speed, every other frame.
            self.npcs[i].half = !self.npcs[i].half;
            if !self.npcs[i].half {
                continue;
            }
            if let Some(mut s) = self.npcs[i].step {
                s.done += 1;
                self.npcs[i].walked += 1;
                if s.done >= s.total {
                    let (dx, dy) = s.dir.delta();
                    self.npcs[i].x += dx;
                    self.npcs[i].y += dy;
                    self.npcs[i].step = None;
                    self.npcs[i].delay = (self.random() & 0x7f) as u16;
                } else {
                    self.npcs[i].step = Some(s);
                }
                continue;
            }
            if self.npcs[i].movement != WALK {
                continue;
            }
            // Only people on the original-sized screen move.
            let sx = self.npcs[i].x - px + 4;
            let sy = self.npcs[i].y - py + 4;
            if !(0..10).contains(&sx) || !(0..9).contains(&sy) {
                continue;
            }
            if self.npcs[i].delay > 0 {
                self.npcs[i].delay -= 1;
                continue;
            }
            let r = self.random();
            let range = self.npcs[i].range;
            let dir = match range {
                0xd0 => Dir::Down,
                0xd1 => Dir::Up,
                0xd2 => Dir::Left,
                0xd3 => Dir::Right,
                1 => {
                    if r & 0x80 == 0 {
                        Dir::Down
                    } else {
                        Dir::Up
                    }
                }
                2 => {
                    if r & 0x80 == 0 {
                        Dir::Left
                    } else {
                        Dir::Right
                    }
                }
                _ => Dir::from_u8((r >> 6) as u8),
            };
            self.npcs[i].facing = dir;
            let (dx, dy) = dir.delta();
            let (nx, ny) = (self.npcs[i].x + dx, self.npcs[i].y + dy);
            let (nsx, nsy) = (sx + dx, sy + dy);
            let n = &self.npcs[i];
            let mut ok = self.map.passable(self.map.step_tile(nx, ny))
                && (0..9).contains(&nsx)
                && (0..8).contains(&nsy);
            // The original's wander limits, kept as they were.
            let mut disp_y = n.disp_y;
            let mut disp_x = n.disp_x;
            if dy < 0 {
                if disp_y == 0 {
                    ok = false;
                }
                disp_y -= 1;
            } else if dy > 0 {
                disp_y += 1;
                if disp_y < 5 {
                    ok = false;
                }
            }
            if dx < 0 {
                if disp_x == 0 {
                    ok = false;
                }
                disp_x -= 1;
            } else if dx > 0 {
                disp_x += 1;
            }
            let player_there = (nx == px && ny == py)
                || self.player.step.is_some_and(|s| {
                    let (pdx, pdy) = s.dir.delta();
                    nx == px + pdx && ny == py + pdy
                });
            let other_there = self.npcs.iter().enumerate().any(|(j, o)| j != i && o.occupies(nx, ny));
            if ok && !player_there && !other_there {
                let n = &mut self.npcs[i];
                n.disp_x = disp_x;
                n.disp_y = disp_y;
                n.step = Some(Step { dir, done: 0, total: 16, jump: false });
            } else {
                self.npcs[i].delay = (self.random() & 0x7f) as u16;
            }
        }
    }
}
