use crate::*;
use paste::paste;

/// Build a component (text, translate, keybind).
///
pub struct ComponentBuilder {
    _private: (),
}

/// The component builder for Text.
impl ComponentBuilder {
    /// a normal text component.
    #[inline]
    pub fn text<S: Into<String>>(value: S) -> TextComponentBuilder {
        TextComponentBuilder::new(value)
    }

    /// The Keybind text.
    #[inline]
    pub fn keybind<S: Into<String>>(keybind: S) -> TextComponent {
        TextComponent {
            content: TextContent::Keybind {
                keybind: keybind.into(),
            },
            ..Default::default()
        }
    }

    /// Translated text.
    #[inline]
    pub fn translate<S: Into<String>>(translate: S, with: Vec<TextComponent>) -> TextComponent {
        TextComponent {
            content: TextContent::Translate {
                translate: translate.into(),
                with,
            },
            ..Default::default()
        }
    }

    /// Space. yeah thats it. just space.
    #[inline]
    pub fn space() -> TextComponent {
        " ".into()
    }
}

/// A builder to build a TextComponent of type text.
///
/// ```rust
/// # use ferrumc_text::*;
/// _ = ComponentBuilder::text("Hello,")
///     .color(NamedColor::Red)
///     .space()
///     .extra(ComponentBuilder::text("World!"))
///     .build();
/// ```
#[derive(Default)]
pub struct TextComponentBuilder {
    /// The text.
    pub(crate) text: String,
    /// Optional Color, defaults to white.
    pub(crate) color: Option<Color>,
    /// The used font.
    pub(crate) font: Option<Font>,
    /// If the text is bold.
    pub(crate) bold: Option<bool>,
    /// If the text is italic.
    pub(crate) italic: Option<bool>,
    /// If the text is underlined.
    pub(crate) underlined: Option<bool>,
    /// If the text is struck through.
    pub(crate) strikethrough: Option<bool>,
    /// If the text is obfuscated.
    pub(crate) obfuscated: Option<bool>,
    /// If the text is inserted.
    pub(crate) insertion: Option<String>,
    /// Fires an even to show a click.
    pub(crate) click_event: Option<ClickEvent>,
    /// Fires an even to show a hover.
    pub(crate) hover_event: Option<HoverEvent>,
    /// Extras.
    pub(crate) extra: Vec<TextComponent>,
}

impl TextComponentBuilder {
    pub fn new<S: Into<String>>(value: S) -> TextComponentBuilder {
        TextComponentBuilder {
            text: value.into(),
            ..Default::default()
        }
    }

    make_setters!(
        (Color, color),
        (Font, font),
        (String, insertion),
        (ClickEvent, click_event),
        (HoverEvent, hover_event)
    );
    make_bool_setters!(bold, italic, underlined, strikethrough, obfuscated);

    /// Adds a space as the extra.
    pub fn space(self) -> Self {
        self.extra(ComponentBuilder::space())
    }

    /// Adds an extra.
    pub fn extra(mut self, component: impl Into<TextComponent>) -> Self {
        self.extra.push(component.into());
        self
    }

    /// Builds the text component.
    pub fn build(self) -> TextComponent {
        TextComponent {
            content: TextContent::Text { text: self.text },
            color: self.color,
            font: self.font,
            bold: self.bold,
            italic: self.italic,
            underlined: self.underlined,
            strikethrough: self.strikethrough,
            obfuscated: self.obfuscated,
            insertion: self.insertion,
            click_event: self.click_event,
            hover_event: self.hover_event,
            extra: self.extra,
        }
    }
}

impl From<TextComponentBuilder> for TextComponent {
    fn from(value: TextComponentBuilder) -> Self {
        value.build()
    }
}
