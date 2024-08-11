#![allow(dead_code)]
use std::collections::HashMap;

use nbt_lib::nbt_spec::named_tag::NamedTag;
use nbt_lib::nbt_spec::serializer::NBTSerialize;
use nbt_lib::nbt_spec::tag::Tag;
use nbt_lib::nbt_spec::tag_types::TAG_COMPOUND;
use nbt_lib::Serialize;

#[derive(Serialize, Debug)]
pub struct NBTTestStruct {
    pub player_name: String,
    pub health: f32,
    pub food_level: i32,
    pub xp_level: i32,
    pub xp_total: i32,
    pub position: Vec<f64>,
    // pub inventory: Vec<Item>,
    // pub abilities: PlayerAbilities,
    // pub stats: HashMap<String, i32>,
}

#[derive(Serialize, Debug)]
pub struct Item {
    pub id: String,
    pub count: i8,
    pub damage: i16,
}

#[derive(Serialize, Debug)]
pub struct PlayerAbilities {
    pub invulnerable: bool,
    pub flying: bool,
    pub allow_flying: bool,
    pub creative_mode: bool,
}

impl NBTTestStruct {
    pub fn new() -> Self {
        NBTTestStruct {
            player_name: "Steve".to_string(),
            health: 20.0,
            food_level: 20,
            xp_level: 30,
            xp_total: 1500,
            position: vec![100.5, 64.0, -200.5],
            /*
            inventory: vec![
                Item { id: "minecraft:diamond_sword".to_string(), count: 1, damage: 0 },
                Item { id: "minecraft:apple".to_string(), count: 64, damage: 0 },
                Item { id: "minecraft:oak_planks".to_string(), count: 32, damage: 0 },
            ],
            abilities: PlayerAbilities {
                invulnerable: false,
                flying: false,
                allow_flying: true,
                creative_mode: false,
            },*/
            /*stats: {
                let mut map = HashMap::new();
                map.insert("mob_kills".to_string(), 100);
                map.insert("distance_walked".to_string(), 10000);
                map.insert("play_time".to_string(), 36000);
                map
            },*/
        }
    }

    /*pub fn to_nbt(self) -> NamedTag {
        let mut compound = HashMap::new();

        compound.insert("PlayerName".to_string(), NamedTag::new("PlayerName".to_string(), Tag::String(self.player_name)));
        compound.insert("Health".to_string(), NamedTag::new("Health".to_string(), Tag::Float(self.health)));
        compound.insert("FoodLevel".to_string(), NamedTag::new("FoodLevel".to_string(), Tag::Int(self.food_level)));
        compound.insert("XpLevel".to_string(), NamedTag::new("XpLevel".to_string(), Tag::Int(self.xp_level)));
        compound.insert("XpTotal".to_string(), NamedTag::new("XpTotal".to_string(), Tag::Int(self.xp_total)));

        let pos = Tag::List(vec![
            Tag::Double(self.position[0]),
            Tag::Double(self.position[1]),
            Tag::Double(self.position[2]),
        ]);
        compound.insert("Pos".to_string(), NamedTag::new("Pos".to_string(), pos));

        let inventory = Tag::List(self.inventory.into_iter().map(|item| {
            let mut item_compound = HashMap::new();
            item_compound.insert("id".to_string(), NamedTag::new("id".to_string(), Tag::String(item.id)));
            item_compound.insert("Count".to_string(), NamedTag::new("Count".to_string(), Tag::Byte(item.count)));
            item_compound.insert("Damage".to_string(), NamedTag::new("Damage".to_string(), Tag::Short(item.damage)));
            Tag::Compound(item_compound)
        }).collect());
        compound.insert("Inventory".to_string(), NamedTag::new("Inventory".to_string(), inventory));

        let mut abilities_compound = HashMap::new();
        abilities_compound.insert("invulnerable".to_string(), NamedTag::new("invulnerable".to_string(), Tag::Byte(self.abilities.invulnerable as i8)));
        abilities_compound.insert("flying".to_string(), NamedTag::new("flying".to_string(), Tag::Byte(self.abilities.flying as i8)));
        abilities_compound.insert("allow_flying".to_string(), NamedTag::new("allow_flying".to_string(), Tag::Byte(self.abilities.allow_flying as i8)));
        abilities_compound.insert("creative_mode".to_string(), NamedTag::new("creative_mode".to_string(), Tag::Byte(self.abilities.creative_mode as i8)));
        compound.insert("Abilities".to_string(), NamedTag::new("Abilities".to_string(), Tag::Compound(abilities_compound)));

        let stats = Tag::Compound(self.stats.into_iter().map(|(k, v)| {
            (k.clone(), NamedTag::new(k, Tag::Int(v)))
        }).collect());
        compound.insert("Stats".to_string(), NamedTag::new("Stats".to_string(), stats));

        NamedTag::new("Player".to_string(), Tag::Compound(compound))
    }*/
}

#[test]
fn validate_generation() {
    use std::fs::File;
    use std::io::Write;

    let root = NBTTestStruct::new();

    let mut buffer = Vec::new();

    TAG_COMPOUND.serialize(&mut buffer).unwrap();
    0u16.serialize(&mut buffer).unwrap();
    root.serialize(&mut buffer).unwrap();

    /*let mut buffer = Vec::new();

    TAG_COMPOUND.serialize(&mut buffer).unwrap();
    0u16.serialize(&mut buffer).unwrap();

    TAG_INT.serialize(&mut buffer).unwrap();
    "test".serialize(&mut buffer).unwrap();
    222222i32.serialize(&mut buffer).unwrap();

    {
        TAG_COMPOUND.serialize(&mut buffer).unwrap();
        "nested".serialize(&mut buffer).unwrap();

        TAG_INT.serialize(&mut buffer).unwrap();
        "nested_int".serialize(&mut buffer).unwrap();
        111i32.serialize(&mut buffer).unwrap();

        0u8.serialize(&mut buffer).unwrap();
    }

    0u8.serialize(&mut buffer).unwrap();*/

    let mut file = File::create("./.etc/nbt-lib_validation.nbt").unwrap();
    file.write_all(&buffer).unwrap();

    println!("Expected NBT data: compound + test + 1u8");
}