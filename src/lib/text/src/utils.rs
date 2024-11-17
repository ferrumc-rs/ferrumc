use crate::*;
use serde::{Serialize, Deserialize};
use ferrumc_macros::NBTSerialize;

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
#[derive(Serialize, Deserialize, Debug, PartialEq, Clone, NBTSerialize)]
#[serde(untagged)]
#[nbt(tag_type = 8)]
pub enum Color {
    Named(NamedColor),
    Hex(String),
}

impl From<NamedColor> for Color {
    fn from(value: NamedColor) -> Self {
        Self::Named(value)
    }
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone, Default, NBTSerialize)]
#[serde(rename_all(serialize = "snake_case"))]
#[nbt(tag_type = 8, tag = "untagged", rename_all = "snake_case")]
pub enum NamedColor {
    Black,
    DarkBlue,
    DarkGreen,
    DarkAqua,
    DarkRed,
    DarkPurple,
    Gold,
    Gray,
    DarkGray,
    Blue,
    Green,
    Aqua,
    Red,
    LightPurple,
    Yellow,
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
#[serde(tag = "action", content = "value", rename_all(serialize = "snake_case"))]
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
#[serde(tag = "action", content = "contents", rename_all(serialize = "snake_case"))]
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
