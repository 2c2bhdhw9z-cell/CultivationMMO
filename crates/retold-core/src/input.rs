//! Buttons, in the Game Boy's own bit order.

use crate::gen1::data::Dir;

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct Buttons(pub u32);

impl Buttons {
    pub const A: Buttons = Buttons(1);
    pub const B: Buttons = Buttons(2);
    pub const SELECT: Buttons = Buttons(4);
    pub const START: Buttons = Buttons(8);
    pub const RIGHT: Buttons = Buttons(16);
    pub const LEFT: Buttons = Buttons(32);
    pub const UP: Buttons = Buttons(64);
    pub const DOWN: Buttons = Buttons(128);

    pub const fn empty() -> Buttons {
        Buttons(0)
    }

    pub fn contains(self, b: Buttons) -> bool {
        self.0 & b.0 == b.0
    }

    pub fn intersects(self, b: Buttons) -> bool {
        self.0 & b.0 != 0
    }

    /// The direction held, checked in the original's order: down, up, left, right.
    pub fn direction(self) -> Option<Dir> {
        if self.contains(Buttons::DOWN) {
            Some(Dir::Down)
        } else if self.contains(Buttons::UP) {
            Some(Dir::Up)
        } else if self.contains(Buttons::LEFT) {
            Some(Dir::Left)
        } else if self.contains(Buttons::RIGHT) {
            Some(Dir::Right)
        } else {
            None
        }
    }
}

impl std::ops::BitOr for Buttons {
    type Output = Buttons;
    fn bitor(self, o: Buttons) -> Buttons {
        Buttons(self.0 | o.0)
    }
}

impl std::ops::BitAnd for Buttons {
    type Output = Buttons;
    fn bitand(self, o: Buttons) -> Buttons {
        Buttons(self.0 & o.0)
    }
}

impl std::ops::Not for Buttons {
    type Output = Buttons;
    fn not(self) -> Buttons {
        Buttons(!self.0)
    }
}
