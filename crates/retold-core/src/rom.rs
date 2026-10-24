//! Recognising the player's cartridge file and reading from it.
//!
//! Only the official US English cartridges are accepted, identified by SHA-1. The file is read
//! in place; nothing from it is ever written into the app or the repository.

/// Every game Retold is meant to play.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[repr(u8)]
pub enum GameId {
    Red = 1,
    Blue = 2,
    Yellow = 3,
    Gold = 4,
    Silver = 5,
    Crystal = 6,
    FireRed = 7,
    LeafGreen = 8,
    Ruby = 9,
    Sapphire = 10,
    Emerald = 11,
}

impl GameId {
    pub const ALL: [GameId; 11] = [
        GameId::Red,
        GameId::Blue,
        GameId::Yellow,
        GameId::Gold,
        GameId::Silver,
        GameId::Crystal,
        GameId::FireRed,
        GameId::LeafGreen,
        GameId::Ruby,
        GameId::Sapphire,
        GameId::Emerald,
    ];

    pub fn from_u8(v: u8) -> Option<GameId> {
        GameId::ALL.iter().copied().find(|g| *g as u8 == v)
    }

    /// The name shown to players.
    pub fn name(self) -> &'static str {
        match self {
            GameId::Red => "Red",
            GameId::Blue => "Blue",
            GameId::Yellow => "Yellow",
            GameId::Gold => "Gold",
            GameId::Silver => "Silver",
            GameId::Crystal => "Crystal",
            GameId::FireRed => "FireRed",
            GameId::LeafGreen => "LeafGreen",
            GameId::Ruby => "Ruby",
            GameId::Sapphire => "Sapphire",
            GameId::Emerald => "Emerald",
        }
    }

    /// Whether this version of Retold can play it yet.
    pub fn playable(self) -> bool {
        matches!(self, GameId::Red | GameId::Blue | GameId::Yellow)
    }
}

/// SHA-1 of every accepted cartridge, with its game and revision.
const KNOWN: &[(&str, GameId, u8)] = &[
    ("ea9bcae617fdf159b045185467ae58b2e4a48b9a", GameId::Red, 0),
    ("d7037c83e1ae5b39bde3c30787637ba1d4c48ce2", GameId::Blue, 0),
    ("cc7d03262ebfaf2f06772c1a480c7d9d5f4a38e1", GameId::Yellow, 0),
    ("d8b8a3600a465308c9953dfa04f0081c05bdcb94", GameId::Gold, 0),
    ("49b163f7e57702bc939d642a18f591de55d92dae", GameId::Silver, 0),
    ("f4cd194bdee0d04ca4eac29e09b8e4e9d818c133", GameId::Crystal, 0),
    ("f2f52230b536214ef7c9924f483392993e226cfb", GameId::Crystal, 1),
    ("41cb23d8dccc8ebd7c649cd8fbb58eeace6e2fdc", GameId::FireRed, 0),
    ("dd5945db9b930750cb39d00c84da8571feebf417", GameId::FireRed, 1),
    ("574fa542ffebb14be69902d1d36f1ec0a4afd71e", GameId::LeafGreen, 0),
    ("7862c67bdecbe21d1d69ce082ce34327e1c6ed5e", GameId::LeafGreen, 1),
    ("f3ae088181bf583e55daf962a92bb46f4f1d07b7", GameId::Emerald, 0),
    ("f28b6ffc97847e94a6c21a63cacf633ee5c8df1e", GameId::Ruby, 0),
    ("610b96a9c9a7d03d2bafb655e7560ccff1a6d894", GameId::Ruby, 1),
    ("5b64eacf892920518db4ec664e62a086dd5f5bc8", GameId::Ruby, 2),
    ("3ccbbd45f8553c36463f13b938e833f652b793e4", GameId::Sapphire, 0),
    ("4722efb8cd45772ca32555b98fd3b9719f8e60a9", GameId::Sapphire, 1),
    ("89b45fb172e6b55d51fc0e61989775187f6fe63c", GameId::Sapphire, 2),
];

/// Which game a file is, if it is one of the accepted cartridges.
pub fn identify(bytes: &[u8]) -> Option<(GameId, u8)> {
    let digest = sha1_smol::Sha1::from(bytes).digest().to_string();
    KNOWN
        .iter()
        .find(|(sha, _, _)| *sha == digest)
        .map(|(_, game, rev)| (*game, *rev))
}

/// A bank and an address inside it, the way the game itself points at data.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct FarPtr {
    pub bank: u8,
    pub addr: u16,
}

impl FarPtr {
    pub const fn new(bank: u8, addr: u16) -> FarPtr {
        FarPtr { bank, addr }
    }

    pub fn offset(self, by: u16) -> FarPtr {
        FarPtr { bank: self.bank, addr: self.addr.wrapping_add(by) }
    }
}

/// The cartridge file, read-only.
pub struct Rom {
    data: Vec<u8>,
}

impl Rom {
    pub fn new(data: Vec<u8>) -> Rom {
        Rom { data }
    }

    /// File position of a Game Boy bank:address.
    pub fn file_offset(p: FarPtr) -> usize {
        if p.addr < 0x4000 {
            p.addr as usize
        } else {
            p.bank as usize * 0x4000 + (p.addr as usize - 0x4000)
        }
    }

    pub fn byte(&self, p: FarPtr) -> u8 {
        self.data.get(Rom::file_offset(p)).copied().unwrap_or(0xff)
    }

    pub fn word(&self, p: FarPtr) -> u16 {
        self.byte(p) as u16 | (self.byte(p.offset(1)) as u16) << 8
    }

    /// A pointer stored at `p`, into bank `bank`.
    pub fn ptr(&self, p: FarPtr, bank: u8) -> FarPtr {
        FarPtr::new(bank, self.word(p))
    }

    pub fn bytes(&self, p: FarPtr, len: usize) -> Vec<u8> {
        (0..len).map(|i| self.byte(p.offset(i as u16))).collect()
    }

    /// Bytes from `p` up to (not including) the first `end` byte, at most `max`.
    pub fn until(&self, p: FarPtr, end: u8, max: usize) -> Vec<u8> {
        let mut out = Vec::new();
        for i in 0..max {
            let b = self.byte(p.offset(i as u16));
            if b == end {
                break;
            }
            out.push(b);
        }
        out
    }
}
