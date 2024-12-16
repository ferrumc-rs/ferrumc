use crate::message::{ast::*, Parser};

#[test]
fn test_parser() {
    let parser = Parser::default();
    /*match parser.parse_ast(RootTag {
        children: vec![Tag::Tag(ParsedTag {
            name: "#b81c11",
            has_end_tag: true,
            children: vec![
                Tag::Literal("Hello, World! "),
                Tag::Tag(ParsedTag {
                    name: "reset",
                    has_end_tag: false,
                    children: vec![Tag::Literal(":)")],
                }),
            ],
        })],
    }) {
        Ok(builder) => {
            println!("{}", builder.build().to_string());
        }
        Err(e) => println!("{}", e),
    }*/
}
