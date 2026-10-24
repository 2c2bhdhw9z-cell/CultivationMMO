//! Reading Red, Blue and Yellow's tables from the cartridge: maps, people, tilesets, sprites,
//! colours. The layouts follow the pret disassembly's documentation of the original data.

use super::layout::Layout;
use crate::gfx::{decode_1bpp, decode_2bpp, rgb555, Palette, Tile};
use crate::rom::{FarPtr, Rom};

/// Facing and moving directions, in the game's own order.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Dir {
    Down = 0,
    Up = 1,
    Left = 2,
    Right = 3,
}

impl Dir {
    pub fn delta(self) -> (i32, i32) {
        match self {
            Dir::Down => (0, 1),
            Dir::Up => (0, -1),
            Dir::Left => (-1, 0),
            Dir::Right => (1, 0),
        }
    }

    pub fn opposite(self) -> Dir {
        match self {
            Dir::Down => Dir::Up,
            Dir::Up => Dir::Down,
            Dir::Left => Dir::Right,
            Dir::Right => Dir::Left,
        }
    }

    pub fn from_u8(v: u8) -> Dir {
        match v & 3 {
            0 => Dir::Down,
            1 => Dir::Up,
            2 => Dir::Left,
            _ => Dir::Right,
        }
    }
}

/// A link to the next map across one edge. `offset` is how far (in blocks) the other map is
/// shifted along that edge, as in the original's connection data.
#[derive(Clone, Copy, Debug)]
pub struct Connection {
    pub dir: Dir,
    pub map: u8,
    pub offset: i32,
}

#[derive(Clone, Debug)]
pub struct MapHeader {
    pub id: u8,
    pub bank: u8,
    pub tileset: u8,
    pub height: u8,
    pub width: u8,
    pub blocks: FarPtr,
    pub texts: FarPtr,
    pub script: FarPtr,
    pub connections: Vec<Connection>,
    pub objects: FarPtr,
}

pub fn map_header(rom: &Rom, lay: &Layout, id: u8) -> Option<MapHeader> {
    if id >= lay.num_maps {
        return None;
    }
    let bank = rom.byte(lay.map_header_banks.offset(id as u16));
    let p = rom.ptr(lay.map_header_pointers.offset(id as u16 * 2), bank);
    let tileset = rom.byte(p);
    let height = rom.byte(p.offset(1));
    let width = rom.byte(p.offset(2));
    if width == 0 || height == 0 || tileset >= lay.num_tilesets {
        return None;
    }
    let flags = rom.byte(p.offset(9));
    let mut q = p.offset(10);
    let mut connections = Vec::new();
    for (bit, dir) in [(8u8, Dir::Up), (4, Dir::Down), (2, Dir::Left), (1, Dir::Right)] {
        if flags & bit != 0 {
            let map = rom.byte(q);
            let y_align = rom.byte(q.offset(7)) as i8 as i32;
            let x_align = rom.byte(q.offset(8)) as i8 as i32;
            let offset = match dir {
                Dir::Up | Dir::Down => -x_align / 2,
                _ => -y_align / 2,
            };
            connections.push(Connection { dir, map, offset });
            q = q.offset(11);
        }
    }
    Some(MapHeader {
        id,
        bank,
        tileset,
        height,
        width,
        blocks: rom.ptr(p.offset(3), bank),
        texts: rom.ptr(p.offset(5), bank),
        script: rom.ptr(p.offset(7), bank),
        connections,
        objects: rom.ptr(q, bank),
    })
}

pub fn map_blocks(rom: &Rom, h: &MapHeader) -> Vec<u8> {
    rom.bytes(h.blocks, h.width as usize * h.height as usize)
}

#[derive(Clone, Copy, Debug)]
pub struct Warp {
    pub x: u8,
    pub y: u8,
    pub dest_warp: u8,
    pub dest_map: u8,
}

#[derive(Clone, Copy, Debug)]
pub struct Sign {
    pub x: u8,
    pub y: u8,
    pub text: u8,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ObjectKind {
    Person,
    Item(u8),
    Trainer { class: u8, number: u8 },
}

#[derive(Clone, Copy, Debug)]
pub struct ObjectEvent {
    /// 1-based, as the game numbers them.
    pub index: u8,
    pub sprite: u8,
    pub x: u8,
    pub y: u8,
    pub movement: u8,
    pub range: u8,
    pub text: u8,
    pub kind: ObjectKind,
}

pub const WALK: u8 = 0xfe;
pub const STAY: u8 = 0xff;

#[derive(Clone, Debug, Default)]
pub struct MapObjects {
    pub border: u8,
    pub warps: Vec<Warp>,
    pub signs: Vec<Sign>,
    pub objects: Vec<ObjectEvent>,
}

pub fn map_objects(rom: &Rom, h: &MapHeader) -> MapObjects {
    let mut p = h.objects;
    let mut take = || {
        let b = rom.byte(p);
        p = p.offset(1);
        b
    };
    let border = take();
    let mut warps = Vec::new();
    for _ in 0..take().min(32) {
        let (y, x, dest_warp, dest_map) = (take(), take(), take(), take());
        warps.push(Warp { x, y, dest_warp, dest_map });
    }
    let mut signs = Vec::new();
    for _ in 0..take().min(32) {
        let (y, x, text) = (take(), take(), take());
        signs.push(Sign { x, y, text });
    }
    let mut objects = Vec::new();
    for i in 0..take().min(32) {
        let sprite = take();
        let y = take().wrapping_sub(4);
        let x = take().wrapping_sub(4);
        let movement = take();
        let range = take();
        let t = take();
        let kind = if t & 0x40 != 0 {
            let class = take();
            let number = take();
            ObjectKind::Trainer { class, number }
        } else if t & 0x80 != 0 {
            ObjectKind::Item(take())
        } else {
            ObjectKind::Person
        };
        objects.push(ObjectEvent { index: i + 1, sprite, x, y, movement, range, text: t & 0x3f, kind });
    }
    MapObjects { border, warps, signs, objects }
}

/// Object numbers (1-based) that start the game hidden on this map.
pub fn hidden_at_start(rom: &Rom, lay: &Layout, map: u8) -> Vec<u8> {
    let table = lay.toggleable_object_map_pointers;
    let mut p = rom.ptr(table.offset(map as u16 * 2), table.bank);
    let mut out = Vec::new();
    for _ in 0..64 {
        if rom.byte(p) != map {
            break;
        }
        if rom.byte(p.offset(2)) == 0x11 {
            out.push(rom.byte(p.offset(1)));
        }
        p = p.offset(3);
    }
    out
}

#[derive(Clone, Debug)]
pub struct Tileset {
    pub id: u8,
    pub blocks: FarPtr,
    pub gfx: FarPtr,
    pub coll: FarPtr,
    pub counters: [u8; 3],
    pub grass: u8,
    pub anim: u8,
}

pub fn tileset(rom: &Rom, lay: &Layout, id: u8) -> Tileset {
    let p = lay.tilesets.offset(id as u16 * 12);
    let bank = rom.byte(p);
    Tileset {
        id,
        blocks: rom.ptr(p.offset(1), bank),
        gfx: rom.ptr(p.offset(3), bank),
        coll: rom.ptr(p.offset(5), bank),
        counters: [rom.byte(p.offset(7)), rom.byte(p.offset(8)), rom.byte(p.offset(9))],
        grass: rom.byte(p.offset(10)),
        anim: rom.byte(p.offset(11)),
    }
}

/// The tileset's 96 background tiles, as the game loads them into video memory.
pub fn tileset_tiles(rom: &Rom, t: &Tileset) -> Vec<Tile> {
    (0..0x60).map(|i| decode_2bpp(&rom.bytes(t.gfx.offset(i * 16), 16))).collect()
}

pub fn tileset_block(rom: &Rom, t: &Tileset, block: u8) -> [u8; 16] {
    let mut out = [0u8; 16];
    out.copy_from_slice(&rom.bytes(t.blocks.offset(block as u16 * 16), 16));
    out
}

/// One 16x16 sprite picture: top-left, top-right, bottom-left, bottom-right.
pub type SpriteFrame = [Tile; 4];

/// A person's pictures: standing down, up, left, then walking down, up, left. Objects that
/// never move (balls, boulders...) have just one picture.
pub fn sprite_frames(rom: &Rom, lay: &Layout, sprite: u8) -> Vec<SpriteFrame> {
    if sprite == 0 || sprite > lay.num_sprites {
        return Vec::new();
    }
    let e = lay.sprite_sheet_pointer_table.offset((sprite as u16 - 1) * 4);
    let addr = rom.word(e);
    let size = rom.byte(e.offset(2)) as u16;
    let bank = rom.byte(e.offset(3));
    let base = FarPtr::new(bank, addr);
    let frame = |at: FarPtr| -> SpriteFrame {
        let t = |i: u16| decode_2bpp(&rom.bytes(at.offset(i * 16), 16));
        [t(0), t(1), t(2), t(3)]
    };
    if size < 0xc0 || sprite >= lay.first_still_sprite {
        return vec![frame(base)];
    }
    let mut out = Vec::with_capacity(6);
    for half in [0u16, 0xc0] {
        for f in 0..3u16 {
            out.push(frame(base.offset(half + f * 0x40)));
        }
    }
    out
}

/// One of the Super Game Boy colour sets the game picks per area.
pub fn sgb_palette(rom: &Rom, lay: &Layout, id: u8) -> Palette {
    let p = lay.super_palettes.offset(id as u16 * 8);
    let mut pal = [0u32; 4];
    for (i, c) in pal.iter_mut().enumerate() {
        *c = rgb555(rom.word(p.offset(i as u16 * 2)));
    }
    pal
}

/// Which colour set an area uses, the way the original chooses it.
pub fn overworld_palette_id(lay: &Layout, map: u8, tileset: u8, last_map: u8) -> u8 {
    if tileset == lay.cemetery {
        return lay.pal_graymon;
    }
    if tileset == lay.cavern {
        return lay.pal_cave;
    }
    let area = if map < lay.first_indoor_map {
        map
    } else if map < lay.cerulean_cave_2f {
        last_map
    } else if map <= lay.cerulean_cave_1f {
        return lay.pal_cave;
    } else if map == lay.loreleis_room {
        return lay.pal_pallet;
    } else if map == lay.brunos_room {
        return lay.pal_cave;
    } else {
        last_map
    };
    if area < lay.num_city_maps {
        area + 1
    } else {
        lay.pal_route
    }
}

/// The 128 letters and symbols, loaded the way the game loads them (tiles $80-$FF).
pub fn font_tiles(rom: &Rom, lay: &Layout) -> Vec<Tile> {
    (0..128).map(|i| decode_1bpp(&rom.bytes(lay.font_graphics.offset(i * 8), 8))).collect()
}

/// Text box borders and symbols (tiles $60-$7F).
pub fn textbox_tiles(rom: &Rom, lay: &Layout) -> Vec<Tile> {
    (0..32).map(|i| decode_2bpp(&rom.bytes(lay.text_box_graphics.offset(i * 16), 16))).collect()
}

/// Lists ending in `end`, read from a table of pointers indexed by tileset.
pub fn tile_list(rom: &Rom, at: FarPtr, end: u8) -> Vec<u8> {
    rom.until(at, end, 64)
}

/// Door tiles for a tileset (DoorTileIDPointers: tileset, pointer; ends with $FF).
pub fn door_tiles(rom: &Rom, lay: &Layout, tileset: u8) -> Vec<u8> {
    let mut p = lay.door_tile_id_pointers;
    for _ in 0..32 {
        let t = rom.byte(p);
        if t == 0xff {
            break;
        }
        if t == tileset {
            return tile_list(rom, rom.ptr(p.offset(1), p.bank), 0);
        }
        p = p.offset(3);
    }
    Vec::new()
}

/// Tiles that warp you as soon as you step on them (stairs, holes...).
pub fn warp_tiles(rom: &Rom, lay: &Layout, tileset: u8) -> Vec<u8> {
    let at = rom.ptr(lay.warp_tile_id_pointers.offset(tileset as u16 * 2), lay.warp_tile_id_pointers.bank);
    tile_list(rom, at, 0xff)
}

/// Warp mats you walk "into" at a map's edge, by facing direction.
pub fn carpet_tiles(rom: &Rom, lay: &Layout, facing: Dir) -> Vec<u8> {
    let t = lay.warp_tile_list_pointers;
    tile_list(rom, rom.ptr(t.offset(facing as u16 * 2), t.bank), 0xff)
}

/// Pairs of tiles you can't step between (cave ledges, water edges): (tileset, a, b).
pub fn tile_pair_collisions(rom: &Rom, at: FarPtr) -> Vec<(u8, u8, u8)> {
    let mut out = Vec::new();
    let mut p = at;
    for _ in 0..64 {
        let t = rom.byte(p);
        if t == 0xff {
            break;
        }
        out.push((t, rom.byte(p.offset(1)), rom.byte(p.offset(2))));
        p = p.offset(3);
    }
    out
}

/// Ledges: (facing, tile you stand on, ledge tile in front).
pub fn ledges(rom: &Rom, lay: &Layout) -> Vec<(Dir, u8, u8)> {
    let mut out = Vec::new();
    let mut p = lay.ledge_tiles;
    for _ in 0..64 {
        let f = rom.byte(p);
        if f == 0xff {
            break;
        }
        out.push((Dir::from_u8(f >> 2), rom.byte(p.offset(1)), rom.byte(p.offset(2))));
        p = p.offset(4);
    }
    out
}

/// The map's text pointer for a text number (1-based).
pub fn text_pointer(rom: &Rom, h: &MapHeader, text: u8) -> Option<FarPtr> {
    if text == 0 {
        return None;
    }
    Some(rom.ptr(h.texts.offset((text as u16 - 1) * 2), h.bank))
}
