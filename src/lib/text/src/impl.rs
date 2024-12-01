use crate::*;
use ferrumc_net_codec::encode::{
    NetEncode, NetEncodeOpts, errors::NetEncodeError
};
use ferrumc_nbt::{NBTSerializable, NBTSerializeOptions};
use std::io::Write;
use std::marker::Unpin;
use tokio::io::AsyncWriteExt;
use std::fmt;
use std::ops::Add;
use std::str::FromStr;
use paste::paste;

impl From<String> for TextComponent {
    fn from(value: String) -> Self {
        Self {
            content: TextContent::Text {
                text: value,
            },
            ..Default::default()
        }
    }
}

impl From<&str> for TextComponent {
    fn from(value: &str) -> Self {
        Self {
            content: TextContent::Text {
                text: value.into(),
            },
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
            write!(f, "{}", value)
        } else {
            write!(f, "Couldn't convert to String")
        }
    }
}

impl TextComponent {
    make_setters!((Color, color), (Font, font), (String, insertion), (ClickEvent, click_event), (HoverEvent, hover_event));
    make_bool_setters!(bold, italic, underlined, strikethrough, obfuscated);

    pub fn serialize_nbt(&self) -> Vec<u8> {
        let mut vec = Vec::new();
        NBTSerializable::serialize(self, &mut vec, &NBTSerializeOptions::Network);
        vec
    }
}

impl NetEncode for TextComponent {
    fn encode<W: Write>(&self, writer: &mut W, _: &NetEncodeOpts) -> Result<(), NetEncodeError> {
        writer.write_all(&self.serialize_nbt()[..])?;
        Ok(())
    }

    async fn encode_async<W: AsyncWriteExt + Unpin>(&self, writer: &mut W, _: &NetEncodeOpts) -> Result<(), NetEncodeError>{
        writer.write_all(&self.serialize_nbt()[..]).await?;
        Ok(())
    }
}

impl Default for TextContent {
    fn default() -> Self {
        TextContent::Text {
            text: String::new(),
        }
    }
}
