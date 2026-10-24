//! Finding the cartridge inside a .zip, so players can add the file as they have it.

use crate::rom::{identify, GameId};

fn u16le(b: &[u8], at: usize) -> Option<usize> {
    Some(u16::from_le_bytes([*b.get(at)?, *b.get(at + 1)?]) as usize)
}

fn u32le(b: &[u8], at: usize) -> Option<usize> {
    Some(u32::from_le_bytes([*b.get(at)?, *b.get(at + 1)?, *b.get(at + 2)?, *b.get(at + 3)?]) as usize)
}

/// Every file in a zip, unpacked (only plain and deflated files; others are skipped).
fn zip_entries(data: &[u8]) -> Vec<Vec<u8>> {
    let mut out = Vec::new();
    // Find the end-of-directory record, then walk the central directory: it has the real sizes
    // even when the local headers leave them out.
    let Some(eocd) = (0..data.len().saturating_sub(21)).rev().take(70_000).find(|&i| data[i..].starts_with(b"PK\x05\x06")) else {
        return out;
    };
    let (Some(count), Some(mut p)) = (u16le(data, eocd + 10), u32le(data, eocd + 16)) else { return out };
    for _ in 0..count.min(512) {
        if !data.get(p..).is_some_and(|d| d.starts_with(b"PK\x01\x02")) {
            break;
        }
        let (Some(method), Some(csize), Some(usize_), Some(nlen), Some(elen), Some(clen), Some(local)) = (
            u16le(data, p + 10),
            u32le(data, p + 20),
            u32le(data, p + 24),
            u16le(data, p + 28),
            u16le(data, p + 30),
            u16le(data, p + 32),
            u32le(data, p + 42),
        ) else {
            break;
        };
        p += 46 + nlen + elen + clen;
        // Cartridges are at most 32 MB; skip anything bigger.
        if usize_ > 32 << 20 || csize > 32 << 20 {
            continue;
        }
        let (Some(lnlen), Some(lelen)) = (u16le(data, local + 26), u16le(data, local + 28)) else { continue };
        let start = local + 30 + lnlen + lelen;
        let Some(body) = data.get(start..start + csize) else { continue };
        match method {
            0 => out.push(body.to_vec()),
            8 => {
                if let Ok(v) = miniz_oxide::inflate::decompress_to_vec_with_limit(body, 32 << 20) {
                    out.push(v);
                }
            }
            _ => {}
        }
    }
    out
}

/// The cartridge in `data` (the file itself, or a supported cartridge inside a zip).
pub fn find_cartridge(data: &[u8]) -> Option<(GameId, Vec<u8>)> {
    if let Some((g, _)) = identify(data) {
        return Some((g, data.to_vec()));
    }
    if data.starts_with(b"PK") {
        for entry in zip_entries(data) {
            if let Some((g, _)) = identify(&entry) {
                return Some((g, entry));
            }
        }
    }
    None
}
