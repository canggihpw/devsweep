#!/bin/bash

# Script to convert image.svg to logo.icns for macOS app bundle
# This creates a proper .icns file with multiple resolutions
# Used by Development Cleaner (GPUI-based app)

set -e

echo "🎨 Creating macOS app icon from SVG..."

# Check if source SVG exists
if [ ! -f "image.svg" ]; then
    echo "❌ Error: image.svg not found!"
    echo "Please place image.svg in the current directory."
    exit 1
fi

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
        rsvg-convert -w $size -h $size "image.svg" -o "$output"
    else
        # Fallback: Use sips with intermediate conversion
        # First convert SVG to PNG using qlmanage, then resize with sips
        qlmanage -t -s $size -o . "image.svg" > /dev/null 2>&1
        mv "image.svg.png" "$output" 2>/dev/null || {
            # Alternative: use macOS native conversion
            sips -s format png "image.svg" --out temp.png > /dev/null 2>&1
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
iconutil -c icns "$ICONSET_DIR" -o logo.icns

# Clean up temporary iconset directory
rm -rf "$ICONSET_DIR"

echo "✅ Successfully created logo.icns"
echo "📍 Location: $(pwd)/logo.icns"
echo ""
echo "The icon is ready to be used in your app bundle!"
echo ""
echo "💡 Next steps:"
echo "   • Use with create-app-bundle.sh to build Development Cleaner.app"
echo "   • Binary name: dev-cleaner"
echo "   • Requires: macOS 11.0+ (GPUI framework requirement)"
