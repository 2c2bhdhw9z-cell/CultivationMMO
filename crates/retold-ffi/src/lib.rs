//! The door the iPhone app talks to the engine through. Every call catches panics, so a bug in
//! the engine shows up as a failed call instead of closing the app. See `retold.h`.

use std::panic::{catch_unwind, AssertUnwindSafe};
use std::slice;

use retold_core::archive::find_cartridge;
use retold_core::input::Buttons;
use retold_core::{GameId, Session};

pub struct RetoldSession {
    session: Session,
    pixels: Vec<u32>,
}

fn bytes<'a>(data: *const u8, len: usize) -> &'a [u8] {
    if data.is_null() || len == 0 {
        &[]
    } else {
        // SAFETY: the app passes a pointer to `len` readable bytes.
        unsafe { slice::from_raw_parts(data, len) }
    }
}

/// Finds the cartridge in a file (or a .zip holding one). Returns the game number (1 Red ...
/// 11 Emerald) or 0. On success, `*out` gets a copy of the cartridge bytes that the caller
/// frees with `retold_free_buffer`.
#[no_mangle]
pub extern "C" fn retold_find_cartridge(data: *const u8, len: usize, out: *mut *mut u8, out_len: *mut usize) -> i32 {
    catch_unwind(|| {
        let Some((game, cart)) = find_cartridge(bytes(data, len)) else { return 0 };
        if !out.is_null() && !out_len.is_null() {
            let mut boxed = cart.into_boxed_slice();
            // SAFETY: both pointers were checked for null and point at the caller's variables.
            unsafe {
                *out_len = boxed.len();
                *out = boxed.as_mut_ptr();
            }
            std::mem::forget(boxed);
        }
        game as i32
    })
    .unwrap_or(0)
}

#[no_mangle]
pub extern "C" fn retold_free_buffer(ptr: *mut u8, len: usize) {
    if !ptr.is_null() {
        // SAFETY: `ptr` and `len` came from `retold_find_cartridge`'s boxed slice.
        unsafe { drop(Box::from_raw(slice::from_raw_parts_mut(ptr, len))) };
    }
}

/// 1 if this version can play the game number.
#[no_mangle]
pub extern "C" fn retold_game_playable(game: i32) -> i32 {
    GameId::from_u8(game as u8).is_some_and(|g| g.playable()) as i32
}

/// Starts a game from the cartridge bytes. NULL if they aren't a playable cartridge.
#[no_mangle]
pub extern "C" fn retold_open(data: *const u8, len: usize) -> *mut RetoldSession {
    catch_unwind(|| match Session::new(bytes(data, len).to_vec()) {
        Ok(session) => Box::into_raw(Box::new(RetoldSession { session, pixels: Vec::new() })),
        Err(_) => std::ptr::null_mut(),
    })
    .unwrap_or(std::ptr::null_mut())
}

#[no_mangle]
pub extern "C" fn retold_close(s: *mut RetoldSession) {
    if !s.is_null() {
        // SAFETY: `s` came from `retold_open` and is closed once.
        let _ = catch_unwind(AssertUnwindSafe(|| unsafe { drop(Box::from_raw(s)) }));
    }
}

fn with<T>(s: *mut RetoldSession, default: T, f: impl FnOnce(&mut RetoldSession) -> T) -> T {
    if s.is_null() {
        return default;
    }
    // SAFETY: `s` is a live session from `retold_open`, used from one thread at a time.
    let s = unsafe { &mut *s };
    catch_unwind(AssertUnwindSafe(|| f(s))).unwrap_or(default)
}

/// How many 8x8 tiles to show: 20x18 is the original screen.
#[no_mangle]
pub extern "C" fn retold_set_view(s: *mut RetoldSession, tiles_w: u32, tiles_h: u32) {
    with(s, (), |s| s.session.set_view(tiles_w as usize, tiles_h as usize));
}

/// Changes one setting (keys in retold.h).
#[no_mangle]
pub extern "C" fn retold_set_option(s: *mut RetoldSession, key: u32, value: i32) {
    with(s, (), |s| {
        s.session.options_mut().set(key, value);
        s.session.redraw();
    });
}

/// Runs one 1/60 s frame with these buttons held (bits: A B Select Start Right Left Up Down).
/// Returns 0 if the engine hit a problem.
#[no_mangle]
pub extern "C" fn retold_step(s: *mut RetoldSession, buttons: u32) -> i32 {
    with(s, 0, |s| {
        s.session.step(Buttons(buttons));
        1
    })
}

/// The current picture: RGBA, `*width` x `*height`, valid until the next call.
#[no_mangle]
pub extern "C" fn retold_pixels(s: *mut RetoldSession, width: *mut u32, height: *mut u32) -> *const u32 {
    with(s, std::ptr::null(), |s| {
        let f = s.session.frame();
        s.pixels.clear();
        s.pixels.extend_from_slice(&f.pixels);
        if !width.is_null() && !height.is_null() {
            // SAFETY: checked for null; the caller's variables.
            unsafe {
                *width = f.width as u32;
                *height = f.height as u32;
            }
        }
        s.pixels.as_ptr()
    })
}

/// Writes where you are into `out` (if it fits) and returns its size.
#[no_mangle]
pub extern "C" fn retold_save(s: *mut RetoldSession, out: *mut u8, cap: usize) -> usize {
    with(s, 0, |s| {
        let data = s.session.save();
        if !out.is_null() && cap >= data.len() {
            // SAFETY: `out` has room for `cap` bytes.
            unsafe { std::ptr::copy_nonoverlapping(data.as_ptr(), out, data.len()) };
        }
        data.len()
    })
}

/// Puts you back where a save said. 1 on success.
#[no_mangle]
pub extern "C" fn retold_load(s: *mut RetoldSession, data: *const u8, len: usize) -> i32 {
    with(s, 0, |s| s.session.load(bytes(data, len)) as i32)
}
