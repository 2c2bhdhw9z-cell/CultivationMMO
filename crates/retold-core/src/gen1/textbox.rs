//! The dialogue box at the bottom of the screen: prints letters one by one, waits for a
//! button, scrolls and clears the way the original does.

use super::text::TextOp;

pub const LINE_LEN: usize = 18;
const BLANK: u8 = 0x7f;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Wait {
    Para,
    Cont,
    Carry { arrow: bool },
    Close { arrow: bool },
    Pause(u8),
}

pub struct TextBox {
    ops: Vec<TextOp>,
    pos: usize,
    pub lines: [[u8; LINE_LEN]; 2],
    row: usize,
    col: usize,
    wait: Option<Wait>,
    delay: u8,
    blink: u16,
    pub closed: bool,
}

pub struct TextInput {
    pub pressed: bool,
    pub held: bool,
    /// Frames between letters: 0 prints a whole page at once.
    pub speed: u8,
}

impl TextBox {
    pub fn new(ops: Vec<TextOp>) -> TextBox {
        TextBox {
            ops,
            pos: 0,
            lines: [[BLANK; LINE_LEN]; 2],
            row: 0,
            col: 0,
            wait: None,
            delay: 0,
            blink: 0,
            closed: false,
        }
    }

    /// True if there is anything to show at all.
    pub fn has_words(&self) -> bool {
        self.ops.iter().any(|o| matches!(o, TextOp::Char(_)))
    }

    /// Whether the down arrow is showing this frame.
    pub fn arrow_visible(&self) -> bool {
        matches!(
            self.wait,
            Some(Wait::Para) | Some(Wait::Cont) | Some(Wait::Carry { arrow: true }) | Some(Wait::Close { arrow: true })
        ) && (self.blink / 16) % 2 == 0
    }

    fn scroll(&mut self) {
        self.lines[0] = self.lines[1];
        self.lines[1] = [BLANK; LINE_LEN];
        self.row = 1;
        self.col = 0;
    }

    pub fn frame(&mut self, input: &TextInput) {
        if self.closed {
            return;
        }
        self.blink = self.blink.wrapping_add(1);
        if let Some(w) = self.wait {
            match w {
                Wait::Pause(n) => {
                    self.wait = if n > 1 && !input.held { Some(Wait::Pause(n - 1)) } else { None };
                }
                _ if input.pressed => {
                    self.wait = None;
                    self.blink = 0;
                    match w {
                        Wait::Para => {
                            self.lines = [[BLANK; LINE_LEN]; 2];
                            self.row = 0;
                            self.col = 0;
                        }
                        Wait::Cont => self.scroll(),
                        Wait::Close { .. } => {
                            self.closed = true;
                            return;
                        }
                        _ => {}
                    }
                }
                _ => return,
            }
            if self.wait.is_some() {
                return;
            }
        }
        if self.delay > 0 && !input.held {
            self.delay -= 1;
            return;
        }
        // Print until one letter is out (or a whole page at instant speed), or we must wait.
        loop {
            let Some(op) = self.ops.get(self.pos).cloned() else {
                self.wait = Some(Wait::Close { arrow: false });
                return;
            };
            self.pos += 1;
            match op {
                TextOp::Char(c) => {
                    if self.col < LINE_LEN {
                        self.lines[self.row][self.col] = c;
                        self.col += 1;
                    }
                    if input.speed > 0 {
                        self.delay = input.speed - 1;
                        return;
                    }
                }
                TextOp::Line => {
                    if self.row == 1 {
                        self.scroll();
                    } else {
                        self.row = 1;
                        self.col = 0;
                    }
                }
                TextOp::Scroll => self.scroll(),
                TextOp::Para => {
                    self.wait = Some(Wait::Para);
                    return;
                }
                TextOp::Cont => {
                    self.wait = Some(Wait::Cont);
                    return;
                }
                TextOp::PromptButton => {
                    self.wait = Some(Wait::Carry { arrow: true });
                    return;
                }
                TextOp::WaitButton => {
                    self.wait = Some(Wait::Carry { arrow: false });
                    return;
                }
                TextOp::Pause => {
                    self.wait = Some(Wait::Pause(30));
                    return;
                }
                TextOp::Prompt => {
                    self.wait = Some(Wait::Close { arrow: true });
                    return;
                }
                TextOp::Done | TextOp::Script(_) | TextOp::Special(_) => {
                    self.wait = Some(Wait::Close { arrow: false });
                    return;
                }
            }
        }
    }
}
