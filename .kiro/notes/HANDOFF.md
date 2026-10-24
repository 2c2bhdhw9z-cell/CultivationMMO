# Handoff: start here

Read `.kiro/steering/owner-rules.md` first, then this, then `STATUS.md` ("Next up") and
`TESTING.md`.

## What Retold is right now (24 October 2026)
A Rust engine (`crates/retold-core`) that reads Red, Blue and Yellow cartridges and rebuilds the
overworld natively: maps, neighbouring maps joined at the edges, doors, stairs, warp mats,
ledges, people who wander, signs and simple dialogue in the original text box, the original
Super Game Boy colours per town. A SwiftUI iPhone app (`native/ios`) with a game library, adding
a cartridge from Files (zips too), touch controls, controllers and keyboards, settings, and
resuming where you left off. The workflow `.github/workflows/ios.yml` builds the .ipa on every
push to `main` that touches the engine or app (unless `[skip ci]`) and publishes it as a Release.

## Desk setup (each new sandbox)
Scratch lives in `/projects/sandbox/.work` (outside the repo) and `.work/` inside the repo
(gitignored). `/root` and `/tmp` don't survive; `CARGO_HOME` must be writable:
```
export CARGO_HOME=/projects/sandbox/.work/cargo-home
mkdir -p /projects/sandbox/.work && cd /projects/sandbox/.work
curl -sSL -o rgbds.tar.xz https://github.com/gbdev/rgbds/releases/download/v1.0.4/rgbds-linux-x86_64.tar.xz
mkdir rgbds && tar xf rgbds.tar.xz -C rgbds && export PATH=$PWD/rgbds:$PATH
git clone -q --depth 1 https://github.com/pret/pokered.git && (cd pokered && make -j8 red blue)
git clone -q --depth 1 https://github.com/pret/pokeyellow.git && (cd pokeyellow && make -j8 yellow)
```
The built cartridges match the official SHA-1s. Never commit them.

- **Addresses:** `python3 tools/gen_gen1_symbols.py <pokered> <pokeyellow>` rewrites
  `crates/retold-core/src/gen1/layout.rs` (needs rgbasm on PATH). Add a label or constant to its
  lists when the engine needs a new table.
- **REA:** `npm install rea-agents` in `.work/rea` (global install fails: read-only toolchain).
  Rizin static build from github.com/rizinorg/rizin releases (v0.9.1) into `.work/rizin`, then
  `export REA_RIZIN_COMMAND=.../rizin/bin/rizin`. Game Boy code:
  `rea execute-rizin-command <cart> "e scr.color=0; e io.va=false; e asm.arch=gb; pd 20 @ <file offset>"`.
  REA's Mach-O tools need macOS; `inspect-artifact`/`extract-artifact` work here. Findings so far:
  `REA_G1R.md`.
- **Desk tool:** `cargo build --release -p retold-cli`, then
  `target/release/retold-cli <cart> out.png [WxH] goto:0 hold:d:32 tap:a wait:60 shot:x.png where`
  (see `crates/retold-cli/src/main.rs`). `sweep` loads every map and counts which texts show.
  Look at the PNGs with read_file.
- **cBoy checker** (the real game, for comparing): `tools/get-cboy.sh`, then
  `cd tools/oracle && CARGO_TARGET_DIR=/projects/sandbox/.work/oracle-target cargo build --release`.
  `retold-oracle <cart> out.png <frames> sgb newgame state=<file>` plays to the bedroom and saves
  a cBoy state; later runs with the same `state=` start there. Inputs are `from-to=mask/period`
  (hex masks: 01 A, 02 B, 08 Start, 10 Right, 20 Left, 40 Up, 80 Down). It prints the map and
  position from the game's memory. Avoid `:` in shell commands that also hold a path; the sandbox
  wrongly flags them as starting a server (that's why the oracle uses `=`).

## Facts learned the hard way
- The player starts in the bedroom facing up (checked on cBoy).
- After every warp the original ignores buttons for half a second (`IgnoreInputForHalfSecond`);
  without it you bounce straight back down the stairs.
- Steps are 16 frames at 1 pixel a frame. People move every other frame.
- Signs are read from the step in front of them; the fence row above Pallet's sign is solid.
