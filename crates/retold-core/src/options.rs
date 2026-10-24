//! Player settings the engine follows. The app stores them and passes them in.

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ColorMode {
    /// The colours the game shows on a Super Game Boy, a different set per town.
    SuperGameBoy,
    /// The original Game Boy's green screen.
    Classic,
    /// Plain greys.
    Gray,
}

#[derive(Clone, Copy, Debug)]
pub struct Options {
    /// Frames between letters: 0 instant, 1 fast, 3 medium (the original's default), 5 slow.
    pub text_speed: u8,
    /// Holding A or B makes the text print faster, as in the original.
    pub hold_to_speed_text: bool,
    /// Pixels per frame when walking. 1 is the original speed.
    pub walk_speed: u8,
    /// Pixels per frame when running.
    pub run_speed: u8,
    /// Hold B to run.
    pub run_with_b: bool,
    pub colors: ColorMode,
    /// Fade the screen when going through doors.
    pub fades: bool,
}

impl Default for Options {
    fn default() -> Options {
        Options {
            text_speed: 3,
            hold_to_speed_text: true,
            walk_speed: 1,
            run_speed: 2,
            run_with_b: true,
            colors: ColorMode::SuperGameBoy,
            fades: true,
        }
    }
}

/// Option numbers the app uses to change one setting.
pub mod key {
    pub const TEXT_SPEED: u32 = 1;
    pub const HOLD_TO_SPEED_TEXT: u32 = 2;
    pub const WALK_SPEED: u32 = 3;
    pub const RUN_SPEED: u32 = 4;
    pub const RUN_WITH_B: u32 = 5;
    pub const COLORS: u32 = 6;
    pub const FADES: u32 = 7;
}

impl Options {
    pub fn set(&mut self, key: u32, value: i32) {
        let v = value.clamp(0, 255) as u8;
        match key {
            key::TEXT_SPEED => self.text_speed = v.min(9),
            key::HOLD_TO_SPEED_TEXT => self.hold_to_speed_text = v != 0,
            key::WALK_SPEED => self.walk_speed = v.clamp(1, 8),
            key::RUN_SPEED => self.run_speed = v.clamp(1, 8),
            key::RUN_WITH_B => self.run_with_b = v != 0,
            key::COLORS => {
                self.colors = match v {
                    1 => ColorMode::Classic,
                    2 => ColorMode::Gray,
                    _ => ColorMode::SuperGameBoy,
                }
            }
            key::FADES => self.fades = v != 0,
            _ => {}
        }
    }
}
