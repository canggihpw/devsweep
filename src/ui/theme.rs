#![allow(dead_code)]

use gpui::*;

// Catppuccin Latte color palette (Light Mode)
pub struct Theme;

impl Theme {
    // Base colors
    pub fn base() -> Rgba {
        rgb(0xeff1f5) // Light background
    }

    pub fn mantle() -> Rgba {
        rgb(0xe6e9ef) // Slightly darker than base
    }

    pub fn crust() -> Rgba {
        rgb(0xdce0e8) // Border/separator color
    }

    pub fn surface0() -> Rgba {
        rgb(0xccd0da) // Surface elements
    }

    pub fn surface1() -> Rgba {
        rgb(0xbcc0cc) // Elevated surface
    }

    pub fn surface2() -> Rgba {
        rgb(0xacb0be) // More elevated surface
    }

    // Text colors
    pub fn text() -> Rgba {
        rgb(0x4c4f69) // Primary text (dark)
    }

    pub fn subtext0() -> Rgba {
        rgb(0x6c6f85) // Secondary text
    }

    pub fn subtext1() -> Rgba {
        rgb(0x5c5f77) // Between text and subtext0
    }

    pub fn overlay0() -> Rgba {
        rgb(0x9ca0b0) // Muted text
    }

    // Accent colors
    pub fn blue() -> Rgba {
        rgb(0x1e66f5) // Primary blue
    }

    pub fn green() -> Rgba {
        rgb(0x40a02b) // Success green
    }

    pub fn red() -> Rgba {
        rgb(0xd20f39) // Error red
    }

    pub fn yellow() -> Rgba {
        rgb(0xdf8e1d) // Warning yellow
    }

    pub fn peach() -> Rgba {
        rgb(0xfe640b) // Peach/orange
    }

    pub fn mauve() -> Rgba {
        rgb(0x8839ef) // Purple
    }

    pub fn teal() -> Rgba {
        rgb(0x179299) // Teal
    }

    pub fn lavender() -> Rgba {
        rgb(0x7287fd) // Lavender
    }

    pub fn sky() -> Rgba {
        rgb(0x04a5e5) // Sky blue
    }

    pub fn sapphire() -> Rgba {
        rgb(0x209fb5) // Sapphire blue
    }

    // Semantic colors
    pub fn success() -> Rgba {
        Self::green()
    }

    pub fn warning() -> Rgba {
        Self::yellow()
    }

    pub fn error() -> Rgba {
        Self::red()
    }

    pub fn info() -> Rgba {
        Self::blue()
    }

    // Helper methods for UI states and transparency

    // Transparent background (for disabled states)
    pub fn transparent() -> Rgba {
        rgba(0x00000000)
    }

    // Border colors with transparency
    pub fn border_subtle() -> Rgba {
        rgba(0xccd0da80) // surface0 with 50% opacity
    }

    // Blue tints for info/backgrounds
    pub fn blue_tint() -> Rgba {
        rgba(0x1e66f520) // blue with ~12% opacity
    }

    pub fn blue_border() -> Rgba {
        rgba(0x1e66f530) // blue with ~19% opacity
    }

    // Hover states (slightly darker in light mode)
    pub fn blue_hover() -> Rgba {
        rgb(0x1654d1) // Darker blue for hover
    }

    pub fn red_hover() -> Rgba {
        rgb(0xb80d31) // Darker red for hover
    }

    pub fn sapphire_hover() -> Rgba {
        rgb(0x1a8da3) // Darker sapphire for hover
    }

    // Active/pressed states (even darker)
    pub fn blue_active() -> Rgba {
        rgb(0x0e47b8) // Even darker blue for active
    }

    pub fn red_active() -> Rgba {
        rgb(0x9e0b29) // Even darker red for active
    }

    pub fn sapphire_active() -> Rgba {
        rgb(0x167a8d) // Even darker sapphire for active
    }
}
