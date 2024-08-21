use nbt_lib::Serialize;
use serde::Deserialize;

mod quarantined {
    #[test]
    fn something() {
        let test_tag: nbt_lib::NBTTag = nbt_lib::NBTTag::Compound(std::collections::HashMap::new());
        // <test_tag as nbt_lib::nbt_spec::serializer::impls::NBTFieldType>.tag_type().serialize(writer)?;
        <nbt_lib::NBTTag as nbt_lib::nbt_spec::serializer::impls::NBTFieldType>::tag_type(
            &test_tag,
        );
    }
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
#[nbt(is_root)]
#[nbt(rename = "Root")]
#[serde(rename = "Root")]
pub struct Root {
    #[nbt(rename = "minecraft:chat_type")]
    #[serde(rename = "minecraft:chat_type")]
    pub minecraft_chat_type: MinecraftChatType,
    #[nbt(rename = "minecraft:damage_type")]
    #[serde(rename = "minecraft:damage_type")]
    pub minecraft_damage_type: MinecraftDamageType,
    #[nbt(rename = "minecraft:dimension_type")]
    #[serde(rename = "minecraft:dimension_type")]
    pub minecraft_dimension_type: MinecraftDimensionType,
    #[nbt(rename = "minecraft:trim_material")]
    #[serde(rename = "minecraft:trim_material")]
    pub minecraft_trim_material: MinecraftTrimMaterial,
    #[nbt(rename = "minecraft:trim_pattern")]
    #[serde(rename = "minecraft:trim_pattern")]
    pub minecraft_trim_pattern: MinecraftTrimPattern,
    #[nbt(rename = "minecraft:worldgen/biome")]
    #[serde(rename = "minecraft:worldgen/biome")]
    pub minecraft_worldgen_biome: MinecraftWorldgenBiome,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
// #[nbt(rename_all = "camelCase")]
pub struct MinecraftChatType {
    #[nbt(rename = "type")]
    #[serde(rename = "type")]
    pub type_field: String,
    pub value: Vec<InternalValue>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
// #[nbt(rename_all = "camelCase")]
pub struct InternalValue {
    pub element: Element,
    pub id: i64,
    pub name: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
// #[nbt(rename_all = "camelCase")]
pub struct Element {
    pub chat: Chat,
    pub narration: Narration,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
// #[nbt(rename_all = "camelCase")]
pub struct Chat {
    pub parameters: Vec<String>,
    #[nbt(rename = "translation_key")]
    #[serde(rename = "translation_key")]
    pub translation_key: String,
    pub style: Option<Style>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
// #[nbt(rename_all = "camelCase")]
pub struct Style {
    pub color: String,
    pub italic: i64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
// #[nbt(rename_all = "camelCase")]
pub struct Narration {
    pub parameters: Vec<String>,
    #[nbt(rename = "translation_key")]
    #[serde(rename = "translation_key")]
    pub translation_key: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
// #[nbt(rename_all = "camelCase")]
pub struct MinecraftDamageType {
    #[nbt(rename = "type")]
    #[serde(rename = "type")]
    pub type_field: String,
    pub value: Vec<Value2>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
// #[nbt(rename_all = "camelCase")]
pub struct Value2 {
    pub element: Element2,
    pub id: i64,
    pub name: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
// #[nbt(rename_all = "camelCase")]
pub struct Element2 {
    pub exhaustion: f64,
    #[nbt(rename = "message_id")]
    #[serde(rename = "message_id")]
    pub message_id: String,
    pub scaling: String,
    #[nbt(rename = "death_message_type")]
    #[serde(rename = "death_message_type")]
    pub death_message_type: Option<String>,
    pub effects: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
// #[nbt(rename_all = "camelCase")]
pub struct MinecraftDimensionType {
    #[nbt(rename = "type")]
    #[serde(rename = "type")]
    pub type_field: String,
    pub value: Vec<Value3>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
// #[nbt(rename_all = "camelCase")]
pub struct Value3 {
    pub element: Element3,
    pub id: i64,
    pub name: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
// #[nbt(rename_all = "camelCase")]
pub struct Element3 {
    #[nbt(rename = "ambient_light")]
    #[serde(rename = "ambient_light")]
    pub ambient_light: f64,
    #[nbt(rename = "bed_works")]
    #[serde(rename = "bed_works")]
    pub bed_works: i64,
    #[nbt(rename = "coordinate_scale")]
    #[serde(rename = "coordinate_scale")]
    pub coordinate_scale: i64,
    pub effects: String,
    #[nbt(rename = "has_ceiling")]
    #[serde(rename = "has_ceiling")]
    pub has_ceiling: i64,
    #[nbt(rename = "has_raids")]
    #[serde(rename = "has_raids")]
    pub has_raids: i64,
    #[nbt(rename = "has_skylight")]
    #[serde(rename = "has_skylight")]
    pub has_skylight: i64,
    pub height: i64,
    pub infiniburn: String,
    #[nbt(rename = "logical_height")]
    #[serde(rename = "logical_height")]
    pub logical_height: i64,
    #[nbt(rename = "min_y")]
    #[serde(rename = "min_y")]
    pub min_y: i64,
    #[nbt(rename = "monster_spawn_block_light_limit")]
    #[serde(rename = "monster_spawn_block_light_limit")]
    pub monster_spawn_block_light_limit: i64,
    /*#[nbt(rename = "monster_spawn_light_level")]
    #[nserde(rename = "monster_spawn_light_level")]
    pub monster_spawn_light_level: Option<i64>,*/
    pub natural: i64,
    #[nbt(rename = "piglin_safe")]
    #[serde(rename = "piglin_safe")]
    pub piglin_safe: i64,
    #[nbt(rename = "respawn_anchor_works")]
    #[serde(rename = "respawn_anchor_works")]
    pub respawn_anchor_works: i64,
    pub ultrawarm: i64,
    #[nbt(rename = "fixed_time")]
    #[serde(rename = "fixed_time")]
    pub fixed_time: Option<i64>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
// #[nbt(rename_all = "camelCase")]
pub struct MinecraftTrimMaterial {
    #[nbt(rename = "type")]
    #[serde(rename = "type")]
    pub type_field: String,
    pub value: Vec<Value4>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
// #[nbt(rename_all = "camelCase")]
pub struct Value4 {
    pub element: Element4,
    pub id: i64,
    pub name: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
// #[nbt(rename_all = "camelCase")]
pub struct Element4 {
    #[nbt(rename = "asset_name")]
    #[serde(rename = "asset_name")]
    pub asset_name: String,
    pub description: Description,
    pub ingredient: String,
    #[nbt(rename = "item_model_index")]
    #[serde(rename = "item_model_index")]
    pub item_model_index: f64,
    #[nbt(rename = "override_armor_materials")]
    #[serde(rename = "override_armor_materials")]
    pub override_armor_materials: Option<OverrideArmorMaterials>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
// #[nbt(rename_all = "camelCase")]
pub struct Description {
    pub color: String,
    pub translate: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
// #[nbt(rename_all = "camelCase")]
pub struct OverrideArmorMaterials {
    pub netherite: Option<String>,
    pub iron: Option<String>,
    pub gold: Option<String>,
    pub diamond: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
// #[nbt(rename_all = "camelCase")]
pub struct MinecraftTrimPattern {
    #[nbt(rename = "type")]
    #[serde(rename = "type")]
    pub type_field: String,
    pub value: Vec<Value5>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
// #[nbt(rename_all = "camelCase")]
pub struct Value5 {
    pub element: Element5,
    pub id: i64,
    pub name: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
// #[nbt(rename_all = "camelCase")]
pub struct Element5 {
    #[nbt(rename = "asset_id")]
    #[serde(rename = "asset_id")]
    pub asset_id: String,
    pub description: Description2,
    #[nbt(rename = "template_item")]
    #[serde(rename = "template_item")]
    pub template_item: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
// #[nbt(rename_all = "camelCase")]
pub struct Description2 {
    pub translate: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
// #[nbt(rename_all = "camelCase")]
pub struct MinecraftWorldgenBiome {
    #[nbt(rename = "type")]
    #[serde(rename = "type")]
    pub type_field: String,
    pub value: Vec<Value6>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
// #[nbt(rename_all = "camelCase")]
pub struct Value6 {
    pub element: Element6,
    pub id: i64,
    pub name: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
// #[nbt(rename_all = "camelCase")]
pub struct Element6 {
    pub downfall: f64,
    pub effects: Effects,
    #[nbt(rename = "has_precipitation")]
    #[serde(rename = "has_precipitation")]
    pub has_precipitation: i64,
    pub temperature: f64,
    #[nbt(rename = "temperature_modifier")]
    #[serde(rename = "temperature_modifier")]
    pub temperature_modifier: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
// #[nbt(rename_all = "camelCase")]
pub struct Effects {
    #[nbt(rename = "fog_color")]
    #[serde(rename = "fog_color")]
    pub fog_color: i64,
    #[nbt(rename = "foliage_color")]
    #[serde(rename = "foliage_color")]
    pub foliage_color: Option<i64>,
    #[nbt(rename = "grass_color")]
    #[serde(rename = "grass_color")]
    pub grass_color: Option<i64>,
    #[nbt(rename = "mood_sound")]
    #[serde(rename = "mood_sound")]
    pub mood_sound: MoodSound,
    pub music: Option<Music>,
    #[nbt(rename = "sky_color")]
    #[serde(rename = "sky_color")]
    pub sky_color: i64,
    #[nbt(rename = "water_color")]
    #[serde(rename = "water_color")]
    pub water_color: i64,
    #[nbt(rename = "water_fog_color")]
    #[serde(rename = "water_fog_color")]
    pub water_fog_color: i64,
    #[nbt(rename = "additions_sound")]
    #[serde(rename = "additions_sound")]
    pub additions_sound: Option<AdditionsSound>,
    #[nbt(rename = "ambient_sound")]
    #[serde(rename = "ambient_sound")]
    pub ambient_sound: Option<String>,
    pub particle: Option<Particle>,
    #[nbt(rename = "grass_color_modifier")]
    #[serde(rename = "grass_color_modifier")]
    pub grass_color_modifier: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
// #[nbt(rename_all = "camelCase")]
pub struct MoodSound {
    #[nbt(rename = "block_search_extent")]
    #[serde(rename = "block_search_extent")]
    pub block_search_extent: i64,
    pub offset: i64,
    pub sound: String,
    #[nbt(rename = "tick_delay")]
    #[serde(rename = "tick_delay")]
    pub tick_delay: i64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
// #[nbt(rename_all = "camelCase")]
pub struct Music {
    #[nbt(rename = "max_delay")]
    #[serde(rename = "max_delay")]
    pub max_delay: i64,
    #[nbt(rename = "min_delay")]
    #[serde(rename = "min_delay")]
    pub min_delay: i64,
    #[nbt(rename = "replace_current_music")]
    #[serde(rename = "replace_current_music")]
    pub replace_current_music: i64,
    pub sound: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
// #[nbt(rename_all = "camelCase")]
pub struct AdditionsSound {
    pub sound: String,
    #[nbt(rename = "tick_chance")]
    #[serde(rename = "tick_chance")]
    pub tick_chance: f64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
// #[nbt(rename_all = "camelCase")]
pub struct Particle {
    pub options: Options,
    pub probability: f64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
// #[nbt(rename_all = "camelCase")]
pub struct Options {
    #[nbt(rename = "type")]
    #[serde(rename = "type")]
    pub type_field: String,
}
