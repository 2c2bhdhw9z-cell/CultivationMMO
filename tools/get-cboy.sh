#!/usr/bin/env bash
# Fetches cBoy (cores/continuum-gb from the owner's Continuum repo) into .work/ for the checker
# in tools/oracle. Owner, 24 October 2026: "You can use cBoy. It's my own code."
set -euo pipefail
ROOT="$(cd "$(dirname "$0")/.." && pwd)"
DEST="$ROOT/.work/continuum"
if [ -d "$DEST/cores/continuum-gb" ]; then
  git -C "$DEST" pull -q --depth 1 || true
else
  git clone -q --depth 1 --filter=blob:none --sparse https://github.com/2c2bhdhw9z-cell/Continuum-.git "$DEST"
  git -C "$DEST" sparse-checkout set cores/continuum-gb
fi
echo "cBoy is at $DEST/cores/continuum-gb"
