//! Desk check: is the cartridge inside this file (or zip) found?
fn main() {
    let d = std::fs::read(std::env::args().nth(1).expect("file")).expect("read");
    match retold_core::archive::find_cartridge(&d) {
        Some((g, c)) => println!("found {} ({} bytes)", g.name(), c.len()),
        None => println!("nothing found"),
    }
}
