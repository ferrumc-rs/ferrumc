#![allow(dead_code)]

use nbt_lib::{NBTDeserialize, NBTSerialize};
use std::f32::consts::PI;
use std::f64::consts::E;
use std::fs::File;
use std::io::Write;

#[derive(NBTSerialize, Debug)]
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

#[derive(NBTSerialize, Debug)]
pub struct Item {
    pub id: String,
    pub count: i8,
    pub damage: i16,
}

#[derive(NBTSerialize, Debug)]
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
                Item {
                    id: "minecraft:diamond_sword".to_string(),
                    count: 1,
                    damage: 0,
                },
                Item {
                    id: "minecraft:apple".to_string(),
                    count: 64,
                    damage: 0,
                },
                Item {
                    id: "minecraft:oak_planks".to_string(),
                    count: 32,
                    damage: 0,
                },
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
/*
#[test]
fn validate_generation() {
    let root = NBTTestStruct::new();

    let mut buffer = Vec::new();

    root.serialize(&mut buffer).unwrap();

    println!("{:?}", buffer);

    let mut file = File::create("./.etc/nbt-lib_validation.nbt").unwrap();
    file.write_all(&buffer).unwrap();

    let decode = NBTTestStruct::decode(buffer).unwrap();

    println!("{:?}", decode);

    // println!("Expected NBT data: compound + test + 1u8");
}
*/
#[test]
fn validate_codec_file() {
    // let root = NBTTestStruct::new();
    // Has all but nested fields

    let root = SimpleRoot::new();

    let mut buffer = Vec::with_capacity(1024);
    root.nbt_serialize(&mut buffer).unwrap();

    let mut file = File::create("./.etc/nbt_lib_validation.nbt").unwrap();
    file.write_all(&buffer).unwrap();
}

#[derive(NBTSerialize, NBTDeserialize, Debug)]
#[nbt(is_root)]
#[nbt(rename = "ImTheRoot")]
pub struct SimpleRoot {
    im_a_byte: i8,
    im_a_short: i16,
    im_an_int: i32,
    im_a_long: i64,
    im_a_float: f32,
    im_a_double: f64,
    im_a_string: String,
    im_a_byte_array: Vec<i8>,
    im_a_compound: SimpleChild,
    im_a_list: Vec<SimpleListInner>,
}

#[derive(Debug, NBTSerialize, NBTDeserialize)]
pub struct SimpleChild {
    im_a_child_byte: i8,
    im_a_child_string: String,
    grand_child: SimpleGrandChild,
}

#[derive(Debug, NBTSerialize, NBTDeserialize)]
pub struct SimpleGrandChild {
    im_a_grand_child_byte: i8,
    im_a_grand_child_string: String,
}

#[derive(Debug, NBTSerialize, NBTDeserialize)]
pub struct SimpleListInner {
    im_a_list_byte: i8,
    im_a_list_string: String,
}

impl SimpleRoot {
    pub fn new() -> Self {
        SimpleRoot {
            im_a_byte: 1,
            im_a_short: 2,
            im_an_int: 3,
            im_a_long: 4,
            im_a_float: PI,
            im_a_double: E,
            im_a_string: "Hello, world!".to_string(),
            im_a_byte_array: vec![1, 2, 3, 4, 5],
            im_a_compound: SimpleChild {
                im_a_child_byte: 7,
                im_a_child_string: "Hello, child!".to_string(),
                grand_child: SimpleGrandChild {
                    im_a_grand_child_byte: 6,
                    im_a_grand_child_string: "Hello, grand child!".to_string(),
                },
            },
            im_a_list: vec![
                SimpleListInner {
                    im_a_list_byte: 8,
                    im_a_list_string: "Hello, alpha!".to_string(),
                },
                SimpleListInner {
                    im_a_list_byte: 9,
                    im_a_list_string: "Hello, beta!".to_string(),
                },
                SimpleListInner {
                    im_a_list_byte: 10,
                    im_a_list_string: "Hello, gamma!".to_string(),
                },
            ],
        }
    }
}
