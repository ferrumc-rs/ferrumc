use std::{collections::HashMap, hint::black_box, io::Cursor};

use simdnbt::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct Item {
    pub id: i16,
    #[simdnbt(rename = "Damage")]
    pub damage: i16,
    #[simdnbt(rename = "Count")]
    pub count: i8,

    pub tag: ItemTag,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct ItemTag {
    #[simdnbt(rename = "SkullOwner")]
    pub skull_owner: Option<SkullOwner>,
    #[simdnbt(rename = "ExtraAttributes")]
    pub extra_attributes: ExtraAttributes,
    pub display: ItemDisplay,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct ExtraAttributes {
    pub id: Option<String>,
    pub modifier: Option<String>,

    pub ench: Option<simdnbt::owned::NbtCompound>,
    pub enchantments: Option<HashMap<String, i32>>,
    pub timestamp: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct SkullOwner {
    pub properties: Properties,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct Properties {
    pub textures: Vec<Texture>,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct Texture {
    #[simdnbt(rename = "Value")]
    pub value: String,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct ItemDisplay {
    #[simdnbt(rename = "Name")]
    pub name: String,
    #[simdnbt(rename = "Lore")]
    pub lore: Vec<String>,

    pub color: Option<i32>,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct Base {
    #[simdnbt(rename = "i")]
    pub items: Vec<Option<Item>>,
}

fn main() {
    let input = black_box(include_bytes!("../tests/hypixel.nbt"));

    for _ in 0..1 {
        let nbt = simdnbt::borrow::read(&mut Cursor::new(input));
        let nbt = black_box(nbt.unwrap().unwrap());

        let data = Base::from_nbt(&nbt).unwrap();

        // roundtrip
        let mut new_nbt_bytes = Vec::new();
        data.clone().to_nbt().write(&mut new_nbt_bytes);
        let new_nbt = simdnbt::borrow::read(&mut Cursor::new(&new_nbt_bytes[..]))
            .unwrap()
            .unwrap();
        let new_data = Base::from_nbt(&new_nbt).unwrap();
        assert_eq!(data, new_data);

        // println!("data: {:?}", data.items);
    }
}
