#!/bin/bash

# Script to convert image.svg to logo.icns for macOS app bundle
# This creates a proper .icns file with multiple resolutions
# Used by DevSweep (GPUI-based app)

set -e

echo "🎨 Creating macOS app icon from SVG..."

# Navigate to project root (we're in scripts directory)
cd "$(dirname "$0")/.."

# Check if source SVG exists
if [ ! -f "assets/image-dark.svg" ]; then
    echo "❌ Error: assets/image-dark.svg not found!"
    echo "Please place image-dark.svg in the assets directory."
    exit 1
fi

SVG_SOURCE="assets/image-dark.svg"

# Create temporary directory for icon generation
ICONSET_DIR="AppIcon.iconset"
rm -rf "$ICONSET_DIR"
mkdir -p "$ICONSET_DIR"

echo "📐 Generating PNG files at different resolutions..."

# Check if we have the required tools
if ! command -v rsvg-convert &> /dev/null && ! command -v qlmanage &> /dev/null; then
    echo "⚠️  Warning: Neither rsvg-convert nor qlmanage found."
    echo "Installing librsvg for SVG conversion..."
    if command -v brew &> /dev/null; then
        brew install librsvg
    else
        echo "❌ Homebrew not found. Please install librsvg manually or use qlmanage."
        exit 1
    fi
fi

# Function to convert SVG to PNG at specific size
convert_svg_to_png() {
    local size=$1
    local output=$2

    if command -v rsvg-convert &> /dev/null; then
        # Use rsvg-convert (better quality)
        rsvg-convert -w $size -h $size "$SVG_SOURCE" -o "$output"
    else
        # Fallback: Use sips with intermediate conversion
        # First convert SVG to PNG using qlmanage, then resize with sips
        qlmanage -t -s $size -o . "$SVG_SOURCE" > /dev/null 2>&1
        mv "image-dark.svg.png" "$output" 2>/dev/null || {
            # Alternative: use macOS native conversion
            sips -s format png "$SVG_SOURCE" --out temp.png > /dev/null 2>&1
            sips -z $size $size temp.png --out "$output" > /dev/null 2>&1
            rm -f temp.png
        }
    fi

    echo "  ✓ Created ${size}x${size} icon"
}

# Generate all required icon sizes for macOS .icns
# Standard sizes: 16, 32, 64, 128, 256, 512, 1024
# Retina sizes: 32 (@2x of 16), 64 (@2x of 32), 256 (@2x of 128), 512 (@2x of 256), 1024 (@2x of 512)

convert_svg_to_png 16   "$ICONSET_DIR/icon_16x16.png"
convert_svg_to_png 32   "$ICONSET_DIR/icon_16x16@2x.png"
convert_svg_to_png 32   "$ICONSET_DIR/icon_32x32.png"
convert_svg_to_png 64   "$ICONSET_DIR/icon_32x32@2x.png"
convert_svg_to_png 128  "$ICONSET_DIR/icon_128x128.png"
convert_svg_to_png 256  "$ICONSET_DIR/icon_128x128@2x.png"
convert_svg_to_png 256  "$ICONSET_DIR/icon_256x256.png"
convert_svg_to_png 512  "$ICONSET_DIR/icon_256x256@2x.png"
convert_svg_to_png 512  "$ICONSET_DIR/icon_512x512.png"
convert_svg_to_png 1024 "$ICONSET_DIR/icon_512x512@2x.png"

echo "🔧 Converting iconset to .icns format..."

# Convert iconset to .icns using macOS iconutil
iconutil -c icns "$ICONSET_DIR" -o assets/logo.icns

# Clean up temporary iconset directory
rm -rf "$ICONSET_DIR"

echo "✅ Successfully created assets/logo.icns"

# Also generate PNG files for in-app icons
echo ""
echo "📐 Generating PNG files for in-app icons..."

# Generate PNG files for both themes (used in sidebar and about tab)
if command -v rsvg-convert &> /dev/null; then
    rsvg-convert -w 96 -h 96 "assets/image-dark.svg" -o "assets/icon-dark.png"
    rsvg-convert -w 192 -h 192 "assets/image-dark.svg" -o "assets/icon-dark@2x.png"
    rsvg-convert -w 96 -h 96 "assets/image-light.svg" -o "assets/icon-light.png"
    rsvg-convert -w 192 -h 192 "assets/image-light.svg" -o "assets/icon-light@2x.png"
    echo "  ✓ Created icon-dark.png and icon-dark@2x.png"
    echo "  ✓ Created icon-light.png and icon-light@2x.png"
fi

echo ""
echo "📍 Generated files:"
echo "   • assets/logo.icns (app bundle icon)"
echo "   • assets/icon-dark.png (for light theme in-app)"
echo "   • assets/icon-light.png (for dark theme in-app)"
echo ""
echo "💡 Next steps:"
echo "   • Run 'cargo build' to embed the new icons"
echo "   • Use create-app-bundle.sh to build DevSweep.app"
