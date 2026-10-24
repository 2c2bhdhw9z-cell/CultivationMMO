# Retold: the plan

Owner's answers, 24 October 2026: Kiro picks the name, fresh code that is all ours, every game
G1R has, better in every way and more. Milestones go in this order; each ends with an .ipa the
owner can try. Small choices made without asking are marked *(Kiro's pick)* so he can change them.

## Engine shape *(Kiro's pick)*
- `crates/retold-core`: the game, in Rust. Reads the player's cartridge file, rebuilds each
  game's world, rules and screens natively. No Game Boy emulation: we draw maps, sprites and
  text ourselves, from the data tables in the file.
- `crates/retold-ffi`: the small door the iPhone app talks through.
- `crates/retold-cli`: a desk tool for Kiro: renders any map or a played-through scene to a PNG
  so every change is checked by eye before a build.
- `native/ios`: the iPhone app (SwiftUI). Library of games, play screen, touch controls,
  controllers, settings.
- Addresses of the tables inside each cartridge are generated from the pret builds
  (`tools/gen_gen1_symbols.py`) and committed as plain numbers.

## Milestones
1. **Kanto to walk around** (Red, Blue, Yellow): add your cartridge, walk every map with the
   original colours, doors, stairs, ledges, map edges, people who wander and talk, signs.
   Widescreen view, run button, walking speed, text speed, colour choices, touch controls,
   controllers, resume where you left off.
2. **Menus and the start of the game**: title screen, Oak's speech, naming, start menu, bag,
   party, Pokédex, trainer card, save/load, items on the ground, PCs, marts.
3. **Sound**: our own synth plays the music and sound effects from the cartridge, with volume
   and filter settings.
4. **Battles**: wild Pokémon, trainers, every move and effect, catching, experience, evolution,
   learning moves, Pokémon Centers.
5. **The whole story**: every map script and event, HMs, gyms, Silph Co., Elite Four, Hall of
   Fame. Import and export real cartridge saves.
6. **Yellow's extras**: Pikachu following, its moods, Game Boy Color colours.
7. **Better than G1R**: everything in its options list (see `REA_G1R.md`) and more: HD
   smoothing filters, 3D tilt and zoom, real 3D blocks built in, battle speed, rules choices
   (original quirks or modern fixes), quick save anywhere, rewind, link trading and battles
   online, mods, skins, a save editor.
8. **Gold, Silver, Crystal** (day and night, phone calls, Johto and Kanto).
9. **FireRed, LeafGreen, Ruby, Sapphire, Emerald** (Game Boy Advance games).
