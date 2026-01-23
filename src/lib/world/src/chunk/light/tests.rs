use crate::chunk::light::engine::LightEngine;
use crate::chunk::light::sky_light::SkyLightEngine;
use crate::chunk::{BlockStateId, Chunk};
use ferrumc_macros::block;

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
