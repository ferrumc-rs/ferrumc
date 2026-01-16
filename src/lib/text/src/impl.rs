use crate::*;
use ferrumc_nbt::{FromNbt, NBTError, NBTSerializable, NBTSerializeOptions, NbtTape, NbtTapeElement};
use paste::paste;
use std::fmt;
use std::hash::{Hash, Hasher};
use std::ops::Add;
use std::str::FromStr;

impl From<String> for TextComponent {
    fn from(value: String) -> Self {
        Self {
            content: TextContent::Text { text: value },
            ..Default::default()
        }
    }
}

impl From<&str> for TextComponent {
    fn from(value: &str) -> Self {
        Self {
            content: TextContent::Text { text: value.into() },
            ..Default::default()
        }
    }
}

impl<T> Add<T> for TextComponent
where
    T: Into<TextComponent>,
{
    type Output = Self;

    fn add(mut self, other: T) -> Self {
        self.extra.push(other.into());
        self
    }
}

impl<T> Add<T> for TextComponentBuilder
where
    T: Into<TextComponent>,
{
    type Output = Self;

    fn add(mut self, other: T) -> Self {
        self.extra.push(other.into());
        self
    }
}

impl FromStr for TextComponent {
    type Err = serde_json::error::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.is_empty() {
            Ok(Self::default())
        } else {
            serde_json::from_str(s)
        }
    }
}

impl From<TextComponent> for String {
    fn from(value: TextComponent) -> String {
        serde_json::to_string(&value).unwrap()
    }
}

impl fmt::Display for TextComponent {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if let Ok(value) = serde_json::to_string(self) {
            write!(f, "{value}")
        } else {
            write!(f, "Couldn't convert to String")
        }
    }
}

impl TextComponent {
    make_setters!(
        (Color, color),
        (Font, font),
        (String, insertion),
        (ClickEvent, click_event),
        (HoverEvent, hover_event)
    );
    make_bool_setters!(bold, italic, underlined, strikethrough, obfuscated);

    pub fn serialize_nbt(&self) -> Vec<u8> {
        let mut vec = Vec::new();
        NBTSerializable::serialize(self, &mut vec, &NBTSerializeOptions::Network);
        vec
    }
}

impl Default for TextContent {
    fn default() -> Self {
        TextContent::Text {
            text: String::new(),
        }
    }
}

fn read_nbt_field<'a, T>(tape : &NbtTape<'a>, element : &NbtTapeElement<'a>, field_name : &'static str)
                         -> Result<T, NBTError>
where T : FromNbt<'a> {
    let child_element = element
        .get(field_name)
        .ok_or(NBTError::ElementNotFound(field_name))?;

    T::from_nbt(tape, child_element)
}

impl<'a> FromNbt<'a> for TextComponent {
    fn from_nbt(tapes: &NbtTape<'a>, element: &NbtTapeElement<'a>) -> ferrumc_nbt::Result<Self> {
        todo!("impl<'a> FromNbt<'a> for TextComponent")
    }
}

impl Hash for TextComponent {
    fn hash<H: Hasher>(&self, state: &mut H) {
        todo!("impl Hash for TextComponent")
    }
}

impl<'a> FromNbt<'a> for ClickEvent {
    fn from_nbt(tape: &NbtTape<'a>, element: &NbtTapeElement<'a>) -> ferrumc_nbt::Result<Self> {
        let action_field_name = "action";
        let action: String = read_nbt_field(tape, element, action_field_name)?;

        Ok(match action.as_str() {
            "open_url" => ClickEvent::OpenUrl(read_nbt_field(tape, element, "url")?),
            "run_command" => ClickEvent::RunCommand(read_nbt_field(tape, element, "path")?),
            "suggest_command" => ClickEvent::SuggestCommand(read_nbt_field(tape, element, "command")?),
            "change_page" => ClickEvent::ChangePage(read_nbt_field(tape, element, "page")?),
            "copy_to_clipboard" => ClickEvent::CopyToClipboard(read_nbt_field(tape, element, "value")?),
            _ => return Err(NBTError::UnexpectedElement(action_field_name, action)),
        })
    }
}
