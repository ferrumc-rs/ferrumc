use crate::{Color, NamedColor, TextComponent, TextContent};

impl TextComponent {
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
                str.push_str(color.to_ansi_color().as_str());
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
}

impl Color {
    pub fn to_ansi_color(self) -> String {
        match self {
            Color::Named(name) => match name {
                NamedColor::Black => "\x1b[38;5;16m".to_string(),
                NamedColor::DarkBlue => "\x1b[38;5;19m".to_string(),
                NamedColor::DarkGreen => "\x1b[38;5;34m".to_string(),
                NamedColor::DarkAqua => "\x1b[38;5;37m".to_string(),
                NamedColor::DarkRed => "\x1b[38;5;124m".to_string(),
                NamedColor::Gold => "\x1b[38;5;214m".to_string(),
                NamedColor::Gray => "\x1b[38;5;250m".to_string(),
                NamedColor::DarkGray => "\x1b[38;5;240m".to_string(),
                NamedColor::Blue => "\x1b[38;5;63m".to_string(),
                NamedColor::Green => "\x1b[38;5;83m".to_string(),
                NamedColor::Aqua => "\x1b[38;5;87m".to_string(),
                NamedColor::Red => "\x1b[38;5;203m".to_string(),
                NamedColor::LightPurple => "\x1b[38;5;207m".to_string(),
                NamedColor::Yellow => "\x1b[38;5;227m".to_string(),
                NamedColor::White => "\x1b[38;5;231m".to_string(),
                NamedColor::DarkPurple => "\x1b[38;5;127m".to_string(),
            },
            Color::Hex(hex) => {
                let hex = hex.strip_prefix('#').unwrap();
                let r = u8::from_str_radix(&hex[0..2], 16).unwrap();
                let g = u8::from_str_radix(&hex[2..4], 16).unwrap();
                let b = u8::from_str_radix(&hex[4..6], 16).unwrap();

                format!("\x1b[38;2;{r};{g};{b}m")
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::{Color, NamedColor, TextComponentBuilder};

    #[test]
    #[ignore]
    fn test_ansi_print() {
        let text = TextComponentBuilder::new("hello, world!")
            .bold()
            .underlined()
            .extra(
                TextComponentBuilder::new(" im second")
                    .not_bold()
                    .not_underlined()
                    .color(Color::Named(NamedColor::DarkGray)),
            )
            .color(Color::Hex("#FF00FF".to_string()))
            .build();

        println!("{}", text.to_ansi_string());
    }
}
