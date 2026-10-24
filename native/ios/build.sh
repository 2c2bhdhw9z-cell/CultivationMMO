#!/usr/bin/env bash
# Builds Retold.ipa on a Mac: the Rust engine for the phone, the Xcode project from
# project.yml, then an ad-hoc signed .ipa ready for any sideloading app.
#   RETOLD_BUILD_NUMBER=<n> native/ios/build.sh   ->  native/ios/build/Retold.ipa
set -euo pipefail
HERE="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
ROOT="$(cd "$HERE/../.." && pwd)"
OUT="$HERE/build"
[ "$(uname -s)" = "Darwin" ] || { echo "error: needs a Mac with Xcode" >&2; exit 1; }
command -v xcodegen >/dev/null || { echo "error: xcodegen missing (brew install xcodegen)" >&2; exit 1; }

echo "==> engine (Rust, aarch64-apple-ios)"
rustup target add aarch64-apple-ios >/dev/null
(cd "$ROOT" && IPHONEOS_DEPLOYMENT_TARGET=17.0 cargo build --release --locked -p retold-ffi --target aarch64-apple-ios)
mkdir -p "$OUT/lib"
cp "$ROOT/target/aarch64-apple-ios/release/libretold_ffi.a" "$OUT/lib/"

BUILD_NUMBER="${RETOLD_BUILD_NUMBER:-1}"
echo "==> build number $BUILD_NUMBER"
python3 - "$HERE/project.yml" "$BUILD_NUMBER" <<'PY'
import re, sys
path, build = sys.argv[1], sys.argv[2]
text = open(path, encoding="utf-8").read()
text, n = re.subn(r'^(\s*CFBundleVersion:\s*)"[^"]*"$', lambda m: f'{m.group(1)}"{build}"', text, flags=re.M)
if n != 1:
    sys.exit(f"expected one CFBundleVersion line, found {n}")
open(path, "w", encoding="utf-8").write(text)
PY

echo "==> xcodegen"
(cd "$HERE" && xcodegen generate --spec project.yml --project .)

echo "==> xcodebuild"
xcodebuild \
  -project "$HERE/Retold.xcodeproj" \
  -scheme Retold \
  -configuration Release \
  -sdk iphoneos \
  -destination 'generic/platform=iOS' \
  -derivedDataPath "$OUT/DerivedData" \
  CODE_SIGNING_ALLOWED=NO CODE_SIGNING_REQUIRED=NO CODE_SIGN_IDENTITY="" \
  build

APP="$OUT/DerivedData/Build/Products/Release-iphoneos/Retold.app"
[ -d "$APP" ] || { echo "error: no app at $APP" >&2; exit 1; }
rm -rf "$OUT/stage" "$OUT/Retold.ipa"
mkdir -p "$OUT/stage/Payload"
cp -R "$APP" "$OUT/stage/Payload/"
echo "==> ad-hoc signing"
codesign --force --sign - --timestamp=none "$OUT/stage/Payload/Retold.app"
(cd "$OUT/stage" && zip -qry "$OUT/Retold.ipa" Payload)
ls -la "$OUT/Retold.ipa"
