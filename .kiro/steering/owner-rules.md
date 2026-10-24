---
inclusion: always
---

# The owner's rules for Retold. Read these first, every session.

These come straight from the owner (GitHub 2c2bhdhw9z-cell). They are not suggestions.

## What Retold is
- Retold is the classic Pokémon games rebuilt as a real iPhone app, a better version of
  gen1recomp++ (G1R, github.com/bryanthaboi/gen1recomp). Owner, 24 October 2026: name picked by
  Kiro (can be renamed), **written fresh and all ours**, **every game G1R has** (Red, Blue,
  Yellow, Gold, Silver, Crystal, FireRed, LeafGreen, Ruby, Sapphire, Emerald), and better in
  **every way and more**.
- **Use REA** (Reverse Engineer Anything, npm `rea-agents`) to understand G1R and the original
  cartridges down to the machine code (owner: "that's what REA is for. Even though it's public,
  the tool helps you understand it better down to the binary"). Setup is in
  `.kiro/notes/HANDOFF.md`.
- **No G1R code, ever.** It is GPL with extra terms and a proprietary launcher. G1R is studied for
  what it does (features, behaviour); we never copy or translate its code. How the original games
  work comes from the cartridge itself (through REA) and the public pret disassemblies.
- **Not an emulator.** No emulator's code goes in. The engine is our own Rust.
- **No game content in the repo or the app.** Maps, graphics, text and music are read from the
  player's own cartridge file at runtime. Only addresses and table layouts (facts about the file)
  are committed. Test cartridges are built locally from pret and never committed.
- Only the official US cartridges are accepted, checked by SHA-1.
- This is the old CultivationMMO repo, reused on the owner's word (24 October 2026: "Just use
  this repo"). The MMO is kept in history under the tag `cultivationmmo-archive`. Kiro cannot
  create or rename repos (its GitHub access has read-only admin rights).
- Public repo with no open licence (all rights reserved), so GitHub's Mac builds are free.

## How to talk to the owner
- The owner does not code and uses only an iPhone, never a computer. Every reply is plain,
  everyday English. Explain any technical word in normal words. Describe what he will see.
- Never ask him to read a file, log or page. Read it yourself and say what it says.
- **Never assume. If anything is unclear, ask first**: short numbered questions with the choices
  written out. But never stop to ask a small question (names, order) while he is asleep or has
  said "stop stopping": decide, note it, carry on.
- Don't open replies by telling him he is right ("You're right", "Got it").
- US Eastern time for every time. Never Pacific.
- Screenshots sent while a reply is in progress arrive as text only. Say so if he mentions them.
- The owner reads the Thought sections. Keep them as respectful as replies.
- A ★ or ⭐ on something means save it permanently: here if it is about Retold, as a Kiro
  learning if it applies everywhere.

## The app
- **EVERYTHING IS CUSTOMISABLE, ALWAYS.** Every look and motion feature gets its own setting with
  a sensible default.
- Never remove something he asked for because it came out badly. Fix it properly.
- Text in the app is short and casual, one or two sentences at most.
- Do only what he asked for. No "while I was there" changes.

## Builds and the .ipa
- The .ipa is the only thing he cares about. With every build: the direct download link, plus
  short numbered test steps written out in that same reply (he often tests at work). Never "see
  my last message".
- Before handing it over, open the .ipa and check the version and contents.
- One build per batch. `[skip ci]` on intermediate pushes, none on the last push of the batch.
- He deletes the app before every install. Never tell him to install over the old one.
- Never re-ask a test he already covered. While he is testing, fix nothing until he says he is
  done; just write down what he reports.
- No scheduled or automatic builds unless he asks for one for a particular night.

## Git and working
- NO branches. NO pull requests. Commit and push straight to `main`.
- Save constantly: push each finished, checked piece right away. Never leave work only in the
  sandbox.
- When he says he is going to bed or "stop stopping", keep working through the plan in the same
  turn. Never end with a summary after a short stretch.
- Start every session with `.kiro/notes/HANDOFF.md`, then `.kiro/notes/STATUS.md` ("Next up"),
  then `.kiro/notes/TESTING.md`. Update HANDOFF and "Next up" before a session ends.
