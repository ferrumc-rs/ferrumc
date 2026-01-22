use std::collections::HashMap;

use crate::{chunk::light::LightSection, pos::SectionPos};

#[derive(Default)]
pub struct LightStorage {
    sections: HashMap<SectionPos, LightSection>,
}
