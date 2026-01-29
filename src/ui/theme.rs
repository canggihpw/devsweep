#![allow(dead_code)]

use gpui::*;

#[derive(Clone, Copy, PartialEq, Eq, Debug, Default)]
pub enum ThemeMode {
    #[default]
    Light,
    Dark,
}

impl ThemeMode {
    pub fn toggle(&self) -> Self {
        match self {
            ThemeMode::Light => ThemeMode::Dark,
            ThemeMode::Dark => ThemeMode::Light,
        }
    }

    pub fn is_dark(&self) -> bool {
        matches!(self, ThemeMode::Dark)
    }

    pub fn icon_path(&self) -> &'static str {
        match self {
            ThemeMode::Light => "icon-dark.png",
            ThemeMode::Dark => "icon-light.png",
        }
    }
}

// Theme struct that holds colors based on current mode
pub struct Theme;

impl Theme {
    // Base colors
    pub fn base(mode: ThemeMode) -> Rgba {
        match mode {
            ThemeMode::Light => rgb(0xeff1f5), // Catppuccin Latte
            ThemeMode::Dark => rgb(0x1e1e2e),  // Catppuccin Mocha
        }
    }

    pub fn mantle(mode: ThemeMode) -> Rgba {
        match mode {
            ThemeMode::Light => rgb(0xe6e9ef),
            ThemeMode::Dark => rgb(0x181825),
        }
    }

    /// Sidebar background - very light gray for native macOS feel
    pub fn sidebar_bg(mode: ThemeMode) -> Rgba {
        match mode {
            ThemeMode::Light => rgb(0xf9fafb), // Very light gray
            ThemeMode::Dark => rgb(0x1e1e2e),  // Keep dark for dark mode
        }
    }

    pub fn crust(mode: ThemeMode) -> Rgba {
        match mode {
            ThemeMode::Light => rgb(0xdce0e8),
            ThemeMode::Dark => rgb(0x11111b),
        }
    }

    pub fn surface0(mode: ThemeMode) -> Rgba {
        match mode {
            ThemeMode::Light => rgb(0xccd0da),
            ThemeMode::Dark => rgb(0x313244),
        }
    }

    pub fn surface1(mode: ThemeMode) -> Rgba {
        match mode {
            ThemeMode::Light => rgb(0xbcc0cc),
            ThemeMode::Dark => rgb(0x45475a),
        }
    }

    pub fn surface2(mode: ThemeMode) -> Rgba {
        match mode {
            ThemeMode::Light => rgb(0xacb0be),
            ThemeMode::Dark => rgb(0x585b70),
        }
    }

    // Text colors
    pub fn text(mode: ThemeMode) -> Rgba {
        match mode {
            ThemeMode::Light => rgb(0x4c4f69),
            ThemeMode::Dark => rgb(0xcdd6f4),
        }
    }

    pub fn subtext0(mode: ThemeMode) -> Rgba {
        match mode {
            ThemeMode::Light => rgb(0x6c6f85),
            ThemeMode::Dark => rgb(0xa6adc8),
        }
    }

    pub fn subtext1(mode: ThemeMode) -> Rgba {
        match mode {
            ThemeMode::Light => rgb(0x5c5f77),
            ThemeMode::Dark => rgb(0xbac2de),
        }
    }

    pub fn overlay0(mode: ThemeMode) -> Rgba {
        match mode {
            ThemeMode::Light => rgb(0x9ca0b0),
            ThemeMode::Dark => rgb(0x6c7086),
        }
    }

    pub fn overlay1(mode: ThemeMode) -> Rgba {
        match mode {
            ThemeMode::Light => rgb(0x8c8fa1),
            ThemeMode::Dark => rgb(0x7f849c),
        }
    }

    pub fn overlay2(mode: ThemeMode) -> Rgba {
        match mode {
            ThemeMode::Light => rgb(0x7c7f93),
            ThemeMode::Dark => rgb(0x9399b2),
        }
    }

    // Accent colors (same for both themes in Catppuccin)
    pub fn blue(mode: ThemeMode) -> Rgba {
        match mode {
            ThemeMode::Light => rgb(0x1e66f5),
            ThemeMode::Dark => rgb(0x89b4fa),
        }
    }

    pub fn green(mode: ThemeMode) -> Rgba {
        match mode {
            ThemeMode::Light => rgb(0x40a02b),
            ThemeMode::Dark => rgb(0xa6e3a1),
        }
    }

    pub fn red(mode: ThemeMode) -> Rgba {
        match mode {
            ThemeMode::Light => rgb(0xd20f39),
            ThemeMode::Dark => rgb(0xf38ba8),
        }
    }

    pub fn yellow(mode: ThemeMode) -> Rgba {
        match mode {
            ThemeMode::Light => rgb(0xdf8e1d),
            ThemeMode::Dark => rgb(0xf9e2af),
        }
    }

    pub fn peach(mode: ThemeMode) -> Rgba {
        match mode {
            ThemeMode::Light => rgb(0xfe640b),
            ThemeMode::Dark => rgb(0xfab387),
        }
    }

    pub fn mauve(mode: ThemeMode) -> Rgba {
        match mode {
            ThemeMode::Light => rgb(0x8839ef),
            ThemeMode::Dark => rgb(0xcba6f7),
        }
    }

    pub fn teal(mode: ThemeMode) -> Rgba {
        match mode {
            ThemeMode::Light => rgb(0x179299),
            ThemeMode::Dark => rgb(0x94e2d5),
        }
    }

    pub fn lavender(mode: ThemeMode) -> Rgba {
        match mode {
            ThemeMode::Light => rgb(0x7287fd),
            ThemeMode::Dark => rgb(0xb4befe),
        }
    }

    pub fn sky(mode: ThemeMode) -> Rgba {
        match mode {
            ThemeMode::Light => rgb(0x04a5e5),
            ThemeMode::Dark => rgb(0x89dceb),
        }
    }

    pub fn sapphire(mode: ThemeMode) -> Rgba {
        match mode {
            ThemeMode::Light => rgb(0x209fb5),
            ThemeMode::Dark => rgb(0x74c7ec),
        }
    }

    pub fn pink(mode: ThemeMode) -> Rgba {
        match mode {
            ThemeMode::Light => rgb(0xea76cb),
            ThemeMode::Dark => rgb(0xf5c2e7),
        }
    }

    pub fn flamingo(mode: ThemeMode) -> Rgba {
        match mode {
            ThemeMode::Light => rgb(0xdd7878),
            ThemeMode::Dark => rgb(0xf2cdcd),
        }
    }

    pub fn rosewater(mode: ThemeMode) -> Rgba {
        match mode {
            ThemeMode::Light => rgb(0xdc8a78),
            ThemeMode::Dark => rgb(0xf5e0dc),
        }
    }

    pub fn maroon(mode: ThemeMode) -> Rgba {
        match mode {
            ThemeMode::Light => rgb(0xe64553),
            ThemeMode::Dark => rgb(0xeba0ac),
        }
    }

    // Semantic colors
    pub fn success(mode: ThemeMode) -> Rgba {
        Self::green(mode)
    }

    pub fn warning(mode: ThemeMode) -> Rgba {
        Self::yellow(mode)
    }

    pub fn error(mode: ThemeMode) -> Rgba {
        Self::red(mode)
    }

    pub fn info(mode: ThemeMode) -> Rgba {
        Self::blue(mode)
    }

    // Helper methods
    pub fn transparent() -> Rgba {
        rgba(0x00000000)
    }

    pub fn border_subtle(mode: ThemeMode) -> Rgba {
        match mode {
            ThemeMode::Light => rgba(0xccd0da80),
            ThemeMode::Dark => rgba(0x31324480),
        }
    }

    pub fn blue_tint(mode: ThemeMode) -> Rgba {
        match mode {
            ThemeMode::Light => rgba(0x1e66f520),
            ThemeMode::Dark => rgba(0x89b4fa20),
        }
    }

    pub fn blue_border(mode: ThemeMode) -> Rgba {
        match mode {
            ThemeMode::Light => rgba(0x1e66f530),
            ThemeMode::Dark => rgba(0x89b4fa30),
        }
    }

    pub fn red_tint(mode: ThemeMode) -> Rgba {
        match mode {
            ThemeMode::Light => rgba(0xd20f3920),
            ThemeMode::Dark => rgba(0xf38ba820),
        }
    }

    pub fn green_tint(mode: ThemeMode) -> Rgba {
        match mode {
            ThemeMode::Light => rgba(0x40a02b20),
            ThemeMode::Dark => rgba(0xa6e3a120),
        }
    }

    pub fn yellow_tint(mode: ThemeMode) -> Rgba {
        match mode {
            ThemeMode::Light => rgba(0xdf8e1d20),
            ThemeMode::Dark => rgba(0xf9e2af20),
        }
    }

    // Hover states
    pub fn blue_hover(mode: ThemeMode) -> Rgba {
        match mode {
            ThemeMode::Light => rgb(0x1654d1),
            ThemeMode::Dark => rgb(0x7aa8f8),
        }
    }

    pub fn red_hover(mode: ThemeMode) -> Rgba {
        match mode {
            ThemeMode::Light => rgb(0xb80d31),
            ThemeMode::Dark => rgb(0xf17a9a),
        }
    }

    pub fn sapphire_hover(mode: ThemeMode) -> Rgba {
        match mode {
            ThemeMode::Light => rgb(0x1a8da3),
            ThemeMode::Dark => rgb(0x65bce0),
        }
    }

    // Active/pressed states
    pub fn blue_active(mode: ThemeMode) -> Rgba {
        match mode {
            ThemeMode::Light => rgb(0x0e47b8),
            ThemeMode::Dark => rgb(0x6b9cf6),
        }
    }

    pub fn red_active(mode: ThemeMode) -> Rgba {
        match mode {
            ThemeMode::Light => rgb(0x9e0b29),
            ThemeMode::Dark => rgb(0xee698c),
        }
    }

    pub fn sapphire_active(mode: ThemeMode) -> Rgba {
        match mode {
            ThemeMode::Light => rgb(0x167a8d),
            ThemeMode::Dark => rgb(0x56b1d4),
        }
    }
}
