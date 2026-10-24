//! Kiro's desk tool. Plays a scripted list of inputs and writes what the screen shows.
//!
//!   retold-cli <cartridge> <out.png> [view WxH] [commands...]
//!
//! Commands: `goto:<map id>`, `wait:<frames>`, `hold:<buttons>:<frames>` with buttons from
//! `udlrabsS` (up down left right A B select Start), `tap:<buttons>`, `shot:<file.png>`,
//! `where` (prints the map and position).

use retold_core::gen1::game::Game;
use retold_core::input::Buttons;
use retold_core::rom::identify;

fn buttons(s: &str) -> Buttons {
    let mut b = Buttons::empty();
    for c in s.chars() {
        b = b | match c {
            'u' => Buttons::UP,
            'd' => Buttons::DOWN,
            'l' => Buttons::LEFT,
            'r' => Buttons::RIGHT,
            'a' => Buttons::A,
            'b' => Buttons::B,
            's' => Buttons::SELECT,
            'S' => Buttons::START,
            _ => Buttons::empty(),
        };
    }
    b
}

fn write_png(game: &Game, path: &str, scale: usize) {
    let f = &game.frame;
    let (w, h) = (f.width * scale, f.height * scale);
    let mut data = Vec::with_capacity(w * h * 3);
    for y in 0..h {
        for x in 0..w {
            let [r, g, b, _] = f.pixels[(y / scale) * f.width + x / scale].to_le_bytes();
            data.extend_from_slice(&[r, g, b]);
        }
    }
    let file = std::fs::File::create(path).expect("create png");
    let mut enc = png::Encoder::new(std::io::BufWriter::new(file), w as u32, h as u32);
    enc.set_color(png::ColorType::Rgb);
    enc.set_depth(png::BitDepth::Eight);
    enc.write_header().unwrap().write_image_data(&data).unwrap();
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    if args.len() < 3 {
        eprintln!("usage: retold-cli <cartridge> <out.png> [WxH] [commands...]");
        std::process::exit(2);
    }
    let data = std::fs::read(&args[1]).expect("read cartridge");
    let (id, _) = identify(&data).expect("not a supported cartridge");
    let mut game = Game::new(id, data).expect("start game");
    let mut rest = &args[3..];
    if let Some(v) = rest.first().and_then(|s| s.split_once('x')) {
        if let (Ok(w), Ok(h)) = (v.0.parse(), v.1.parse()) {
            game.set_view(w, h);
            rest = &rest[1..];
        }
    }
    let mut held = Buttons::empty();
    for cmd in rest {
        let parts: Vec<&str> = cmd.split(':').collect();
        match parts[0] {
            "goto" => {
                let m: u8 = parts[1].parse().expect("map id");
                assert!(game.debug_goto(m), "no map {m}");
            }
            "wait" => {
                for _ in 0..parts[1].parse::<u32>().unwrap() {
                    game.step(held);
                }
            }
            "hold" => {
                held = buttons(parts[1]);
                for _ in 0..parts[2].parse::<u32>().unwrap() {
                    game.step(held);
                }
                held = Buttons::empty();
                game.step(held);
            }
            "tap" => {
                game.step(buttons(parts[1]));
                game.step(Buttons::empty());
            }
            "shot" => write_png(&game, parts[1], 2),
            "tile" => {
                let (x, y): (i32, i32) = (parts[1].parse().unwrap(), parts[2].parse().unwrap());
                let m = &game.ow.map;
                let t = m.step_tile(x, y);
                println!("step ({x},{y}) tile {t:#04x} passable={} list={:02x?}", m.passable(t), m.passable);
            }
            "where" => {
                let p = &game.ow.player;
                println!("{} ({}) x={} y={} facing={:?}", game.map_name(), game.map_id(), p.x, p.y, p.facing);
            }
            other => panic!("unknown command {other}"),
        }
    }
    write_png(&game, &args[2], 2);
}
