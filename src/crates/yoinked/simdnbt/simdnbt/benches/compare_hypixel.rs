use std::{
    collections::HashMap,
    fs::File,
    io::{Cursor, Read},
};

use criterion::{black_box, criterion_group, criterion_main, Criterion, Throughput};
use flate2::read::GzDecoder;

pub fn bench_read_file(filename: &str, c: &mut Criterion) {
    let mut file = File::open(format!("tests/{filename}")).unwrap();
    let mut contents = Vec::new();
    file.read_to_end(&mut contents).unwrap();
    let mut src = &contents[..];

    // decode the original src so most of the time isn't spent on unzipping
    let mut decoded_src_decoder = GzDecoder::new(&mut src);
    let mut input = Vec::new();
    if decoded_src_decoder.read_to_end(&mut input).is_err() {
        // oh probably wasn't gzipped then
        input = contents;
    }
    let input = input.as_slice();

    let mut group = c.benchmark_group(format!("compare_hypixel/{filename}"));
    group.throughput(Throughput::Bytes(input.len() as u64));

    {
        // compare to make sure they decode equally
        let azalea_nbt =
            azalea_items_from_nbt(azalea_nbt::Nbt::read(&mut Cursor::new(input)).unwrap()).unwrap();
        let graphite_nbt =
            graphite_items_from_nbt(graphite_binary::nbt::decode::read(&mut &input[..]).unwrap())
                .unwrap();
        let simdnbt_nbt = simdnbt_items_from_nbt(
            simdnbt::borrow::read(&mut Cursor::new(input))
                .unwrap()
                .unwrap(),
        )
        .unwrap();

        assert_eq!(azalea_nbt, graphite_nbt);
        assert_eq!(azalea_nbt, simdnbt_nbt);
    }

    group.bench_function("azalea_parse", |b| {
        b.iter(|| {
            let input = black_box(input);
            let nbt = black_box(azalea_nbt::Nbt::read(&mut Cursor::new(input)).unwrap());
            black_box(azalea_items_from_nbt(nbt));
        })
    });

    group.bench_function("graphite_parse", |b| {
        b.iter(|| {
            let input = black_box(input);
            let nbt = black_box(graphite_binary::nbt::decode::read(&mut &input[..]).unwrap());
            // black_box(nbt);
            black_box(graphite_items_from_nbt(nbt));
        })
    });

    group.bench_function("simdnbt_parse", |b| {
        b.iter(|| {
            let input = black_box(input);
            let nbt = black_box(simdnbt::borrow::read(&mut Cursor::new(input)));
            let nbt = nbt.unwrap().unwrap();
            black_box(simdnbt_items_from_nbt(nbt));
        })
    });
}

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

fn simdnbt_items_from_nbt(nbt: simdnbt::borrow::BaseNbt) -> Option<Vec<Option<Item>>> {
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

fn azalea_items_from_nbt(nbt: azalea_nbt::Nbt) -> Option<Vec<Option<Item>>> {
    let mut items = Vec::new();
    let azalea_nbt::NbtList::Compound(item_compound_list) = nbt
        .as_compound()
        .and_then(|c| c.get(""))
        .and_then(|c| c.as_compound())
        .and_then(|c| c.get("i"))
        .and_then(|i| i.as_list())?
    else {
        return None;
    };
    for item_nbt in item_compound_list {
        // check if "id" is present, if not, skip
        let Some(id) = item_nbt.get("id") else {
            // this just means the item isn't present
            items.push(None);
            continue;
        };

        // let item_tag = item_nbt.compound("tag")?;
        // let item_extra_attributes = item_tag.compound("ExtraAttributes");
        // let item_display = item_tag.compound("display");
        let item_tag = item_nbt.get("tag")?.as_compound()?;
        let item_extra_attributes = item_tag
            .get("ExtraAttributes")
            .and_then(|e| e.as_compound());
        let item_display = item_tag.get("display").and_then(|d| d.as_compound());

        items.push(Some(Item {
            id: *id.as_short()?,
            damage: *item_nbt.get("Damage")?.as_short()?,
            count: *item_nbt.get("Count")?.as_byte()?,
            head_texture_id: item_tag
                .get("SkullOwner")
                .and_then(|skull_owner| skull_owner.as_compound())
                .and_then(|skull_owner| {
                    skull_owner
                        .get("Properties")
                        .and_then(|properties| properties.as_compound())
                })
                .and_then(|properties| {
                    properties
                        .get("textures")
                        .and_then(|textures| textures.as_list())
                })
                .and_then(|textures| {
                    if let azalea_nbt::NbtList::Compound(textures) = textures {
                        textures
                            .first()
                            .and_then(|texture| texture.get("Value"))
                            .and_then(|value| value.as_string().cloned())
                            .map(|string| string.to_string())
                    } else {
                        None
                    }
                }),
            skyblock_id: item_extra_attributes
                .and_then(|e| e.get("id"))
                .and_then(|id| id.as_string().cloned())
                .map(|string| string.to_string()),
            reforge: item_extra_attributes
                .and_then(|e| e.get("modifier"))
                .and_then(|id| id.as_string().cloned())
                .map(|string| string.to_string()),
            display: ItemDisplay {
                name: item_display
                    .and_then(|d| d.get("Name"))
                    .and_then(|n| n.as_string().cloned())
                    .unwrap_or_default()
                    .to_string(),
                lore: item_display
                    .and_then(|d| d.get("Lore"))
                    .and_then(|l| l.as_list())
                    .and_then(|l| {
                        if let azalea_nbt::NbtList::String(l) = l {
                            Some(l)
                        } else {
                            None
                        }
                    })
                    .map(|l| l.iter().map(|s| s.to_string()).collect())
                    .unwrap_or_default(),
                color: item_display
                    .and_then(|d| d.get("color"))
                    .and_then(|c| c.as_int())
                    .copied(),
                has_glint: item_extra_attributes
                    .map(|e| e.get("ench").is_some())
                    .unwrap_or_default(),
            },
            enchantments: item_extra_attributes
                .and_then(|e| e.get("enchantments"))
                .and_then(|e| e.as_compound())
                .map(|e| {
                    e.iter()
                        .map(|(k, v)| (k.to_string(), v.as_int().copied().unwrap_or_default()))
                        .collect()
                })
                .unwrap_or_default(),
            timestamp: item_extra_attributes
                .and_then(|e| e.get("timestamp"))
                .and_then(|t| t.as_string().cloned())
                .map(|string| string.to_string()),
        }));
    }
    Some(items)
}

fn graphite_items_from_nbt(nbt: graphite_binary::nbt::NBT) -> Option<Vec<Option<Item>>> {
    let mut items = Vec::new();
    for item_nbt in nbt.find_root("i").and_then(|i| nbt.iter(i))? {
        // check if "id" is present, if not, skip
        let Some(id) = nbt.find(item_nbt, "id") else {
            // this just means the item isn't present
            items.push(None);
            continue;
        };

        let item_tag = nbt.find(item_nbt, "tag")?;
        let item_extra_attributes = nbt.find(item_tag, "ExtraAttributes");
        let item_display = nbt.find(item_tag, "display");

        items.push(Some(Item {
            id: id.as_short()?,
            damage: nbt.find(item_nbt, "Damage")?.as_short()?,
            count: nbt.find(item_nbt, "Count")?.as_byte()?,

            head_texture_id: nbt
                .find(item_tag, "SkullOwner")
                .and_then(|skull_owner| nbt.find(skull_owner, "Properties"))
                .and_then(|properties| nbt.find(properties, "textures"))
                .and_then(|textures| nbt.iter(textures)?.next())
                .and_then(|texture| nbt.find(texture, "Value"))
                // the real program does some base64+json decoding here but that's unnecessary for the benchmark
                .and_then(|value| value.as_string().cloned()),
            skyblock_id: item_extra_attributes
                .and_then(|e| nbt.find(e, "id"))
                .and_then(|id| id.as_string().cloned()),
            reforge: item_extra_attributes
                .and_then(|e| nbt.find(e, "modifier"))
                .and_then(|id| id.as_string().cloned()),

            display: ItemDisplay {
                name: item_display
                    .and_then(|d| nbt.find(d, "Name"))
                    .and_then(|n| n.as_string().cloned())
                    .unwrap_or_default(),
                lore: item_display
                    .and_then(|d| nbt.find(d, "Lore"))
                    .and_then(|l| nbt.iter(l))
                    .map(|l| l.filter_map(|s| s.as_string().cloned()).collect())
                    .unwrap_or_default(),
                color: item_display
                    .and_then(|d| nbt.find(d, "color"))
                    .and_then(|c| c.as_int()),
                has_glint: item_extra_attributes
                    .map(|e| nbt.find(e, "ench").is_some())
                    .unwrap_or_default(),
            },
            enchantments: item_extra_attributes
                .and_then(|e| nbt.find(e, "enchantments"))
                .and_then(|e| nbt.iter(e))
                .map(|e| {
                    e.map(|n| {
                        (
                            nbt.find(n, "key")
                                .and_then(|k| k.as_string())
                                .cloned()
                                .unwrap_or_default(),
                            nbt.find(n, "value")
                                .and_then(|v| v.as_int())
                                .unwrap_or_default(),
                        )
                    })
                    .collect()
                })
                .unwrap_or_default(),
            timestamp: item_extra_attributes
                .and_then(|e| nbt.find(e, "timestamp"))
                .and_then(|t| t.as_string().cloned()),
        }));
    }
    Some(items)
}

fn bench(c: &mut Criterion) {
    bench_read_file("hypixel.nbt", c);
}

criterion_group!(benches, bench);
criterion_main!(benches);
