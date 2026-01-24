#[allow(unused_imports)]
use ferrumc_block_properties::*;
#[allow(dead_code)]
#[derive(Clone, Debug)]
pub struct VaultBlock {
    pub facing: Direction,
    pub ominous: bool,
    pub vault_state: VaultState,
}
