use crate::*;
use valence_text::{IntoText, Text};

#[test]
fn test_to_string() {
    let component = TextComponent::from("This is a test!");
    assert_eq!(
        component.to_string(),
        "{\"text\":\"This is a test!\"}".to_string()
    );
    let component = ComponentBuilder::text("This is a test!")
        .color(NamedColor::Blue)
        .build();
    assert_eq!(
        component.to_string(),
        "{\"text\":\"This is a test!\",\"color\":\"blue\"}".to_string()
    );
    let component = ComponentBuilder::keybind("key.jump");
    assert_eq!(
        component.to_string(),
        "{\"keybind\":\"key.jump\"}".to_string()
    );
    let component = TextComponent::from("This is a test!") + TextComponent::from(" extra!");
    assert_eq!(
        component.to_string(),
        "{\"text\":\"This is a test!\",\"extra\":[{\"text\":\" extra!\"}]}".to_string()
    );
    let component = ComponentBuilder::text("This is a test!")
        .hover_event(HoverEvent::ShowText(Box::new(TextComponent::from("boo"))))
        .build();
    assert_eq!(
        component.to_string(),
        ("This is a test!".into_text().on_hover_show_text("boo")).to_string()
    );
    let component = ComponentBuilder::text("This is a test!")
        .underlined()
        .hover_event(HoverEvent::ShowText(Box::new(TextComponent::from("boo"))))
        .build();
    assert_eq!(
        component.to_string(),
        ("This is a test!"
            .into_text()
            .underlined()
            .on_hover_show_text("boo"))
        .to_string()
    );
    let component = ComponentBuilder::text("This is a test!")
        .underlined()
        .bold()
        .hover_event(HoverEvent::ShowText(Box::new(TextComponent::from("boo"))))
        .build();
    assert_eq!(
        component.to_string(),
        ("This is a test!"
            .underlined()
            .bold()
            .on_hover_show_text("boo"))
        .to_string()
    );
    let component = ComponentBuilder::keybind("key.jump");
    assert_eq!(component.to_string(), Text::keybind("key.jump").to_string());
}
