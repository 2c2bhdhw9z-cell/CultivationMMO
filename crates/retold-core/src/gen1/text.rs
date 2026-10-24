//! Reading the game's words: its text commands and its letter codes.

use crate::rom::{FarPtr, Rom};

/// One step of a piece of dialogue, ready for the text box.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum TextOp {
    /// A letter or symbol tile ($60-$FF).
    Char(u8),
    /// Go to the second line.
    Line,
    /// Wait for a button with the arrow showing, then clear the box.
    Para,
    /// Wait for a button with the arrow showing, then scroll up one line.
    Cont,
    /// Scroll up one line without waiting.
    Scroll,
    /// Wait for a button with the arrow showing, then carry on.
    PromptButton,
    /// Wait for a button (no arrow), then carry on.
    WaitButton,
    /// A short pause.
    Pause,
    /// Wait for a button with the arrow, then close the box.
    Prompt,
    /// Wait for a button, then close the box.
    Done,
    /// The original runs its own code here; Retold has not rebuilt this one yet.
    Script(FarPtr),
    /// One of the game's built-in scripts (nurse, shop, PC...): the byte says which.
    Special(u8),
}

/// What `<PLAYER>` and `<RIVAL>` turn into.
pub struct Names<'a> {
    pub player: &'a [u8],
    pub rival: &'a [u8],
}

/// Letter codes for a plain ASCII string (capitals, digits, a little punctuation).
pub fn encode(s: &str) -> Vec<u8> {
    s.bytes()
        .map(|c| match c {
            b'A'..=b'Z' => 0x80 + (c - b'A'),
            b'a'..=b'z' => 0xa0 + (c - b'a'),
            b'0'..=b'9' => 0xf6 + (c - b'0'),
            b'(' => 0x9a,
            b')' => 0x9b,
            b':' => 0x9c,
            b'\'' => 0xe0,
            b'-' => 0xe3,
            b'?' => 0xe6,
            b'!' => 0xe7,
            b'.' => 0xe8,
            b'/' => 0xf3,
            b',' => 0xf4,
            _ => 0x7f,
        })
        .collect()
}

fn push_str(out: &mut Vec<TextOp>, s: &str) {
    out.extend(encode(s).into_iter().map(TextOp::Char));
}

/// Decodes a text script starting at `p` into steps for the text box.
pub fn decode(rom: &Rom, p: FarPtr, names: &Names) -> Vec<TextOp> {
    let mut out = Vec::new();
    commands(rom, p, names, &mut out, 0);
    out
}

/// Returns false once the text has ended (so callers stop).
fn commands(rom: &Rom, start: FarPtr, names: &Names, out: &mut Vec<TextOp>, depth: u8) -> bool {
    if depth > 4 {
        return false;
    }
    let mut p = start;
    for _ in 0..256 {
        let cmd = rom.byte(p);
        match cmd {
            0x00 => {
                if !chars(rom, &mut p, names, out) {
                    return false;
                }
            }
            0x01 => {
                // A word from the game's memory (an item or Pokémon name). Filled in once the
                // game systems that set those exist; skipped for now.
                p = p.offset(3);
            }
            0x02 | 0x09 => {
                push_str(out, "0");
                p = p.offset(4);
            }
            0x03 => p = p.offset(3),
            0x04 => p = p.offset(5),
            0x05 => {
                out.push(TextOp::Line);
                p = p.offset(1);
            }
            0x06 => {
                out.push(TextOp::PromptButton);
                p = p.offset(1);
            }
            0x07 => {
                out.push(TextOp::Scroll);
                p = p.offset(1);
            }
            0x08 => {
                out.push(TextOp::Script(p.offset(1)));
                return false;
            }
            0x0a => {
                out.push(TextOp::Pause);
                p = p.offset(1);
            }
            0x0c => {
                let n = rom.byte(p.offset(1));
                for _ in 0..n.min(20) {
                    out.push(TextOp::Char(0x75));
                    out.push(TextOp::Pause);
                }
                p = p.offset(2);
            }
            0x0d => {
                out.push(TextOp::WaitButton);
                p = p.offset(1);
            }
            0x0b | 0x0e..=0x16 => p = p.offset(1), // sound effects: sound comes later
            0x17 => {
                let target = FarPtr::new(rom.byte(p.offset(3)), rom.word(p.offset(1)));
                if !commands(rom, target, names, out, depth + 1) {
                    return false;
                }
                p = p.offset(4);
            }
            0x50 => return true,
            0xf5..=0xff => {
                out.push(TextOp::Special(cmd));
                return false;
            }
            _ => return false,
        }
    }
    false
}

/// Reads letters after a text-start command. Returns false if the text ended for good.
fn chars(rom: &Rom, p: &mut FarPtr, names: &Names, out: &mut Vec<TextOp>) -> bool {
    *p = p.offset(1);
    for _ in 0..1024 {
        let c = rom.byte(*p);
        *p = p.offset(1);
        match c {
            0x50 => return true,
            0x4e | 0x4f => out.push(TextOp::Line),
            0x51 | 0x49 => out.push(TextOp::Para),
            0x55 | 0x4b => out.push(TextOp::Cont),
            0x4c => out.push(TextOp::Scroll),
            0x57 => {
                out.push(TextOp::Done);
                return false;
            }
            0x58 => {
                out.push(TextOp::Prompt);
                return false;
            }
            0x5f => {
                out.push(TextOp::Char(0xe8));
                out.push(TextOp::Done);
                return false;
            }
            0x52 => out.extend(names.player.iter().map(|&c| TextOp::Char(c))),
            0x53 => out.extend(names.rival.iter().map(|&c| TextOp::Char(c))),
            0x54 => out.extend([0x8f, 0x8e, 0x8a, 0xba].map(TextOp::Char)),
            0x56 => out.extend([0x75, 0x75].map(TextOp::Char)),
            0x4a => out.extend([0xe1, 0xe2].map(TextOp::Char)),
            0x5b => push_str(out, "PC"),
            0x5c => push_str(out, "TM"),
            0x5d => push_str(out, "TRAINER"),
            0x5e => push_str(out, "ROCKET"),
            0x59 | 0x5a => {}
            0x00 => {}
            c if c >= 0x60 => out.push(TextOp::Char(c)),
            _ => {}
        }
    }
    false
}
