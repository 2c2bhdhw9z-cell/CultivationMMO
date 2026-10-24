#!/usr/bin/env python3
"""Draws Retold's app icon (our own art): a red-to-blue sky with a big pixel-art R.
Writes native/ios/Assets.xcassets/AppIcon.appiconset/icon-1024.png."""
import os
import struct
import zlib

N = 1024
GLYPH = [
    "XXXXXX.",
    "XXXXXXX",
    "XX...XX",
    "XX...XX",
    "XXXXXXX",
    "XXXXXX.",
    "XX..XX.",
    "XX...XX",
    "XX...XX",
]
CELL = 72


def lerp(a, b, t):
    return int(a + (b - a) * t)


def main():
    top_left = (232, 64, 56)
    bottom_right = (40, 72, 200)
    gw, gh = len(GLYPH[0]) * CELL, len(GLYPH) * CELL
    ox, oy = (N - gw) // 2, (N - gh) // 2
    rows = []
    for y in range(N):
        row = bytearray([0])
        for x in range(N):
            t = (x + y) / (2 * N - 2)
            r, g, b = (lerp(top_left[i], bottom_right[i], t) for i in range(3))
            # Soft light from the top.
            glow = max(0.0, 1 - ((x - N / 2) ** 2 + (y - N * 0.2) ** 2) / (N * 0.55) ** 2) * 0.25
            r, g, b = (min(255, int(c + (255 - c) * glow)) for c in (r, g, b))
            for dx, dy, colour in ((22, 26, (20, 16, 40)), (0, 0, (255, 255, 255))):
                gx, gy = x - ox - dx, y - oy - dy
                if 0 <= gx < gw and 0 <= gy < gh and GLYPH[gy // CELL][gx // CELL] == "X":
                    r, g, b = colour
            row += bytes((r, g, b))
        rows.append(bytes(row))
    raw = zlib.compress(b"".join(rows), 9)

    def chunk(kind, data):
        return struct.pack(">I", len(data)) + kind + data + struct.pack(">I", zlib.crc32(kind + data) & 0xFFFFFFFF)

    png = b"\x89PNG\r\n\x1a\n" + chunk(b"IHDR", struct.pack(">IIBBBBB", N, N, 8, 2, 0, 0, 0))
    png += chunk(b"IDAT", raw) + chunk(b"IEND", b"")
    out = os.path.join(os.path.dirname(__file__), "..", "native/ios/Assets.xcassets/AppIcon.appiconset/icon-1024.png")
    os.makedirs(os.path.dirname(out), exist_ok=True)
    with open(out, "wb") as f:
        f.write(png)
    print("wrote", os.path.normpath(out))


if __name__ == "__main__":
    main()
