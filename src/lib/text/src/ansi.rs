use crate::{Color, NamedColor, TextComponent, TextContent};

impl TextComponent {
    /// Turns this TextComponent into an ANSI string
    ///
    /// # Returns
    /// - A console-printable ANSI string
    pub fn to_ansi_string(self) -> String {
        fn to_ansi_inner(component: TextComponent) -> String {
            let mut str = String::new();

            if let Some(bold) = component.bold {
                str.push_str("\x1b[");
                str.push_str(if bold { "1" } else { "22" });
                str.push('m');
            }

            if let Some(italic) = component.italic {
                str.push_str("\x1b[");
                str.push_str(if italic { "3" } else { "23" });
                str.push('m');
            }

            if let Some(underlined) = component.underlined {
                str.push_str("\x1b[");
                str.push_str(if underlined { "4" } else { "24" });
                str.push('m');
            }

            if let Some(strikethrough) = component.strikethrough {
                str.push_str("\x1b[");
                str.push_str(if strikethrough { "9" } else { "29" });
                str.push('m');
            }

            if let Some(color) = component.color {
                str.push_str(
                    color
                        .to_ansi_color()
                        .unwrap_or("\x1b[39m".to_string())
                        .as_str(),
                );
            }

            str.push_str(
                match component.content {
                    TextContent::Text { text } => text,
                    TextContent::Translate { with, .. } => format!(
                        "Translate{:?}",
                        with.into_iter().map(to_ansi_inner).collect::<Vec<_>>()
                    ),
                    TextContent::Keybind { keybind } => format!("Keybind: {keybind}"),
                }
                .as_str(),
            );

            for component in component.extra.into_iter().map(to_ansi_inner) {
                str.push_str(&component);
            }

            str
        }

        format!("\x1b[0m{}\x1b[0m", to_ansi_inner(self))
    }

    pub fn to_plain_text(self) -> String {
        fn to_plain_text_inner(component: TextComponent) -> String {
            let mut str = String::new();

            str.push_str(
                match component.content {
                    TextContent::Text { text } => text,
                    TextContent::Translate { with, .. } => format!(
                        "Translate{:?}",
                        with.into_iter()
                            .map(to_plain_text_inner)
                            .collect::<Vec<_>>()
                    ),
                    TextContent::Keybind { keybind } => format!("Keybind: {keybind}"),
                }
                .as_str(),
            );

            for component in component.extra.into_iter().map(to_plain_text_inner) {
                str.push_str(&component);
            }

            str
        }

        to_plain_text_inner(self)
    }
}

impl Color {
    /// Returns this color as an ANSI color code
    pub fn to_ansi_color(self) -> Option<String> {
        fn ansi_from_hex(hex: &str) -> Option<String> {
            let hex = hex.strip_prefix("#")?;

            let r = u8::from_str_radix(&hex[0..2], 16).ok()?;
            let g = u8::from_str_radix(&hex[2..4], 16).ok()?;
            let b = u8::from_str_radix(&hex[4..6], 16).ok()?;

            Some(format!("\x1b[38;2;{r};{g};{b}m"))
        }

        match self {
            Color::Named(name) => match name {
                NamedColor::Black => ansi_from_hex("#000000"),
                NamedColor::DarkBlue => ansi_from_hex("#0000AA"),
                NamedColor::DarkGreen => ansi_from_hex("#00AA00"),
                NamedColor::DarkAqua => ansi_from_hex("#00AAAA"),
                NamedColor::DarkRed => ansi_from_hex("#AA0000"),
                NamedColor::Gold => ansi_from_hex("#FFAA00"),
                NamedColor::Gray => ansi_from_hex("#AAAAAA"),
                NamedColor::DarkGray => ansi_from_hex("#555555"),
                NamedColor::Blue => ansi_from_hex("#5555FF"),
                NamedColor::Green => ansi_from_hex("#55FF55"),
                NamedColor::Aqua => ansi_from_hex("#55FFFF"),
                NamedColor::Red => ansi_from_hex("#FF5555"),
                NamedColor::LightPurple => ansi_from_hex("#FF55FF"),
                NamedColor::Yellow => ansi_from_hex("#FFFF55"),
                NamedColor::White => ansi_from_hex("#FFFFFF"),
                NamedColor::DarkPurple => ansi_from_hex("#AA00AA"),
            },
            Color::Hex(hex) => ansi_from_hex(hex.as_str()),
        }
    }
}
