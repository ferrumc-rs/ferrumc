use crate::*;
use valence_text::{IntoText, Text};

fn bytes_to_readable_string(bytes: &[u8]) -> String {
    bytes
        .iter()
        .map(|&byte| {
            if byte.is_ascii_graphic() || byte == b' ' {
                (byte as char).to_string()
            } else {
                format!("{:02X}", byte)
            }
        })
        .collect::<Vec<String>>()
        .join(" ")
}

fn bytes_to_string(bytes: &[u8]) -> String {
    bytes
        .iter()
        .map(|&byte| {
            format!("{:02X}", byte)
        })
        .collect::<Vec<String>>()
        .join(" ")
}

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
    let component = 
        TextComponent::from("This is a test!")
        + TextComponent::from(" extra!");
    assert_eq!(
        component.to_string(),
        "{\"text\":\"This is a test!\",\"extra\":[{\"text\":\" extra!\"}]}".to_string()
    );
    let component = ComponentBuilder::text("This is a test!")
        .hover_event(HoverEvent::ShowText(Box::new(TextComponent::from("boo"))))
        .build();
    assert_eq!(
        component.to_string(), 
        ("This is a test!".into_text()
            .on_hover_show_text("boo"))
            .to_string()
    );
    let component = ComponentBuilder::text("This is a test!")
        .underlined()
        .hover_event(HoverEvent::ShowText(Box::new(TextComponent::from("boo"))))
        .build();
    assert_eq!(
        component.to_string(), 
        ("This is a test!".into_text()
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
    assert_eq!(
        component.to_string(), 
        Text::keybind("key.jump").to_string()
    );

}

use std::io::{Cursor, Write};
use ferrumc_macros::{NetEncode, packet};
use ferrumc_net_codec::{
    encode::{NetEncode, NetEncodeOpts},
    decode::{NetDecode, NetDecodeOpts},
    net_types::var_int::VarInt
};
use ferrumc_nbt::NBTSerializable;
use ferrumc_nbt::NBTSerializeOptions;
use std::fs::File;

#[derive(NetEncode)]
#[packet(packet_id = 0x6C)]
struct TestPacket {
    message: TextComponent,
    overlay: bool,
}

#[tokio::test]
#[ignore]
async fn test_serialize_to_nbt() {
    let component = ComponentBuilder::translate("chat.type.text", vec![
        ComponentBuilder::text("GStudiosX")
            .click_event(ClickEvent::SuggestCommand("/msg GStudiosX".to_string()))
            .hover_event(HoverEvent::ShowEntity {
                entity_type: "minecraft:player".to_string(),
                id: uuid::Uuid::new_v4(),
                name: Some("GStudiosX".to_string()),
            })
            .color(NamedColor::Blue)
            .build(),
        ComponentBuilder::text("Hi")
            .font("custom:test")
            .extra(ComponentBuilder::keybind("key.jump"))
            .build(),
    ]);
    //println!("{:#?}", component.color);
    println!("{}", component.to_string());
    println!("{}", bytes_to_readable_string(&component.serialize_nbt()[..]));

    println!("{}", component.serialize_nbt().len());

    //println!("\n{}", bytes_to_readable_string(&component.content.serialize_as_network()[..]));

    let mut file = File::create("foo.nbt").unwrap();
    let mut bytes = Vec::new();
    NBTSerializable::serialize(&vec![component.clone()], &mut bytes, &NBTSerializeOptions::Network);
    //file.write_all(&bytes).unwrap();
    println!("\n{}\n", bytes_to_readable_string(&bytes[..]));
    file.write_all(&component.serialize_nbt()[..]).unwrap();

    let mut cursor = Cursor::new(Vec::new());
    TestPacket::encode_async(&TestPacket {
        message: TextComponentBuilder::new("test")
            .color(NamedColor::Blue)
            .build(),
        overlay: false,
    }, &mut cursor, &NetEncodeOpts::WithLength).await.unwrap();

    println!("\n{}\n", bytes_to_string(&cursor.get_ref()[..]));

    cursor.set_position(0);

    let length = VarInt::decode(&mut cursor, &NetDecodeOpts::None).unwrap();
    let id = VarInt::decode(&mut cursor, &NetDecodeOpts::None).unwrap();

    println!("{}\n", bytes_to_string(&component.serialize_nbt()[..]));

    println!("id: {}, length: {}, left: {}", id.val, length.val, length.val as u64 - cursor.position());
    println!("{}", bytes_to_readable_string(&cursor.get_ref()[cursor.position() as usize..]));
}
