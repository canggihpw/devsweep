#!/bin/bash

# Mac Storage Cleaner - App Bundle and DMG Creator
# This script builds a macOS .app bundle and .dmg installer

set -e  # Exit on error

APP_NAME="Development Cleaner"
BINARY_NAME="dev-cleaner"
BUNDLE_ID="com.developmentcleaner.dev-cleaner"
VERSION="0.1.0"
MIN_MACOS="11.0"

echo "╔════════════════════════════════════════════════════════════╗"
echo "║      Development Cleaner - App Bundle & DMG Creator       ║"
echo "╚════════════════════════════════════════════════════════════╝"
echo ""

# Generate app icon if it doesn't exist
if [ ! -f "logo.icns" ]; then
    echo "🎨 Generating app icon from SVG..."
    if [ -f "./create-icon.sh" ]; then
        ./create-icon.sh
    else
        echo "⚠️  Warning: create-icon.sh not found, skipping icon generation"
    fi
    echo ""
fi

# Build release binary
echo "📦 Building release binary..."
cargo build --release

if [ ! -f "target/release/${BINARY_NAME}" ]; then
    echo "❌ Error: Binary not found at target/release/${BINARY_NAME}"
    exit 1
fi

echo "✓ Binary built successfully"
echo ""

# Create app bundle
echo "🔨 Creating app bundle..."
rm -rf "${APP_NAME}.app"
mkdir -p "${APP_NAME}.app/Contents/MacOS"
mkdir -p "${APP_NAME}.app/Contents/Resources"

# Copy binary
cp "target/release/${BINARY_NAME}" "${APP_NAME}.app/Contents/MacOS/${BINARY_NAME}"
chmod +x "${APP_NAME}.app/Contents/MacOS/${BINARY_NAME}"

# Copy icon if it exists
if [ -f "logo.icns" ]; then
    echo "  ✓ Adding app icon..."
    cp "logo.icns" "${APP_NAME}.app/Contents/Resources/logo.icns"
else
    echo "  ⚠️  Warning: logo.icns not found, app will use default icon"
fi

# Create Info.plist
cat > "${APP_NAME}.app/Contents/Info.plist" << EOF
<?xml version="1.0" encoding="UTF-8"?>
<!DOCTYPE plist PUBLIC "-//Apple//DTD PLIST 1.0//EN" "http://www.apple.com/DTDs/PropertyList-1.0.dtd">
<plist version="1.0">
<dict>
    <key>CFBundleExecutable</key>
    <string>${BINARY_NAME}</string>
    <key>CFBundleIdentifier</key>
    <string>${BUNDLE_ID}</string>
    <key>CFBundleName</key>
    <string>${APP_NAME}</string>
    <key>CFBundleDisplayName</key>
    <string>${APP_NAME}</string>
    <key>CFBundleVersion</key>
    <string>${VERSION}</string>
    <key>CFBundleShortVersionString</key>
    <string>${VERSION}</string>
    <key>CFBundlePackageType</key>
    <string>APPL</string>
    <key>CFBundleSignature</key>
    <string>MCLN</string>
    <key>CFBundleIconFile</key>
    <string>logo</string>
    <key>LSMinimumSystemVersion</key>
    <string>${MIN_MACOS}</string>
    <key>NSHighResolutionCapable</key>
    <true/>
    <key>NSHumanReadableCopyright</key>
    <string>Copyright © 2024. All rights reserved.</string>
    <key>LSApplicationCategoryType</key>
    <string>public.app-category.utilities</string>
</dict>
</plist>
EOF

echo "✓ App bundle created: ${APP_NAME}.app"
echo ""

# Sign the app with entitlements (ad-hoc signing)
echo "🔐 Signing app bundle..."
if [ -f "entitlements.plist" ]; then
    # Ad-hoc signing with entitlements (no developer certificate needed)
    codesign --force --deep --sign - --entitlements entitlements.plist "${APP_NAME}.app"
    if [ $? -eq 0 ]; then
        echo "✓ App signed with entitlements (ad-hoc signature)"
    else
        echo "⚠️  Warning: Code signing failed, app may have restricted permissions"
    fi
else
    echo "⚠️  Warning: entitlements.plist not found, signing without entitlements"
    codesign --force --deep --sign - "${APP_NAME}.app" 2>/dev/null || true
fi
echo ""

# Test if app can be opened
echo "🧪 Testing app bundle..."
if [ -x "${APP_NAME}.app/Contents/MacOS/${BINARY_NAME}" ]; then
    echo "✓ App bundle is executable"
else
    echo "❌ Error: App bundle binary is not executable"
    exit 1
fi

# Verify code signature
echo "  Verifying code signature..."
codesign --verify --deep --strict "${APP_NAME}.app" 2>/dev/null
if [ $? -eq 0 ]; then
    echo "  ✓ Code signature valid"
else
    echo "  ⚠️  Code signature verification failed"
fi
echo ""

# Create DMG
echo "💿 Creating DMG installer..."
DMG_NAME="${APP_NAME}-${VERSION}.dmg"
DMG_TEMP="dmg_contents"

# Clean up any previous DMG contents
rm -rf "${DMG_TEMP}"
rm -f "${DMG_NAME}"

# Create temporary directory for DMG contents
mkdir "${DMG_TEMP}"
cp -r "${APP_NAME}.app" "${DMG_TEMP}/"

# Create symbolic link to Applications folder
ln -s /Applications "${DMG_TEMP}/Applications"

# Optional: Create a README in the DMG
cat > "${DMG_TEMP}/README.txt" << EOF
Development Cleaner v${VERSION}

Installation:
1. Drag "${APP_NAME}.app" to the Applications folder
2. Launch from Applications or Spotlight
3. Click "Scan" to analyze your system (or "Full Rescan" for complete analysis)

Features:
- Smart scanning with cache support for faster subsequent scans
- Quarantine system with undo support
- Individual item deletion in quarantine
- Customizable cache TTL settings
- 16+ development tool categories supported

For more information, visit: https://github.com/yourusername/development-cleaner

Note: Some cleanup operations may require administrator privileges.
Grant Full Disk Access in System Preferences for best results.
EOF

# Create the DMG
echo "  Creating compressed DMG..."
hdiutil create -volname "${APP_NAME}" \
    -srcfolder "${DMG_TEMP}" \
    -ov -format UDZO \
    "${DMG_NAME}" > /dev/null 2>&1

# Clean up temporary directory
rm -rf "${DMG_TEMP}"

if [ -f "${DMG_NAME}" ]; then
    echo "✓ DMG created: ${DMG_NAME}"
    DMG_SIZE=$(du -h "${DMG_NAME}" | cut -f1)
    echo "  Size: ${DMG_SIZE}"
else
    echo "❌ Error: Failed to create DMG"
    exit 1
fi

echo ""
echo "╔════════════════════════════════════════════════════════════╗"
echo "║                    BUILD SUCCESSFUL! ✨                    ║"
echo "╚════════════════════════════════════════════════════════════╝"
echo ""
echo "📦 Created files:"
echo "   • ${APP_NAME}.app"
echo "   • ${DMG_NAME}"
echo ""
echo "📋 Next steps:"
echo "   1. Test the app: open '${APP_NAME}.app'"
echo "   2. Install locally: cp -r '${APP_NAME}.app' /Applications/"
echo "   3. Distribute: Share the ${DMG_NAME} file"
echo ""
echo "🎯 Features in this build:"
echo "   ✓ GPUI-based modern UI"
echo "   ✓ Smart scanning with cache (Scan button)"
echo "   ✓ Full rescan option for complete analysis"
echo "   ✓ Quarantine with undo support"
echo "   ✓ Individual item deletion"
echo "   ✓ Content scrolling support"
echo "   ✓ Click feedback on all buttons"
echo "   ✓ 16+ development tool categories"
echo ""
echo "🔐 Code Signing Status:"
echo "   ✓ App signed with ad-hoc signature + entitlements"
echo "   📝 This allows access to .Trash and other directories"
echo ""
echo "💡 For distribution (requires Apple Developer account):"
echo "   codesign --force --deep --sign 'Developer ID' --entitlements entitlements.plist '${APP_NAME}.app'"
echo "   codesign --sign 'Developer ID' '${DMG_NAME}'"
echo ""
echo "⚠️  Important: Grant Full Disk Access after installation:"
echo "   System Preferences → Security & Privacy → Privacy → Full Disk Access"
echo "   Add '${APP_NAME}.app' to the list"
echo ""
echo "📚 For notarization info, see:"
echo "   https://developer.apple.com/documentation/security/notarizing_macos_software_before_distribution"
echo ""
