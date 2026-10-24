//! Retold's engine. The classic Pokémon games rebuilt natively from the player's own
//! cartridge file: we read the game's tables and draw and run everything ourselves.

pub mod archive;
pub mod gen1;
pub mod gfx;
pub mod input;
pub mod options;
pub mod rom;

pub use rom::{identify, GameId};

/// A running game of any generation.
pub enum Session {
    Gen1(Box<gen1::game::Game>),
}

impl Session {
    pub fn new(data: Vec<u8>) -> Result<Session, String> {
        let (id, _rev) = identify(&data).ok_or("That file isn't one of the supported cartridges")?;
        match id {
            GameId::Red | GameId::Blue | GameId::Yellow => {
                Ok(Session::Gen1(Box::new(gen1::game::Game::new(id, data)?)))
            }
            _ => Err(format!("{} is coming later", id.name())),
        }
    }

    pub fn game(&self) -> GameId {
        match self {
            Session::Gen1(g) => g.id,
        }
    }

    pub fn step(&mut self, held: input::Buttons) {
        match self {
            Session::Gen1(g) => g.step(held),
        }
    }

    pub fn set_view(&mut self, w: usize, h: usize) {
        match self {
            Session::Gen1(g) => g.set_view(w, h),
        }
    }

    pub fn frame(&self) -> &gfx::Frame {
        match self {
            Session::Gen1(g) => &g.frame,
        }
    }

    pub fn options_mut(&mut self) -> &mut options::Options {
        match self {
            Session::Gen1(g) => &mut g.options,
        }
    }

    pub fn redraw(&mut self) {
        match self {
            Session::Gen1(g) => g.render(),
        }
    }

    pub fn save(&self) -> Vec<u8> {
        match self {
            Session::Gen1(g) => g.save(),
        }
    }

    pub fn load(&mut self, data: &[u8]) -> bool {
        match self {
            Session::Gen1(g) => g.load(data),
        }
    }
}
