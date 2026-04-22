#!/bin/bash
set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "$0")" && pwd)"
APP_ROOT="$(cd "$SCRIPT_DIR/.." && pwd)"
BUILD_DIR="$APP_ROOT/build/debug"
APP_NAME="HWP Quick Look.app"
BUILT_APP="$BUILD_DIR/$APP_NAME"
INSTALLED_APP="/Applications/$APP_NAME"

echo "[1/4] Build signed Debug app..."
xcodebuild \
  -project "$APP_ROOT/RhwpMacOS.xcodeproj" \
  -scheme HostApp \
  -configuration Debug \
  -derivedDataPath "$APP_ROOT/build/DerivedData" \
  "CONFIGURATION_BUILD_DIR=$BUILD_DIR" \
  build

if [ ! -d "$BUILT_APP" ]; then
  echo "ERROR: missing built app: $BUILT_APP" >&2
  exit 1
fi

echo "[2/4] Verify bundle signature..."
codesign --verify --deep --strict --verbose=2 "$BUILT_APP"

echo "[3/4] Install app bundle to /Applications..."
/usr/bin/ditto "$BUILT_APP" "$INSTALLED_APP"

echo "[4/4] Register LaunchServices and refresh Quick Look generators..."
/System/Library/Frameworks/CoreServices.framework/Versions/Current/Frameworks/LaunchServices.framework/Versions/Current/Support/lsregister \
  -f -R -trusted "$INSTALLED_APP"
/usr/bin/open "$INSTALLED_APP"
/usr/bin/qlmanage -r

echo
echo "Installed: $INSTALLED_APP"
echo "Registered extensions containing com.postmelee.rhwpmac:"
/usr/bin/pluginkit -mAvvv | /usr/bin/grep 'com.postmelee.rhwpmac' || true
echo
echo "If Finder still shows a stale thumbnail, restart Finder or reset the Quick Look cache manually."
