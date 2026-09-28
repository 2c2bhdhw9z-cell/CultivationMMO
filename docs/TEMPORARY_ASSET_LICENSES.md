# Temporary Asset and Dependency License Ledger

> **Recorded:** September 28, 2026
> **Status:** No external game-content assets are selected.

## Current content decision

The first technical preview uses only:

- Project-authored primitive meshes and collision shapes
- Project-authored flat colors/materials
- Project-authored interface shapes and text
- Simple project-generated test tones if sound is needed
- Built-in Godot engine capabilities

No third-party character, animation, model, texture, icon, music, sound effect, font, shader, addon, sample project, generated-asset service, or marketplace pack is approved or included.

“Free to download” never means “safe to redistribute.” Every future asset must be entered and approved here before its files are committed or placed in a build.

## Engine dependency

| Name | Version | Source | License | Required action | Status |
|---|---|---|---|---|---|
| Godot Engine | 4.7.2-stable | [Official release](https://github.com/godotengine/godot-builds/releases/tag/4.7.2-stable) | MIT | Include Godot copyright/license notice in game documentation or credits; preserve applicable third-party notices | Approved engine dependency; not a game-content asset |

Godot's [official license page](https://godotengine.org/license/) says games may use a different license, while distributed engine code must carry the Godot copyright and MIT license statement. The root `THIRD_PARTY_NOTICES.md` records that notice now. Task 3 must bundle it and preserve applicable official Godot third-party notices in **every** distributed package, including the first private tester build.

## Required entry for every future asset

Copy this table row before downloading or committing an asset:

| Field | Required information |
|---|---|
| Asset name and version | Exact title, ID, and version |
| Creator/rightsholder | Name shown by the canonical source |
| Canonical source | Direct original page URL; reposts are not enough |
| Evidence date | Date the license was checked |
| Evidence | Saved license text or durable reference plus file/hash where permitted |
| Local paths | Every repository path containing the asset or derivative |
| License name/version | Exact license, not “free” or “royalty free” |
| Project use | Whether modification and commercial game use are allowed |
| Redistribution | Whether source files and compiled game distribution are allowed |
| Attribution | Exact required credit and placement |
| Other duties | Share-alike, notice, source, trademark, AI-training, or territory limits |
| Modifications | What the project changed |
| Reviewer decision | Approved, rejected, replace-before-release, or needs legal review |
| Removal proof | Commit/build version where a rejected/expired asset disappeared |

## Approval rules

An asset is rejected unless its documented terms clearly allow the intended use and redistribution. In particular:

- Do not copy art, characters, names, text, techniques, symbols, music, or story content from the works used as inspiration.
- Do not use assets ripped from games, anime, comics, novels, films, or social media.
- Do not use an asset when the uploader may not own it.
- Do not rely on a store's “free” label without reading the exact license.
- Preserve required notices in source and shipped credits.
- Keep editable source assets out of a public download if the license allows only compiled redistribution.
- Track generated assets with the generator, model/version, input ownership, terms date, and commercial-use rights.
- Recheck licenses before a public test, store release, or monetization change.
- Remove an asset from source, caches, build artifacts, and distributed packages if its rights cannot be proved.

## First-checkpoint declaration

As of the recorded date, the external game-asset count is **zero**. The graybox checkpoint can proceed without outside game-content licenses.

External documentation was checked on September 28, 2026. Source material was paraphrased for compliance with licensing restrictions.
