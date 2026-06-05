use indexmap::IndexMap;
use quote::quote;
use serde_json::Value;
use std::collections::HashMap;

use craftflow_nbt::DynNBT;

pub(crate) fn build_mapping(_: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let json_file = include_bytes!("../../../../../assets/data/registry_packets.json");
    let val: IndexMap<String, IndexMap<String, Value>> = serde_json::from_slice(json_file).unwrap();

    let mut registry_entries = vec![];

    for (reg_entry, value_set) in val {
        let mut packets = vec![];
        for (value_name, value) in &value_set {
            let mut nbt_data_buf = Vec::new();
            // The registry data is sourced from JSON, which cannot express NBT's distinct numeric
            // tags: every JSON integer would otherwise serialise as a `Long` and every real as a
            // `Double`. The vanilla client coerces numeric tags leniently and tolerates that, but
            // strict clients deserialise the registry into typed structs and reject a field whose
            // tag is not exactly what the schema expects (e.g. `dimension_type.height` must be an
            // `Int`, not a `Long`). `dimension_type` is encoded through a schema-aware converter so
            // every field carries its correct tag; all other registries keep the byte-for-byte
            // output of the previous generic path until they, too, need a schema.
            if reg_entry == "minecraft:dimension_type" {
                let nbt = dimension_type_to_nbt(value);
                craftflow_nbt::to_writer(&mut nbt_data_buf, &nbt).unwrap();
            } else {
                craftflow_nbt::to_writer(&mut nbt_data_buf, &value).unwrap();
            }
            let kv = (value_name.clone(), nbt_data_buf);
            packets.push(kv);
        }
        registry_entries.push((reg_entry, packets));
    }
    let pairs = registry_entries
        .iter()
        .map(|(key, packets)| {
            let raw_packets_data = bitcode::encode(packets);
            quote! {
                (#key.to_string(), vec![#(#raw_packets_data),*])
            }
        })
        .collect::<Vec<_>>();

    quote! {
        indexmap::IndexMap::from([
            #(#pairs),*
        ])
    }
    .into()
}

/// The NBT numeric tag a value must use, when it differs from the generic default (integers → Int,
/// reals → Double). Only the tags actually needed by the current schema overrides are listed.
#[derive(Clone, Copy)]
enum NumTag {
    Long,
    Float,
    Double,
}

/// The `dimension_type` fields whose vanilla NBT tag differs from the generic default. Every other
/// field is an `Int` (e.g. `height`, `min_y`, `logical_height`), a `Byte` boolean, a `String`, or a
/// nested compound, all of which the generic conversion already produces correctly.
fn dimension_type_field_tag(field: &str) -> Option<NumTag> {
    match field {
        // Stored as `0`/`0.x` in JSON but a float in the dimension codec.
        "ambient_light" => Some(NumTag::Float),
        // A double in the dimension codec; JSON carries it as the integer `1`.
        "coordinate_scale" => Some(NumTag::Double),
        // A long in the dimension codec (optional; present for the Nether and the End).
        "fixed_time" => Some(NumTag::Long),
        _ => None,
    }
}

/// Converts one `dimension_type` entry's JSON into correctly-typed NBT, applying the per-field tag
/// overrides above to the entry's top-level fields. Nested values (e.g. the
/// `monster_spawn_light_level` int-provider compound) use the generic conversion, which already
/// yields `Int` for their integers.
fn dimension_type_to_nbt(entry: &Value) -> DynNBT {
    let obj = entry
        .as_object()
        .expect("dimension_type entry must be a JSON object");
    let mut map = HashMap::with_capacity(obj.len());
    for (key, value) in obj {
        map.insert(
            key.clone(),
            json_to_nbt(value, dimension_type_field_tag(key)),
        );
    }
    DynNBT::Compound(map)
}

/// Generic JSON → NBT conversion with Minecraft-appropriate defaults: integers become `Int` (not
/// `Long`), reals become `Double`, and booleans become `Byte`. `force` overrides the numeric tag
/// for a single scalar where the schema demands a non-default tag; it does not propagate into
/// nested lists or compounds.
fn json_to_nbt(value: &Value, force: Option<NumTag>) -> DynNBT {
    match value {
        Value::Bool(b) => DynNBT::Byte(i8::from(*b)),
        Value::Number(n) => match force {
            Some(NumTag::Float) => DynNBT::Float(num_f64(n) as f32),
            Some(NumTag::Double) => DynNBT::Double(num_f64(n)),
            Some(NumTag::Long) => DynNBT::Long(num_i64(n)),
            None => {
                if let Some(i) = n.as_i64() {
                    // Default integers to Int (the common registry tag), widening to Long only when
                    // the value genuinely does not fit in an i32.
                    match i32::try_from(i) {
                        Ok(v) => DynNBT::Int(v),
                        Err(_) => DynNBT::Long(i),
                    }
                } else {
                    DynNBT::Double(num_f64(n))
                }
            }
        },
        Value::String(s) => DynNBT::String(s.clone()),
        Value::Array(items) => DynNBT::List(items.iter().map(|v| json_to_nbt(v, None)).collect()),
        Value::Object(obj) => {
            let mut map = HashMap::with_capacity(obj.len());
            for (key, value) in obj {
                map.insert(key.clone(), json_to_nbt(value, None));
            }
            DynNBT::Compound(map)
        }
        // Registries contain no JSON nulls; encode defensively as a zero byte rather than panicking.
        Value::Null => DynNBT::Byte(0),
    }
}

fn num_f64(n: &serde_json::Number) -> f64 {
    n.as_f64()
        .expect("registry numeric value is representable as f64")
}

fn num_i64(n: &serde_json::Number) -> i64 {
    n.as_i64()
        .expect("registry numeric value forced to an integer tag must be an integer")
}
