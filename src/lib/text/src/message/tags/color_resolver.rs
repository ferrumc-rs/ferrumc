use super::*;
use crate::*;
use phf::phf_map;

static COLORS: phf::Map<&'static str, NamedColor> = phf_map! {
    "black" => NamedColor::Black,
    "dark_blue" => NamedColor::DarkBlue,
    "dark_green" => NamedColor::DarkGreen,
    "dark_aqua" => NamedColor::DarkAqua,
    "dark_red" => NamedColor::DarkRed,
    "dark_purple" => NamedColor::DarkPurple,
    "gold" => NamedColor::Gold,
    "gray" => NamedColor::Gray,
    "grey" => NamedColor::Gray,
    "dark_gray" => NamedColor::DarkGray,
    "dark_grey" => NamedColor::DarkGray,
    "blue" => NamedColor::Blue,
    "green" => NamedColor::Green,
    "aqua" => NamedColor::Aqua,
    "red" => NamedColor::Red,
    "light_purple" => NamedColor::LightPurple,
    "yellow" => NamedColor::Yellow,
    "white" => NamedColor::White,
};

pub struct ColorResolver;

impl TagResolver for ColorResolver {
    fn resolve(
        &self,
        name: &str,
        builder: &mut TextComponentBuilder,
        _opts: &ParserOptions,
    ) -> Result<(), ResolveError> {
        let name = match name {
            "c" | "color" | "colour" => todo!(), // TODO: arguments
            name => name,
        };

        if let Some(color) = COLORS.get(name) {
            builder.color_mut(*color);
            Ok(())
        } else if let Some(color) = RGBColor::from_string(name) {
            builder.color_mut(Color::RGB(color));
            Ok(())
        } else {
            Err(ResolveError::ExpectedType {
                expected_type: "color",
                expected: name.to_string(),
                description: "Please use named colours or hex (#RRGGBB) colors.".to_string(),
            })
        }
    }

    fn can_process_tag(&self, name: &str) -> bool {
        match name {
            "c" | "color" | "colour" => true,
            x if COLORS.contains_key(x) => true,
            x if x.starts_with("#") && x.len() == 7 => true,
            _ => false,
        }
    }
}
