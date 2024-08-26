use serde::Deserialize;
use serde::Serialize;

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
#[serde(rename = "Root")]
pub struct Root {
    #[serde(rename = "minecraft:chat_type")]
    pub minecraft_chat_type: MinecraftChatType,
    #[serde(rename = "minecraft:damage_type")]
    pub minecraft_damage_type: MinecraftDamageType,
    #[serde(rename = "minecraft:dimension_type")]
    pub minecraft_dimension_type: MinecraftDimensionType,
    #[serde(rename = "minecraft:trim_material")]
    pub minecraft_trim_material: MinecraftTrimMaterial,
    #[serde(rename = "minecraft:trim_pattern")]
    pub minecraft_trim_pattern: MinecraftTrimPattern,
    #[serde(rename = "minecraft:worldgen/biome")]
    pub minecraft_worldgen_biome: MinecraftWorldgenBiome,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct MinecraftChatType {
    #[serde(rename = "type")]
    pub type_field: String,
    pub value: Vec<InternalValue>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct InternalValue {
    pub element: Element,
    pub id: i64,
    pub name: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Element {
    pub chat: Chat,
    pub narration: Narration,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Chat {
    pub parameters: Vec<String>,
    #[serde(rename = "translation_key")]
    pub translation_key: String,
    pub style: Option<Style>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Style {
    pub color: String,
    pub italic: i64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Narration {
    pub parameters: Vec<String>,
    #[serde(rename = "translation_key")]
    pub translation_key: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct MinecraftDamageType {
    #[serde(rename = "type")]
    pub type_field: String,
    pub value: Vec<Value2>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Value2 {
    pub element: Element2,
    pub id: i64,
    pub name: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Element2 {
    pub exhaustion: f64,
    #[serde(rename = "message_id")]
    pub message_id: String,
    pub scaling: String,
    #[serde(rename = "death_message_type")]
    pub death_message_type: Option<String>,
    pub effects: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct MinecraftDimensionType {
    #[serde(rename = "type")]
    pub type_field: String,
    pub value: Vec<Value3>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Value3 {
    pub element: Element3,
    pub id: i64,
    pub name: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Element3 {
    #[serde(rename = "ambient_light")]
    pub ambient_light: f64,
    #[serde(rename = "bed_works")]
    pub bed_works: i64,
    #[serde(rename = "coordinate_scale")]
    pub coordinate_scale: i64,
    pub effects: String,
    #[serde(rename = "has_ceiling")]
    pub has_ceiling: i64,
    #[serde(rename = "has_raids")]
    pub has_raids: i64,
    #[serde(rename = "has_skylight")]
    pub has_skylight: i64,
    pub height: i64,
    pub infiniburn: String,
    #[serde(rename = "logical_height")]
    pub logical_height: i64,
    #[serde(rename = "min_y")]
    pub min_y: i64,
    #[serde(rename = "monster_spawn_block_light_limit")]
    pub monster_spawn_block_light_limit: i64,
    #[serde(rename = "monster_spawn_light_level")]
    pub monster_spawn_light_level: Option<i64>,
    pub natural: i64,
    #[serde(rename = "piglin_safe")]
    pub piglin_safe: i64,
    #[serde(rename = "respawn_anchor_works")]
    pub respawn_anchor_works: i64,
    pub ultrawarm: i64,
    #[serde(rename = "fixed_time")]
    pub fixed_time: Option<i64>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct MinecraftTrimMaterial {
    #[serde(rename = "type")]
    pub type_field: String,
    pub value: Vec<Value4>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Value4 {
    pub element: Element4,
    pub id: i64,
    pub name: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Element4 {
    #[serde(rename = "asset_name")]
    pub asset_name: String,
    pub description: Description,
    pub ingredient: String,
    #[serde(rename = "item_model_index")]
    pub item_model_index: f64,
    #[serde(rename = "override_armor_materials")]
    pub override_armor_materials: Option<OverrideArmorMaterials>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Description {
    pub color: String,
    pub translate: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct OverrideArmorMaterials {
    pub netherite: Option<String>,
    pub iron: Option<String>,
    pub gold: Option<String>,
    pub diamond: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct MinecraftTrimPattern {
    #[serde(rename = "type")]
    pub type_field: String,
    pub value: Vec<Value5>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Value5 {
    pub element: Element5,
    pub id: i64,
    pub name: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Element5 {
    #[serde(rename = "asset_id")]
    pub asset_id: String,
    pub description: Description2,
    #[serde(rename = "template_item")]
    pub template_item: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Description2 {
    pub translate: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct MinecraftWorldgenBiome {
    #[serde(rename = "type")]
    pub type_field: String,
    pub value: Vec<Value6>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Value6 {
    pub element: Element6,
    pub id: i64,
    pub name: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Element6 {
    pub downfall: f64,
    pub effects: Effects,
    #[serde(rename = "has_precipitation")]
    pub has_precipitation: i64,
    pub temperature: f64,
    #[serde(rename = "temperature_modifier")]
    pub temperature_modifier: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Effects {
    #[serde(rename = "fog_color")]
    pub fog_color: i64,
    #[serde(rename = "foliage_color")]
    pub foliage_color: Option<i64>,
    #[serde(rename = "grass_color")]
    pub grass_color: Option<i64>,
    #[serde(rename = "mood_sound")]
    pub mood_sound: MoodSound,
    pub music: Option<Music>,
    #[serde(rename = "sky_color")]
    pub sky_color: i64,
    #[serde(rename = "water_color")]
    pub water_color: i64,
    #[serde(rename = "water_fog_color")]
    pub water_fog_color: i64,
    #[serde(rename = "additions_sound")]
    pub additions_sound: Option<AdditionsSound>,
    #[serde(rename = "ambient_sound")]
    pub ambient_sound: Option<String>,
    pub particle: Option<Particle>,
    #[serde(rename = "grass_color_modifier")]
    pub grass_color_modifier: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct MoodSound {
    #[serde(rename = "block_search_extent")]
    pub block_search_extent: i64,
    pub offset: i64,
    pub sound: String,
    #[serde(rename = "tick_delay")]
    pub tick_delay: i64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Music {
    #[serde(rename = "max_delay")]
    pub max_delay: i64,
    #[serde(rename = "min_delay")]
    pub min_delay: i64,
    #[serde(rename = "replace_current_music")]
    pub replace_current_music: i64,
    pub sound: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct AdditionsSound {
    pub sound: String,
    #[serde(rename = "tick_chance")]
    pub tick_chance: f64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Particle {
    pub options: Options,
    pub probability: f64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Options {
    #[serde(rename = "type")]
    pub type_field: String,
}
