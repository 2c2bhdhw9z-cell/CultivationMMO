//! Pictures: decoding the cartridge's tile graphics and drawing into a frame.

/// One 8x8 tile, each pixel a colour number 0 to 3.
pub type Tile = [u8; 64];

pub const BLANK_TILE: Tile = [0; 64];

/// A Game Boy 2-bits-per-pixel tile (16 bytes).
pub fn decode_2bpp(b: &[u8]) -> Tile {
    let mut t = BLANK_TILE;
    for row in 0..8 {
        let lo = *b.get(row * 2).unwrap_or(&0);
        let hi = *b.get(row * 2 + 1).unwrap_or(&0);
        for col in 0..8 {
            let bit = 7 - col;
            t[row * 8 + col] = ((lo >> bit) & 1) | (((hi >> bit) & 1) << 1);
        }
    }
    t
}

/// A 1-bit-per-pixel tile (8 bytes), as the game copies it: set bits become colour 3.
pub fn decode_1bpp(b: &[u8]) -> Tile {
    let mut t = BLANK_TILE;
    for row in 0..8 {
        let v = *b.get(row).unwrap_or(&0);
        for col in 0..8 {
            t[row * 8 + col] = if (v >> (7 - col)) & 1 == 1 { 3 } else { 0 };
        }
    }
    t
}

pub fn decode_tiles_2bpp(bytes: &[u8]) -> Vec<Tile> {
    bytes.chunks(16).map(decode_2bpp).collect()
}

/// A colour packed for the phone: bytes R, G, B, 255 in memory.
pub const fn rgb(r: u8, g: u8, b: u8) -> u32 {
    u32::from_le_bytes([r, g, b, 255])
}

/// A Super Game Boy / Game Boy Color 15-bit colour.
pub fn rgb555(c: u16) -> u32 {
    let ch = |v: u16| -> u8 {
        let v = (v & 31) as u8;
        (v << 3) | (v >> 2)
    };
    rgb(ch(c), ch(c >> 5), ch(c >> 10))
}

pub type Palette = [u32; 4];

/// The picture the game draws into, RGBA.
pub struct Frame {
    pub width: usize,
    pub height: usize,
    pub pixels: Vec<u32>,
}

impl Frame {
    pub fn new(width: usize, height: usize) -> Frame {
        Frame { width, height, pixels: vec![rgb(255, 255, 255); width * height] }
    }

    pub fn resize(&mut self, width: usize, height: usize) {
        if self.width != width || self.height != height {
            *self = Frame::new(width, height);
        }
    }

    pub fn fill(&mut self, c: u32) {
        self.pixels.iter_mut().for_each(|p| *p = c);
    }

    /// Draws a tile with its colour numbers turned into colours by `pal`. `None` = see-through.
    pub fn blit(&mut self, x: i32, y: i32, tile: &Tile, map: impl Fn(u8) -> Option<u32>, flip_x: bool) {
        for row in 0..8i32 {
            let py = y + row;
            if py < 0 || py >= self.height as i32 {
                continue;
            }
            for col in 0..8i32 {
                let px = x + col;
                if px < 0 || px >= self.width as i32 {
                    continue;
                }
                let src_col = if flip_x { 7 - col } else { col };
                if let Some(c) = map(tile[(row * 8 + src_col) as usize]) {
                    self.pixels[py as usize * self.width + px as usize] = c;
                }
            }
        }
    }

    /// Blends the whole frame toward `c` by `amount` (0 = unchanged, 255 = all `c`).
    pub fn fade_toward(&mut self, c: u32, amount: u8) {
        if amount == 0 {
            return;
        }
        let [cr, cg, cb, _] = c.to_le_bytes();
        let a = amount as u32;
        for p in self.pixels.iter_mut() {
            let [r, g, b, _] = p.to_le_bytes();
            let mix = |s: u8, d: u8| ((s as u32 * (255 - a) + d as u32 * a) / 255) as u8;
            *p = rgb(mix(r, cr), mix(g, cg), mix(b, cb));
        }
    }
}
