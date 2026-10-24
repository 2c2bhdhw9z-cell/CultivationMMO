//! Drawing the overworld and the text box ourselves, from the decoded map and sprites.
//! The view can be wider and taller than the original 20x18 tiles.

use super::textbox::{TextBox, LINE_LEN};
use super::world::Overworld;
use super::data::{Dir, SpriteFrame};
use crate::gfx::{Frame, Palette, Tile};

/// Pictures the renderer needs that aren't part of a map.
pub struct Art<'a> {
    pub sprite: &'a dyn Fn(u8) -> Option<&'a [SpriteFrame]>,
    pub font: &'a [Tile],
    pub box_tiles: &'a [Tile],
}

/// Where the player stands on screen, in tiles: the original puts them at column 8, row 8 of
/// 20x18, and wider views keep them centred the same way.
fn player_tile(view_w: usize, view_h: usize) -> (i32, i32) {
    (view_w as i32 / 2 - 2, view_h as i32 / 2 - 1)
}

pub fn draw_overworld(frame: &mut Frame, ow: &Overworld, art: &Art, pal: &Palette, view_w: usize, view_h: usize) {
    let (ptx, pty) = player_tile(view_w, view_h);
    let (pox, poy) = ow.player.pixel_offset();
    let cam_x = ow.player.x * 16 + pox - ptx * 8;
    let cam_y = ow.player.y * 16 + poy - pty * 8;
    let map = &ow.map;
    let grass = map.tileset.grass;

    // Background, remembering where grass covers people's feet.
    let mut grass_mask = vec![false; frame.width * frame.height];
    let first_tx = cam_x.div_euclid(8);
    let first_ty = cam_y.div_euclid(8);
    let fine_x = cam_x.rem_euclid(8);
    let fine_y = cam_y.rem_euclid(8);
    for sy in 0..=view_h as i32 {
        for sx in 0..=view_w as i32 {
            let tile_id = map.tile_at(first_tx + sx, first_ty + sy);
            let tile = map.tiles.get(tile_id as usize).copied().unwrap_or([0; 64]);
            let x0 = sx * 8 - fine_x;
            let y0 = sy * 8 - fine_y;
            frame.blit(x0, y0, &tile, |c| Some(pal[c as usize]), false);
            if tile_id == grass {
                for row in 0..8 {
                    for col in 0..8 {
                        let (x, y) = (x0 + col, y0 + row);
                        if x >= 0 && y >= 0 && (x as usize) < frame.width && (y as usize) < frame.height {
                            grass_mask[y as usize * frame.width + x as usize] = tile[(row * 8 + col) as usize] != 0;
                        }
                    }
                }
            }
        }
    }

    // People, then the player on top (the original's sprite order).
    for n in &ow.npcs {
        let (ox, oy) = n.pixel_offset();
        let x = n.x * 16 + ox - cam_x;
        let y = n.y * 16 + oy - cam_y - 4;
        if x < -16 || y < -16 || x > frame.width as i32 || y > frame.height as i32 {
            continue;
        }
        if let Some(frames) = (art.sprite)(n.sprite) {
            draw_person(frame, frames, n.facing, n.anim_frame(), x, y, pal, &grass_mask);
        }
    }
    if let Some(frames) = (art.sprite)(1) {
        let x = ptx * 8;
        let y = pty * 8 - 4 - ow.player.hop();
        let anim = if ow.player.step.is_some_and(|s| s.jump) { 0 } else { ow.player.anim_frame() };
        draw_person(frame, frames, ow.player.facing, anim, x, y, pal, &grass_mask);
    }
}

#[allow(clippy::too_many_arguments)]
fn draw_person(
    frame: &mut Frame,
    frames: &[SpriteFrame],
    facing: Dir,
    anim: u8,
    x: i32,
    y: i32,
    pal: &Palette,
    grass_mask: &[bool],
) {
    let (index, flip) = if frames.len() < 6 {
        (0, false)
    } else {
        let walking = anim % 2 == 1;
        match facing {
            Dir::Down => (if walking { 3 } else { 0 }, anim == 3),
            Dir::Up => (if walking { 4 } else { 1 }, anim == 3),
            Dir::Left => (if walking { 5 } else { 2 }, false),
            Dir::Right => (if walking { 5 } else { 2 }, true),
        }
    };
    let Some(f) = frames.get(index) else { return };
    // Sprite colours go through the original's object palette: 0 is see-through,
    // 1 shows as the lightest shade, 2 as light, 3 as darkest.
    let obj = |c: u8| -> Option<u32> {
        match c {
            0 => None,
            1 => Some(pal[0]),
            2 => Some(pal[1]),
            _ => Some(pal[3]),
        }
    };
    let quads = [(0, 0, 0), (1, 8, 0), (2, 0, 8), (3, 8, 8)];
    for (i, qx, qy) in quads {
        let qx = if flip { 8 - qx } else { qx };
        let tile = &f[i];
        let (tx, ty) = (x + qx, y + qy);
        let bottom = qy == 8;
        for row in 0..8i32 {
            let py = ty + row;
            if py < 0 || py >= frame.height as i32 {
                continue;
            }
            for col in 0..8i32 {
                let px = tx + col;
                if px < 0 || px >= frame.width as i32 {
                    continue;
                }
                let src = if flip { 7 - col } else { col };
                let idx = py as usize * frame.width + px as usize;
                if bottom && grass_mask[idx] {
                    continue;
                }
                if let Some(c) = obj(tile[(row * 8 + src) as usize]) {
                    frame.pixels[idx] = c;
                }
            }
        }
    }
}

/// The text box: 20 tiles wide and 6 high at the bottom, centred in wider views.
pub fn draw_textbox(frame: &mut Frame, tb: &TextBox, art: &Art, pal: &Palette, view_w: usize, view_h: usize) {
    let x0 = (view_w as i32 - 20) / 2;
    let y0 = view_h as i32 - 6;
    let tile_for = |id: u8| -> Tile {
        match id {
            0x60..=0x7f => art.box_tiles.get(id as usize - 0x60).copied().unwrap_or([0; 64]),
            0x80..=0xff => art.font.get(id as usize - 0x80).copied().unwrap_or([0; 64]),
            _ => [0; 64],
        }
    };
    let put = |frame: &mut Frame, col: i32, row: i32, id: u8| {
        let t = tile_for(id);
        frame.blit((x0 + col) * 8, (y0 + row) * 8, &t, |c| Some(pal[c as usize]), false);
    };
    for row in 0..6 {
        for col in 0..20 {
            let id = match (row, col) {
                (0, 0) => 0x79,
                (0, 19) => 0x7b,
                (5, 0) => 0x7d,
                (5, 19) => 0x7e,
                (0, _) | (5, _) => 0x7a,
                (_, 0) | (_, 19) => 0x7c,
                _ => 0x7f,
            };
            put(frame, col, row, id);
        }
    }
    for (line, row) in [(0usize, 2), (1, 4)] {
        for col in 0..LINE_LEN {
            put(frame, 1 + col as i32, row, tb.lines[line][col]);
        }
    }
    if tb.arrow_visible() {
        put(frame, 18, 4, 0xee);
    }
}
