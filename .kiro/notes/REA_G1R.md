# What REA showed about gen1recomp++ 0.3.96 (24 October 2026)

Studied to learn what it does, never to copy code (see owner-rules). Target: the official
`gen1recomp++-0.3.96-ios.ipa` from github.com/bryanthaboi/gen1recomp releases, sha256
`3e76461bdc45bf210025e6a91d4326e479b765eb24a0389f3677f275334deb54`, 44.7 MB.

## How it is built (REA `inspect-artifact`, `extract-artifact`, Rizin `il` on the main program)
- 27 files. The program `gen1recomp++` (33.9 MB, Mach-O) is the LÖVE game engine on SDL3, plus
  `game.love` (31.2 MB zip of Lua scripts and assets) and `SDL3.framework`.
- Drawing goes through OpenGL ES and Metal (LÖVE 12). Sound through OpenAL. It links
  GameController, CoreMotion, CoreHaptics, HealthKit, SafariServices, CryptoKit.
- So the whole game runs as interpreted Lua inside a general game engine. Retold is native Rust
  drawn straight to the screen, which leaves far more room for speed and battery.

## What it has (from its folder and option names)
- About 3,000 Lua files in `game.love`: core 779, ui 435, import 409, save_convert 49, world 48,
  battle 45, mods 42, render 35, online 30, box 20, script 15, link 13, sync 9.
- Options it offers: text speed, menu speed, overworld speed, battle speed, battle animation,
  battle background, battle HUD, battle layout, battle size, battle style, colours, palette
  screen, shader effects (two layers), tilt, zoom, void fill, video mode, faithful ratio,
  orientation, screen position, letterbox, max FPS, vsync, logic clock, performance tiers, music
  and sound volumes, Pikachu volume, music filter, vibration, touch pad and touch controls editor,
  key bar, control rebinding, ruleset, date and time formats, mods.
- Screens it has: title, Oak's speech, Yellow intro, naming, start menu, bag, party, summary,
  Pokédex, trainer card, PCs and boxes, marts, prize counter, slot machine, surfing minigame,
  town map, fly menu, Hall of Fame, diploma, trade animation, evolution, link play, online lobby,
  Discord presence, skin studio, save conversion, printer.

## REA on the cartridges
- REA's Rizin route reads Game Boy machine code. Rizin opens a .gb file with the wrong loader by
  default, so add `e io.va=false; e asm.arch=gb` to the command and give file offsets
  (`bank * 0x4000 + (address - 0x4000)`). `tools/rea-gb.sh` wraps this.
- Checked on Red: `EnterMap` at 00:03a6 reads `ld a,$ff / ld [$cd6b],a / call $1241`, matching
  pret's `ld [wJoyIgnore], a / call LoadMapData`.
