use ferrumc_macros::NBTSerialize;
use serde::{Serialize, Deserialize};

#[cfg(test)]
mod tests;

mod utils;
mod builders;
mod r#impl;

pub use builders::*;
pub use utils::*;

pub type JsonTextComponent = String;

/// A TextComponent that can be a Text, Translate or Keybind.
///
#[derive(Clone, PartialEq, Debug, Serialize, Deserialize, Default, NBTSerialize)]
#[serde(rename_all = "camelCase")]
#[nbt(rename_all = "camel_case")]
pub struct TextComponent {
    #[serde(flatten)]
    #[nbt(flatten)]
    /// The content field of this TextComponent.
    ///
    /// ```ignore
    /// # use ferrumc_text::*;
    /// TextContent::Text { text: "text".to_string() };
    /// TextContent::Translate {
    ///     translate: "translation".to_string(),
    ///     with: vec![],
    /// };
    /// TextContent::Keybind { keybind: "key.jump".to_string() };
    /// ```
    pub content: TextContent,

    #[serde(default, skip_serializing_if = "Option::is_none")]
    /// The color field of this TextComponent.
    pub color: Option<Color>,

    #[serde(default, skip_serializing_if = "Option::is_none")]
    /// The font field of this TextComponent.
    pub font: Option<Font>,

    #[serde(default, skip_serializing_if = "Option::is_none")]
    /// The bold field of this TextComponent.
    pub bold: Option<bool>,

    #[serde(default, skip_serializing_if = "Option::is_none")]
    /// The italic field of this TextComponent.
    pub italic: Option<bool>,

    #[serde(default, skip_serializing_if = "Option::is_none")]
    /// The underlined field of this TextComponent.
    pub underlined: Option<bool>,

    #[serde(default, skip_serializing_if = "Option::is_none")]
    /// The strikethrough field of this TextComponent.
    pub strikethrough: Option<bool>,

    #[serde(default, skip_serializing_if = "Option::is_none")]
    /// The obfuscated field of this TextComponent.
    pub obfuscated: Option<bool>,

    #[serde(default, skip_serializing_if = "Option::is_none")]
    /// Text to be inserted into chat at the cursor when shift-clicked.
    ///
    /// Only used for messages in chat; has no effect in other locations at this time.
    pub insertion: Option<String>,

    #[serde(default, skip_serializing_if = "Option::is_none")]
    /// Defines an event that occurs when this component is clicked.
    ///
    pub click_event: Option<ClickEvent>,

    #[serde(default, skip_serializing_if = "Option::is_none")]
    /// Defines an event that occurs when this component is hovered over.
    ///
    pub hover_event: Option<HoverEvent>,

    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    #[nbt(skip_if = "Vec::is_empty")]
    /// The with field of this TextComponent.
    pub extra: Vec<TextComponent>,
}

#[derive(Clone, PartialEq, Debug, Serialize, Deserialize, NBTSerialize)]
#[serde(untagged)]
pub enum TextContent {
    Text {
        text: String,
    },
    Translate {
        translate: String,
        #[serde(default, skip_serializing_if = "Vec::is_empty")]
        #[nbt(skip_if = "Vec::is_empty")]
        with: Vec<TextComponent>,
    },
    Keybind {
        keybind: String,
    },
}
