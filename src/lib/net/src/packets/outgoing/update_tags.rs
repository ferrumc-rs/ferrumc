//! The clientbound **Update Tags** packet (configuration state).
//!
//! Tags group registry entries (blocks, items, fluids, ...) under named sets like
//! `minecraft:water` or `minecraft:lava`. The vanilla client relies on these heavily and *will not
//! synthesise them itself*: fluid rendering picks the lava vs water sprites from the
//! `minecraft:fluid` / `minecraft:lava` / `minecraft:water` tags, the translucent-vs-opaque render
//! layer for water comes from the water tag, and entity fluid physics (the "resistance" you feel
//! wading through water) is gated on the water tag too. If this packet is never sent, the client
//! treats every fluid tag as empty, so lava falls back to the water sprite (but stays opaque) and
//! water applies no movement resistance.
//!
//! The payload is built once at startup from the extracted vanilla data:
//! * `assets/extracted/tags.json` — tag name -> list of entry names, per registry.
//! * `assets/data/registries.json` — entry name -> numeric protocol id, per registry.
//!
//! Only the built-in (non-datapack) registries are sent here; the dynamic registries (biomes,
//! damage types, ...) carry their tags through the registry sync instead.

use ferrumc_macros::{packet, NetEncode};
use ferrumc_net_codec::net_types::length_prefixed_vec::LengthPrefixedVec;
use ferrumc_net_codec::net_types::var_int::VarInt;
use lazy_static::lazy_static;
use serde_json::Value;
use std::collections::HashMap;

const TAGS_JSON: &str = include_str!("../../../../../../assets/extracted/tags.json");
const REGISTRIES_JSON: &str = include_str!("../../../../../../assets/data/registries.json");

/// Maps a `tags.json` group name to the registry identifier the client expects in the packet.
///
/// These are the built-in registries whose entries have stable numeric protocol ids in
/// `registries.json`. Datapack-driven registries (worldgen/biome, damage_type, enchantment, ...)
/// are intentionally omitted: their tags travel with the synced registry data, not here.
const SENT_REGISTRIES: &[(&str, &str)] = &[
    ("block", "minecraft:block"),
    ("item", "minecraft:item"),
    ("fluid", "minecraft:fluid"),
    ("entity_type", "minecraft:entity_type"),
    ("game_event", "minecraft:game_event"),
    ("point_of_interest_type", "minecraft:point_of_interest_type"),
];

#[derive(NetEncode)]
#[packet(packet_id = "update_tags", state = "configuration")]
pub struct UpdateTagsPacket {
    pub registries: LengthPrefixedVec<TagRegistry>,
}

#[derive(NetEncode)]
pub struct TagRegistry {
    /// Registry identifier, e.g. `minecraft:fluid`.
    pub registry: String,
    pub tags: LengthPrefixedVec<TagEntry>,
}

#[derive(NetEncode)]
pub struct TagEntry {
    /// Tag identifier, e.g. `minecraft:lava`.
    pub name: String,
    /// Numeric protocol ids of the entries in this tag.
    pub entries: LengthPrefixedVec<VarInt>,
}

/// Ensures an identifier carries an explicit namespace, defaulting to `minecraft:`.
fn namespaced(id: &str) -> String {
    if id.contains(':') {
        id.to_string()
    } else {
        format!("minecraft:{id}")
    }
}

/// Builds `entry name -> protocol id` lookups for every registry we send tags for.
fn build_registry_id_maps() -> HashMap<String, HashMap<String, i32>> {
    let registries: Value =
        serde_json::from_str(REGISTRIES_JSON).expect("registries.json should be valid JSON");
    let mut out: HashMap<String, HashMap<String, i32>> = HashMap::new();

    for (_group, registry_id) in SENT_REGISTRIES {
        let Some(entries) = registries
            .get(registry_id)
            .and_then(|r| r.get("entries"))
            .and_then(Value::as_object)
        else {
            continue;
        };

        let mut id_map = HashMap::with_capacity(entries.len());
        for (entry_name, info) in entries {
            if let Some(id) = info.get("protocol_id").and_then(Value::as_i64) {
                id_map.insert(entry_name.clone(), id as i32);
            }
        }
        out.insert((*registry_id).to_string(), id_map);
    }

    out
}

fn build_packet() -> UpdateTagsPacket {
    let tags: Value = serde_json::from_str(TAGS_JSON).expect("tags.json should be valid JSON");
    let id_maps = build_registry_id_maps();

    let mut registries = Vec::with_capacity(SENT_REGISTRIES.len());

    for (group, registry_id) in SENT_REGISTRIES {
        let Some(group_tags) = tags.get(group).and_then(Value::as_object) else {
            continue;
        };
        let Some(id_map) = id_maps.get(*registry_id) else {
            continue;
        };

        let mut tag_entries = Vec::with_capacity(group_tags.len());
        for (tag_name, values) in group_tags {
            let Some(values) = values.as_array() else {
                continue;
            };

            let mut ids = Vec::with_capacity(values.len());
            for value in values {
                let Some(entry_name) = value.as_str() else {
                    continue;
                };
                // Tag values may be unprefixed ("lava"); registry keys are namespaced.
                let key = namespaced(entry_name);
                if let Some(&id) = id_map.get(&key) {
                    ids.push(VarInt::new(id));
                }
                // Entries with no protocol id (unknown to this build) are skipped rather than
                // sent as a bogus id, which the client would reject.
            }

            tag_entries.push(TagEntry {
                name: tag_name.clone(),
                entries: LengthPrefixedVec::new(ids),
            });
        }

        registries.push(TagRegistry {
            registry: (*registry_id).to_string(),
            tags: LengthPrefixedVec::new(tag_entries),
        });
    }

    UpdateTagsPacket {
        registries: LengthPrefixedVec::new(registries),
    }
}

lazy_static! {
    /// The Update Tags packet, built once from the bundled vanilla data. Cloned cheaply when sent
    /// to each connecting client.
    pub static ref UPDATE_TAGS_PACKET: UpdateTagsPacket = build_packet();
}

#[cfg(test)]
mod tests {
    use super::*;

    fn find_registry<'a>(packet: &'a UpdateTagsPacket, id: &str) -> &'a TagRegistry {
        packet
            .registries
            .data
            .iter()
            .find(|r| r.registry == id)
            .unwrap_or_else(|| panic!("registry {id} should be present"))
    }

    fn find_tag<'a>(registry: &'a TagRegistry, name: &str) -> &'a TagEntry {
        registry
            .tags
            .data
            .iter()
            .find(|t| t.name == name)
            .unwrap_or_else(|| panic!("tag {name} should be present"))
    }

    #[test]
    fn fluid_tags_are_populated() {
        let packet = build_packet();
        let fluid = find_registry(&packet, "minecraft:fluid");

        // minecraft:lava must contain lava (4) and flowing_lava (3); minecraft:water must contain
        // water (2) and flowing_water (1). These are the tags the client uses to tell the two
        // fluids apart for rendering and physics.
        let lava = find_tag(fluid, "minecraft:lava");
        let lava_ids: Vec<i32> = lava.entries.data.iter().map(|v| v.0).collect();
        assert!(
            lava_ids.contains(&4),
            "lava tag must include lava fluid id 4"
        );
        assert!(
            lava_ids.contains(&3),
            "lava tag must include flowing_lava fluid id 3"
        );

        let water = find_tag(fluid, "minecraft:water");
        let water_ids: Vec<i32> = water.entries.data.iter().map(|v| v.0).collect();
        assert!(
            water_ids.contains(&2),
            "water tag must include water fluid id 2"
        );
        assert!(
            water_ids.contains(&1),
            "water tag must include flowing_water fluid id 1"
        );
    }

    #[test]
    fn every_sent_registry_has_tags() {
        let packet = build_packet();
        for (_group, registry_id) in SENT_REGISTRIES {
            let reg = find_registry(&packet, registry_id);
            assert!(
                !reg.tags.data.is_empty(),
                "registry {registry_id} should carry at least one tag"
            );
        }
    }

    #[test]
    fn packet_encodes_without_error() {
        use ferrumc_net_codec::encode::{NetEncode, NetEncodeOpts};
        use std::io::Cursor;

        // The packet is sent during configuration; if it fails to encode (or encodes to nonsense)
        // every client would be disconnected mid-handshake. Encode it the same way the wire path
        // does (length-prefixed) and sanity-check the output.
        let packet = build_packet();
        let mut buf = Cursor::new(Vec::new());
        packet
            .encode(&mut buf, &NetEncodeOpts::WithLength)
            .expect("update_tags must encode");
        let bytes = buf.into_inner();
        assert!(
            bytes.len() > 2,
            "encoded packet should be non-trivial, got {} bytes",
            bytes.len()
        );
    }
}
