use nom_locate::LocatedSpan;

pub(crate) use crate::*;
pub mod ast;
pub mod tags;

mod errors;

#[cfg(test)]
mod tests;

pub use errors::*;

use ast::*;
use tags::*;

use nom::{
    bytes::complete::{tag, is_not},
    character::complete::char,
    sequence::{tuple, delimited},
    multi::{many0, fold_many0},
    Err, Needed,
};

pub type Span<'a> = LocatedSpan<&'a str>;
pub type IResult<'a, O> = nom::IResult<Span<'a>, O, MessageError<'a>>;

#[derive(Default)]
pub struct ParserOptions {
    pub strict: bool,
}

pub enum Context<'a> {
    Pre {
        tag: ParsedTag<'a>,
        clear_stack: bool,
    },
    Parse {
        builder: &'a mut TextComponentBuilder,
        tag: &'a ParsedTag<'a>,
    },
}

pub struct Parser {
    resolvers: Vec<Box<dyn TagResolver>>,
    opts: ParserOptions,
}

impl Parser {
    pub fn new(opts: ParserOptions) -> Self {
        Self {
            resolvers: vec![Box::new(color_resolver::ColorResolver)],
            opts,
        }
    }

    pub fn resolve<'a>(&self, context: &mut Context<'a>) -> Result<(), ResolveError> {
        for resolver in &self.resolvers {
            if resolver.call(context, &self.opts)? {
                return Ok(());
            }
        }

        match context {
            Context::Parse { tag, .. } => Err(ResolveError::NoResolverFor(tag.name.to_string())),
            Context::Pre { tag, .. } => Err(ResolveError::NoResolverFor(tag.name.to_string())),
        }
    }

    fn parse_ast_tag(&self, tag: &Tag) -> Result<TextComponentBuilder, ResolveError> {
        match tag {
            Tag::Literal(literal) => Ok(TextComponentBuilder::new(*literal)),
            Tag::Tag(tag) => {
                let mut builder = TextComponentBuilder::new("");
                self.resolve(&mut Context::Parse {
                    builder: &mut builder,
                    tag,
                })?;

                for child in &tag.children {
                    builder.extra_mut(self.parse_ast_tag(&child)?);
                }

                Ok(builder)
            }
        }
    }

    pub fn parse_ast(&self, ast: RootTag) -> Result<TextComponentBuilder, ResolveError> {
        let mut builder = TextComponentBuilder::new("");
        for child in ast.children {
            builder.extra_mut(self.parse_ast_tag(&child)?);
        }

        Ok(builder)
    }

    pub fn parse_tag<'a>(&self, input: Span<'a>) -> IResult<Context<'a>> {
        if i.len() < 3 {
            return Err(Err::Incomplete(Needed::new(3)));
        }

        let (input, name) = delimited(char('<'), is_not(">"), char('>'))(input)?;

        let tag = ParsedTag {
            name,
            has_end_tag: false,
            children: Vec::with_capacity(4),
        };
        let context = Context::Pre {
            tag,
            clear_stack: false,
        };

        Ok((input, context))
    }

    pub fn deserialize<'a>(&self, input: Span<'a>) -> IResult<RootTag<'a>> {
        let mut root = RootTag { children: Vec::with_capacity(4) };
        //let mut stack: Vec<ParsedTag<'a>> = Vec::with_capacity(16);
        let (input, left): (Span<'a>, Vec<Context<'a>>) = fold_many0(
            move |input| {
                let mut context = self.parse_tag(input)?;
                self.resolve(&mut context)?;
                context
            },
            || Vec::with_capacity(16),
            |mut acc: Vec<_>, item| {
                if let Context::Pre { .., clear_stack } = item {
                    if clear_stack {
                        let prev_node = None;
                        while let Some(mut node) = acc.pop() {
                            if let Some(Context::Pre { tag, .. }) = prev_node.as_mut() {
                                prev.children.push(tag);
                            }

                            prev_node = Some(node);
                        }

                        if let Some(Context::Pre { tag, .. }) = prev_node {
                            root.children.push(prev_node);
                        }
                    }
                } else {
                    acc.push(item);
                }
                acc
            }
        )(input)?;

        if left.len() > 0 {
            left.clear();
        }

        Ok((input, root))
    }
}

impl Default for Parser {
    fn default() -> Self {
        Self::new(ParserOptions::default())
    }
}

/*
use crate::*;
use nom::{
    branch::alt,
    bytes::complete::{is_not, tag, tag_no_case, take_until},
    character::complete::char,
    combinator::{cut, fail, opt},
    error::{context, convert_error, ErrorKind, VerboseError, VerboseErrorKind},
    multi::{many0, fold_many0},
    sequence::{delimited, preceded, tuple},
    Finish,
};

type IResult<A, B> = nom::IResult<A, B, VerboseError<A>>;

fn pretty_print_error(s: &str, mut e: VerboseError<&str>) -> String {
    let (_root_s, root_error) = e.errors[0].clone();
    if matches!(root_error, VerboseErrorKind::Nom(ErrorKind::Fail))
        || matches!(root_error, VerboseErrorKind::Nom(ErrorKind::Eof))
    {
        e.errors.remove(0);
    }
    convert_error(s, e)
}

#[derive(Clone, Debug)]
pub struct ParsedTag<'a> {
    pub name: &'a str,
    pub text: Vec<ParsedMessage<'a>>,
    pub arguments: Option<&'a str>,
}

#[derive(Clone, Debug)]
pub enum ParsedMessage<'a> {
    String(&'a str),
    ParsedTag(ParsedTag<'a>),
}

pub struct MiniMessageParser<'a> {
    resolvers: HashMap<
        &'a str,
        Box<dyn for<'b> Fn(&'b ParsedTag<'b>, TextComponentBuilder) -> TextComponentBuilder>,
    >,
    strict: bool,
}

impl<'a> MiniMessageParser<'a> {
    pub fn new(strict: bool) -> Self {
        Self {
            resolvers: HashMap::from([
                (
                    "bold",
                    Box::new(|_: &ParsedTag<'_>, builder: TextComponentBuilder| builder.bold())
                        as Box<_>,
                ),
                (
                    "italic",
                    Box::new(|_: &ParsedTag<'_>, builder: TextComponentBuilder| builder.italic())
                        as Box<_>,
                ),
                (
                    "strikethrough",
                    Box::new(|_: &ParsedTag<'_>, builder: TextComponentBuilder| builder.strikethrough())
                        as Box<_>,
                ),
            ]),
            strict,
        }
    }

    pub fn parse_start_tag(&self, input: &'a str) -> IResult<&'a str, &'a str> {
        delimited(tag("<"), is_not(">"), tag(">"))(input)
    }

    pub fn parse_end_tag(
        &self,
        input: &'a str,
        name: &'a str,
    ) -> IResult<&'a str, Option<&'a str>> {
        opt(delimited(tag("</"), tag_no_case(name), tag(">")))(input)
    }

    pub fn parse_tag_values() -> impl FnMut(&'a str) -> IResult<&'a str, Option<&'a str>> {
        preceded(char(':'), opt(take_until(">")))
    }

    pub fn parse_tag_args(
        &self,
        input: &'a str,
    ) -> IResult<&'a str, (&'a str, Option<&'a str>)> {
        tuple((take_until(":"), Self::parse_tag_values()))(input).or(Ok((input, (input, None))))
    }

    pub fn parse_text(&self, input: &'a str) -> IResult<&'a str, &'a str> {
        if input.len() == 0 {
            return context("Input is empty", fail)(input);
        }
        is_not("<")(input)
    }

    pub fn parse_tag(&self, input: &'a str) -> IResult<&'a str, ParsedTag<'a>> {
        if input.len() == 0 {
            return context("Input is empty", fail)(input);
        }

        let (input, name) = self.parse_start_tag(input)?;
        if name.starts_with("/") {
            // this is to make sure it fails if this matches with a end tag
            return context("", fail)(input);
        }

        let (_, (name, arguments)) = self.parse_tag_args(name)?;
        let (input, text) = self.parse(input)?;
        let (input, end_name) = self.parse_end_tag(input, name)?;

        if self.strict && end_name.is_none() {
            cut(context("Missing end tag", fail))(input)
        } else if end_name.unwrap_or(name) != name {
            cut(context("Expected end tag of same type", fail))(input)
        } else {
            Ok((
                input,
                ParsedTag {
                    name,
                    text,
                    arguments,
                },
            ))
        }
    }

    pub fn parse(&self, input: &'a str) -> IResult<&'a str, Vec<ParsedMessage<'a>>> {
        many0(alt((
            |input| {
                let (input, str) = self.parse_text(input)?;
                Ok((input, ParsedMessage::String(str)))
            },
            |input| {
                let (input, tag) = self.parse_tag(input)?;
                Ok((input, ParsedMessage::ParsedTag(tag)))
            },
        )))(input)
    }

    pub fn serialize_tag<'b>(
        &self,
        tags: impl Iterator<Item = &'b ParsedMessage<'b>>,
        mut builder: Option<TextComponentBuilder>,
    ) -> TextComponentBuilder {
        for tag in tags {
            match tag {
                ParsedMessage::String(str) => {
                    let component = ComponentBuilder::text(*str);
                    if let Some(b) = builder {
                        builder = Some(b.extra(component.build()));
                    } else {
                        builder = Some(component);
                    }
                }
                ParsedMessage::ParsedTag(tag) => {
                    let iter = tag.text.iter();
                    let mut component = self.serialize_tag(iter, None);

                    if let Some(tagfn) = self.resolvers.get(tag.name) {
                        component = (tagfn)(&tag, component);
                    }

                    if let Some(b) = builder {
                        builder = Some(b.extra(component.build()));
                    } else {
                        builder = Some(component);
                    }
                }
            }
        }

        builder.unwrap_or_default()
    }

    pub fn serialize(&self, input: &'a str) -> Result<TextComponent, String> {
        let (input, tag) = self
            .parse(input)
            .finish()
            .map_err(|e| pretty_print_error(input, e))?;
        Ok(self.serialize_tag(tag.iter(), Some(TextComponentBuilder::default())).build())
    }
}

impl Default for MiniMessageParser<'_> {
    fn default() -> Self {
        Self::new(false)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_tag() {
        const TEXT: &str =
            "Hi <bold>Bold <italic>Italic Text <strikethrough>Cool</bold>. <italic>Hello, World!<reset> Test</italic>";

        let parser = MiniMessageParser::new(false);
        match parser.serialize(TEXT) {
            Ok(text) => println!("{}", text.to_string()),
            Err(e) => println!("{}", e),
        };

        let parser = MiniMessageParser::new(true);
        match parser.serialize(TEXT) {
            Ok(text) => println!("{}", text.to_string()),
            Err(e) => println!("{}", e),
        };
    }
}
*/
