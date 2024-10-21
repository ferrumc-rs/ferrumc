use ferrumc_macros::{packet, NetEncode};
use ferrumc_nbt::{NBTSerializeOptions, NbtTape};
use ferrumc_net_codec::net_types::length_prefixed_vec::LengthPrefixedVec;
use std::io::Write;
use tokio::io::AsyncWriteExt;

#[derive(NetEncode)]
#[packet(packet_id = 0x07)]
pub struct RegistryDataPacket<'a> {
    pub registry_id: &'a str,
    pub entries: LengthPrefixedVec<RegistryEntry<'a>>,
}

impl<'a> RegistryDataPacket<'a> {
    pub fn new(registry_id: &'a str, entries: Vec<RegistryEntry<'a>>) -> Self {
        Self {
            registry_id,
            entries: LengthPrefixedVec::new(entries),
        }
    }
}

#[derive(NetEncode)]
pub struct RegistryEntry<'a> {
    pub id: &'a str,
    pub has_data: bool,
    pub data: Vec<u8>,
}

impl<'a> RegistryDataPacket<'a> {
    // TODO: bake this. and make it return just the bytes instead.
    pub fn get_registry_packets() -> Vec<Self> {
        let registry_nbt_buf = include_bytes!("../../../../../../.etc/registry.nbt");

        let mut tape = NbtTape::new(registry_nbt_buf);
        tape.parse();
        let mut serializer_machine = NbtTape::new(registry_nbt_buf);
        serializer_machine.parse();

        let root = tape.root.as_ref().map(|(_, b)| b).unwrap();
        let root = root.as_compound().unwrap();

        let mut packets = vec![];

        /*let (name, element) = &root[1];*/
        for (name, element) in root {
            // TOP LEVEL
            let element = element.as_compound().unwrap();

            let mut entries = vec![];
            for (name, element) in element {
                let has_data = true;
                let mut data = vec![];
                element
                    .serialize_as_network(
                        &mut serializer_machine,
                        &mut data,
                        &NBTSerializeOptions::Network,
                    )
                    .unwrap_or_else(|_| panic!("Failed to serialize entry for {}", name));

                entries.push(RegistryEntry {
                    id: name,
                    has_data,
                    data,
                });
            }
            packets.push(RegistryDataPacket::new(name, entries));
        }

        packets
    }
}

pub const fn get_registry_packets() -> &'static [u8] {
    include_bytes!("../../../../../../.etc/registry.packet")
}

#[cfg(test)]
mod tests {
    use std::io::Write;

    use ferrumc_nbt::NbtTape;
    use ferrumc_net_codec::encode::{NetEncode, NetEncodeOpts};
    use tracing::debug;

    use crate::packets::outgoing::registry_data::RegistryEntry;

    use super::RegistryDataPacket;

    // I had to almost manually type all this shit out. And y'all still hate me. smh.
    pub mod registry_parser {
        use ferrumc_macros::{NBTDeserialize, NBTSerialize};
        use serde_derive::{Deserialize, Serialize};

        #[derive(NBTSerialize, NBTDeserialize, Serialize, Deserialize)]
        pub struct Effects6 {
            pub additions_sound: AdditionsSound,
            pub ambient_sound: String,
            pub fog_color: i64,
            pub mood_sound: MoodSound,
            pub music: Music,
            pub sky_color: i64,
            pub water_color: i64,
            pub water_fog_color: i64,
        }

        #[derive(NBTSerialize, NBTDeserialize, Serialize, Deserialize)]
        pub struct MinecraftNetherWastes {
            pub downfall: f64,
            pub effects: Effects6,
            pub has_precipitation: bool,
            pub temperature: f64,
        }

        #[derive(NBTSerialize, NBTDeserialize, Serialize, Deserialize)]
        pub struct Effects5 {
            pub fog_color: i64,
            pub foliage_color: i64,
            pub grass_color_modifier: String,
            pub mood_sound: MoodSound,
            pub music: Music,
            pub sky_color: i64,
            pub water_color: i64,
            pub water_fog_color: i64,
        }

        #[derive(NBTSerialize, NBTDeserialize, Serialize, Deserialize)]
        pub struct Struct13 {
            pub downfall: f64,
            pub effects: Effects5,
            pub has_precipitation: bool,
            pub temperature: f64,
        }

        #[derive(NBTSerialize, NBTDeserialize, Serialize, Deserialize)]
        pub struct Effects4 {
            pub fog_color: i64,
            pub grass_color_modifier: String,
            pub mood_sound: MoodSound,
            pub music: Music,
            pub sky_color: i64,
            pub water_color: i64,
            pub water_fog_color: i64,
        }

        #[derive(NBTSerialize, NBTDeserialize, Serialize, Deserialize)]
        pub struct MinecraftDarkForest {
            pub downfall: f64,
            pub effects: Effects4,
            pub has_precipitation: bool,
            pub temperature: f64,
        }

        #[derive(NBTSerialize, NBTDeserialize, Serialize, Deserialize)]
        pub struct Effects3 {
            pub fog_color: i64,
            pub mood_sound: MoodSound,
            pub sky_color: i64,
            pub water_color: i64,
            pub water_fog_color: i64,
        }

        #[derive(NBTSerialize, NBTDeserialize, Serialize, Deserialize)]
        pub struct Struct12 {
            pub downfall: f64,
            pub effects: Effects3,
            pub has_precipitation: bool,
            pub temperature: f64,
        }

        #[derive(NBTSerialize, NBTDeserialize, Serialize, Deserialize)]
        pub struct Options {
            #[serde(rename = "type")]
            #[nbt(rename = "type")]
            pub r#type: String,
        }

        #[derive(NBTSerialize, NBTDeserialize, Serialize, Deserialize)]
        pub struct Particle {
            pub options: Options,
            pub probability: f64,
        }

        #[derive(NBTSerialize, NBTDeserialize, Serialize, Deserialize)]
        pub struct AdditionsSound {
            pub sound: String,
            pub tick_chance: f64,
        }

        #[derive(NBTSerialize, NBTDeserialize, Serialize, Deserialize)]
        pub struct Effects2 {
            pub additions_sound: AdditionsSound,
            pub ambient_sound: String,
            pub fog_color: i64,
            pub mood_sound: MoodSound,
            pub music: Music,
            pub particle: Particle,
            pub sky_color: i64,
            pub water_color: i64,
            pub water_fog_color: i64,
        }

        #[derive(NBTSerialize, NBTDeserialize, Serialize, Deserialize)]
        pub struct Struct11 {
            pub downfall: f64,
            pub effects: Effects2,
            pub has_precipitation: bool,
            pub temperature: f64,
        }

        #[derive(NBTSerialize, NBTDeserialize, Serialize, Deserialize)]
        pub struct Effects1 {
            pub fog_color: i64,
            pub mood_sound: MoodSound,
            pub music: Music,
            pub sky_color: i64,
            pub water_color: i64,
            pub water_fog_color: i64,
        }

        #[derive(NBTSerialize, NBTDeserialize, Serialize, Deserialize)]
        pub struct Struct10 {
            pub downfall: f64,
            pub effects: Effects1,
            pub has_precipitation: bool,
            pub temperature: f64,
        }

        #[derive(NBTSerialize, NBTDeserialize, Serialize, Deserialize)]
        pub struct Music {
            pub max_delay: i64,
            pub min_delay: i64,
            pub replace_current_music: bool,
            pub sound: String,
        }

        #[derive(NBTSerialize, NBTDeserialize, Serialize, Deserialize)]
        pub struct MoodSound {
            pub block_search_extent: i64,
            pub offset: f64,
            pub sound: String,
            pub tick_delay: i64,
        }

        #[derive(NBTSerialize, NBTDeserialize, Serialize, Deserialize)]
        pub struct Effects {
            pub fog_color: i64,
            pub foliage_color: i64,
            pub grass_color: i64,
            pub mood_sound: MoodSound,
            pub music: Music,
            pub sky_color: i64,
            pub water_color: i64,
            pub water_fog_color: i64,
        }

        #[derive(NBTSerialize, NBTDeserialize, Serialize, Deserialize)]
        pub struct Struct9 {
            pub downfall: f64,
            pub effects: Effects,
            pub has_precipitation: bool,
            pub temperature: f64,
        }

        #[derive(NBTSerialize, NBTDeserialize, Serialize, Deserialize)]
        pub struct MinecraftWorldgenBiome {
            #[serde(rename = "minecraft:badlands")]
            #[nbt(rename = "minecraft:badlands")]
            pub minecraft_badlands: Struct9,
            #[serde(rename = "minecraft:bamboo_jungle")]
            #[nbt(rename = "minecraft:bamboo_jungle")]
            pub minecraft_bamboo_jungle: Struct10,
            #[serde(rename = "minecraft:basalt_deltas")]
            #[nbt(rename = "minecraft:basalt_deltas")]
            pub minecraft_basalt_deltas: Struct11,
            #[serde(rename = "minecraft:beach")]
            #[nbt(rename = "minecraft:beach")]
            pub minecraft_beach: Struct12,
            #[serde(rename = "minecraft:birch_forest")]
            #[nbt(rename = "minecraft:birch_forest")]
            pub minecraft_birch_forest: Struct10,
            #[serde(rename = "minecraft:cherry_grove")]
            #[nbt(rename = "minecraft:cherry_grove")]
            pub minecraft_cherry_grove: Struct9,
            #[serde(rename = "minecraft:cold_ocean")]
            #[nbt(rename = "minecraft:cold_ocean")]
            pub minecraft_cold_ocean: Struct12,
            #[serde(rename = "minecraft:crimson_forest")]
            #[nbt(rename = "minecraft:crimson_forest")]
            pub minecraft_crimson_forest: Struct11,
            #[serde(rename = "minecraft:dark_forest")]
            #[nbt(rename = "minecraft:dark_forest")]
            pub minecraft_dark_forest: MinecraftDarkForest,
            #[serde(rename = "minecraft:deep_cold_ocean")]
            #[nbt(rename = "minecraft:deep_cold_ocean")]
            pub minecraft_deep_cold_ocean: Struct12,
            #[serde(rename = "minecraft:deep_dark")]
            #[nbt(rename = "minecraft:deep_dark")]
            pub minecraft_deep_dark: Struct10,
            #[serde(rename = "minecraft:deep_frozen_ocean")]
            #[nbt(rename = "minecraft:deep_frozen_ocean")]
            pub minecraft_deep_frozen_ocean: Struct12,
            #[serde(rename = "minecraft:deep_lukewarm_ocean")]
            #[nbt(rename = "minecraft:deep_lukewarm_ocean")]
            pub minecraft_deep_lukewarm_ocean: Struct12,
            #[serde(rename = "minecraft:deep_ocean")]
            #[nbt(rename = "minecraft:deep_ocean")]
            pub minecraft_deep_ocean: Struct12,
            #[serde(rename = "minecraft:desert")]
            #[nbt(rename = "minecraft:desert")]
            pub minecraft_desert: Struct10,
            #[serde(rename = "minecraft:dripstone_caves")]
            #[nbt(rename = "minecraft:dripstone_caves")]
            pub minecraft_dripstone_caves: Struct10,
            #[serde(rename = "minecraft:end_barrens")]
            #[nbt(rename = "minecraft:end_barrens")]
            pub minecraft_end_barrens: Struct12,
            #[serde(rename = "minecraft:end_highlands")]
            #[nbt(rename = "minecraft:end_highlands")]
            pub minecraft_end_highlands: Struct12,
            #[serde(rename = "minecraft:end_midlands")]
            #[nbt(rename = "minecraft:end_midlands")]
            pub minecraft_end_midlands: Struct12,
            #[serde(rename = "minecraft:eroded_badlands")]
            #[nbt(rename = "minecraft:eroded_badlands")]
            pub minecraft_eroded_badlands: Struct9,
            #[serde(rename = "minecraft:flower_forest")]
            #[nbt(rename = "minecraft:flower_forest")]
            pub minecraft_flower_forest: Struct10,
            #[serde(rename = "minecraft:forest")]
            #[nbt(rename = "minecraft:forest")]
            pub minecraft_forest: Struct10,
            #[serde(rename = "minecraft:frozen_ocean")]
            #[nbt(rename = "minecraft:frozen_ocean")]
            pub minecraft_frozen_ocean: Struct12,
            #[serde(rename = "minecraft:frozen_peaks")]
            #[nbt(rename = "minecraft:frozen_peaks")]
            pub minecraft_frozen_peaks: Struct10,
            #[serde(rename = "minecraft:frozen_river")]
            #[nbt(rename = "minecraft:frozen_river")]
            pub minecraft_frozen_river: Struct12,
            #[serde(rename = "minecraft:grove")]
            #[nbt(rename = "minecraft:grove")]
            pub minecraft_grove: Struct10,
            #[serde(rename = "minecraft:ice_spikes")]
            #[nbt(rename = "minecraft:ice_spikes")]
            pub minecraft_ice_spikes: Struct12,
            #[serde(rename = "minecraft:jagged_peaks")]
            #[nbt(rename = "minecraft:jagged_peaks")]
            pub minecraft_jagged_peaks: Struct10,
            #[serde(rename = "minecraft:jungle")]
            #[nbt(rename = "minecraft:jungle")]
            pub minecraft_jungle: Struct10,
            #[serde(rename = "minecraft:lukewarm_ocean")]
            #[nbt(rename = "minecraft:lukewarm_ocean")]
            pub minecraft_lukewarm_ocean: Struct12,
            #[serde(rename = "minecraft:lush_caves")]
            #[nbt(rename = "minecraft:lush_caves")]
            pub minecraft_lush_caves: Struct10,
            #[serde(rename = "minecraft:mangrove_swamp")]
            #[nbt(rename = "minecraft:mangrove_swamp")]
            pub minecraft_mangrove_swamp: Struct13,
            #[serde(rename = "minecraft:meadow")]
            #[nbt(rename = "minecraft:meadow")]
            pub minecraft_meadow: Struct10,
            #[serde(rename = "minecraft:mushroom_fields")]
            #[nbt(rename = "minecraft:mushroom_fields")]
            pub minecraft_mushroom_fields: Struct12,
            #[serde(rename = "minecraft:nether_wastes")]
            #[nbt(rename = "minecraft:nether_wastes")]
            pub minecraft_nether_wastes: MinecraftNetherWastes,
            #[serde(rename = "minecraft:ocean")]
            #[nbt(rename = "minecraft:ocean")]
            pub minecraft_ocean: Struct12,
            #[serde(rename = "minecraft:old_growth_birch_forest")]
            #[nbt(rename = "minecraft:old_growth_birch_forest")]
            pub minecraft_old_growth_birch_forest: Struct10,
            #[serde(rename = "minecraft:old_growth_pine_taiga")]
            #[nbt(rename = "minecraft:old_growth_pine_taiga")]
            pub minecraft_old_growth_pine_taiga: Struct10,
            #[serde(rename = "minecraft:old_growth_spruce_taiga")]
            #[nbt(rename = "minecraft:old_growth_spruce_taiga")]
            pub minecraft_old_growth_spruce_taiga: Struct10,
            #[serde(rename = "minecraft:plains")]
            #[nbt(rename = "minecraft:plains")]
            pub minecraft_plains: Struct12,
            #[serde(rename = "minecraft:river")]
            #[nbt(rename = "minecraft:river")]
            pub minecraft_river: Struct12,
            #[serde(rename = "minecraft:savanna")]
            #[nbt(rename = "minecraft:savanna")]
            pub minecraft_savanna: Struct12,
            #[serde(rename = "minecraft:savanna_plateau")]
            #[nbt(rename = "minecraft:savanna_plateau")]
            pub minecraft_savanna_plateau: Struct12,
            #[serde(rename = "minecraft:small_end_islands")]
            #[nbt(rename = "minecraft:small_end_islands")]
            pub minecraft_small_end_islands: Struct12,
            #[serde(rename = "minecraft:snowy_beach")]
            #[nbt(rename = "minecraft:snowy_beach")]
            pub minecraft_snowy_beach: Struct12,
            #[serde(rename = "minecraft:snowy_plains")]
            #[nbt(rename = "minecraft:snowy_plains")]
            pub minecraft_snowy_plains: Struct12,
            #[serde(rename = "minecraft:snowy_slopes")]
            #[nbt(rename = "minecraft:snowy_slopes")]
            pub minecraft_snowy_slopes: Struct10,
            #[serde(rename = "minecraft:snowy_taiga")]
            #[nbt(rename = "minecraft:snowy_taiga")]
            pub minecraft_snowy_taiga: Struct12,
            #[serde(rename = "minecraft:soul_sand_valley")]
            #[nbt(rename = "minecraft:soul_sand_valley")]
            pub minecraft_soul_sand_valley: Struct11,
            #[serde(rename = "minecraft:sparse_jungle")]
            #[nbt(rename = "minecraft:sparse_jungle")]
            pub minecraft_sparse_jungle: Struct10,
            #[serde(rename = "minecraft:stony_peaks")]
            #[nbt(rename = "minecraft:stony_peaks")]
            pub minecraft_stony_peaks: Struct10,
            #[serde(rename = "minecraft:stony_shore")]
            #[nbt(rename = "minecraft:stony_shore")]
            pub minecraft_stony_shore: Struct12,
            #[serde(rename = "minecraft:sunflower_plains")]
            #[nbt(rename = "minecraft:sunflower_plains")]
            pub minecraft_sunflower_plains: Struct12,
            #[serde(rename = "minecraft:swamp")]
            #[nbt(rename = "minecraft:swamp")]
            pub minecraft_swamp: Struct13,
            #[serde(rename = "minecraft:taiga")]
            #[nbt(rename = "minecraft:taiga")]
            pub minecraft_taiga: Struct12,
            #[serde(rename = "minecraft:the_end")]
            #[nbt(rename = "minecraft:the_end")]
            pub minecraft_the_end: Struct12,
            #[serde(rename = "minecraft:the_void")]
            #[nbt(rename = "minecraft:the_void")]
            pub minecraft_the_void: Struct12,
            #[serde(rename = "minecraft:warm_ocean")]
            #[nbt(rename = "minecraft:warm_ocean")]
            pub minecraft_warm_ocean: Struct12,
            #[serde(rename = "minecraft:warped_forest")]
            #[nbt(rename = "minecraft:warped_forest")]
            pub minecraft_warped_forest: Struct11,
            #[serde(rename = "minecraft:windswept_forest")]
            #[nbt(rename = "minecraft:windswept_forest")]
            pub minecraft_windswept_forest: Struct12,
            #[serde(rename = "minecraft:windswept_gravelly_hills")]
            #[nbt(rename = "minecraft:windswept_gravelly_hills")]
            pub minecraft_windswept_gravelly_hills: Struct12,
            #[serde(rename = "minecraft:windswept_hills")]
            #[nbt(rename = "minecraft:windswept_hills")]
            pub minecraft_windswept_hills: Struct12,
            #[serde(rename = "minecraft:windswept_savanna")]
            #[nbt(rename = "minecraft:windswept_savanna")]
            pub minecraft_windswept_savanna: Struct12,
            #[serde(rename = "minecraft:wooded_badlands")]
            #[nbt(rename = "minecraft:wooded_badlands")]
            pub minecraft_wooded_badlands: Struct9,
        }

        #[derive(NBTSerialize, NBTDeserialize, Serialize, Deserialize)]
        pub struct Struct8 {
            pub angry_texture: String,
            pub biomes: String,
            pub tame_texture: String,
            pub wild_texture: String,
        }

        #[derive(NBTSerialize, NBTDeserialize, Serialize, Deserialize)]
        pub struct MinecraftWolfVariant {
            #[serde(rename = "minecraft:ashen")]
            #[nbt(rename = "minecraft:ashen")]
            pub minecraft_ashen: Struct8,
            #[serde(rename = "minecraft:black")]
            #[nbt(rename = "minecraft:black")]
            pub minecraft_black: Struct8,
            #[serde(rename = "minecraft:chestnut")]
            #[nbt(rename = "minecraft:chestnut")]
            pub minecraft_chestnut: Struct8,
            #[serde(rename = "minecraft:pale")]
            #[nbt(rename = "minecraft:pale")]
            pub minecraft_pale: Struct8,
            #[serde(rename = "minecraft:rusty")]
            #[nbt(rename = "minecraft:rusty")]
            pub minecraft_rusty: Struct8,
            #[serde(rename = "minecraft:snowy")]
            #[nbt(rename = "minecraft:snowy")]
            pub minecraft_snowy: Struct8,
            #[serde(rename = "minecraft:spotted")]
            #[nbt(rename = "minecraft:spotted")]
            pub minecraft_spotted: Struct8,
            #[serde(rename = "minecraft:striped")]
            #[nbt(rename = "minecraft:striped")]
            pub minecraft_striped: Struct8,
            #[serde(rename = "minecraft:woods")]
            #[nbt(rename = "minecraft:woods")]
            pub minecraft_woods: Struct8,
        }

        #[derive(NBTSerialize, NBTDeserialize, Serialize, Deserialize)]
        pub struct Description1 {
            pub translate: String,
        }

        #[derive(NBTSerialize, NBTDeserialize, Serialize, Deserialize)]
        pub struct Struct7 {
            pub asset_id: String,
            pub description: Description1,
            pub template_item: String,
        }

        #[derive(NBTSerialize, NBTDeserialize, Serialize, Deserialize)]
        pub struct MinecraftTrimPattern {
            #[serde(rename = "minecraft:bolt")]
            #[nbt(rename = "minecraft:bolt")]
            pub minecraft_bolt: Struct7,
            #[serde(rename = "minecraft:coast")]
            #[nbt(rename = "minecraft:coast")]
            pub minecraft_coast: Struct7,
            #[serde(rename = "minecraft:dune")]
            #[nbt(rename = "minecraft:dune")]
            pub minecraft_dune: Struct7,
            #[serde(rename = "minecraft:eye")]
            #[nbt(rename = "minecraft:eye")]
            pub minecraft_eye: Struct7,
            #[serde(rename = "minecraft:flow")]
            #[nbt(rename = "minecraft:flow")]
            pub minecraft_flow: Struct7,
            #[serde(rename = "minecraft:host")]
            #[nbt(rename = "minecraft:host")]
            pub minecraft_host: Struct7,
            #[serde(rename = "minecraft:raiser")]
            #[nbt(rename = "minecraft:raiser")]
            pub minecraft_raiser: Struct7,
            #[serde(rename = "minecraft:rib")]
            #[nbt(rename = "minecraft:rib")]
            pub minecraft_rib: Struct7,
            #[serde(rename = "minecraft:sentry")]
            #[nbt(rename = "minecraft:sentry")]
            pub minecraft_sentry: Struct7,
            #[serde(rename = "minecraft:shaper")]
            #[nbt(rename = "minecraft:shaper")]
            pub minecraft_shaper: Struct7,
            #[serde(rename = "minecraft:silence")]
            #[nbt(rename = "minecraft:silence")]
            pub minecraft_silence: Struct7,
            #[serde(rename = "minecraft:snout")]
            #[nbt(rename = "minecraft:snout")]
            pub minecraft_snout: Struct7,
            #[serde(rename = "minecraft:spire")]
            #[nbt(rename = "minecraft:spire")]
            pub minecraft_spire: Struct7,
            #[serde(rename = "minecraft:tide")]
            #[nbt(rename = "minecraft:tide")]
            pub minecraft_tide: Struct7,
            #[serde(rename = "minecraft:vex")]
            #[nbt(rename = "minecraft:vex")]
            pub minecraft_vex: Struct7,
            #[serde(rename = "minecraft:ward")]
            #[nbt(rename = "minecraft:ward")]
            pub minecraft_ward: Struct7,
            #[serde(rename = "minecraft:wayfinder")]
            #[nbt(rename = "minecraft:wayfinder")]
            pub minecraft_wayfinder: Struct7,
            #[serde(rename = "minecraft:wild")]
            #[nbt(rename = "minecraft:wild")]
            pub minecraft_wild: Struct7,
        }

        #[derive(NBTSerialize, NBTDeserialize, Serialize, Deserialize)]
        pub struct Description {
            pub color: String,
            pub translate: String,
        }

        #[derive(NBTSerialize, NBTDeserialize, Serialize, Deserialize)]
        pub struct Struct6 {
            pub asset_name: String,
            pub description: Description,
            pub ingredient: String,
            pub item_model_index: f64,
        }

        #[derive(NBTSerialize, NBTDeserialize, Serialize, Deserialize)]
        pub struct MinecraftTrimMaterial {
            #[serde(rename = "minecraft:amethyst")]
            #[nbt(rename = "minecraft:amethyst")]
            pub minecraft_amethyst: Struct6,
            #[serde(rename = "minecraft:copper")]
            #[nbt(rename = "minecraft:copper")]
            pub minecraft_copper: Struct6,
            #[serde(rename = "minecraft:diamond")]
            #[nbt(rename = "minecraft:diamond")]
            pub minecraft_diamond: Struct6,
            #[serde(rename = "minecraft:emerald")]
            #[nbt(rename = "minecraft:emerald")]
            pub minecraft_emerald: Struct6,
            #[serde(rename = "minecraft:gold")]
            #[nbt(rename = "minecraft:gold")]
            pub minecraft_gold: Struct6,
            #[serde(rename = "minecraft:iron")]
            #[nbt(rename = "minecraft:iron")]
            pub minecraft_iron: Struct6,
            #[serde(rename = "minecraft:lapis")]
            #[nbt(rename = "minecraft:lapis")]
            pub minecraft_lapis: Struct6,
            #[serde(rename = "minecraft:netherite")]
            #[nbt(rename = "minecraft:netherite")]
            pub minecraft_netherite: Struct6,
            #[serde(rename = "minecraft:quartz")]
            #[nbt(rename = "minecraft:quartz")]
            pub minecraft_quartz: Struct6,
            #[serde(rename = "minecraft:redstone")]
            #[nbt(rename = "minecraft:redstone")]
            pub minecraft_redstone: Struct6,
        }

        #[derive(NBTSerialize, NBTDeserialize, Serialize, Deserialize)]
        pub struct Struct5 {
            pub asset_id: String,
            pub height: i64,
            pub width: i64,
        }

        #[derive(NBTSerialize, NBTDeserialize, Serialize, Deserialize)]
        pub struct MinecraftPaintingVariant {
            #[serde(rename = "minecraft:alban")]
            #[nbt(rename = "minecraft:alban")]
            pub minecraft_alban: Struct5,
            #[serde(rename = "minecraft:aztec")]
            #[nbt(rename = "minecraft:aztec")]
            pub minecraft_aztec: Struct5,
            #[serde(rename = "minecraft:aztec2")]
            #[nbt(rename = "minecraft:aztec2")]
            pub minecraft_aztec2: Struct5,
            #[serde(rename = "minecraft:backyard")]
            #[nbt(rename = "minecraft:backyard")]
            pub minecraft_backyard: Struct5,
            #[serde(rename = "minecraft:baroque")]
            #[nbt(rename = "minecraft:baroque")]
            pub minecraft_baroque: Struct5,
            #[serde(rename = "minecraft:bomb")]
            #[nbt(rename = "minecraft:bomb")]
            pub minecraft_bomb: Struct5,
            #[serde(rename = "minecraft:bouquet")]
            #[nbt(rename = "minecraft:bouquet")]
            pub minecraft_bouquet: Struct5,
            #[serde(rename = "minecraft:burning_skull")]
            #[nbt(rename = "minecraft:burning_skull")]
            pub minecraft_burning_skull: Struct5,
            #[serde(rename = "minecraft:bust")]
            #[nbt(rename = "minecraft:bust")]
            pub minecraft_bust: Struct5,
            #[serde(rename = "minecraft:cavebird")]
            #[nbt(rename = "minecraft:cavebird")]
            pub minecraft_cavebird: Struct5,
            #[serde(rename = "minecraft:changing")]
            #[nbt(rename = "minecraft:changing")]
            pub minecraft_changing: Struct5,
            #[serde(rename = "minecraft:cotan")]
            #[nbt(rename = "minecraft:cotan")]
            pub minecraft_cotan: Struct5,
            #[serde(rename = "minecraft:courbet")]
            #[nbt(rename = "minecraft:courbet")]
            pub minecraft_courbet: Struct5,
            #[serde(rename = "minecraft:creebet")]
            #[nbt(rename = "minecraft:creebet")]
            pub minecraft_creebet: Struct5,
            #[serde(rename = "minecraft:donkey_kong")]
            #[nbt(rename = "minecraft:donkey_kong")]
            pub minecraft_donkey_kong: Struct5,
            #[serde(rename = "minecraft:earth")]
            #[nbt(rename = "minecraft:earth")]
            pub minecraft_earth: Struct5,
            #[serde(rename = "minecraft:endboss")]
            #[nbt(rename = "minecraft:endboss")]
            pub minecraft_endboss: Struct5,
            #[serde(rename = "minecraft:fern")]
            #[nbt(rename = "minecraft:fern")]
            pub minecraft_fern: Struct5,
            #[serde(rename = "minecraft:fighters")]
            #[nbt(rename = "minecraft:fighters")]
            pub minecraft_fighters: Struct5,
            #[serde(rename = "minecraft:finding")]
            #[nbt(rename = "minecraft:finding")]
            pub minecraft_finding: Struct5,
            #[serde(rename = "minecraft:fire")]
            #[nbt(rename = "minecraft:fire")]
            pub minecraft_fire: Struct5,
            #[serde(rename = "minecraft:graham")]
            #[nbt(rename = "minecraft:graham")]
            pub minecraft_graham: Struct5,
            #[serde(rename = "minecraft:humble")]
            #[nbt(rename = "minecraft:humble")]
            pub minecraft_humble: Struct5,
            #[serde(rename = "minecraft:kebab")]
            #[nbt(rename = "minecraft:kebab")]
            pub minecraft_kebab: Struct5,
            #[serde(rename = "minecraft:lowmist")]
            #[nbt(rename = "minecraft:lowmist")]
            pub minecraft_lowmist: Struct5,
            #[serde(rename = "minecraft:match")]
            #[nbt(rename = "minecraft:match")]
            pub minecraft_match: Struct5,
            #[serde(rename = "minecraft:meditative")]
            #[nbt(rename = "minecraft:meditative")]
            pub minecraft_meditative: Struct5,
            #[serde(rename = "minecraft:orb")]
            #[nbt(rename = "minecraft:orb")]
            pub minecraft_orb: Struct5,
            #[serde(rename = "minecraft:owlemons")]
            #[nbt(rename = "minecraft:owlemons")]
            pub minecraft_owlemons: Struct5,
            #[serde(rename = "minecraft:passage")]
            #[nbt(rename = "minecraft:passage")]
            pub minecraft_passage: Struct5,
            #[serde(rename = "minecraft:pigscene")]
            #[nbt(rename = "minecraft:pigscene")]
            pub minecraft_pigscene: Struct5,
            #[serde(rename = "minecraft:plant")]
            #[nbt(rename = "minecraft:plant")]
            pub minecraft_plant: Struct5,
            #[serde(rename = "minecraft:pointer")]
            #[nbt(rename = "minecraft:pointer")]
            pub minecraft_pointer: Struct5,
            #[serde(rename = "minecraft:pond")]
            #[nbt(rename = "minecraft:pond")]
            pub minecraft_pond: Struct5,
            #[serde(rename = "minecraft:pool")]
            #[nbt(rename = "minecraft:pool")]
            pub minecraft_pool: Struct5,
            #[serde(rename = "minecraft:prairie_ride")]
            #[nbt(rename = "minecraft:prairie_ride")]
            pub minecraft_prairie_ride: Struct5,
            #[serde(rename = "minecraft:sea")]
            #[nbt(rename = "minecraft:sea")]
            pub minecraft_sea: Struct5,
            #[serde(rename = "minecraft:skeleton")]
            #[nbt(rename = "minecraft:skeleton")]
            pub minecraft_skeleton: Struct5,
            #[serde(rename = "minecraft:skull_and_roses")]
            #[nbt(rename = "minecraft:skull_and_roses")]
            pub minecraft_skull_and_roses: Struct5,
            #[serde(rename = "minecraft:stage")]
            #[nbt(rename = "minecraft:stage")]
            pub minecraft_stage: Struct5,
            #[serde(rename = "minecraft:sunflowers")]
            #[nbt(rename = "minecraft:sunflowers")]
            pub minecraft_sunflowers: Struct5,
            #[serde(rename = "minecraft:sunset")]
            #[nbt(rename = "minecraft:sunset")]
            pub minecraft_sunset: Struct5,
            #[serde(rename = "minecraft:tides")]
            #[nbt(rename = "minecraft:tides")]
            pub minecraft_tides: Struct5,
            #[serde(rename = "minecraft:unpacked")]
            #[nbt(rename = "minecraft:unpacked")]
            pub minecraft_unpacked: Struct5,
            #[serde(rename = "minecraft:void")]
            #[nbt(rename = "minecraft:void")]
            pub minecraft_void: Struct5,
            #[serde(rename = "minecraft:wanderer")]
            #[nbt(rename = "minecraft:wanderer")]
            pub minecraft_wanderer: Struct5,
            #[serde(rename = "minecraft:wasteland")]
            #[nbt(rename = "minecraft:wasteland")]
            pub minecraft_wasteland: Struct5,
            #[serde(rename = "minecraft:water")]
            #[nbt(rename = "minecraft:water")]
            pub minecraft_water: Struct5,
            #[serde(rename = "minecraft:wind")]
            #[nbt(rename = "minecraft:wind")]
            pub minecraft_wind: Struct5,
            #[serde(rename = "minecraft:wither")]
            #[nbt(rename = "minecraft:wither")]
            pub minecraft_wither: Struct5,
        }

        #[derive(NBTSerialize, NBTDeserialize, Serialize, Deserialize)]
        pub struct MonsterSpawnLightLevel {
            pub max_inclusive: i64,
            pub min_inclusive: i64,
            #[serde(rename = "type")]
            #[nbt(rename = "type")]
            pub r#type: String,
        }

        #[derive(NBTSerialize, NBTDeserialize, Serialize, Deserialize)]
        pub struct Struct4 {
            pub ambient_light: f64,
            pub bed_works: i64,
            pub coordinate_scale: f64,
            pub effects: String,
            pub has_ceiling: i64,
            pub has_raids: i64,
            pub has_skylight: i64,
            pub height: i64,
            pub infiniburn: String,
            pub logical_height: i64,
            pub min_y: i64,
            pub monster_spawn_block_light_limit: i64,
            pub monster_spawn_light_level: MonsterSpawnLightLevel,
            pub natural: i64,
            pub piglin_safe: i64,
            pub respawn_anchor_works: i64,
            pub ultrawarm: i64,
        }

        #[derive(NBTSerialize, NBTDeserialize, Serialize, Deserialize)]
        pub struct MinecraftDimensionType {
            #[serde(rename = "minecraft:overworld")]
            #[nbt(rename = "minecraft:overworld")]
            pub minecraft_overworld: Struct4,
            #[serde(rename = "minecraft:overworld_caves")]
            #[nbt(rename = "minecraft:overworld_caves")]
            pub minecraft_overworld_caves: Struct4,
            #[serde(rename = "minecraft:the_end")]
            #[nbt(rename = "minecraft:the_end")]
            pub minecraft_the_end: Struct4,
            #[serde(rename = "minecraft:the_nether")]
            #[nbt(rename = "minecraft:the_nether")]
            pub minecraft_the_nether: Struct4,
        }

        #[derive(NBTSerialize, NBTDeserialize, Serialize, Deserialize)]
        pub struct Struct3 {
            pub exhaustion: f64,
            pub message_id: String,
            pub scaling: String,
        }

        #[derive(NBTSerialize, NBTDeserialize, Serialize, Deserialize)]
        pub struct MinecraftDamageType {
            #[serde(rename = "minecraft:arrow")]
            #[nbt(rename = "minecraft:arrow")]
            pub minecraft_arrow: Struct3,
            #[serde(rename = "minecraft:bad_respawn_point")]
            #[nbt(rename = "minecraft:bad_respawn_point")]
            pub minecraft_bad_respawn_point: Struct3,
            #[serde(rename = "minecraft:cactus")]
            #[nbt(rename = "minecraft:cactus")]
            pub minecraft_cactus: Struct3,
            #[serde(rename = "minecraft:campfire")]
            #[nbt(rename = "minecraft:campfire")]
            pub minecraft_campfire: Struct3,
            #[serde(rename = "minecraft:cramming")]
            #[nbt(rename = "minecraft:cramming")]
            pub minecraft_cramming: Struct3,
            #[serde(rename = "minecraft:dragon_breath")]
            #[nbt(rename = "minecraft:dragon_breath")]
            pub minecraft_dragon_breath: Struct3,
            #[serde(rename = "minecraft:drown")]
            #[nbt(rename = "minecraft:drown")]
            pub minecraft_drown: Struct3,
            #[serde(rename = "minecraft:dry_out")]
            #[nbt(rename = "minecraft:dry_out")]
            pub minecraft_dry_out: Struct3,
            #[serde(rename = "minecraft:explosion")]
            #[nbt(rename = "minecraft:explosion")]
            pub minecraft_explosion: Struct3,
            #[serde(rename = "minecraft:fall")]
            #[nbt(rename = "minecraft:fall")]
            pub minecraft_fall: Struct3,
            #[serde(rename = "minecraft:falling_anvil")]
            #[nbt(rename = "minecraft:falling_anvil")]
            pub minecraft_falling_anvil: Struct3,
            #[serde(rename = "minecraft:falling_block")]
            #[nbt(rename = "minecraft:falling_block")]
            pub minecraft_falling_block: Struct3,
            #[serde(rename = "minecraft:falling_stalactite")]
            #[nbt(rename = "minecraft:falling_stalactite")]
            pub minecraft_falling_stalactite: Struct3,
            #[serde(rename = "minecraft:fireball")]
            #[nbt(rename = "minecraft:fireball")]
            pub minecraft_fireball: Struct3,
            #[serde(rename = "minecraft:fireworks")]
            #[nbt(rename = "minecraft:fireworks")]
            pub minecraft_fireworks: Struct3,
            #[serde(rename = "minecraft:fly_into_wall")]
            #[nbt(rename = "minecraft:fly_into_wall")]
            pub minecraft_fly_into_wall: Struct3,
            #[serde(rename = "minecraft:freeze")]
            #[nbt(rename = "minecraft:freeze")]
            pub minecraft_freeze: Struct3,
            #[serde(rename = "minecraft:generic")]
            #[nbt(rename = "minecraft:generic")]
            pub minecraft_generic: Struct3,
            #[serde(rename = "minecraft:generic_kill")]
            #[nbt(rename = "minecraft:generic_kill")]
            pub minecraft_generic_kill: Struct3,
            #[serde(rename = "minecraft:hot_floor")]
            #[nbt(rename = "minecraft:hot_floor")]
            pub minecraft_hot_floor: Struct3,
            #[serde(rename = "minecraft:in_fire")]
            #[nbt(rename = "minecraft:in_fire")]
            pub minecraft_in_fire: Struct3,
            #[serde(rename = "minecraft:in_wall")]
            #[nbt(rename = "minecraft:in_wall")]
            pub minecraft_in_wall: Struct3,
            #[serde(rename = "minecraft:indirect_magic")]
            #[nbt(rename = "minecraft:indirect_magic")]
            pub minecraft_indirect_magic: Struct3,
            #[serde(rename = "minecraft:lava")]
            #[nbt(rename = "minecraft:lava")]
            pub minecraft_lava: Struct3,
            #[serde(rename = "minecraft:lightning_bolt")]
            #[nbt(rename = "minecraft:lightning_bolt")]
            pub minecraft_lightning_bolt: Struct3,
            #[serde(rename = "minecraft:magic")]
            #[nbt(rename = "minecraft:magic")]
            pub minecraft_magic: Struct3,
            #[serde(rename = "minecraft:mob_attack")]
            #[nbt(rename = "minecraft:mob_attack")]
            pub minecraft_mob_attack: Struct3,
            #[serde(rename = "minecraft:mob_attack_no_aggro")]
            #[nbt(rename = "minecraft:mob_attack_no_aggro")]
            pub minecraft_mob_attack_no_aggro: Struct3,
            #[serde(rename = "minecraft:mob_projectile")]
            #[nbt(rename = "minecraft:mob_projectile")]
            pub minecraft_mob_projectile: Struct3,
            #[serde(rename = "minecraft:on_fire")]
            #[nbt(rename = "minecraft:on_fire")]
            pub minecraft_on_fire: Struct3,
            #[serde(rename = "minecraft:out_of_world")]
            #[nbt(rename = "minecraft:out_of_world")]
            pub minecraft_out_of_world: Struct3,
            #[serde(rename = "minecraft:outside_border")]
            #[nbt(rename = "minecraft:outside_border")]
            pub minecraft_outside_border: Struct3,
            #[serde(rename = "minecraft:player_attack")]
            #[nbt(rename = "minecraft:player_attack")]
            pub minecraft_player_attack: Struct3,
            #[serde(rename = "minecraft:player_explosion")]
            #[nbt(rename = "minecraft:player_explosion")]
            pub minecraft_player_explosion: Struct3,
            #[serde(rename = "minecraft:sonic_boom")]
            #[nbt(rename = "minecraft:sonic_boom")]
            pub minecraft_sonic_boom: Struct3,
            #[serde(rename = "minecraft:spit")]
            #[nbt(rename = "minecraft:spit")]
            pub minecraft_spit: Struct3,
            #[serde(rename = "minecraft:stalagmite")]
            #[nbt(rename = "minecraft:stalagmite")]
            pub minecraft_stalagmite: Struct3,
            #[serde(rename = "minecraft:starve")]
            #[nbt(rename = "minecraft:starve")]
            pub minecraft_starve: Struct3,
            #[serde(rename = "minecraft:sting")]
            #[nbt(rename = "minecraft:sting")]
            pub minecraft_sting: Struct3,
            #[serde(rename = "minecraft:sweet_berry_bush")]
            #[nbt(rename = "minecraft:sweet_berry_bush")]
            pub minecraft_sweet_berry_bush: Struct3,
            #[serde(rename = "minecraft:thorns")]
            #[nbt(rename = "minecraft:thorns")]
            pub minecraft_thorns: Struct3,
            #[serde(rename = "minecraft:thrown")]
            #[nbt(rename = "minecraft:thrown")]
            pub minecraft_thrown: Struct3,
            #[serde(rename = "minecraft:trident")]
            #[nbt(rename = "minecraft:trident")]
            pub minecraft_trident: Struct3,
            #[serde(rename = "minecraft:unattributed_fireball")]
            #[nbt(rename = "minecraft:unattributed_fireball")]
            pub minecraft_unattributed_fireball: Struct3,
            #[serde(rename = "minecraft:wind_charge")]
            #[nbt(rename = "minecraft:wind_charge")]
            pub minecraft_wind_charge: Struct3,
            #[serde(rename = "minecraft:wither")]
            #[nbt(rename = "minecraft:wither")]
            pub minecraft_wither: Struct3,
            #[serde(rename = "minecraft:wither_skull")]
            #[nbt(rename = "minecraft:wither_skull")]
            pub minecraft_wither_skull: Struct3,
        }

        #[derive(NBTSerialize, NBTDeserialize, Serialize, Deserialize)]
        pub struct Struct2 {
            pub parameters: Vec<String>,
            pub translation_key: String,
        }

        #[derive(NBTSerialize, NBTDeserialize, Serialize, Deserialize)]
        pub struct Struct1 {
            pub chat: Struct2,
            pub narration: Struct2,
        }

        #[derive(NBTSerialize, NBTDeserialize, Serialize, Deserialize)]
        pub struct MinecraftChatType {
            #[serde(rename = "minecraft:chat")]
            #[nbt(rename = "minecraft:chat")]
            pub minecraft_chat: Struct1,
            #[serde(rename = "minecraft:emote_command")]
            #[nbt(rename = "minecraft:emote_command")]
            pub minecraft_emote_command: Struct1,
            #[serde(rename = "minecraft:msg_command_incoming")]
            #[nbt(rename = "minecraft:msg_command_incoming")]
            pub minecraft_msg_command_incoming: Struct1,
            #[serde(rename = "minecraft:msg_command_outgoing")]
            #[nbt(rename = "minecraft:msg_command_outgoing")]
            pub minecraft_msg_command_outgoing: Struct1,
            #[serde(rename = "minecraft:say_command")]
            #[nbt(rename = "minecraft:say_command")]
            pub minecraft_say_command: Struct1,
            #[serde(rename = "minecraft:team_msg_command_incoming")]
            #[nbt(rename = "minecraft:team_msg_command_incoming")]
            pub minecraft_team_msg_command_incoming: Struct1,
            #[serde(rename = "minecraft:team_msg_command_outgoing")]
            #[nbt(rename = "minecraft:team_msg_command_outgoing")]
            pub minecraft_team_msg_command_outgoing: Struct1,
        }

        #[derive(NBTSerialize, NBTDeserialize, Serialize, Deserialize)]
        pub struct Struct {
            pub asset_id: String,
            pub translation_key: String,
        }

        #[derive(NBTSerialize, NBTDeserialize, Serialize, Deserialize)]
        pub struct MinecraftBannerPattern {
            #[serde(rename = "minecraft:base")]
            #[nbt(rename = "minecraft:base")]
            pub minecraft_base: Struct,
            #[serde(rename = "minecraft:border")]
            #[nbt(rename = "minecraft:border")]
            pub minecraft_border: Struct,
            #[serde(rename = "minecraft:bricks")]
            #[nbt(rename = "minecraft:bricks")]
            pub minecraft_bricks: Struct,
            #[serde(rename = "minecraft:circle")]
            #[nbt(rename = "minecraft:circle")]
            pub minecraft_circle: Struct,
            #[serde(rename = "minecraft:creeper")]
            #[nbt(rename = "minecraft:creeper")]
            pub minecraft_creeper: Struct,
            #[serde(rename = "minecraft:cross")]
            #[nbt(rename = "minecraft:cross")]
            pub minecraft_cross: Struct,
            #[serde(rename = "minecraft:curly_border")]
            #[nbt(rename = "minecraft:curly_border")]
            pub minecraft_curly_border: Struct,
            #[serde(rename = "minecraft:diagonal_left")]
            #[nbt(rename = "minecraft:diagonal_left")]
            pub minecraft_diagonal_left: Struct,
            #[serde(rename = "minecraft:diagonal_right")]
            #[nbt(rename = "minecraft:diagonal_right")]
            pub minecraft_diagonal_right: Struct,
            #[serde(rename = "minecraft:diagonal_up_left")]
            #[nbt(rename = "minecraft:diagonal_up_left")]
            pub minecraft_diagonal_up_left: Struct,
            #[serde(rename = "minecraft:diagonal_up_right")]
            #[nbt(rename = "minecraft:diagonal_up_right")]
            pub minecraft_diagonal_up_right: Struct,
            #[serde(rename = "minecraft:flow")]
            #[nbt(rename = "minecraft:flow")]
            pub minecraft_flow: Struct,
            #[serde(rename = "minecraft:flower")]
            #[nbt(rename = "minecraft:flower")]
            pub minecraft_flower: Struct,
            #[serde(rename = "minecraft:globe")]
            #[nbt(rename = "minecraft:globe")]
            pub minecraft_globe: Struct,
            #[serde(rename = "minecraft:gradient")]
            #[nbt(rename = "minecraft:gradient")]
            pub minecraft_gradient: Struct,
            #[serde(rename = "minecraft:gradient_up")]
            #[nbt(rename = "minecraft:gradient_up")]
            pub minecraft_gradient_up: Struct,
            #[serde(rename = "minecraft:guster")]
            #[nbt(rename = "minecraft:guster")]
            pub minecraft_guster: Struct,
            #[serde(rename = "minecraft:half_horizontal")]
            #[nbt(rename = "minecraft:half_horizontal")]
            pub minecraft_half_horizontal: Struct,
            #[serde(rename = "minecraft:half_horizontal_bottom")]
            #[nbt(rename = "minecraft:half_horizontal_bottom")]
            pub minecraft_half_horizontal_bottom: Struct,
            #[serde(rename = "minecraft:half_vertical")]
            #[nbt(rename = "minecraft:half_vertical")]
            pub minecraft_half_vertical: Struct,
            #[serde(rename = "minecraft:half_vertical_right")]
            #[nbt(rename = "minecraft:half_vertical_right")]
            pub minecraft_half_vertical_right: Struct,
            #[serde(rename = "minecraft:mojang")]
            #[nbt(rename = "minecraft:mojang")]
            pub minecraft_mojang: Struct,
            #[serde(rename = "minecraft:piglin")]
            #[nbt(rename = "minecraft:piglin")]
            pub minecraft_piglin: Struct,
            #[serde(rename = "minecraft:rhombus")]
            #[nbt(rename = "minecraft:rhombus")]
            pub minecraft_rhombus: Struct,
            #[serde(rename = "minecraft:skull")]
            #[nbt(rename = "minecraft:skull")]
            pub minecraft_skull: Struct,
            #[serde(rename = "minecraft:small_stripes")]
            #[nbt(rename = "minecraft:small_stripes")]
            pub minecraft_small_stripes: Struct,
            #[serde(rename = "minecraft:square_bottom_left")]
            #[nbt(rename = "minecraft:square_bottom_left")]
            pub minecraft_square_bottom_left: Struct,
            #[serde(rename = "minecraft:square_bottom_right")]
            #[nbt(rename = "minecraft:square_bottom_right")]
            pub minecraft_square_bottom_right: Struct,
            #[serde(rename = "minecraft:square_top_left")]
            #[nbt(rename = "minecraft:square_top_left")]
            pub minecraft_square_top_left: Struct,
            #[serde(rename = "minecraft:square_top_right")]
            #[nbt(rename = "minecraft:square_top_right")]
            pub minecraft_square_top_right: Struct,
            #[serde(rename = "minecraft:straight_cross")]
            #[nbt(rename = "minecraft:straight_cross")]
            pub minecraft_straight_cross: Struct,
            #[serde(rename = "minecraft:stripe_bottom")]
            #[nbt(rename = "minecraft:stripe_bottom")]
            pub minecraft_stripe_bottom: Struct,
            #[serde(rename = "minecraft:stripe_center")]
            #[nbt(rename = "minecraft:stripe_center")]
            pub minecraft_stripe_center: Struct,
            #[serde(rename = "minecraft:stripe_downleft")]
            #[nbt(rename = "minecraft:stripe_downleft")]
            pub minecraft_stripe_downleft: Struct,
            #[serde(rename = "minecraft:stripe_downright")]
            #[nbt(rename = "minecraft:stripe_downright")]
            pub minecraft_stripe_downright: Struct,
            #[serde(rename = "minecraft:stripe_left")]
            #[nbt(rename = "minecraft:stripe_left")]
            pub minecraft_stripe_left: Struct,
            #[serde(rename = "minecraft:stripe_middle")]
            #[nbt(rename = "minecraft:stripe_middle")]
            pub minecraft_stripe_middle: Struct,
            #[serde(rename = "minecraft:stripe_right")]
            #[nbt(rename = "minecraft:stripe_right")]
            pub minecraft_stripe_right: Struct,
            #[serde(rename = "minecraft:stripe_top")]
            #[nbt(rename = "minecraft:stripe_top")]
            pub minecraft_stripe_top: Struct,
            #[serde(rename = "minecraft:triangle_bottom")]
            #[nbt(rename = "minecraft:triangle_bottom")]
            pub minecraft_triangle_bottom: Struct,
            #[serde(rename = "minecraft:triangle_top")]
            #[nbt(rename = "minecraft:triangle_top")]
            pub minecraft_triangle_top: Struct,
            #[serde(rename = "minecraft:triangles_bottom")]
            #[nbt(rename = "minecraft:triangles_bottom")]
            pub minecraft_triangles_bottom: Struct,
            #[serde(rename = "minecraft:triangles_top")]
            #[nbt(rename = "minecraft:triangles_top")]
            pub minecraft_triangles_top: Struct,
        }

        #[derive(NBTSerialize, NBTDeserialize, Serialize, Deserialize)]
        pub struct Root {
            #[serde(rename = "minecraft:banner_pattern")]
            #[nbt(rename = "minecraft:banner_pattern")]
            pub minecraft_banner_pattern: MinecraftBannerPattern,
            #[serde(rename = "minecraft:chat_type")]
            #[nbt(rename = "minecraft:chat_type")]
            pub minecraft_chat_type: MinecraftChatType,
            #[serde(rename = "minecraft:damage_type")]
            #[nbt(rename = "minecraft:damage_type")]
            pub minecraft_damage_type: MinecraftDamageType,
            #[serde(rename = "minecraft:dimension_type")]
            #[nbt(rename = "minecraft:dimension_type")]
            pub minecraft_dimension_type: MinecraftDimensionType,
            #[serde(rename = "minecraft:painting_variant")]
            #[nbt(rename = "minecraft:painting_variant")]
            pub minecraft_painting_variant: MinecraftPaintingVariant,
            #[serde(rename = "minecraft:trim_material")]
            #[nbt(rename = "minecraft:trim_material")]
            pub minecraft_trim_material: MinecraftTrimMaterial,
            #[serde(rename = "minecraft:trim_pattern")]
            #[nbt(rename = "minecraft:trim_pattern")]
            pub minecraft_trim_pattern: MinecraftTrimPattern,
            #[serde(rename = "minecraft:wolf_variant")]
            #[nbt(rename = "minecraft:wolf_variant")]
            pub minecraft_wolf_variant: MinecraftWolfVariant,
            #[serde(rename = "minecraft:worldgen/biome")]
            #[nbt(rename = "minecraft:worldgen/biome")]
            pub minecraft_worldgen_biome: MinecraftWorldgenBiome,
        }

        pub fn get_registry_root() -> Root {
            let json = include_str!("../../../../../../.etc/registry.json");
            let serde_json: Root = serde_json::from_str(json).unwrap();

            serde_json
        }
    }

    #[test]
    #[ignore]
    fn generate_nbt() {
        let registry_data = registry_parser::get_registry_root();
        let nbt = registry_data.serialize_with_header();

        let open_file = std::fs::OpenOptions::new()
            .write(true)
            .create(true)
            .truncate(true)
            .open(r#"D:\Minecraft\framework\ferrumc\ferrumc-2_0\ferrumc\.etc/registry.nbt"#);
        //
        open_file.unwrap().write_all(nbt.as_slice()).unwrap();
    }

    #[test]
    fn generate_packet() {
        let registry_nbt_buf = include_bytes!("../../../../../../.etc/registry.nbt");
        // for each top level key in the registry, generate a packet
        let mut packets: Vec<u8> = vec![];

        let mut tape = NbtTape::new(registry_nbt_buf);
        tape.parse();

        let root = tape.root.as_ref().map(|(_, b)| b).unwrap();
        let root = root.as_compound().unwrap();

        for (registry_id, registry_data) in root.iter() {
            let mut rewind_machine = NbtTape::new(registry_nbt_buf);
            rewind_machine.parse();

            let registry_packet = {
                let registry_compound = registry_data.as_compound().unwrap();

                let entries = registry_compound
                    .iter()
                    .map(|(name, element)| {
                        debug!(
                            "Serializing entry for {}. ELEMENT: {:#?}",
                            registry_id, element
                        );

                        let has_data = true;
                        let mut data = vec![];
                        element
                            .serialize_as_network(&mut rewind_machine, &mut data)
                            .unwrap_or_else(|_| {
                                panic!("Failed to serialize entry for {}", registry_id)
                            });

                        RegistryEntry {
                            id: name,
                            has_data,
                            data,
                        }
                    })
                    .collect::<Vec<_>>();

                RegistryDataPacket::new(registry_id, entries)
            };

            registry_packet
                .encode(&mut packets, &NetEncodeOpts::WithLength)
                .unwrap_or_else(|_| panic!("Failed to encode packet for {}", registry_id));
        }

        std::fs::write(
            r#"D:\Minecraft\framework\ferrumc\ferrumc-2_0\ferrumc\.etc/registry.packet"#,
            packets,
        )
        .unwrap();
    }
}
