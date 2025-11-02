use crate::*;
use ferrumc_macros::NBTSerialize;
use serde::{Deserialize, Serialize};

#[macro_export]
macro_rules! make_bool_setters {
    ($($field:ident),*) => {
        paste! {
            $(
                pub fn $field(mut self) -> Self {
                    self.$field = Some(true);
                    self
                }

                pub fn [<not_ $field>](mut self) -> Self {
                    self.$field = Some(true);
                    self
                }

                pub fn [<clear_ $field>](mut self) -> Self {
                    self.$field = None;
                    self
                }
            )*
        }
    }
}

#[macro_export]
macro_rules! make_setters {
    ($(($ty:ident, $field:ident)),*) => {
        paste! {
            $(
                pub fn $field(mut self, $field: impl Into<$ty>) -> Self {
                    self.$field = Some($field.into());
                    self
                }

                pub fn [<clear_ $field>](mut self) -> Self {
                    self.$field = None;
                    self
                }
            )*
        }
    }
}

// TODO: better api for custom colors
/// Possibilities to print in color.
#[derive(Serialize, Deserialize, Debug, PartialEq, Clone, NBTSerialize)]
#[serde(untagged)]
#[nbt(tag_type = 8)]
pub enum Color {
    /// Names from an [enum of colors](NamedColor)
    Named(NamedColor),
    /// Color from a hex string.
    Hex(String),
}

impl From<NamedColor> for Color {
    fn from(value: NamedColor) -> Self {
        Self::Named(value)
    }
}

/// An enum representing Minecraft's predefined text colors.
///
/// These correspond to the standard Minecraft color codes
/// (e.g., §0–§f) and their modern hex equivalents.
///
/// | Name          | Color Code | Hex Value  |
/// |----------------|-------------|-------------|
/// | Black          | §0          | `#000000`   |
/// | Dark Blue      | §1          | `#0000AA`   |
/// | Dark Green     | §2          | `#00AA00`   |
/// | Dark Aqua      | §3          | `#00AAAA`   |
/// | Dark Red       | §4          | `#AA0000`   |
/// | Dark Purple    | §5          | `#AA00AA`   |
/// | Gold           | §6          | `#FFAA00`   |
/// | Gray           | §7          | `#AAAAAA`   |
/// | Dark Gray      | §8          | `#555555`   |
/// | Blue           | §9          | `#5555FF`   |
/// | Green          | §a          | `#55FF55`   |
/// | Aqua           | §b          | `#55FFFF`   |
/// | Red            | §c          | `#FF5555`   |
/// | Light Purple   | §d          | `#FF55FF`   |
/// | Yellow         | §e          | `#FFFF55`   |
/// | White (default)| §f          | `#FFFFFF`   |
///
#[derive(Serialize, Deserialize, Debug, PartialEq, Clone, Default, NBTSerialize)]
#[serde(rename_all(serialize = "snake_case"))]
#[nbt(tag_type = 8, tag = "untagged", rename_all = "snake_case")]
pub enum NamedColor {
    /// Black (`#000000`, §0)
    Black,
    /// Dark Blue (`#0000AA`, §1)
    DarkBlue,
    /// Dark Green (`#00AA00`, §2)
    DarkGreen,
    /// Dark Aqua (`#00AAAA`, §3)
    DarkAqua,
    /// Dark Red (`#AA0000`, §4)
    DarkRed,
    /// Dark Purple (`#AA00AA`, §5)
    DarkPurple,
    /// Gold (`#FFAA00`, §6)
    Gold,
    /// Gray (`#AAAAAA`, §7)
    Gray,
    /// Dark Gray (`#555555`, §8)
    DarkGray,
    /// Blue (`#5555FF`, §9)
    Blue,
    /// Green (`#55FF55`, §a)
    Green,
    /// Aqua (`#55FFFF`, §b)
    Aqua,
    /// Red (`#FF5555`, §c)
    Red,
    /// Light Purple (`#FF55FF`, §d)
    LightPurple,
    /// Yellow (`#FFFF55`, §e)
    Yellow,
    /// White (`#FFFFFF`, §f)
    #[default]
    White,
}

/// The font of the text component.
///
#[derive(Clone, PartialEq, Debug, Serialize, Deserialize, NBTSerialize)]
#[nbt(tag_type = 8, tag = "untagged")]
pub enum Font {
    /// The default font.
    #[serde(rename = "minecraft:default")]
    #[nbt(rename = "minecraft:default")]
    Default,
    /// Unicode font.
    #[serde(rename = "minecraft:uniform")]
    #[nbt(rename = "minecraft:uniform")]
    Uniform,
    /// Enchanting table font.
    #[serde(rename = "minecraft:alt")]
    #[nbt(rename = "minecraft:alt")]
    Alt,
    /// Custom font.
    #[serde(untagged)]
    Custom(String),
}

impl From<String> for Font {
    fn from(value: String) -> Self {
        Self::Custom(value)
    }
}

impl From<&str> for Font {
    fn from(value: &str) -> Self {
        Self::Custom(value.to_string())
    }
}

/// The click event of the text component
///
#[derive(Clone, PartialEq, Debug, Serialize, Deserialize, NBTSerialize)]
#[serde(
    tag = "action",
    content = "value",
    rename_all(serialize = "snake_case")
)]
#[nbt(tag = "action", content = "value", rename_all = "snake_case")]
pub enum ClickEvent {
    /// Opens an URL
    ///
    OpenUrl(String),
    /// Sends a chat command. Doesn't actually have to be a command, can be a normal chat message.
    ///
    RunCommand(String),
    /// Replaces the contents of the chat box with the text, not necessarily command.
    ///
    SuggestCommand(String),
    /// Only usable within written books. Changes the page of the book. Indexing
    /// starts at 1.
    ChangePage(i32),
    /// Copies the given text to the client's clipboard when clicked.
    ///
    CopyToClipboard(String),
}

/// The hover event of the text component
///
#[derive(Clone, PartialEq, Debug, Serialize, Deserialize, NBTSerialize)]
#[serde(
    tag = "action",
    content = "contents",
    rename_all(serialize = "snake_case")
)]
#[nbt(tag = "action", content = "contents", rename_all = "snake_case")]
pub enum HoverEvent {
    ShowText(Box<TextComponent>),
    ShowItem {
        /// The identifier of the item.
        ///
        id: String,
        /// The number of items in the item stack.
        ///
        count: u32,
        /// The item's sNBT as you would use in /give.
        ///
        tag: String,
    },
    ShowEntity {
        #[serde(rename = "type", default)]
        #[nbt(rename = "type")]
        /// Identifier of entities type.
        ///
        entity_type: String,
        /// The entities uuid.
        ///
        id: uuid::Uuid,
        /// The entities custom name.
        ///
        name: Option<String>,
    },
}
