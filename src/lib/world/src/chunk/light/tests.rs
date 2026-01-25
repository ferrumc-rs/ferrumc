use crate::chunk::light::engine::LightEngine;
use crate::chunk::light::sky_light::SkyLightEngine;
use crate::chunk::{BlockStateId, Chunk};
use ferrumc_macros::block;
use crate::chunk::light::{LightSection, LightType, SectionLightData};
use crate::pos::SectionBlockPos;

#[test]
fn test_block_opacities() {
    let air = block!("air");
    let grass = block!("grass_block", {snowy:false});

    assert_eq!(
        SkyLightEngine::opacity(air),
        15,
        "Air should have opacity of 15"
    );
    assert_eq!(
        SkyLightEngine::opacity(grass),
        0,
        "Grass should have opacity of 0"
    );
}

#[test]
fn test_light_storage() {
    let mut section_light = SectionLightData::default();

    section_light.set_light(SectionBlockPos::new(0, 0, 0), 13, LightType::Sky);
    section_light.set_light(SectionBlockPos::new(1, 0, 0), 7, LightType::Sky);
    section_light.set_light(SectionBlockPos::new(0, 1, 0), 15, LightType::Sky);

    assert_eq!(section_light.get_light(SectionBlockPos::new(0, 0, 0), LightType::Sky), 13);
    assert_eq!(section_light.get_light(SectionBlockPos::new(1, 0, 0), LightType::Sky), 7);
    assert_eq!(section_light.get_light(SectionBlockPos::new(0, 1, 0), LightType::Sky), 15);
}
