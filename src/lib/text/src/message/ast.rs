// <root>    - empty text component
//   test    - literal
//   <blue>  - tag
//     test  - literal
//   </blue>
//   hello   - literal
// </root>
pub struct RootTag<'a> {
    pub children: Vec<Tag<'a>>,
}

pub struct ParsedTag<'a> {
    pub name: &'a str,
    pub has_end_tag: bool,
    pub children: Vec<Tag<'a>>,
}

pub enum Tag<'a> {
    Literal(&'a str),
    Tag(ParsedTag<'a>),
}
