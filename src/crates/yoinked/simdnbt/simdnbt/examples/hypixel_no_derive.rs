use std::{collections::HashMap, hint::black_box, io::Cursor};

use simdnbt::borrow::BaseNbt;

#[derive(Clone, PartialEq, Debug)]
pub struct Item {
    pub id: i16,
    pub damage: i16,
    pub count: i8,

    pub head_texture_id: Option<String>,

    pub skyblock_id: Option<String>,
    pub reforge: Option<String>,

    pub display: ItemDisplay,

    pub enchantments: HashMap<String, i32>,
    pub timestamp: Option<String>,
}

#[derive(Clone, PartialEq, Debug)]
pub struct ItemDisplay {
    pub name: String,
    pub lore: Vec<String>,

    pub has_glint: bool,

    pub color: Option<i32>,
}

fn items_from_nbt(nbt: BaseNbt) -> Option<Vec<Option<Item>>> {
    let mut items = Vec::new();
    for item_nbt in nbt
        .list("i")
        .and_then(|list| list.compounds())
        .unwrap_or_default()
    {
        // check if "id" is present, if not, skip
        if !item_nbt.contains("id") {
            // this just means the item isn't present
            items.push(None);
            continue;
        }

        let item_tag = item_nbt.compound("tag")?;
        let item_extra_attributes = item_tag.compound("ExtraAttributes");
        let item_display = item_tag.compound("display");

        items.push(Some(Item {
            id: item_nbt.short("id")?,
            damage: item_nbt.short("Damage")?,
            count: item_nbt.byte("Count")?,

            head_texture_id: item_tag
                .compound("SkullOwner")
                .and_then(|skull_owner| skull_owner.compound("Properties"))
                .and_then(|properties| properties.list("textures"))
                .and_then(|textures| textures.compounds())
                .and_then(|textures| textures.first())
                .and_then(|texture| texture.string("Value"))
                // the real program does some base64+json decoding here but that's unnecessary for the benchmark
                .map(|value| value.to_string()),
            skyblock_id: item_extra_attributes
                .and_then(|e| e.string("id"))
                .map(|id| id.to_string()),
            reforge: item_extra_attributes
                .and_then(|e| e.string("modifier"))
                .map(|id| id.to_string()),

            display: ItemDisplay {
                name: item_display
                    .and_then(|d| d.string("Name"))
                    .map(|n| n.to_string())
                    .unwrap_or_default(),
                lore: item_display
                    .and_then(|d| d.list("Lore"))
                    .and_then(|l| l.strings())
                    .map(|l| l.iter().map(|s| s.to_string()).collect())
                    .unwrap_or_default(),
                color: item_display.and_then(|d| d.int("color")),
                has_glint: item_extra_attributes
                    .map(|e| e.contains("ench"))
                    .unwrap_or_default(),
            },
            enchantments: item_extra_attributes
                .and_then(|e| e.compound("enchantments"))
                .map(|e| {
                    e.iter()
                        .map(|(k, v)| (k.to_string(), v.int().unwrap_or_default()))
                        .collect()
                })
                .unwrap_or_default(),
            timestamp: item_extra_attributes
                .and_then(|e| e.string("timestamp"))
                .map(|t| t.to_string()),
        }));
    }
    Some(items)
}

fn main() {
    let input = black_box(include_bytes!("../tests/hypixel.nbt"));

    for _ in 0..1 {
        let nbt = simdnbt::borrow::read(&mut Cursor::new(input));
        let nbt = black_box(nbt.unwrap().unwrap());
        black_box(items_from_nbt(nbt));
    }
}
