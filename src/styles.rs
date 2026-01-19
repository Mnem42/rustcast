use iced::Theme;
use iced::border::Radius;
use iced::widget::{button, container};
use iced::{Background, Border, Color, widget::text_input};

use crate::config::Theme as ConfigTheme;

/// Helper: mix base color with white (simple “tint”)
pub fn tint(mut c: Color, amount: f32) -> Color {
    c.r = c.r + (1.0 - c.r) * amount;
    c.g = c.g + (1.0 - c.g) * amount;
    c.b = c.b + (1.0 - c.b) * amount;
    c
}

/// Helper: apply alpha
pub fn with_alpha(mut c: Color, a: f32) -> Color {
    c.a = a;
    c
}

pub fn rustcast_text_input_style(
    theme: &Theme,
) -> impl Fn(&Theme, text_input::Status) -> text_input::Style + '_ {
    move |_, status| {
        let palette = theme.palette();
        let base_bg = palette.background;
        let surface = with_alpha(tint(base_bg, 0.06), 1.0);

        let (border_color, border_width) = match status {
            text_input::Status::Focused { .. } => (palette.text, 1.2),
            text_input::Status::Hovered => (palette.text, 1.0),
            text_input::Status::Active => (palette.text, 0.9),
            text_input::Status::Disabled => (palette.text, 0.8),
        };

        text_input::Style {
            background: Background::Color(surface),
            border: Border {
                color: border_color,
                width: border_width,
                radius: Radius::new(5),
            },
            icon: palette.text,
            placeholder: palette.text,
            value: palette.text,
            selection: palette.text,
        }
    }
}

pub fn contents_style(theme: &ConfigTheme) -> container::Style {
    container::Style {
        background: None,
        text_color: None,
        border: iced::Border {
            color: theme.text_color(0.7),
            width: 1.0,
            radius: Radius::new(14.0),
        },
        ..Default::default()
    }
}

pub fn result_button_style(theme: &ConfigTheme) -> button::Style {
    button::Style {
        text_color: theme.text_color(1.),
        background: Some(Background::Color(theme.bg_color())),
        ..Default::default()
    }
}

pub fn result_row_container_style(tile: &ConfigTheme, focused: bool) -> container::Style {
    let base = tile.bg_color();
    let row_bg = if focused {
        with_alpha(tint(base, 0.10), 1.0)
    } else {
        with_alpha(tint(base, 0.04), 1.0)
    };

    container::Style {
        background: Some(Background::Color(row_bg)),
        border: Border {
            color: if focused {
                tile.text_color(0.35)
            } else {
                tile.text_color(0.10)
            },
            width: 0.2,
            radius: Radius::new(0.),
        },
        ..Default::default()
    }
}
