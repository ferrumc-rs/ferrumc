use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct PotionContentsJson {
    pub potion_id: Option<i32>,
    pub custom_color: Option<i32>,
    pub custom_effects: Option<Vec<PotionEffectJson>>,
    pub custom_name: Option<String>,
}

#[derive(Serialize, Deserialize)]
pub struct PotionEffectJson {
    pub effect_id: i32,
    pub amplifier: i32,
    pub duration: i32,
    pub ambient: bool,
    pub show_particles: bool,
    pub show_icon: bool,
    //https://minecraft.wiki/w/Java_Edition_protocol/Slot_data#Potion_Effect
    //todo hidden_effect not implemented
}