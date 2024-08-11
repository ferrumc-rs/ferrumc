#![allow(dead_code)]

use nbt_lib::nbt_spec::serializer::NBTSerialize;
use nbt_lib::Serialize;

#[derive(Serialize, Debug)]
#[nbt(is_root)]
#[nbt(rename = "Player")]
pub struct NBTTestStruct {
    pub player_name: String,
    pub health: f32,
    pub food_level: i32,
    pub xp_level: i32,
    pub xp_total: i32,
    pub position: Vec<f64>,
    pub inventory: Vec<Item>,
    #[nbt(rename = "Abilities")]
    pub abilities: PlayerAbilities,
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
            },
        }
    }
}

#[test]
fn validate_generation() {
    use std::fs::File;
    use std::io::Write;

    #[derive(Serialize, Debug)]
    #[nbt(is_root)]
    struct Root {
        #[nbt(rename = "test")]
        test: u8,
        optional_test: Option<u8>,
    }

    let root = Root {
        test: 1,
        optional_test: Some(2),
    };
    // let root = NBTTestStruct::new();

    let mut buffer = Vec::new();

    /*TAG_COMPOUND.serialize(&mut buffer).unwrap();
    0u16.serialize(&mut buffer).unwrap();*/
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