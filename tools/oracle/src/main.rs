//! Plays the real cartridge on cBoy and writes the screen, to compare with Retold.
//!
//!   retold-oracle <cartridge> <out.png> <frames> [sgb] [from-to=mask/period ...]
//!
//! Masks are the Game Boy's button bits in hex: 01 A, 02 B, 04 Select, 08 Start, 10 Right,
//! 20 Left, 40 Up, 80 Down. `/period` taps instead of holding. `newgame` is a ready-made script
//! that gets Red or Blue from power-on to standing in the bedroom. `state=<file>` saves (or, if
//! it exists, loads) a cBoy save state, so later runs can start in the bedroom.
//! Prints the map number and position from the game's memory (Red/Blue addresses).

use continuum_gb::gb::{GameBoy, Model};

fn save_png(gb: &GameBoy, path: &str) {
    let f = std::fs::File::create(path).unwrap();
    let mut enc = png::Encoder::new(std::io::BufWriter::new(f), 320, 288);
    enc.set_color(png::ColorType::Rgb);
    let mut w = enc.write_header().unwrap();
    let frame = &gb.bus.ppu.frame;
    let mut px = Vec::with_capacity(320 * 288 * 3);
    for y in 0..288 {
        for x in 0..320 {
            let p = frame[(y / 2) * 160 + x / 2];
            px.extend_from_slice(&[(p >> 16) as u8, (p >> 8) as u8, p as u8]);
        }
    }
    w.write_image_data(&px).unwrap();
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let rom = std::fs::read(&args[1]).expect("cartridge");
    let frames: u32 = args[3].parse().unwrap();
    let mut rest: Vec<String> = args[4..].to_vec();
    let sgb = rest.first().is_some_and(|s| s == "sgb");
    if sgb {
        rest.remove(0);
    }
    let mut gb = if sgb { GameBoy::with_model(rom, Model::Sgb, None).unwrap() } else { GameBoy::new(rom).unwrap() };
    let mut script: Vec<(u32, u32, u8, u32)> = Vec::new();
    let mut state: Option<String> = None;
    for part in &rest {
        if part == "newgame" {
            script.push((200, 9000, 0x01, 40));
            script.push((200, 9000, 0x08, 400));
            continue;
        }
        if let Some(p) = part.strip_prefix("state=") {
            state = Some(p.to_string());
            continue;
        }
        let (range, mask) = part.split_once('=').unwrap();
        let (mask, period) = mask.split_once('/').map(|(m, p)| (m, p.parse().unwrap())).unwrap_or((mask, 1));
        let (from, to) = range.split_once('-').unwrap();
        script.push((from.parse().unwrap(), to.parse().unwrap(), u8::from_str_radix(mask, 16).unwrap(), period));
    }
    let mut loaded = false;
    if let Some(s) = &state {
        if let Ok(d) = std::fs::read(s) {
            gb.load_state(&d).expect("load state");
            loaded = true;
        }
    }
    for f in 0..frames {
        let mut b = 0;
        for &(from, to, mask, period) in &script {
            if f >= from && f <= to && (period == 1 || (f - from) % period < 3) {
                b |= mask;
            }
        }
        gb.bus.set_buttons(b);
        gb.run_frame();
        gb.bus.events.clear();
    }
    if let (Some(s), false) = (&state, loaded) {
        std::fs::write(s, gb.save_state()).unwrap();
    }
    let (map, y, x, dir) = (gb.bus.read(0xd35e), gb.bus.read(0xd361), gb.bus.read(0xd362), gb.bus.read(0xc109));
    println!("map {map} x={x} y={y} facing={dir:#04x}");
    save_png(&gb, &args[2]);
}
