use std::{
    collections::{HashMap, HashSet},
    env,
    fs::File,
    io::{BufWriter, Write},
    path::Path,
};

use simd_json::{
    self,
    base::{ValueAsObject, ValueAsScalar},
    derived::ValueTryAsObject,
};

const JSON_FILE: &[u8] = include_bytes!("../../../assets/data/blockstates.json");

fn main() {
    let mut buf = JSON_FILE.to_owned();
    let v = simd_json::to_borrowed_value(&mut buf).unwrap();
    let mut map = phf_codegen::Map::new();
    let mut prop_map = phf_codegen::Map::new();
    let mut rev_map: HashMap<String, Vec<u32>> = HashMap::new();
    let mut rev_prop: HashMap<String, HashSet<String>> = HashMap::new();
    for (key, value) in v.try_as_object().expect("object value") {
        let id = key.parse::<u32>().expect("integer value");
        let block = value.as_object().expect("object value");
        let name = block
            .get("name")
            .expect("all block states have names")
            .as_str()
            .expect("names are strings")
            .split_once("minecraft:")
            .expect("names start with \"minecraft:\"")
            .1;
        let props = block.get("properties");
        rev_map.entry(name.to_owned()).or_default().push(id);
        rev_prop.entry(name.to_string()).or_default();
        if let Some(props) = props {
            for (prop_key, prop_val) in props.as_object().expect("properties is object") {
                let map_key = format!("{}:{}", prop_key, prop_val);
                rev_prop
                    .entry(prop_key.to_string())
                    .or_default()
                    .insert(prop_val.to_string());
                rev_prop
                    .entry(name.to_string())
                    .or_default()
                    .insert(prop_key.to_string());
                rev_map.entry(map_key).or_default().push(id);
            }
        }
    }
    for (k, v) in rev_map.iter_mut() {
        v.sort();
        map.entry(k.clone(), format!("&{:?}", v));
    }

    for (k, v) in rev_prop.iter() {
        let mut v: Vec<String> = v.iter().cloned().collect();
        v.sort();
        prop_map.entry(k.clone(), format!("&{:?}", v));
    }
    let map = map.build();
    let prop_map = prop_map.build();
    let path = Path::new(&env::var("OUT_DIR").unwrap()).join("codegen.rs");
    let mut file = BufWriter::new(File::create(&path).unwrap());
    writeln!(
        &mut file,
        "static BLOCK_STATES: phf::Map<&'static str, &[u16]> = {};",
        map
    )
    .expect("able to write to file");
    writeln!(
        &mut file,
        "static PROP_PARTS: phf::Map<&'static str, &[&'static str]> = {};",
        prop_map
    )
    .expect("able to write to file");
    println!("created {}", &path.to_string_lossy());
}
