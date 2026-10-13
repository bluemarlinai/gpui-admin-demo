use gpui::*;
use crate::models::ThemeMode;

pub const fn c(hex: u32) -> Rgba {
    let r = ((hex >> 16) & 0xff) as f32 / 255.0;
    let g = ((hex >> 8) & 0xff) as f32 / 255.0;
    let b = (hex & 0xff) as f32 / 255.0;
    Rgba { r, g, b, a: 1.0 }
}

pub const PRIMARY_BLUE: Rgba = c(0x3b82f6);
pub const PRIMARY_BLUE_HOVER: Rgba = c(0x2563eb);
pub const ACCENT_INDIGO: Rgba = c(0x6366f1);
pub const ACCENT_CYAN: Rgba = c(0x06b6d4);
pub const ACCENT_EMERALD: Rgba = c(0x10b981);
pub const ACCENT_AMBER: Rgba = c(0xf59e0b);
pub const ACCENT_ROSE: Rgba = c(0xf43f5e);

#[derive(Debug, Clone, Copy)]
pub struct ThemePalette {
    pub is_dark: bool,
    pub bg_app: Rgba,
    pub bg_sidebar: Rgba,
    pub bg_header: Rgba,
    pub bg_card: Rgba,
    pub bg_card_alt: Rgba,
    pub bg_hover: Rgba,
    pub bg_input: Rgba,
    pub border: Rgba,
    pub text_primary: Rgba,
    pub text_secondary: Rgba,
    pub text_muted: Rgba,
    pub badge_success_bg: Rgba,
    pub badge_warn_bg: Rgba,
    pub badge_danger_bg: Rgba,
    pub badge_info_bg: Rgba,
}

impl ThemePalette {
    pub const fn dark() -> Self {
        Self {
            is_dark: true,
            bg_app: c(0x0d1117),
            bg_sidebar: c(0x131722),
            bg_header: c(0x161b26),
            bg_card: c(0x1b202e),
            bg_card_alt: c(0x23293b),
            bg_hover: c(0x2c3349),
            bg_input: c(0x141824),
            border: c(0x272d3f),
            text_primary: c(0xf3f4f6),
            text_secondary: c(0x9ca3af),
            text_muted: c(0x6b7280),
            badge_success_bg: c(0x133026),
            badge_warn_bg: c(0x352815),
            badge_danger_bg: c(0x381822),
            badge_info_bg: c(0x1a2638),
        }
    }

    pub const fn light() -> Self {
        Self {
            is_dark: false,
            bg_app: c(0xf1f5f9),
            bg_sidebar: c(0xffffff),
            bg_header: c(0xffffff),
            bg_card: c(0xffffff),
            bg_card_alt: c(0xf8fafc),
            bg_hover: c(0xe2e8f0),
            bg_input: c(0xf1f5f9),
            border: c(0xe2e8f0),
            text_primary: c(0x0f172a),
            text_secondary: c(0x475569),
            text_muted: c(0x64748b),
            badge_success_bg: c(0xd1fae5),
            badge_warn_bg: c(0xfef3c7),
            badge_danger_bg: c(0xffe4e6),
            badge_info_bg: c(0xdbeafe),
        }
    }

    pub fn resolve(mode: ThemeMode, window_appearance: WindowAppearance) -> Self {
        match mode {
            ThemeMode::Dark => Self::dark(),
            ThemeMode::Light => Self::light(),
            ThemeMode::System => match window_appearance {
                WindowAppearance::Dark | WindowAppearance::VibrantDark => Self::dark(),
                WindowAppearance::Light | WindowAppearance::VibrantLight => Self::light(),
            },
        }
    }
}
