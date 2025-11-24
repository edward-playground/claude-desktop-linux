#!/bin/bash

# Generate icons from SVG source
# Requires: inkscape or imagemagick

set -e

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
PROJECT_DIR="$(dirname "$SCRIPT_DIR")"
ICONS_DIR="$PROJECT_DIR/src-tauri/icons"
SVG_SOURCE="$ICONS_DIR/icon.svg"

echo "Generating icons from $SVG_SOURCE..."

# Check if inkscape is available
if command -v inkscape &> /dev/null; then
    echo "Using Inkscape..."

    # Generate PNG icons
    inkscape -w 32 -h 32 "$SVG_SOURCE" -o "$ICONS_DIR/32x32.png"
    inkscape -w 128 -h 128 "$SVG_SOURCE" -o "$ICONS_DIR/128x128.png"
    inkscape -w 256 -h 256 "$SVG_SOURCE" -o "$ICONS_DIR/128x128@2x.png"
    inkscape -w 512 -h 512 "$SVG_SOURCE" -o "$ICONS_DIR/icon.png"

    echo "PNG icons generated!"

elif command -v convert &> /dev/null; then
    echo "Using ImageMagick..."

    # Generate PNG icons using ImageMagick
    convert -background none -resize 32x32 "$SVG_SOURCE" "$ICONS_DIR/32x32.png"
    convert -background none -resize 128x128 "$SVG_SOURCE" "$ICONS_DIR/128x128.png"
    convert -background none -resize 256x256 "$SVG_SOURCE" "$ICONS_DIR/128x128@2x.png"
    convert -background none -resize 512x512 "$SVG_SOURCE" "$ICONS_DIR/icon.png"

    echo "PNG icons generated!"

else
    echo "Error: Neither Inkscape nor ImageMagick found."
    echo "Please install one of them:"
    echo "  Ubuntu/Debian: sudo apt install inkscape"
    echo "  Fedora: sudo dnf install inkscape"
    echo "  Arch: sudo pacman -S inkscape"
    exit 1
fi

# Generate .ico for Windows (optional, not needed for Linux)
if command -v convert &> /dev/null; then
    convert "$ICONS_DIR/32x32.png" "$ICONS_DIR/128x128.png" "$ICONS_DIR/icon.ico"
    echo "Windows .ico generated!"
fi

# Note: .icns (macOS) generation requires additional tools and is not needed for Linux
echo "Skipping .icns generation (not needed for Linux)"

# Create placeholder if .icns doesn't exist
touch "$ICONS_DIR/icon.icns" 2>/dev/null || true

echo "Done! Icons generated in $ICONS_DIR"
ls -la "$ICONS_DIR"
