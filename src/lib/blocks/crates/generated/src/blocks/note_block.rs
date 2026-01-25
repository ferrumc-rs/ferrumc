#[allow(unused_imports)]
use ferrumc_block_properties::*;
#[allow(dead_code)]
#[derive(Clone, Debug)]
pub struct NoteBlock {
    pub instrument: NoteBlockInstrument,
    pub note: i32,
    pub powered: bool,
}
