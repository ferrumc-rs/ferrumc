use crate::NoteBlock;
#[allow(unused_imports)]
use ferrumc_block_properties::*;
impl TryFrom<u32> for NoteBlock {
    type Error = ();
    fn try_from(data: u32) -> Result<Self, Self::Error> {
        match data {
            581u32 => Ok(NoteBlock {
                instrument: NoteBlockInstrument::Harp,
                note: 0i32,
                powered: true,
            }),
            582u32 => Ok(NoteBlock {
                instrument: NoteBlockInstrument::Harp,
                note: 0i32,
                powered: false,
            }),
            583u32 => Ok(NoteBlock {
                instrument: NoteBlockInstrument::Harp,
                note: 1i32,
                powered: true,
            }),
            584u32 => Ok(NoteBlock {
                instrument: NoteBlockInstrument::Harp,
                note: 1i32,
                powered: false,
            }),
            585u32 => Ok(NoteBlock {
                instrument: NoteBlockInstrument::Harp,
                note: 2i32,
                powered: true,
            }),
            586u32 => Ok(NoteBlock {
                instrument: NoteBlockInstrument::Harp,
                note: 2i32,
                powered: false,
            }),
            587u32 => Ok(NoteBlock {
                instrument: NoteBlockInstrument::Harp,
                note: 3i32,
                powered: true,
            }),
            588u32 => Ok(NoteBlock {
                instrument: NoteBlockInstrument::Harp,
                note: 3i32,
                powered: false,
            }),
            589u32 => Ok(NoteBlock {
                instrument: NoteBlockInstrument::Harp,
                note: 4i32,
                powered: true,
            }),
            590u32 => Ok(NoteBlock {
                instrument: NoteBlockInstrument::Harp,
                note: 4i32,
                powered: false,
            }),
            591u32 => Ok(NoteBlock {
                instrument: NoteBlockInstrument::Harp,
                note: 5i32,
                powered: true,
            }),
            592u32 => Ok(NoteBlock {
                instrument: NoteBlockInstrument::Harp,
                note: 5i32,
                powered: false,
            }),
            593u32 => Ok(NoteBlock {
                instrument: NoteBlockInstrument::Harp,
                note: 6i32,
                powered: true,
            }),
            594u32 => Ok(NoteBlock {
                instrument: NoteBlockInstrument::Harp,
                note: 6i32,
                powered: false,
            }),
            595u32 => Ok(NoteBlock {
                instrument: NoteBlockInstrument::Harp,
                note: 7i32,
                powered: true,
            }),
            596u32 => Ok(NoteBlock {
                instrument: NoteBlockInstrument::Harp,
                note: 7i32,
                powered: false,
            }),
            597u32 => Ok(NoteBlock {
                instrument: NoteBlockInstrument::Harp,
                note: 8i32,
                powered: true,
            }),
            598u32 => Ok(NoteBlock {
                instrument: NoteBlockInstrument::Harp,
                note: 8i32,
                powered: false,
            }),
            599u32 => Ok(NoteBlock {
                instrument: NoteBlockInstrument::Harp,
                note: 9i32,
                powered: true,
            }),
            600u32 => Ok(NoteBlock {
                instrument: NoteBlockInstrument::Harp,
                note: 9i32,
                powered: false,
            }),
            601u32 => Ok(NoteBlock {
                instrument: NoteBlockInstrument::Harp,
                note: 10i32,
                powered: true,
            }),
            602u32 => Ok(NoteBlock {
                instrument: NoteBlockInstrument::Harp,
                note: 10i32,
                powered: false,
            }),
            603u32 => Ok(NoteBlock {
                instrument: NoteBlockInstrument::Harp,
                note: 11i32,
                powered: true,
            }),
            604u32 => Ok(NoteBlock {
                instrument: NoteBlockInstrument::Harp,
                note: 11i32,
                powered: false,
            }),
            605u32 => Ok(NoteBlock {
                instrument: NoteBlockInstrument::Harp,
                note: 12i32,
                powered: true,
            }),
            606u32 => Ok(NoteBlock {
                instrument: NoteBlockInstrument::Harp,
                note: 12i32,
                powered: false,
            }),
            607u32 => Ok(NoteBlock {
                instrument: NoteBlockInstrument::Harp,
                note: 13i32,
                powered: true,
            }),
            608u32 => Ok(NoteBlock {
                instrument: NoteBlockInstrument::Harp,
                note: 13i32,
                powered: false,
            }),
            609u32 => Ok(NoteBlock {
                instrument: NoteBlockInstrument::Harp,
                note: 14i32,
                powered: true,
            }),
            610u32 => Ok(NoteBlock {
                instrument: NoteBlockInstrument::Harp,
                note: 14i32,
                powered: false,
            }),
            611u32 => Ok(NoteBlock {
                instrument: NoteBlockInstrument::Harp,
                note: 15i32,
                powered: true,
            }),
            612u32 => Ok(NoteBlock {
                instrument: NoteBlockInstrument::Harp,
                note: 15i32,
                powered: false,
            }),
            613u32 => Ok(NoteBlock {
                instrument: NoteBlockInstrument::Harp,
                note: 16i32,
                powered: true,
            }),
            614u32 => Ok(NoteBlock {
                instrument: NoteBlockInstrument::Harp,
                note: 16i32,
                powered: false,
            }),
            615u32 => Ok(NoteBlock {
                instrument: NoteBlockInstrument::Harp,
                note: 17i32,
                powered: true,
            }),
            616u32 => Ok(NoteBlock {
                instrument: NoteBlockInstrument::Harp,
                note: 17i32,
                powered: false,
            }),
            617u32 => Ok(NoteBlock {
                instrument: NoteBlockInstrument::Harp,
                note: 18i32,
                powered: true,
            }),
            618u32 => Ok(NoteBlock {
                instrument: NoteBlockInstrument::Harp,
                note: 18i32,
                powered: false,
            }),
            619u32 => Ok(NoteBlock {
                instrument: NoteBlockInstrument::Harp,
                note: 19i32,
                powered: true,
            }),
            620u32 => Ok(NoteBlock {
                instrument: NoteBlockInstrument::Harp,
                note: 19i32,
                powered: false,
            }),
            621u32 => Ok(NoteBlock {
                instrument: NoteBlockInstrument::Harp,
                note: 20i32,
                powered: true,
            }),
            622u32 => Ok(NoteBlock {
                instrument: NoteBlockInstrument::Harp,
                note: 20i32,
                powered: false,
            }),
            623u32 => Ok(NoteBlock {
                instrument: NoteBlockInstrument::Harp,
                note: 21i32,
                powered: true,
            }),
            624u32 => Ok(NoteBlock {
                instrument: NoteBlockInstrument::Harp,
                note: 21i32,
                powered: false,
            }),
            625u32 => Ok(NoteBlock {
                instrument: NoteBlockInstrument::Harp,
                note: 22i32,
                powered: true,
            }),
            626u32 => Ok(NoteBlock {
                instrument: NoteBlockInstrument::Harp,
                note: 22i32,
                powered: false,
            }),
            627u32 => Ok(NoteBlock {
                instrument: NoteBlockInstrument::Harp,
                note: 23i32,
                powered: true,
            }),
            628u32 => Ok(NoteBlock {
                instrument: NoteBlockInstrument::Harp,
                note: 23i32,
                powered: false,
            }),
            629u32 => Ok(NoteBlock {
                instrument: NoteBlockInstrument::Harp,
                note: 24i32,
                powered: true,
            }),
            630u32 => Ok(NoteBlock {
                instrument: NoteBlockInstrument::Harp,
                note: 24i32,
                powered: false,
            }),
            631u32 => Ok(NoteBlock {
                instrument: NoteBlockInstrument::BaseDrum,
                note: 0i32,
                powered: true,
            }),
            632u32 => Ok(NoteBlock {
                instrument: NoteBlockInstrument::BaseDrum,
                note: 0i32,
                powered: false,
            }),
            633u32 => Ok(NoteBlock {
                instrument: NoteBlockInstrument::BaseDrum,
                note: 1i32,
                powered: true,
            }),
            634u32 => Ok(NoteBlock {
                instrument: NoteBlockInstrument::BaseDrum,
                note: 1i32,
                powered: false,
            }),
            635u32 => Ok(NoteBlock {
                instrument: NoteBlockInstrument::BaseDrum,
                note: 2i32,
                powered: true,
            }),
            636u32 => Ok(NoteBlock {
                instrument: NoteBlockInstrument::BaseDrum,
                note: 2i32,
                powered: false,
            }),
            637u32 => Ok(NoteBlock {
                instrument: NoteBlockInstrument::BaseDrum,
                note: 3i32,
                powered: true,
            }),
            638u32 => Ok(NoteBlock {
                instrument: NoteBlockInstrument::BaseDrum,
                note: 3i32,
                powered: false,
            }),
            639u32 => Ok(NoteBlock {
                instrument: NoteBlockInstrument::BaseDrum,
                note: 4i32,
                powered: true,
            }),
            640u32 => Ok(NoteBlock {
                instrument: NoteBlockInstrument::BaseDrum,
                note: 4i32,
                powered: false,
            }),
            641u32 => Ok(NoteBlock {
                instrument: NoteBlockInstrument::BaseDrum,
                note: 5i32,
                powered: true,
            }),
            642u32 => Ok(NoteBlock {
                instrument: NoteBlockInstrument::BaseDrum,
                note: 5i32,
                powered: false,
            }),
            643u32 => Ok(NoteBlock {
                instrument: NoteBlockInstrument::BaseDrum,
                note: 6i32,
                powered: true,
            }),
            644u32 => Ok(NoteBlock {
                instrument: NoteBlockInstrument::BaseDrum,
                note: 6i32,
                powered: false,
            }),
            645u32 => Ok(NoteBlock {
                instrument: NoteBlockInstrument::BaseDrum,
                note: 7i32,
                powered: true,
            }),
            646u32 => Ok(NoteBlock {
                instrument: NoteBlockInstrument::BaseDrum,
                note: 7i32,
                powered: false,
            }),
            647u32 => Ok(NoteBlock {
                instrument: NoteBlockInstrument::BaseDrum,
                note: 8i32,
                powered: true,
            }),
            648u32 => Ok(NoteBlock {
                instrument: NoteBlockInstrument::BaseDrum,
                note: 8i32,
                powered: false,
            }),
            649u32 => Ok(NoteBlock {
                instrument: NoteBlockInstrument::BaseDrum,
                note: 9i32,
                powered: true,
            }),
            650u32 => Ok(NoteBlock {
                instrument: NoteBlockInstrument::BaseDrum,
                note: 9i32,
                powered: false,
            }),
            651u32 => Ok(NoteBlock {
                instrument: NoteBlockInstrument::BaseDrum,
                note: 10i32,
                powered: true,
            }),
            652u32 => Ok(NoteBlock {
                instrument: NoteBlockInstrument::BaseDrum,
                note: 10i32,
                powered: false,
            }),
            653u32 => Ok(NoteBlock {
                instrument: NoteBlockInstrument::BaseDrum,
                note: 11i32,
                powered: true,
            }),
            654u32 => Ok(NoteBlock {
                instrument: NoteBlockInstrument::BaseDrum,
                note: 11i32,
                powered: false,
            }),
            655u32 => Ok(NoteBlock {
                instrument: NoteBlockInstrument::BaseDrum,
                note: 12i32,
                powered: true,
            }),
            656u32 => Ok(NoteBlock {
                instrument: NoteBlockInstrument::BaseDrum,
                note: 12i32,
                powered: false,
            }),
            657u32 => Ok(NoteBlock {
                instrument: NoteBlockInstrument::BaseDrum,
                note: 13i32,
                powered: true,
            }),
            658u32 => Ok(NoteBlock {
                instrument: NoteBlockInstrument::BaseDrum,
                note: 13i32,
                powered: false,
            }),
            659u32 => Ok(NoteBlock {
                instrument: NoteBlockInstrument::BaseDrum,
                note: 14i32,
                powered: true,
            }),
            660u32 => Ok(NoteBlock {
                instrument: NoteBlockInstrument::BaseDrum,
                note: 14i32,
                powered: false,
            }),
            661u32 => Ok(NoteBlock {
                instrument: NoteBlockInstrument::BaseDrum,
                note: 15i32,
                powered: true,
            }),
            662u32 => Ok(NoteBlock {
                instrument: NoteBlockInstrument::BaseDrum,
                note: 15i32,
                powered: false,
            }),
            663u32 => Ok(NoteBlock {
                instrument: NoteBlockInstrument::BaseDrum,
                note: 16i32,
                powered: true,
            }),
            664u32 => Ok(NoteBlock {
                instrument: NoteBlockInstrument::BaseDrum,
                note: 16i32,
                powered: false,
            }),
            665u32 => Ok(NoteBlock {
                instrument: NoteBlockInstrument::BaseDrum,
                note: 17i32,
                powered: true,
            }),
            666u32 => Ok(NoteBlock {
                instrument: NoteBlockInstrument::BaseDrum,
                note: 17i32,
                powered: false,
            }),
            667u32 => Ok(NoteBlock {
                instrument: NoteBlockInstrument::BaseDrum,
                note: 18i32,
                powered: true,
            }),
            668u32 => Ok(NoteBlock {
                instrument: NoteBlockInstrument::BaseDrum,
                note: 18i32,
                powered: false,
            }),
            669u32 => Ok(NoteBlock {
                instrument: NoteBlockInstrument::BaseDrum,
                note: 19i32,
                powered: true,
            }),
            670u32 => Ok(NoteBlock {
                instrument: NoteBlockInstrument::BaseDrum,
                note: 19i32,
                powered: false,
            }),
            671u32 => Ok(NoteBlock {
                instrument: NoteBlockInstrument::BaseDrum,
                note: 20i32,
                powered: true,
            }),
            672u32 => Ok(NoteBlock {
                instrument: NoteBlockInstrument::BaseDrum,
                note: 20i32,
                powered: false,
            }),
            673u32 => Ok(NoteBlock {
                instrument: NoteBlockInstrument::BaseDrum,
                note: 21i32,
                powered: true,
            }),
            674u32 => Ok(NoteBlock {
                instrument: NoteBlockInstrument::BaseDrum,
                note: 21i32,
                powered: false,
            }),
            675u32 => Ok(NoteBlock {
                instrument: NoteBlockInstrument::BaseDrum,
                note: 22i32,
                powered: true,
            }),
            676u32 => Ok(NoteBlock {
                instrument: NoteBlockInstrument::BaseDrum,
                note: 22i32,
                powered: false,
            }),
            677u32 => Ok(NoteBlock {
                instrument: NoteBlockInstrument::BaseDrum,
                note: 23i32,
                powered: true,
            }),
            678u32 => Ok(NoteBlock {
                instrument: NoteBlockInstrument::BaseDrum,
                note: 23i32,
                powered: false,
            }),
            679u32 => Ok(NoteBlock {
                instrument: NoteBlockInstrument::BaseDrum,
                note: 24i32,
                powered: true,
            }),
            680u32 => Ok(NoteBlock {
                instrument: NoteBlockInstrument::BaseDrum,
                note: 24i32,
                powered: false,
            }),
            681u32 => Ok(NoteBlock {
                instrument: NoteBlockInstrument::Snare,
                note: 0i32,
                powered: true,
            }),
            682u32 => Ok(NoteBlock {
                instrument: NoteBlockInstrument::Snare,
                note: 0i32,
                powered: false,
            }),
            683u32 => Ok(NoteBlock {
                instrument: NoteBlockInstrument::Snare,
                note: 1i32,
                powered: true,
            }),
            684u32 => Ok(NoteBlock {
                instrument: NoteBlockInstrument::Snare,
                note: 1i32,
                powered: false,
            }),
            685u32 => Ok(NoteBlock {
                instrument: NoteBlockInstrument::Snare,
                note: 2i32,
                powered: true,
            }),
            686u32 => Ok(NoteBlock {
                instrument: NoteBlockInstrument::Snare,
                note: 2i32,
                powered: false,
            }),
            687u32 => Ok(NoteBlock {
                instrument: NoteBlockInstrument::Snare,
                note: 3i32,
                powered: true,
            }),
            688u32 => Ok(NoteBlock {
                instrument: NoteBlockInstrument::Snare,
                note: 3i32,
                powered: false,
            }),
            689u32 => Ok(NoteBlock {
                instrument: NoteBlockInstrument::Snare,
                note: 4i32,
                powered: true,
            }),
            690u32 => Ok(NoteBlock {
                instrument: NoteBlockInstrument::Snare,
                note: 4i32,
                powered: false,
            }),
            691u32 => Ok(NoteBlock {
                instrument: NoteBlockInstrument::Snare,
                note: 5i32,
                powered: true,
            }),
            692u32 => Ok(NoteBlock {
                instrument: NoteBlockInstrument::Snare,
                note: 5i32,
                powered: false,
            }),
            693u32 => Ok(NoteBlock {
                instrument: NoteBlockInstrument::Snare,
                note: 6i32,
                powered: true,
            }),
            694u32 => Ok(NoteBlock {
                instrument: NoteBlockInstrument::Snare,
                note: 6i32,
                powered: false,
            }),
            695u32 => Ok(NoteBlock {
                instrument: NoteBlockInstrument::Snare,
                note: 7i32,
                powered: true,
            }),
            696u32 => Ok(NoteBlock {
                instrument: NoteBlockInstrument::Snare,
                note: 7i32,
                powered: false,
            }),
            697u32 => Ok(NoteBlock {
                instrument: NoteBlockInstrument::Snare,
                note: 8i32,
                powered: true,
            }),
            698u32 => Ok(NoteBlock {
                instrument: NoteBlockInstrument::Snare,
                note: 8i32,
                powered: false,
            }),
            699u32 => Ok(NoteBlock {
                instrument: NoteBlockInstrument::Snare,
                note: 9i32,
                powered: true,
            }),
            700u32 => Ok(NoteBlock {
                instrument: NoteBlockInstrument::Snare,
                note: 9i32,
                powered: false,
            }),
            701u32 => Ok(NoteBlock {
                instrument: NoteBlockInstrument::Snare,
                note: 10i32,
                powered: true,
            }),
            702u32 => Ok(NoteBlock {
                instrument: NoteBlockInstrument::Snare,
                note: 10i32,
                powered: false,
            }),
            703u32 => Ok(NoteBlock {
                instrument: NoteBlockInstrument::Snare,
                note: 11i32,
                powered: true,
            }),
            704u32 => Ok(NoteBlock {
                instrument: NoteBlockInstrument::Snare,
                note: 11i32,
                powered: false,
            }),
            705u32 => Ok(NoteBlock {
                instrument: NoteBlockInstrument::Snare,
                note: 12i32,
                powered: true,
            }),
            706u32 => Ok(NoteBlock {
                instrument: NoteBlockInstrument::Snare,
                note: 12i32,
                powered: false,
            }),
            707u32 => Ok(NoteBlock {
                instrument: NoteBlockInstrument::Snare,
                note: 13i32,
                powered: true,
            }),
            708u32 => Ok(NoteBlock {
                instrument: NoteBlockInstrument::Snare,
                note: 13i32,
                powered: false,
            }),
            709u32 => Ok(NoteBlock {
                instrument: NoteBlockInstrument::Snare,
                note: 14i32,
                powered: true,
            }),
            710u32 => Ok(NoteBlock {
                instrument: NoteBlockInstrument::Snare,
                note: 14i32,
                powered: false,
            }),
            711u32 => Ok(NoteBlock {
                instrument: NoteBlockInstrument::Snare,
                note: 15i32,
                powered: true,
            }),
            712u32 => Ok(NoteBlock {
                instrument: NoteBlockInstrument::Snare,
                note: 15i32,
                powered: false,
            }),
            713u32 => Ok(NoteBlock {
                instrument: NoteBlockInstrument::Snare,
                note: 16i32,
                powered: true,
            }),
            714u32 => Ok(NoteBlock {
                instrument: NoteBlockInstrument::Snare,
                note: 16i32,
                powered: false,
            }),
            715u32 => Ok(NoteBlock {
                instrument: NoteBlockInstrument::Snare,
                note: 17i32,
                powered: true,
            }),
            716u32 => Ok(NoteBlock {
                instrument: NoteBlockInstrument::Snare,
                note: 17i32,
                powered: false,
            }),
            717u32 => Ok(NoteBlock {
                instrument: NoteBlockInstrument::Snare,
                note: 18i32,
                powered: true,
            }),
            718u32 => Ok(NoteBlock {
                instrument: NoteBlockInstrument::Snare,
                note: 18i32,
                powered: false,
            }),
            719u32 => Ok(NoteBlock {
                instrument: NoteBlockInstrument::Snare,
                note: 19i32,
                powered: true,
            }),
            720u32 => Ok(NoteBlock {
                instrument: NoteBlockInstrument::Snare,
                note: 19i32,
                powered: false,
            }),
            721u32 => Ok(NoteBlock {
                instrument: NoteBlockInstrument::Snare,
                note: 20i32,
                powered: true,
            }),
            722u32 => Ok(NoteBlock {
                instrument: NoteBlockInstrument::Snare,
                note: 20i32,
                powered: false,
            }),
            723u32 => Ok(NoteBlock {
                instrument: NoteBlockInstrument::Snare,
                note: 21i32,
                powered: true,
            }),
            724u32 => Ok(NoteBlock {
                instrument: NoteBlockInstrument::Snare,
                note: 21i32,
                powered: false,
            }),
            725u32 => Ok(NoteBlock {
                instrument: NoteBlockInstrument::Snare,
                note: 22i32,
                powered: true,
            }),
            726u32 => Ok(NoteBlock {
                instrument: NoteBlockInstrument::Snare,
                note: 22i32,
                powered: false,
            }),
            727u32 => Ok(NoteBlock {
                instrument: NoteBlockInstrument::Snare,
                note: 23i32,
                powered: true,
            }),
            728u32 => Ok(NoteBlock {
                instrument: NoteBlockInstrument::Snare,
                note: 23i32,
                powered: false,
            }),
            729u32 => Ok(NoteBlock {
                instrument: NoteBlockInstrument::Snare,
                note: 24i32,
                powered: true,
            }),
            730u32 => Ok(NoteBlock {
                instrument: NoteBlockInstrument::Snare,
                note: 24i32,
                powered: false,
            }),
            731u32 => Ok(NoteBlock {
                instrument: NoteBlockInstrument::Hat,
                note: 0i32,
                powered: true,
            }),
            732u32 => Ok(NoteBlock {
                instrument: NoteBlockInstrument::Hat,
                note: 0i32,
                powered: false,
            }),
            733u32 => Ok(NoteBlock {
                instrument: NoteBlockInstrument::Hat,
                note: 1i32,
                powered: true,
            }),
            734u32 => Ok(NoteBlock {
                instrument: NoteBlockInstrument::Hat,
                note: 1i32,
                powered: false,
            }),
            735u32 => Ok(NoteBlock {
                instrument: NoteBlockInstrument::Hat,
                note: 2i32,
                powered: true,
            }),
            736u32 => Ok(NoteBlock {
                instrument: NoteBlockInstrument::Hat,
                note: 2i32,
                powered: false,
            }),
            737u32 => Ok(NoteBlock {
                instrument: NoteBlockInstrument::Hat,
                note: 3i32,
                powered: true,
            }),
            738u32 => Ok(NoteBlock {
                instrument: NoteBlockInstrument::Hat,
                note: 3i32,
                powered: false,
            }),
            739u32 => Ok(NoteBlock {
                instrument: NoteBlockInstrument::Hat,
                note: 4i32,
                powered: true,
            }),
            740u32 => Ok(NoteBlock {
                instrument: NoteBlockInstrument::Hat,
                note: 4i32,
                powered: false,
            }),
            741u32 => Ok(NoteBlock {
                instrument: NoteBlockInstrument::Hat,
                note: 5i32,
                powered: true,
            }),
            742u32 => Ok(NoteBlock {
                instrument: NoteBlockInstrument::Hat,
                note: 5i32,
                powered: false,
            }),
            743u32 => Ok(NoteBlock {
                instrument: NoteBlockInstrument::Hat,
                note: 6i32,
                powered: true,
            }),
            744u32 => Ok(NoteBlock {
                instrument: NoteBlockInstrument::Hat,
                note: 6i32,
                powered: false,
            }),
            745u32 => Ok(NoteBlock {
                instrument: NoteBlockInstrument::Hat,
                note: 7i32,
                powered: true,
            }),
            746u32 => Ok(NoteBlock {
                instrument: NoteBlockInstrument::Hat,
                note: 7i32,
                powered: false,
            }),
            747u32 => Ok(NoteBlock {
                instrument: NoteBlockInstrument::Hat,
                note: 8i32,
                powered: true,
            }),
            748u32 => Ok(NoteBlock {
                instrument: NoteBlockInstrument::Hat,
                note: 8i32,
                powered: false,
            }),
            749u32 => Ok(NoteBlock {
                instrument: NoteBlockInstrument::Hat,
                note: 9i32,
                powered: true,
            }),
            750u32 => Ok(NoteBlock {
                instrument: NoteBlockInstrument::Hat,
                note: 9i32,
                powered: false,
            }),
            751u32 => Ok(NoteBlock {
                instrument: NoteBlockInstrument::Hat,
                note: 10i32,
                powered: true,
            }),
            752u32 => Ok(NoteBlock {
                instrument: NoteBlockInstrument::Hat,
                note: 10i32,
                powered: false,
            }),
            753u32 => Ok(NoteBlock {
                instrument: NoteBlockInstrument::Hat,
                note: 11i32,
                powered: true,
            }),
            754u32 => Ok(NoteBlock {
                instrument: NoteBlockInstrument::Hat,
                note: 11i32,
                powered: false,
            }),
            755u32 => Ok(NoteBlock {
                instrument: NoteBlockInstrument::Hat,
                note: 12i32,
                powered: true,
            }),
            756u32 => Ok(NoteBlock {
                instrument: NoteBlockInstrument::Hat,
                note: 12i32,
                powered: false,
            }),
            757u32 => Ok(NoteBlock {
                instrument: NoteBlockInstrument::Hat,
                note: 13i32,
                powered: true,
            }),
            758u32 => Ok(NoteBlock {
                instrument: NoteBlockInstrument::Hat,
                note: 13i32,
                powered: false,
            }),
            759u32 => Ok(NoteBlock {
                instrument: NoteBlockInstrument::Hat,
                note: 14i32,
                powered: true,
            }),
            760u32 => Ok(NoteBlock {
                instrument: NoteBlockInstrument::Hat,
                note: 14i32,
                powered: false,
            }),
            761u32 => Ok(NoteBlock {
                instrument: NoteBlockInstrument::Hat,
                note: 15i32,
                powered: true,
            }),
            762u32 => Ok(NoteBlock {
                instrument: NoteBlockInstrument::Hat,
                note: 15i32,
                powered: false,
            }),
            763u32 => Ok(NoteBlock {
                instrument: NoteBlockInstrument::Hat,
                note: 16i32,
                powered: true,
            }),
            764u32 => Ok(NoteBlock {
                instrument: NoteBlockInstrument::Hat,
                note: 16i32,
                powered: false,
            }),
            765u32 => Ok(NoteBlock {
                instrument: NoteBlockInstrument::Hat,
                note: 17i32,
                powered: true,
            }),
            766u32 => Ok(NoteBlock {
                instrument: NoteBlockInstrument::Hat,
                note: 17i32,
                powered: false,
            }),
            767u32 => Ok(NoteBlock {
                instrument: NoteBlockInstrument::Hat,
                note: 18i32,
                powered: true,
            }),
            768u32 => Ok(NoteBlock {
                instrument: NoteBlockInstrument::Hat,
                note: 18i32,
                powered: false,
            }),
            769u32 => Ok(NoteBlock {
                instrument: NoteBlockInstrument::Hat,
                note: 19i32,
                powered: true,
            }),
            770u32 => Ok(NoteBlock {
                instrument: NoteBlockInstrument::Hat,
                note: 19i32,
                powered: false,
            }),
            771u32 => Ok(NoteBlock {
                instrument: NoteBlockInstrument::Hat,
                note: 20i32,
                powered: true,
            }),
            772u32 => Ok(NoteBlock {
                instrument: NoteBlockInstrument::Hat,
                note: 20i32,
                powered: false,
            }),
            773u32 => Ok(NoteBlock {
                instrument: NoteBlockInstrument::Hat,
                note: 21i32,
                powered: true,
            }),
            774u32 => Ok(NoteBlock {
                instrument: NoteBlockInstrument::Hat,
                note: 21i32,
                powered: false,
            }),
            775u32 => Ok(NoteBlock {
                instrument: NoteBlockInstrument::Hat,
                note: 22i32,
                powered: true,
            }),
            776u32 => Ok(NoteBlock {
                instrument: NoteBlockInstrument::Hat,
                note: 22i32,
                powered: false,
            }),
            777u32 => Ok(NoteBlock {
                instrument: NoteBlockInstrument::Hat,
                note: 23i32,
                powered: true,
            }),
            778u32 => Ok(NoteBlock {
                instrument: NoteBlockInstrument::Hat,
                note: 23i32,
                powered: false,
            }),
            779u32 => Ok(NoteBlock {
                instrument: NoteBlockInstrument::Hat,
                note: 24i32,
                powered: true,
            }),
            780u32 => Ok(NoteBlock {
                instrument: NoteBlockInstrument::Hat,
                note: 24i32,
                powered: false,
            }),
            781u32 => Ok(NoteBlock {
                instrument: NoteBlockInstrument::Bass,
                note: 0i32,
                powered: true,
            }),
            782u32 => Ok(NoteBlock {
                instrument: NoteBlockInstrument::Bass,
                note: 0i32,
                powered: false,
            }),
            783u32 => Ok(NoteBlock {
                instrument: NoteBlockInstrument::Bass,
                note: 1i32,
                powered: true,
            }),
            784u32 => Ok(NoteBlock {
                instrument: NoteBlockInstrument::Bass,
                note: 1i32,
                powered: false,
            }),
            785u32 => Ok(NoteBlock {
                instrument: NoteBlockInstrument::Bass,
                note: 2i32,
                powered: true,
            }),
            786u32 => Ok(NoteBlock {
                instrument: NoteBlockInstrument::Bass,
                note: 2i32,
                powered: false,
            }),
            787u32 => Ok(NoteBlock {
                instrument: NoteBlockInstrument::Bass,
                note: 3i32,
                powered: true,
            }),
            788u32 => Ok(NoteBlock {
                instrument: NoteBlockInstrument::Bass,
                note: 3i32,
                powered: false,
            }),
            789u32 => Ok(NoteBlock {
                instrument: NoteBlockInstrument::Bass,
                note: 4i32,
                powered: true,
            }),
            790u32 => Ok(NoteBlock {
                instrument: NoteBlockInstrument::Bass,
                note: 4i32,
                powered: false,
            }),
            791u32 => Ok(NoteBlock {
                instrument: NoteBlockInstrument::Bass,
                note: 5i32,
                powered: true,
            }),
            792u32 => Ok(NoteBlock {
                instrument: NoteBlockInstrument::Bass,
                note: 5i32,
                powered: false,
            }),
            793u32 => Ok(NoteBlock {
                instrument: NoteBlockInstrument::Bass,
                note: 6i32,
                powered: true,
            }),
            794u32 => Ok(NoteBlock {
                instrument: NoteBlockInstrument::Bass,
                note: 6i32,
                powered: false,
            }),
            795u32 => Ok(NoteBlock {
                instrument: NoteBlockInstrument::Bass,
                note: 7i32,
                powered: true,
            }),
            796u32 => Ok(NoteBlock {
                instrument: NoteBlockInstrument::Bass,
                note: 7i32,
                powered: false,
            }),
            797u32 => Ok(NoteBlock {
                instrument: NoteBlockInstrument::Bass,
                note: 8i32,
                powered: true,
            }),
            798u32 => Ok(NoteBlock {
                instrument: NoteBlockInstrument::Bass,
                note: 8i32,
                powered: false,
            }),
            799u32 => Ok(NoteBlock {
                instrument: NoteBlockInstrument::Bass,
                note: 9i32,
                powered: true,
            }),
            800u32 => Ok(NoteBlock {
                instrument: NoteBlockInstrument::Bass,
                note: 9i32,
                powered: false,
            }),
            801u32 => Ok(NoteBlock {
                instrument: NoteBlockInstrument::Bass,
                note: 10i32,
                powered: true,
            }),
            802u32 => Ok(NoteBlock {
                instrument: NoteBlockInstrument::Bass,
                note: 10i32,
                powered: false,
            }),
            803u32 => Ok(NoteBlock {
                instrument: NoteBlockInstrument::Bass,
                note: 11i32,
                powered: true,
            }),
            804u32 => Ok(NoteBlock {
                instrument: NoteBlockInstrument::Bass,
                note: 11i32,
                powered: false,
            }),
            805u32 => Ok(NoteBlock {
                instrument: NoteBlockInstrument::Bass,
                note: 12i32,
                powered: true,
            }),
            806u32 => Ok(NoteBlock {
                instrument: NoteBlockInstrument::Bass,
                note: 12i32,
                powered: false,
            }),
            807u32 => Ok(NoteBlock {
                instrument: NoteBlockInstrument::Bass,
                note: 13i32,
                powered: true,
            }),
            808u32 => Ok(NoteBlock {
                instrument: NoteBlockInstrument::Bass,
                note: 13i32,
                powered: false,
            }),
            809u32 => Ok(NoteBlock {
                instrument: NoteBlockInstrument::Bass,
                note: 14i32,
                powered: true,
            }),
            810u32 => Ok(NoteBlock {
                instrument: NoteBlockInstrument::Bass,
                note: 14i32,
                powered: false,
            }),
            811u32 => Ok(NoteBlock {
                instrument: NoteBlockInstrument::Bass,
                note: 15i32,
                powered: true,
            }),
            812u32 => Ok(NoteBlock {
                instrument: NoteBlockInstrument::Bass,
                note: 15i32,
                powered: false,
            }),
            813u32 => Ok(NoteBlock {
                instrument: NoteBlockInstrument::Bass,
                note: 16i32,
                powered: true,
            }),
            814u32 => Ok(NoteBlock {
                instrument: NoteBlockInstrument::Bass,
                note: 16i32,
                powered: false,
            }),
            815u32 => Ok(NoteBlock {
                instrument: NoteBlockInstrument::Bass,
                note: 17i32,
                powered: true,
            }),
            816u32 => Ok(NoteBlock {
                instrument: NoteBlockInstrument::Bass,
                note: 17i32,
                powered: false,
            }),
            817u32 => Ok(NoteBlock {
                instrument: NoteBlockInstrument::Bass,
                note: 18i32,
                powered: true,
            }),
            818u32 => Ok(NoteBlock {
                instrument: NoteBlockInstrument::Bass,
                note: 18i32,
                powered: false,
            }),
            819u32 => Ok(NoteBlock {
                instrument: NoteBlockInstrument::Bass,
                note: 19i32,
                powered: true,
            }),
            820u32 => Ok(NoteBlock {
                instrument: NoteBlockInstrument::Bass,
                note: 19i32,
                powered: false,
            }),
            821u32 => Ok(NoteBlock {
                instrument: NoteBlockInstrument::Bass,
                note: 20i32,
                powered: true,
            }),
            822u32 => Ok(NoteBlock {
                instrument: NoteBlockInstrument::Bass,
                note: 20i32,
                powered: false,
            }),
            823u32 => Ok(NoteBlock {
                instrument: NoteBlockInstrument::Bass,
                note: 21i32,
                powered: true,
            }),
            824u32 => Ok(NoteBlock {
                instrument: NoteBlockInstrument::Bass,
                note: 21i32,
                powered: false,
            }),
            825u32 => Ok(NoteBlock {
                instrument: NoteBlockInstrument::Bass,
                note: 22i32,
                powered: true,
            }),
            826u32 => Ok(NoteBlock {
                instrument: NoteBlockInstrument::Bass,
                note: 22i32,
                powered: false,
            }),
            827u32 => Ok(NoteBlock {
                instrument: NoteBlockInstrument::Bass,
                note: 23i32,
                powered: true,
            }),
            828u32 => Ok(NoteBlock {
                instrument: NoteBlockInstrument::Bass,
                note: 23i32,
                powered: false,
            }),
            829u32 => Ok(NoteBlock {
                instrument: NoteBlockInstrument::Bass,
                note: 24i32,
                powered: true,
            }),
            830u32 => Ok(NoteBlock {
                instrument: NoteBlockInstrument::Bass,
                note: 24i32,
                powered: false,
            }),
            831u32 => Ok(NoteBlock {
                instrument: NoteBlockInstrument::Flute,
                note: 0i32,
                powered: true,
            }),
            832u32 => Ok(NoteBlock {
                instrument: NoteBlockInstrument::Flute,
                note: 0i32,
                powered: false,
            }),
            833u32 => Ok(NoteBlock {
                instrument: NoteBlockInstrument::Flute,
                note: 1i32,
                powered: true,
            }),
            834u32 => Ok(NoteBlock {
                instrument: NoteBlockInstrument::Flute,
                note: 1i32,
                powered: false,
            }),
            835u32 => Ok(NoteBlock {
                instrument: NoteBlockInstrument::Flute,
                note: 2i32,
                powered: true,
            }),
            836u32 => Ok(NoteBlock {
                instrument: NoteBlockInstrument::Flute,
                note: 2i32,
                powered: false,
            }),
            837u32 => Ok(NoteBlock {
                instrument: NoteBlockInstrument::Flute,
                note: 3i32,
                powered: true,
            }),
            838u32 => Ok(NoteBlock {
                instrument: NoteBlockInstrument::Flute,
                note: 3i32,
                powered: false,
            }),
            839u32 => Ok(NoteBlock {
                instrument: NoteBlockInstrument::Flute,
                note: 4i32,
                powered: true,
            }),
            840u32 => Ok(NoteBlock {
                instrument: NoteBlockInstrument::Flute,
                note: 4i32,
                powered: false,
            }),
            841u32 => Ok(NoteBlock {
                instrument: NoteBlockInstrument::Flute,
                note: 5i32,
                powered: true,
            }),
            842u32 => Ok(NoteBlock {
                instrument: NoteBlockInstrument::Flute,
                note: 5i32,
                powered: false,
            }),
            843u32 => Ok(NoteBlock {
                instrument: NoteBlockInstrument::Flute,
                note: 6i32,
                powered: true,
            }),
            844u32 => Ok(NoteBlock {
                instrument: NoteBlockInstrument::Flute,
                note: 6i32,
                powered: false,
            }),
            845u32 => Ok(NoteBlock {
                instrument: NoteBlockInstrument::Flute,
                note: 7i32,
                powered: true,
            }),
            846u32 => Ok(NoteBlock {
                instrument: NoteBlockInstrument::Flute,
                note: 7i32,
                powered: false,
            }),
            847u32 => Ok(NoteBlock {
                instrument: NoteBlockInstrument::Flute,
                note: 8i32,
                powered: true,
            }),
            848u32 => Ok(NoteBlock {
                instrument: NoteBlockInstrument::Flute,
                note: 8i32,
                powered: false,
            }),
            849u32 => Ok(NoteBlock {
                instrument: NoteBlockInstrument::Flute,
                note: 9i32,
                powered: true,
            }),
            850u32 => Ok(NoteBlock {
                instrument: NoteBlockInstrument::Flute,
                note: 9i32,
                powered: false,
            }),
            851u32 => Ok(NoteBlock {
                instrument: NoteBlockInstrument::Flute,
                note: 10i32,
                powered: true,
            }),
            852u32 => Ok(NoteBlock {
                instrument: NoteBlockInstrument::Flute,
                note: 10i32,
                powered: false,
            }),
            853u32 => Ok(NoteBlock {
                instrument: NoteBlockInstrument::Flute,
                note: 11i32,
                powered: true,
            }),
            854u32 => Ok(NoteBlock {
                instrument: NoteBlockInstrument::Flute,
                note: 11i32,
                powered: false,
            }),
            855u32 => Ok(NoteBlock {
                instrument: NoteBlockInstrument::Flute,
                note: 12i32,
                powered: true,
            }),
            856u32 => Ok(NoteBlock {
                instrument: NoteBlockInstrument::Flute,
                note: 12i32,
                powered: false,
            }),
            857u32 => Ok(NoteBlock {
                instrument: NoteBlockInstrument::Flute,
                note: 13i32,
                powered: true,
            }),
            858u32 => Ok(NoteBlock {
                instrument: NoteBlockInstrument::Flute,
                note: 13i32,
                powered: false,
            }),
            859u32 => Ok(NoteBlock {
                instrument: NoteBlockInstrument::Flute,
                note: 14i32,
                powered: true,
            }),
            860u32 => Ok(NoteBlock {
                instrument: NoteBlockInstrument::Flute,
                note: 14i32,
                powered: false,
            }),
            861u32 => Ok(NoteBlock {
                instrument: NoteBlockInstrument::Flute,
                note: 15i32,
                powered: true,
            }),
            862u32 => Ok(NoteBlock {
                instrument: NoteBlockInstrument::Flute,
                note: 15i32,
                powered: false,
            }),
            863u32 => Ok(NoteBlock {
                instrument: NoteBlockInstrument::Flute,
                note: 16i32,
                powered: true,
            }),
            864u32 => Ok(NoteBlock {
                instrument: NoteBlockInstrument::Flute,
                note: 16i32,
                powered: false,
            }),
            865u32 => Ok(NoteBlock {
                instrument: NoteBlockInstrument::Flute,
                note: 17i32,
                powered: true,
            }),
            866u32 => Ok(NoteBlock {
                instrument: NoteBlockInstrument::Flute,
                note: 17i32,
                powered: false,
            }),
            867u32 => Ok(NoteBlock {
                instrument: NoteBlockInstrument::Flute,
                note: 18i32,
                powered: true,
            }),
            868u32 => Ok(NoteBlock {
                instrument: NoteBlockInstrument::Flute,
                note: 18i32,
                powered: false,
            }),
            869u32 => Ok(NoteBlock {
                instrument: NoteBlockInstrument::Flute,
                note: 19i32,
                powered: true,
            }),
            870u32 => Ok(NoteBlock {
                instrument: NoteBlockInstrument::Flute,
                note: 19i32,
                powered: false,
            }),
            871u32 => Ok(NoteBlock {
                instrument: NoteBlockInstrument::Flute,
                note: 20i32,
                powered: true,
            }),
            872u32 => Ok(NoteBlock {
                instrument: NoteBlockInstrument::Flute,
                note: 20i32,
                powered: false,
            }),
            873u32 => Ok(NoteBlock {
                instrument: NoteBlockInstrument::Flute,
                note: 21i32,
                powered: true,
            }),
            874u32 => Ok(NoteBlock {
                instrument: NoteBlockInstrument::Flute,
                note: 21i32,
                powered: false,
            }),
            875u32 => Ok(NoteBlock {
                instrument: NoteBlockInstrument::Flute,
                note: 22i32,
                powered: true,
            }),
            876u32 => Ok(NoteBlock {
                instrument: NoteBlockInstrument::Flute,
                note: 22i32,
                powered: false,
            }),
            877u32 => Ok(NoteBlock {
                instrument: NoteBlockInstrument::Flute,
                note: 23i32,
                powered: true,
            }),
            878u32 => Ok(NoteBlock {
                instrument: NoteBlockInstrument::Flute,
                note: 23i32,
                powered: false,
            }),
            879u32 => Ok(NoteBlock {
                instrument: NoteBlockInstrument::Flute,
                note: 24i32,
                powered: true,
            }),
            880u32 => Ok(NoteBlock {
                instrument: NoteBlockInstrument::Flute,
                note: 24i32,
                powered: false,
            }),
            881u32 => Ok(NoteBlock {
                instrument: NoteBlockInstrument::Bell,
                note: 0i32,
                powered: true,
            }),
            882u32 => Ok(NoteBlock {
                instrument: NoteBlockInstrument::Bell,
                note: 0i32,
                powered: false,
            }),
            883u32 => Ok(NoteBlock {
                instrument: NoteBlockInstrument::Bell,
                note: 1i32,
                powered: true,
            }),
            884u32 => Ok(NoteBlock {
                instrument: NoteBlockInstrument::Bell,
                note: 1i32,
                powered: false,
            }),
            885u32 => Ok(NoteBlock {
                instrument: NoteBlockInstrument::Bell,
                note: 2i32,
                powered: true,
            }),
            886u32 => Ok(NoteBlock {
                instrument: NoteBlockInstrument::Bell,
                note: 2i32,
                powered: false,
            }),
            887u32 => Ok(NoteBlock {
                instrument: NoteBlockInstrument::Bell,
                note: 3i32,
                powered: true,
            }),
            888u32 => Ok(NoteBlock {
                instrument: NoteBlockInstrument::Bell,
                note: 3i32,
                powered: false,
            }),
            889u32 => Ok(NoteBlock {
                instrument: NoteBlockInstrument::Bell,
                note: 4i32,
                powered: true,
            }),
            890u32 => Ok(NoteBlock {
                instrument: NoteBlockInstrument::Bell,
                note: 4i32,
                powered: false,
            }),
            891u32 => Ok(NoteBlock {
                instrument: NoteBlockInstrument::Bell,
                note: 5i32,
                powered: true,
            }),
            892u32 => Ok(NoteBlock {
                instrument: NoteBlockInstrument::Bell,
                note: 5i32,
                powered: false,
            }),
            893u32 => Ok(NoteBlock {
                instrument: NoteBlockInstrument::Bell,
                note: 6i32,
                powered: true,
            }),
            894u32 => Ok(NoteBlock {
                instrument: NoteBlockInstrument::Bell,
                note: 6i32,
                powered: false,
            }),
            895u32 => Ok(NoteBlock {
                instrument: NoteBlockInstrument::Bell,
                note: 7i32,
                powered: true,
            }),
            896u32 => Ok(NoteBlock {
                instrument: NoteBlockInstrument::Bell,
                note: 7i32,
                powered: false,
            }),
            897u32 => Ok(NoteBlock {
                instrument: NoteBlockInstrument::Bell,
                note: 8i32,
                powered: true,
            }),
            898u32 => Ok(NoteBlock {
                instrument: NoteBlockInstrument::Bell,
                note: 8i32,
                powered: false,
            }),
            899u32 => Ok(NoteBlock {
                instrument: NoteBlockInstrument::Bell,
                note: 9i32,
                powered: true,
            }),
            900u32 => Ok(NoteBlock {
                instrument: NoteBlockInstrument::Bell,
                note: 9i32,
                powered: false,
            }),
            901u32 => Ok(NoteBlock {
                instrument: NoteBlockInstrument::Bell,
                note: 10i32,
                powered: true,
            }),
            902u32 => Ok(NoteBlock {
                instrument: NoteBlockInstrument::Bell,
                note: 10i32,
                powered: false,
            }),
            903u32 => Ok(NoteBlock {
                instrument: NoteBlockInstrument::Bell,
                note: 11i32,
                powered: true,
            }),
            904u32 => Ok(NoteBlock {
                instrument: NoteBlockInstrument::Bell,
                note: 11i32,
                powered: false,
            }),
            905u32 => Ok(NoteBlock {
                instrument: NoteBlockInstrument::Bell,
                note: 12i32,
                powered: true,
            }),
            906u32 => Ok(NoteBlock {
                instrument: NoteBlockInstrument::Bell,
                note: 12i32,
                powered: false,
            }),
            907u32 => Ok(NoteBlock {
                instrument: NoteBlockInstrument::Bell,
                note: 13i32,
                powered: true,
            }),
            908u32 => Ok(NoteBlock {
                instrument: NoteBlockInstrument::Bell,
                note: 13i32,
                powered: false,
            }),
            909u32 => Ok(NoteBlock {
                instrument: NoteBlockInstrument::Bell,
                note: 14i32,
                powered: true,
            }),
            910u32 => Ok(NoteBlock {
                instrument: NoteBlockInstrument::Bell,
                note: 14i32,
                powered: false,
            }),
            911u32 => Ok(NoteBlock {
                instrument: NoteBlockInstrument::Bell,
                note: 15i32,
                powered: true,
            }),
            912u32 => Ok(NoteBlock {
                instrument: NoteBlockInstrument::Bell,
                note: 15i32,
                powered: false,
            }),
            913u32 => Ok(NoteBlock {
                instrument: NoteBlockInstrument::Bell,
                note: 16i32,
                powered: true,
            }),
            914u32 => Ok(NoteBlock {
                instrument: NoteBlockInstrument::Bell,
                note: 16i32,
                powered: false,
            }),
            915u32 => Ok(NoteBlock {
                instrument: NoteBlockInstrument::Bell,
                note: 17i32,
                powered: true,
            }),
            916u32 => Ok(NoteBlock {
                instrument: NoteBlockInstrument::Bell,
                note: 17i32,
                powered: false,
            }),
            917u32 => Ok(NoteBlock {
                instrument: NoteBlockInstrument::Bell,
                note: 18i32,
                powered: true,
            }),
            918u32 => Ok(NoteBlock {
                instrument: NoteBlockInstrument::Bell,
                note: 18i32,
                powered: false,
            }),
            919u32 => Ok(NoteBlock {
                instrument: NoteBlockInstrument::Bell,
                note: 19i32,
                powered: true,
            }),
            920u32 => Ok(NoteBlock {
                instrument: NoteBlockInstrument::Bell,
                note: 19i32,
                powered: false,
            }),
            921u32 => Ok(NoteBlock {
                instrument: NoteBlockInstrument::Bell,
                note: 20i32,
                powered: true,
            }),
            922u32 => Ok(NoteBlock {
                instrument: NoteBlockInstrument::Bell,
                note: 20i32,
                powered: false,
            }),
            923u32 => Ok(NoteBlock {
                instrument: NoteBlockInstrument::Bell,
                note: 21i32,
                powered: true,
            }),
            924u32 => Ok(NoteBlock {
                instrument: NoteBlockInstrument::Bell,
                note: 21i32,
                powered: false,
            }),
            925u32 => Ok(NoteBlock {
                instrument: NoteBlockInstrument::Bell,
                note: 22i32,
                powered: true,
            }),
            926u32 => Ok(NoteBlock {
                instrument: NoteBlockInstrument::Bell,
                note: 22i32,
                powered: false,
            }),
            927u32 => Ok(NoteBlock {
                instrument: NoteBlockInstrument::Bell,
                note: 23i32,
                powered: true,
            }),
            928u32 => Ok(NoteBlock {
                instrument: NoteBlockInstrument::Bell,
                note: 23i32,
                powered: false,
            }),
            929u32 => Ok(NoteBlock {
                instrument: NoteBlockInstrument::Bell,
                note: 24i32,
                powered: true,
            }),
            930u32 => Ok(NoteBlock {
                instrument: NoteBlockInstrument::Bell,
                note: 24i32,
                powered: false,
            }),
            931u32 => Ok(NoteBlock {
                instrument: NoteBlockInstrument::Guitar,
                note: 0i32,
                powered: true,
            }),
            932u32 => Ok(NoteBlock {
                instrument: NoteBlockInstrument::Guitar,
                note: 0i32,
                powered: false,
            }),
            933u32 => Ok(NoteBlock {
                instrument: NoteBlockInstrument::Guitar,
                note: 1i32,
                powered: true,
            }),
            934u32 => Ok(NoteBlock {
                instrument: NoteBlockInstrument::Guitar,
                note: 1i32,
                powered: false,
            }),
            935u32 => Ok(NoteBlock {
                instrument: NoteBlockInstrument::Guitar,
                note: 2i32,
                powered: true,
            }),
            936u32 => Ok(NoteBlock {
                instrument: NoteBlockInstrument::Guitar,
                note: 2i32,
                powered: false,
            }),
            937u32 => Ok(NoteBlock {
                instrument: NoteBlockInstrument::Guitar,
                note: 3i32,
                powered: true,
            }),
            938u32 => Ok(NoteBlock {
                instrument: NoteBlockInstrument::Guitar,
                note: 3i32,
                powered: false,
            }),
            939u32 => Ok(NoteBlock {
                instrument: NoteBlockInstrument::Guitar,
                note: 4i32,
                powered: true,
            }),
            940u32 => Ok(NoteBlock {
                instrument: NoteBlockInstrument::Guitar,
                note: 4i32,
                powered: false,
            }),
            941u32 => Ok(NoteBlock {
                instrument: NoteBlockInstrument::Guitar,
                note: 5i32,
                powered: true,
            }),
            942u32 => Ok(NoteBlock {
                instrument: NoteBlockInstrument::Guitar,
                note: 5i32,
                powered: false,
            }),
            943u32 => Ok(NoteBlock {
                instrument: NoteBlockInstrument::Guitar,
                note: 6i32,
                powered: true,
            }),
            944u32 => Ok(NoteBlock {
                instrument: NoteBlockInstrument::Guitar,
                note: 6i32,
                powered: false,
            }),
            945u32 => Ok(NoteBlock {
                instrument: NoteBlockInstrument::Guitar,
                note: 7i32,
                powered: true,
            }),
            946u32 => Ok(NoteBlock {
                instrument: NoteBlockInstrument::Guitar,
                note: 7i32,
                powered: false,
            }),
            947u32 => Ok(NoteBlock {
                instrument: NoteBlockInstrument::Guitar,
                note: 8i32,
                powered: true,
            }),
            948u32 => Ok(NoteBlock {
                instrument: NoteBlockInstrument::Guitar,
                note: 8i32,
                powered: false,
            }),
            949u32 => Ok(NoteBlock {
                instrument: NoteBlockInstrument::Guitar,
                note: 9i32,
                powered: true,
            }),
            950u32 => Ok(NoteBlock {
                instrument: NoteBlockInstrument::Guitar,
                note: 9i32,
                powered: false,
            }),
            951u32 => Ok(NoteBlock {
                instrument: NoteBlockInstrument::Guitar,
                note: 10i32,
                powered: true,
            }),
            952u32 => Ok(NoteBlock {
                instrument: NoteBlockInstrument::Guitar,
                note: 10i32,
                powered: false,
            }),
            953u32 => Ok(NoteBlock {
                instrument: NoteBlockInstrument::Guitar,
                note: 11i32,
                powered: true,
            }),
            954u32 => Ok(NoteBlock {
                instrument: NoteBlockInstrument::Guitar,
                note: 11i32,
                powered: false,
            }),
            955u32 => Ok(NoteBlock {
                instrument: NoteBlockInstrument::Guitar,
                note: 12i32,
                powered: true,
            }),
            956u32 => Ok(NoteBlock {
                instrument: NoteBlockInstrument::Guitar,
                note: 12i32,
                powered: false,
            }),
            957u32 => Ok(NoteBlock {
                instrument: NoteBlockInstrument::Guitar,
                note: 13i32,
                powered: true,
            }),
            958u32 => Ok(NoteBlock {
                instrument: NoteBlockInstrument::Guitar,
                note: 13i32,
                powered: false,
            }),
            959u32 => Ok(NoteBlock {
                instrument: NoteBlockInstrument::Guitar,
                note: 14i32,
                powered: true,
            }),
            960u32 => Ok(NoteBlock {
                instrument: NoteBlockInstrument::Guitar,
                note: 14i32,
                powered: false,
            }),
            961u32 => Ok(NoteBlock {
                instrument: NoteBlockInstrument::Guitar,
                note: 15i32,
                powered: true,
            }),
            962u32 => Ok(NoteBlock {
                instrument: NoteBlockInstrument::Guitar,
                note: 15i32,
                powered: false,
            }),
            963u32 => Ok(NoteBlock {
                instrument: NoteBlockInstrument::Guitar,
                note: 16i32,
                powered: true,
            }),
            964u32 => Ok(NoteBlock {
                instrument: NoteBlockInstrument::Guitar,
                note: 16i32,
                powered: false,
            }),
            965u32 => Ok(NoteBlock {
                instrument: NoteBlockInstrument::Guitar,
                note: 17i32,
                powered: true,
            }),
            966u32 => Ok(NoteBlock {
                instrument: NoteBlockInstrument::Guitar,
                note: 17i32,
                powered: false,
            }),
            967u32 => Ok(NoteBlock {
                instrument: NoteBlockInstrument::Guitar,
                note: 18i32,
                powered: true,
            }),
            968u32 => Ok(NoteBlock {
                instrument: NoteBlockInstrument::Guitar,
                note: 18i32,
                powered: false,
            }),
            969u32 => Ok(NoteBlock {
                instrument: NoteBlockInstrument::Guitar,
                note: 19i32,
                powered: true,
            }),
            970u32 => Ok(NoteBlock {
                instrument: NoteBlockInstrument::Guitar,
                note: 19i32,
                powered: false,
            }),
            971u32 => Ok(NoteBlock {
                instrument: NoteBlockInstrument::Guitar,
                note: 20i32,
                powered: true,
            }),
            972u32 => Ok(NoteBlock {
                instrument: NoteBlockInstrument::Guitar,
                note: 20i32,
                powered: false,
            }),
            973u32 => Ok(NoteBlock {
                instrument: NoteBlockInstrument::Guitar,
                note: 21i32,
                powered: true,
            }),
            974u32 => Ok(NoteBlock {
                instrument: NoteBlockInstrument::Guitar,
                note: 21i32,
                powered: false,
            }),
            975u32 => Ok(NoteBlock {
                instrument: NoteBlockInstrument::Guitar,
                note: 22i32,
                powered: true,
            }),
            976u32 => Ok(NoteBlock {
                instrument: NoteBlockInstrument::Guitar,
                note: 22i32,
                powered: false,
            }),
            977u32 => Ok(NoteBlock {
                instrument: NoteBlockInstrument::Guitar,
                note: 23i32,
                powered: true,
            }),
            978u32 => Ok(NoteBlock {
                instrument: NoteBlockInstrument::Guitar,
                note: 23i32,
                powered: false,
            }),
            979u32 => Ok(NoteBlock {
                instrument: NoteBlockInstrument::Guitar,
                note: 24i32,
                powered: true,
            }),
            980u32 => Ok(NoteBlock {
                instrument: NoteBlockInstrument::Guitar,
                note: 24i32,
                powered: false,
            }),
            981u32 => Ok(NoteBlock {
                instrument: NoteBlockInstrument::Chime,
                note: 0i32,
                powered: true,
            }),
            982u32 => Ok(NoteBlock {
                instrument: NoteBlockInstrument::Chime,
                note: 0i32,
                powered: false,
            }),
            983u32 => Ok(NoteBlock {
                instrument: NoteBlockInstrument::Chime,
                note: 1i32,
                powered: true,
            }),
            984u32 => Ok(NoteBlock {
                instrument: NoteBlockInstrument::Chime,
                note: 1i32,
                powered: false,
            }),
            985u32 => Ok(NoteBlock {
                instrument: NoteBlockInstrument::Chime,
                note: 2i32,
                powered: true,
            }),
            986u32 => Ok(NoteBlock {
                instrument: NoteBlockInstrument::Chime,
                note: 2i32,
                powered: false,
            }),
            987u32 => Ok(NoteBlock {
                instrument: NoteBlockInstrument::Chime,
                note: 3i32,
                powered: true,
            }),
            988u32 => Ok(NoteBlock {
                instrument: NoteBlockInstrument::Chime,
                note: 3i32,
                powered: false,
            }),
            989u32 => Ok(NoteBlock {
                instrument: NoteBlockInstrument::Chime,
                note: 4i32,
                powered: true,
            }),
            990u32 => Ok(NoteBlock {
                instrument: NoteBlockInstrument::Chime,
                note: 4i32,
                powered: false,
            }),
            991u32 => Ok(NoteBlock {
                instrument: NoteBlockInstrument::Chime,
                note: 5i32,
                powered: true,
            }),
            992u32 => Ok(NoteBlock {
                instrument: NoteBlockInstrument::Chime,
                note: 5i32,
                powered: false,
            }),
            993u32 => Ok(NoteBlock {
                instrument: NoteBlockInstrument::Chime,
                note: 6i32,
                powered: true,
            }),
            994u32 => Ok(NoteBlock {
                instrument: NoteBlockInstrument::Chime,
                note: 6i32,
                powered: false,
            }),
            995u32 => Ok(NoteBlock {
                instrument: NoteBlockInstrument::Chime,
                note: 7i32,
                powered: true,
            }),
            996u32 => Ok(NoteBlock {
                instrument: NoteBlockInstrument::Chime,
                note: 7i32,
                powered: false,
            }),
            997u32 => Ok(NoteBlock {
                instrument: NoteBlockInstrument::Chime,
                note: 8i32,
                powered: true,
            }),
            998u32 => Ok(NoteBlock {
                instrument: NoteBlockInstrument::Chime,
                note: 8i32,
                powered: false,
            }),
            999u32 => Ok(NoteBlock {
                instrument: NoteBlockInstrument::Chime,
                note: 9i32,
                powered: true,
            }),
            1000u32 => Ok(NoteBlock {
                instrument: NoteBlockInstrument::Chime,
                note: 9i32,
                powered: false,
            }),
            1001u32 => Ok(NoteBlock {
                instrument: NoteBlockInstrument::Chime,
                note: 10i32,
                powered: true,
            }),
            1002u32 => Ok(NoteBlock {
                instrument: NoteBlockInstrument::Chime,
                note: 10i32,
                powered: false,
            }),
            1003u32 => Ok(NoteBlock {
                instrument: NoteBlockInstrument::Chime,
                note: 11i32,
                powered: true,
            }),
            1004u32 => Ok(NoteBlock {
                instrument: NoteBlockInstrument::Chime,
                note: 11i32,
                powered: false,
            }),
            1005u32 => Ok(NoteBlock {
                instrument: NoteBlockInstrument::Chime,
                note: 12i32,
                powered: true,
            }),
            1006u32 => Ok(NoteBlock {
                instrument: NoteBlockInstrument::Chime,
                note: 12i32,
                powered: false,
            }),
            1007u32 => Ok(NoteBlock {
                instrument: NoteBlockInstrument::Chime,
                note: 13i32,
                powered: true,
            }),
            1008u32 => Ok(NoteBlock {
                instrument: NoteBlockInstrument::Chime,
                note: 13i32,
                powered: false,
            }),
            1009u32 => Ok(NoteBlock {
                instrument: NoteBlockInstrument::Chime,
                note: 14i32,
                powered: true,
            }),
            1010u32 => Ok(NoteBlock {
                instrument: NoteBlockInstrument::Chime,
                note: 14i32,
                powered: false,
            }),
            1011u32 => Ok(NoteBlock {
                instrument: NoteBlockInstrument::Chime,
                note: 15i32,
                powered: true,
            }),
            1012u32 => Ok(NoteBlock {
                instrument: NoteBlockInstrument::Chime,
                note: 15i32,
                powered: false,
            }),
            1013u32 => Ok(NoteBlock {
                instrument: NoteBlockInstrument::Chime,
                note: 16i32,
                powered: true,
            }),
            1014u32 => Ok(NoteBlock {
                instrument: NoteBlockInstrument::Chime,
                note: 16i32,
                powered: false,
            }),
            1015u32 => Ok(NoteBlock {
                instrument: NoteBlockInstrument::Chime,
                note: 17i32,
                powered: true,
            }),
            1016u32 => Ok(NoteBlock {
                instrument: NoteBlockInstrument::Chime,
                note: 17i32,
                powered: false,
            }),
            1017u32 => Ok(NoteBlock {
                instrument: NoteBlockInstrument::Chime,
                note: 18i32,
                powered: true,
            }),
            1018u32 => Ok(NoteBlock {
                instrument: NoteBlockInstrument::Chime,
                note: 18i32,
                powered: false,
            }),
            1019u32 => Ok(NoteBlock {
                instrument: NoteBlockInstrument::Chime,
                note: 19i32,
                powered: true,
            }),
            1020u32 => Ok(NoteBlock {
                instrument: NoteBlockInstrument::Chime,
                note: 19i32,
                powered: false,
            }),
            1021u32 => Ok(NoteBlock {
                instrument: NoteBlockInstrument::Chime,
                note: 20i32,
                powered: true,
            }),
            1022u32 => Ok(NoteBlock {
                instrument: NoteBlockInstrument::Chime,
                note: 20i32,
                powered: false,
            }),
            1023u32 => Ok(NoteBlock {
                instrument: NoteBlockInstrument::Chime,
                note: 21i32,
                powered: true,
            }),
            1024u32 => Ok(NoteBlock {
                instrument: NoteBlockInstrument::Chime,
                note: 21i32,
                powered: false,
            }),
            1025u32 => Ok(NoteBlock {
                instrument: NoteBlockInstrument::Chime,
                note: 22i32,
                powered: true,
            }),
            1026u32 => Ok(NoteBlock {
                instrument: NoteBlockInstrument::Chime,
                note: 22i32,
                powered: false,
            }),
            1027u32 => Ok(NoteBlock {
                instrument: NoteBlockInstrument::Chime,
                note: 23i32,
                powered: true,
            }),
            1028u32 => Ok(NoteBlock {
                instrument: NoteBlockInstrument::Chime,
                note: 23i32,
                powered: false,
            }),
            1029u32 => Ok(NoteBlock {
                instrument: NoteBlockInstrument::Chime,
                note: 24i32,
                powered: true,
            }),
            1030u32 => Ok(NoteBlock {
                instrument: NoteBlockInstrument::Chime,
                note: 24i32,
                powered: false,
            }),
            1031u32 => Ok(NoteBlock {
                instrument: NoteBlockInstrument::Xylophone,
                note: 0i32,
                powered: true,
            }),
            1032u32 => Ok(NoteBlock {
                instrument: NoteBlockInstrument::Xylophone,
                note: 0i32,
                powered: false,
            }),
            1033u32 => Ok(NoteBlock {
                instrument: NoteBlockInstrument::Xylophone,
                note: 1i32,
                powered: true,
            }),
            1034u32 => Ok(NoteBlock {
                instrument: NoteBlockInstrument::Xylophone,
                note: 1i32,
                powered: false,
            }),
            1035u32 => Ok(NoteBlock {
                instrument: NoteBlockInstrument::Xylophone,
                note: 2i32,
                powered: true,
            }),
            1036u32 => Ok(NoteBlock {
                instrument: NoteBlockInstrument::Xylophone,
                note: 2i32,
                powered: false,
            }),
            1037u32 => Ok(NoteBlock {
                instrument: NoteBlockInstrument::Xylophone,
                note: 3i32,
                powered: true,
            }),
            1038u32 => Ok(NoteBlock {
                instrument: NoteBlockInstrument::Xylophone,
                note: 3i32,
                powered: false,
            }),
            1039u32 => Ok(NoteBlock {
                instrument: NoteBlockInstrument::Xylophone,
                note: 4i32,
                powered: true,
            }),
            1040u32 => Ok(NoteBlock {
                instrument: NoteBlockInstrument::Xylophone,
                note: 4i32,
                powered: false,
            }),
            1041u32 => Ok(NoteBlock {
                instrument: NoteBlockInstrument::Xylophone,
                note: 5i32,
                powered: true,
            }),
            1042u32 => Ok(NoteBlock {
                instrument: NoteBlockInstrument::Xylophone,
                note: 5i32,
                powered: false,
            }),
            1043u32 => Ok(NoteBlock {
                instrument: NoteBlockInstrument::Xylophone,
                note: 6i32,
                powered: true,
            }),
            1044u32 => Ok(NoteBlock {
                instrument: NoteBlockInstrument::Xylophone,
                note: 6i32,
                powered: false,
            }),
            1045u32 => Ok(NoteBlock {
                instrument: NoteBlockInstrument::Xylophone,
                note: 7i32,
                powered: true,
            }),
            1046u32 => Ok(NoteBlock {
                instrument: NoteBlockInstrument::Xylophone,
                note: 7i32,
                powered: false,
            }),
            1047u32 => Ok(NoteBlock {
                instrument: NoteBlockInstrument::Xylophone,
                note: 8i32,
                powered: true,
            }),
            1048u32 => Ok(NoteBlock {
                instrument: NoteBlockInstrument::Xylophone,
                note: 8i32,
                powered: false,
            }),
            1049u32 => Ok(NoteBlock {
                instrument: NoteBlockInstrument::Xylophone,
                note: 9i32,
                powered: true,
            }),
            1050u32 => Ok(NoteBlock {
                instrument: NoteBlockInstrument::Xylophone,
                note: 9i32,
                powered: false,
            }),
            1051u32 => Ok(NoteBlock {
                instrument: NoteBlockInstrument::Xylophone,
                note: 10i32,
                powered: true,
            }),
            1052u32 => Ok(NoteBlock {
                instrument: NoteBlockInstrument::Xylophone,
                note: 10i32,
                powered: false,
            }),
            1053u32 => Ok(NoteBlock {
                instrument: NoteBlockInstrument::Xylophone,
                note: 11i32,
                powered: true,
            }),
            1054u32 => Ok(NoteBlock {
                instrument: NoteBlockInstrument::Xylophone,
                note: 11i32,
                powered: false,
            }),
            1055u32 => Ok(NoteBlock {
                instrument: NoteBlockInstrument::Xylophone,
                note: 12i32,
                powered: true,
            }),
            1056u32 => Ok(NoteBlock {
                instrument: NoteBlockInstrument::Xylophone,
                note: 12i32,
                powered: false,
            }),
            1057u32 => Ok(NoteBlock {
                instrument: NoteBlockInstrument::Xylophone,
                note: 13i32,
                powered: true,
            }),
            1058u32 => Ok(NoteBlock {
                instrument: NoteBlockInstrument::Xylophone,
                note: 13i32,
                powered: false,
            }),
            1059u32 => Ok(NoteBlock {
                instrument: NoteBlockInstrument::Xylophone,
                note: 14i32,
                powered: true,
            }),
            1060u32 => Ok(NoteBlock {
                instrument: NoteBlockInstrument::Xylophone,
                note: 14i32,
                powered: false,
            }),
            1061u32 => Ok(NoteBlock {
                instrument: NoteBlockInstrument::Xylophone,
                note: 15i32,
                powered: true,
            }),
            1062u32 => Ok(NoteBlock {
                instrument: NoteBlockInstrument::Xylophone,
                note: 15i32,
                powered: false,
            }),
            1063u32 => Ok(NoteBlock {
                instrument: NoteBlockInstrument::Xylophone,
                note: 16i32,
                powered: true,
            }),
            1064u32 => Ok(NoteBlock {
                instrument: NoteBlockInstrument::Xylophone,
                note: 16i32,
                powered: false,
            }),
            1065u32 => Ok(NoteBlock {
                instrument: NoteBlockInstrument::Xylophone,
                note: 17i32,
                powered: true,
            }),
            1066u32 => Ok(NoteBlock {
                instrument: NoteBlockInstrument::Xylophone,
                note: 17i32,
                powered: false,
            }),
            1067u32 => Ok(NoteBlock {
                instrument: NoteBlockInstrument::Xylophone,
                note: 18i32,
                powered: true,
            }),
            1068u32 => Ok(NoteBlock {
                instrument: NoteBlockInstrument::Xylophone,
                note: 18i32,
                powered: false,
            }),
            1069u32 => Ok(NoteBlock {
                instrument: NoteBlockInstrument::Xylophone,
                note: 19i32,
                powered: true,
            }),
            1070u32 => Ok(NoteBlock {
                instrument: NoteBlockInstrument::Xylophone,
                note: 19i32,
                powered: false,
            }),
            1071u32 => Ok(NoteBlock {
                instrument: NoteBlockInstrument::Xylophone,
                note: 20i32,
                powered: true,
            }),
            1072u32 => Ok(NoteBlock {
                instrument: NoteBlockInstrument::Xylophone,
                note: 20i32,
                powered: false,
            }),
            1073u32 => Ok(NoteBlock {
                instrument: NoteBlockInstrument::Xylophone,
                note: 21i32,
                powered: true,
            }),
            1074u32 => Ok(NoteBlock {
                instrument: NoteBlockInstrument::Xylophone,
                note: 21i32,
                powered: false,
            }),
            1075u32 => Ok(NoteBlock {
                instrument: NoteBlockInstrument::Xylophone,
                note: 22i32,
                powered: true,
            }),
            1076u32 => Ok(NoteBlock {
                instrument: NoteBlockInstrument::Xylophone,
                note: 22i32,
                powered: false,
            }),
            1077u32 => Ok(NoteBlock {
                instrument: NoteBlockInstrument::Xylophone,
                note: 23i32,
                powered: true,
            }),
            1078u32 => Ok(NoteBlock {
                instrument: NoteBlockInstrument::Xylophone,
                note: 23i32,
                powered: false,
            }),
            1079u32 => Ok(NoteBlock {
                instrument: NoteBlockInstrument::Xylophone,
                note: 24i32,
                powered: true,
            }),
            1080u32 => Ok(NoteBlock {
                instrument: NoteBlockInstrument::Xylophone,
                note: 24i32,
                powered: false,
            }),
            1081u32 => Ok(NoteBlock {
                instrument: NoteBlockInstrument::IronXylophone,
                note: 0i32,
                powered: true,
            }),
            1082u32 => Ok(NoteBlock {
                instrument: NoteBlockInstrument::IronXylophone,
                note: 0i32,
                powered: false,
            }),
            1083u32 => Ok(NoteBlock {
                instrument: NoteBlockInstrument::IronXylophone,
                note: 1i32,
                powered: true,
            }),
            1084u32 => Ok(NoteBlock {
                instrument: NoteBlockInstrument::IronXylophone,
                note: 1i32,
                powered: false,
            }),
            1085u32 => Ok(NoteBlock {
                instrument: NoteBlockInstrument::IronXylophone,
                note: 2i32,
                powered: true,
            }),
            1086u32 => Ok(NoteBlock {
                instrument: NoteBlockInstrument::IronXylophone,
                note: 2i32,
                powered: false,
            }),
            1087u32 => Ok(NoteBlock {
                instrument: NoteBlockInstrument::IronXylophone,
                note: 3i32,
                powered: true,
            }),
            1088u32 => Ok(NoteBlock {
                instrument: NoteBlockInstrument::IronXylophone,
                note: 3i32,
                powered: false,
            }),
            1089u32 => Ok(NoteBlock {
                instrument: NoteBlockInstrument::IronXylophone,
                note: 4i32,
                powered: true,
            }),
            1090u32 => Ok(NoteBlock {
                instrument: NoteBlockInstrument::IronXylophone,
                note: 4i32,
                powered: false,
            }),
            1091u32 => Ok(NoteBlock {
                instrument: NoteBlockInstrument::IronXylophone,
                note: 5i32,
                powered: true,
            }),
            1092u32 => Ok(NoteBlock {
                instrument: NoteBlockInstrument::IronXylophone,
                note: 5i32,
                powered: false,
            }),
            1093u32 => Ok(NoteBlock {
                instrument: NoteBlockInstrument::IronXylophone,
                note: 6i32,
                powered: true,
            }),
            1094u32 => Ok(NoteBlock {
                instrument: NoteBlockInstrument::IronXylophone,
                note: 6i32,
                powered: false,
            }),
            1095u32 => Ok(NoteBlock {
                instrument: NoteBlockInstrument::IronXylophone,
                note: 7i32,
                powered: true,
            }),
            1096u32 => Ok(NoteBlock {
                instrument: NoteBlockInstrument::IronXylophone,
                note: 7i32,
                powered: false,
            }),
            1097u32 => Ok(NoteBlock {
                instrument: NoteBlockInstrument::IronXylophone,
                note: 8i32,
                powered: true,
            }),
            1098u32 => Ok(NoteBlock {
                instrument: NoteBlockInstrument::IronXylophone,
                note: 8i32,
                powered: false,
            }),
            1099u32 => Ok(NoteBlock {
                instrument: NoteBlockInstrument::IronXylophone,
                note: 9i32,
                powered: true,
            }),
            1100u32 => Ok(NoteBlock {
                instrument: NoteBlockInstrument::IronXylophone,
                note: 9i32,
                powered: false,
            }),
            1101u32 => Ok(NoteBlock {
                instrument: NoteBlockInstrument::IronXylophone,
                note: 10i32,
                powered: true,
            }),
            1102u32 => Ok(NoteBlock {
                instrument: NoteBlockInstrument::IronXylophone,
                note: 10i32,
                powered: false,
            }),
            1103u32 => Ok(NoteBlock {
                instrument: NoteBlockInstrument::IronXylophone,
                note: 11i32,
                powered: true,
            }),
            1104u32 => Ok(NoteBlock {
                instrument: NoteBlockInstrument::IronXylophone,
                note: 11i32,
                powered: false,
            }),
            1105u32 => Ok(NoteBlock {
                instrument: NoteBlockInstrument::IronXylophone,
                note: 12i32,
                powered: true,
            }),
            1106u32 => Ok(NoteBlock {
                instrument: NoteBlockInstrument::IronXylophone,
                note: 12i32,
                powered: false,
            }),
            1107u32 => Ok(NoteBlock {
                instrument: NoteBlockInstrument::IronXylophone,
                note: 13i32,
                powered: true,
            }),
            1108u32 => Ok(NoteBlock {
                instrument: NoteBlockInstrument::IronXylophone,
                note: 13i32,
                powered: false,
            }),
            1109u32 => Ok(NoteBlock {
                instrument: NoteBlockInstrument::IronXylophone,
                note: 14i32,
                powered: true,
            }),
            1110u32 => Ok(NoteBlock {
                instrument: NoteBlockInstrument::IronXylophone,
                note: 14i32,
                powered: false,
            }),
            1111u32 => Ok(NoteBlock {
                instrument: NoteBlockInstrument::IronXylophone,
                note: 15i32,
                powered: true,
            }),
            1112u32 => Ok(NoteBlock {
                instrument: NoteBlockInstrument::IronXylophone,
                note: 15i32,
                powered: false,
            }),
            1113u32 => Ok(NoteBlock {
                instrument: NoteBlockInstrument::IronXylophone,
                note: 16i32,
                powered: true,
            }),
            1114u32 => Ok(NoteBlock {
                instrument: NoteBlockInstrument::IronXylophone,
                note: 16i32,
                powered: false,
            }),
            1115u32 => Ok(NoteBlock {
                instrument: NoteBlockInstrument::IronXylophone,
                note: 17i32,
                powered: true,
            }),
            1116u32 => Ok(NoteBlock {
                instrument: NoteBlockInstrument::IronXylophone,
                note: 17i32,
                powered: false,
            }),
            1117u32 => Ok(NoteBlock {
                instrument: NoteBlockInstrument::IronXylophone,
                note: 18i32,
                powered: true,
            }),
            1118u32 => Ok(NoteBlock {
                instrument: NoteBlockInstrument::IronXylophone,
                note: 18i32,
                powered: false,
            }),
            1119u32 => Ok(NoteBlock {
                instrument: NoteBlockInstrument::IronXylophone,
                note: 19i32,
                powered: true,
            }),
            1120u32 => Ok(NoteBlock {
                instrument: NoteBlockInstrument::IronXylophone,
                note: 19i32,
                powered: false,
            }),
            1121u32 => Ok(NoteBlock {
                instrument: NoteBlockInstrument::IronXylophone,
                note: 20i32,
                powered: true,
            }),
            1122u32 => Ok(NoteBlock {
                instrument: NoteBlockInstrument::IronXylophone,
                note: 20i32,
                powered: false,
            }),
            1123u32 => Ok(NoteBlock {
                instrument: NoteBlockInstrument::IronXylophone,
                note: 21i32,
                powered: true,
            }),
            1124u32 => Ok(NoteBlock {
                instrument: NoteBlockInstrument::IronXylophone,
                note: 21i32,
                powered: false,
            }),
            1125u32 => Ok(NoteBlock {
                instrument: NoteBlockInstrument::IronXylophone,
                note: 22i32,
                powered: true,
            }),
            1126u32 => Ok(NoteBlock {
                instrument: NoteBlockInstrument::IronXylophone,
                note: 22i32,
                powered: false,
            }),
            1127u32 => Ok(NoteBlock {
                instrument: NoteBlockInstrument::IronXylophone,
                note: 23i32,
                powered: true,
            }),
            1128u32 => Ok(NoteBlock {
                instrument: NoteBlockInstrument::IronXylophone,
                note: 23i32,
                powered: false,
            }),
            1129u32 => Ok(NoteBlock {
                instrument: NoteBlockInstrument::IronXylophone,
                note: 24i32,
                powered: true,
            }),
            1130u32 => Ok(NoteBlock {
                instrument: NoteBlockInstrument::IronXylophone,
                note: 24i32,
                powered: false,
            }),
            1131u32 => Ok(NoteBlock {
                instrument: NoteBlockInstrument::CowBell,
                note: 0i32,
                powered: true,
            }),
            1132u32 => Ok(NoteBlock {
                instrument: NoteBlockInstrument::CowBell,
                note: 0i32,
                powered: false,
            }),
            1133u32 => Ok(NoteBlock {
                instrument: NoteBlockInstrument::CowBell,
                note: 1i32,
                powered: true,
            }),
            1134u32 => Ok(NoteBlock {
                instrument: NoteBlockInstrument::CowBell,
                note: 1i32,
                powered: false,
            }),
            1135u32 => Ok(NoteBlock {
                instrument: NoteBlockInstrument::CowBell,
                note: 2i32,
                powered: true,
            }),
            1136u32 => Ok(NoteBlock {
                instrument: NoteBlockInstrument::CowBell,
                note: 2i32,
                powered: false,
            }),
            1137u32 => Ok(NoteBlock {
                instrument: NoteBlockInstrument::CowBell,
                note: 3i32,
                powered: true,
            }),
            1138u32 => Ok(NoteBlock {
                instrument: NoteBlockInstrument::CowBell,
                note: 3i32,
                powered: false,
            }),
            1139u32 => Ok(NoteBlock {
                instrument: NoteBlockInstrument::CowBell,
                note: 4i32,
                powered: true,
            }),
            1140u32 => Ok(NoteBlock {
                instrument: NoteBlockInstrument::CowBell,
                note: 4i32,
                powered: false,
            }),
            1141u32 => Ok(NoteBlock {
                instrument: NoteBlockInstrument::CowBell,
                note: 5i32,
                powered: true,
            }),
            1142u32 => Ok(NoteBlock {
                instrument: NoteBlockInstrument::CowBell,
                note: 5i32,
                powered: false,
            }),
            1143u32 => Ok(NoteBlock {
                instrument: NoteBlockInstrument::CowBell,
                note: 6i32,
                powered: true,
            }),
            1144u32 => Ok(NoteBlock {
                instrument: NoteBlockInstrument::CowBell,
                note: 6i32,
                powered: false,
            }),
            1145u32 => Ok(NoteBlock {
                instrument: NoteBlockInstrument::CowBell,
                note: 7i32,
                powered: true,
            }),
            1146u32 => Ok(NoteBlock {
                instrument: NoteBlockInstrument::CowBell,
                note: 7i32,
                powered: false,
            }),
            1147u32 => Ok(NoteBlock {
                instrument: NoteBlockInstrument::CowBell,
                note: 8i32,
                powered: true,
            }),
            1148u32 => Ok(NoteBlock {
                instrument: NoteBlockInstrument::CowBell,
                note: 8i32,
                powered: false,
            }),
            1149u32 => Ok(NoteBlock {
                instrument: NoteBlockInstrument::CowBell,
                note: 9i32,
                powered: true,
            }),
            1150u32 => Ok(NoteBlock {
                instrument: NoteBlockInstrument::CowBell,
                note: 9i32,
                powered: false,
            }),
            1151u32 => Ok(NoteBlock {
                instrument: NoteBlockInstrument::CowBell,
                note: 10i32,
                powered: true,
            }),
            1152u32 => Ok(NoteBlock {
                instrument: NoteBlockInstrument::CowBell,
                note: 10i32,
                powered: false,
            }),
            1153u32 => Ok(NoteBlock {
                instrument: NoteBlockInstrument::CowBell,
                note: 11i32,
                powered: true,
            }),
            1154u32 => Ok(NoteBlock {
                instrument: NoteBlockInstrument::CowBell,
                note: 11i32,
                powered: false,
            }),
            1155u32 => Ok(NoteBlock {
                instrument: NoteBlockInstrument::CowBell,
                note: 12i32,
                powered: true,
            }),
            1156u32 => Ok(NoteBlock {
                instrument: NoteBlockInstrument::CowBell,
                note: 12i32,
                powered: false,
            }),
            1157u32 => Ok(NoteBlock {
                instrument: NoteBlockInstrument::CowBell,
                note: 13i32,
                powered: true,
            }),
            1158u32 => Ok(NoteBlock {
                instrument: NoteBlockInstrument::CowBell,
                note: 13i32,
                powered: false,
            }),
            1159u32 => Ok(NoteBlock {
                instrument: NoteBlockInstrument::CowBell,
                note: 14i32,
                powered: true,
            }),
            1160u32 => Ok(NoteBlock {
                instrument: NoteBlockInstrument::CowBell,
                note: 14i32,
                powered: false,
            }),
            1161u32 => Ok(NoteBlock {
                instrument: NoteBlockInstrument::CowBell,
                note: 15i32,
                powered: true,
            }),
            1162u32 => Ok(NoteBlock {
                instrument: NoteBlockInstrument::CowBell,
                note: 15i32,
                powered: false,
            }),
            1163u32 => Ok(NoteBlock {
                instrument: NoteBlockInstrument::CowBell,
                note: 16i32,
                powered: true,
            }),
            1164u32 => Ok(NoteBlock {
                instrument: NoteBlockInstrument::CowBell,
                note: 16i32,
                powered: false,
            }),
            1165u32 => Ok(NoteBlock {
                instrument: NoteBlockInstrument::CowBell,
                note: 17i32,
                powered: true,
            }),
            1166u32 => Ok(NoteBlock {
                instrument: NoteBlockInstrument::CowBell,
                note: 17i32,
                powered: false,
            }),
            1167u32 => Ok(NoteBlock {
                instrument: NoteBlockInstrument::CowBell,
                note: 18i32,
                powered: true,
            }),
            1168u32 => Ok(NoteBlock {
                instrument: NoteBlockInstrument::CowBell,
                note: 18i32,
                powered: false,
            }),
            1169u32 => Ok(NoteBlock {
                instrument: NoteBlockInstrument::CowBell,
                note: 19i32,
                powered: true,
            }),
            1170u32 => Ok(NoteBlock {
                instrument: NoteBlockInstrument::CowBell,
                note: 19i32,
                powered: false,
            }),
            1171u32 => Ok(NoteBlock {
                instrument: NoteBlockInstrument::CowBell,
                note: 20i32,
                powered: true,
            }),
            1172u32 => Ok(NoteBlock {
                instrument: NoteBlockInstrument::CowBell,
                note: 20i32,
                powered: false,
            }),
            1173u32 => Ok(NoteBlock {
                instrument: NoteBlockInstrument::CowBell,
                note: 21i32,
                powered: true,
            }),
            1174u32 => Ok(NoteBlock {
                instrument: NoteBlockInstrument::CowBell,
                note: 21i32,
                powered: false,
            }),
            1175u32 => Ok(NoteBlock {
                instrument: NoteBlockInstrument::CowBell,
                note: 22i32,
                powered: true,
            }),
            1176u32 => Ok(NoteBlock {
                instrument: NoteBlockInstrument::CowBell,
                note: 22i32,
                powered: false,
            }),
            1177u32 => Ok(NoteBlock {
                instrument: NoteBlockInstrument::CowBell,
                note: 23i32,
                powered: true,
            }),
            1178u32 => Ok(NoteBlock {
                instrument: NoteBlockInstrument::CowBell,
                note: 23i32,
                powered: false,
            }),
            1179u32 => Ok(NoteBlock {
                instrument: NoteBlockInstrument::CowBell,
                note: 24i32,
                powered: true,
            }),
            1180u32 => Ok(NoteBlock {
                instrument: NoteBlockInstrument::CowBell,
                note: 24i32,
                powered: false,
            }),
            1181u32 => Ok(NoteBlock {
                instrument: NoteBlockInstrument::Didgeridoo,
                note: 0i32,
                powered: true,
            }),
            1182u32 => Ok(NoteBlock {
                instrument: NoteBlockInstrument::Didgeridoo,
                note: 0i32,
                powered: false,
            }),
            1183u32 => Ok(NoteBlock {
                instrument: NoteBlockInstrument::Didgeridoo,
                note: 1i32,
                powered: true,
            }),
            1184u32 => Ok(NoteBlock {
                instrument: NoteBlockInstrument::Didgeridoo,
                note: 1i32,
                powered: false,
            }),
            1185u32 => Ok(NoteBlock {
                instrument: NoteBlockInstrument::Didgeridoo,
                note: 2i32,
                powered: true,
            }),
            1186u32 => Ok(NoteBlock {
                instrument: NoteBlockInstrument::Didgeridoo,
                note: 2i32,
                powered: false,
            }),
            1187u32 => Ok(NoteBlock {
                instrument: NoteBlockInstrument::Didgeridoo,
                note: 3i32,
                powered: true,
            }),
            1188u32 => Ok(NoteBlock {
                instrument: NoteBlockInstrument::Didgeridoo,
                note: 3i32,
                powered: false,
            }),
            1189u32 => Ok(NoteBlock {
                instrument: NoteBlockInstrument::Didgeridoo,
                note: 4i32,
                powered: true,
            }),
            1190u32 => Ok(NoteBlock {
                instrument: NoteBlockInstrument::Didgeridoo,
                note: 4i32,
                powered: false,
            }),
            1191u32 => Ok(NoteBlock {
                instrument: NoteBlockInstrument::Didgeridoo,
                note: 5i32,
                powered: true,
            }),
            1192u32 => Ok(NoteBlock {
                instrument: NoteBlockInstrument::Didgeridoo,
                note: 5i32,
                powered: false,
            }),
            1193u32 => Ok(NoteBlock {
                instrument: NoteBlockInstrument::Didgeridoo,
                note: 6i32,
                powered: true,
            }),
            1194u32 => Ok(NoteBlock {
                instrument: NoteBlockInstrument::Didgeridoo,
                note: 6i32,
                powered: false,
            }),
            1195u32 => Ok(NoteBlock {
                instrument: NoteBlockInstrument::Didgeridoo,
                note: 7i32,
                powered: true,
            }),
            1196u32 => Ok(NoteBlock {
                instrument: NoteBlockInstrument::Didgeridoo,
                note: 7i32,
                powered: false,
            }),
            1197u32 => Ok(NoteBlock {
                instrument: NoteBlockInstrument::Didgeridoo,
                note: 8i32,
                powered: true,
            }),
            1198u32 => Ok(NoteBlock {
                instrument: NoteBlockInstrument::Didgeridoo,
                note: 8i32,
                powered: false,
            }),
            1199u32 => Ok(NoteBlock {
                instrument: NoteBlockInstrument::Didgeridoo,
                note: 9i32,
                powered: true,
            }),
            1200u32 => Ok(NoteBlock {
                instrument: NoteBlockInstrument::Didgeridoo,
                note: 9i32,
                powered: false,
            }),
            1201u32 => Ok(NoteBlock {
                instrument: NoteBlockInstrument::Didgeridoo,
                note: 10i32,
                powered: true,
            }),
            1202u32 => Ok(NoteBlock {
                instrument: NoteBlockInstrument::Didgeridoo,
                note: 10i32,
                powered: false,
            }),
            1203u32 => Ok(NoteBlock {
                instrument: NoteBlockInstrument::Didgeridoo,
                note: 11i32,
                powered: true,
            }),
            1204u32 => Ok(NoteBlock {
                instrument: NoteBlockInstrument::Didgeridoo,
                note: 11i32,
                powered: false,
            }),
            1205u32 => Ok(NoteBlock {
                instrument: NoteBlockInstrument::Didgeridoo,
                note: 12i32,
                powered: true,
            }),
            1206u32 => Ok(NoteBlock {
                instrument: NoteBlockInstrument::Didgeridoo,
                note: 12i32,
                powered: false,
            }),
            1207u32 => Ok(NoteBlock {
                instrument: NoteBlockInstrument::Didgeridoo,
                note: 13i32,
                powered: true,
            }),
            1208u32 => Ok(NoteBlock {
                instrument: NoteBlockInstrument::Didgeridoo,
                note: 13i32,
                powered: false,
            }),
            1209u32 => Ok(NoteBlock {
                instrument: NoteBlockInstrument::Didgeridoo,
                note: 14i32,
                powered: true,
            }),
            1210u32 => Ok(NoteBlock {
                instrument: NoteBlockInstrument::Didgeridoo,
                note: 14i32,
                powered: false,
            }),
            1211u32 => Ok(NoteBlock {
                instrument: NoteBlockInstrument::Didgeridoo,
                note: 15i32,
                powered: true,
            }),
            1212u32 => Ok(NoteBlock {
                instrument: NoteBlockInstrument::Didgeridoo,
                note: 15i32,
                powered: false,
            }),
            1213u32 => Ok(NoteBlock {
                instrument: NoteBlockInstrument::Didgeridoo,
                note: 16i32,
                powered: true,
            }),
            1214u32 => Ok(NoteBlock {
                instrument: NoteBlockInstrument::Didgeridoo,
                note: 16i32,
                powered: false,
            }),
            1215u32 => Ok(NoteBlock {
                instrument: NoteBlockInstrument::Didgeridoo,
                note: 17i32,
                powered: true,
            }),
            1216u32 => Ok(NoteBlock {
                instrument: NoteBlockInstrument::Didgeridoo,
                note: 17i32,
                powered: false,
            }),
            1217u32 => Ok(NoteBlock {
                instrument: NoteBlockInstrument::Didgeridoo,
                note: 18i32,
                powered: true,
            }),
            1218u32 => Ok(NoteBlock {
                instrument: NoteBlockInstrument::Didgeridoo,
                note: 18i32,
                powered: false,
            }),
            1219u32 => Ok(NoteBlock {
                instrument: NoteBlockInstrument::Didgeridoo,
                note: 19i32,
                powered: true,
            }),
            1220u32 => Ok(NoteBlock {
                instrument: NoteBlockInstrument::Didgeridoo,
                note: 19i32,
                powered: false,
            }),
            1221u32 => Ok(NoteBlock {
                instrument: NoteBlockInstrument::Didgeridoo,
                note: 20i32,
                powered: true,
            }),
            1222u32 => Ok(NoteBlock {
                instrument: NoteBlockInstrument::Didgeridoo,
                note: 20i32,
                powered: false,
            }),
            1223u32 => Ok(NoteBlock {
                instrument: NoteBlockInstrument::Didgeridoo,
                note: 21i32,
                powered: true,
            }),
            1224u32 => Ok(NoteBlock {
                instrument: NoteBlockInstrument::Didgeridoo,
                note: 21i32,
                powered: false,
            }),
            1225u32 => Ok(NoteBlock {
                instrument: NoteBlockInstrument::Didgeridoo,
                note: 22i32,
                powered: true,
            }),
            1226u32 => Ok(NoteBlock {
                instrument: NoteBlockInstrument::Didgeridoo,
                note: 22i32,
                powered: false,
            }),
            1227u32 => Ok(NoteBlock {
                instrument: NoteBlockInstrument::Didgeridoo,
                note: 23i32,
                powered: true,
            }),
            1228u32 => Ok(NoteBlock {
                instrument: NoteBlockInstrument::Didgeridoo,
                note: 23i32,
                powered: false,
            }),
            1229u32 => Ok(NoteBlock {
                instrument: NoteBlockInstrument::Didgeridoo,
                note: 24i32,
                powered: true,
            }),
            1230u32 => Ok(NoteBlock {
                instrument: NoteBlockInstrument::Didgeridoo,
                note: 24i32,
                powered: false,
            }),
            1231u32 => Ok(NoteBlock {
                instrument: NoteBlockInstrument::Bit,
                note: 0i32,
                powered: true,
            }),
            1232u32 => Ok(NoteBlock {
                instrument: NoteBlockInstrument::Bit,
                note: 0i32,
                powered: false,
            }),
            1233u32 => Ok(NoteBlock {
                instrument: NoteBlockInstrument::Bit,
                note: 1i32,
                powered: true,
            }),
            1234u32 => Ok(NoteBlock {
                instrument: NoteBlockInstrument::Bit,
                note: 1i32,
                powered: false,
            }),
            1235u32 => Ok(NoteBlock {
                instrument: NoteBlockInstrument::Bit,
                note: 2i32,
                powered: true,
            }),
            1236u32 => Ok(NoteBlock {
                instrument: NoteBlockInstrument::Bit,
                note: 2i32,
                powered: false,
            }),
            1237u32 => Ok(NoteBlock {
                instrument: NoteBlockInstrument::Bit,
                note: 3i32,
                powered: true,
            }),
            1238u32 => Ok(NoteBlock {
                instrument: NoteBlockInstrument::Bit,
                note: 3i32,
                powered: false,
            }),
            1239u32 => Ok(NoteBlock {
                instrument: NoteBlockInstrument::Bit,
                note: 4i32,
                powered: true,
            }),
            1240u32 => Ok(NoteBlock {
                instrument: NoteBlockInstrument::Bit,
                note: 4i32,
                powered: false,
            }),
            1241u32 => Ok(NoteBlock {
                instrument: NoteBlockInstrument::Bit,
                note: 5i32,
                powered: true,
            }),
            1242u32 => Ok(NoteBlock {
                instrument: NoteBlockInstrument::Bit,
                note: 5i32,
                powered: false,
            }),
            1243u32 => Ok(NoteBlock {
                instrument: NoteBlockInstrument::Bit,
                note: 6i32,
                powered: true,
            }),
            1244u32 => Ok(NoteBlock {
                instrument: NoteBlockInstrument::Bit,
                note: 6i32,
                powered: false,
            }),
            1245u32 => Ok(NoteBlock {
                instrument: NoteBlockInstrument::Bit,
                note: 7i32,
                powered: true,
            }),
            1246u32 => Ok(NoteBlock {
                instrument: NoteBlockInstrument::Bit,
                note: 7i32,
                powered: false,
            }),
            1247u32 => Ok(NoteBlock {
                instrument: NoteBlockInstrument::Bit,
                note: 8i32,
                powered: true,
            }),
            1248u32 => Ok(NoteBlock {
                instrument: NoteBlockInstrument::Bit,
                note: 8i32,
                powered: false,
            }),
            1249u32 => Ok(NoteBlock {
                instrument: NoteBlockInstrument::Bit,
                note: 9i32,
                powered: true,
            }),
            1250u32 => Ok(NoteBlock {
                instrument: NoteBlockInstrument::Bit,
                note: 9i32,
                powered: false,
            }),
            1251u32 => Ok(NoteBlock {
                instrument: NoteBlockInstrument::Bit,
                note: 10i32,
                powered: true,
            }),
            1252u32 => Ok(NoteBlock {
                instrument: NoteBlockInstrument::Bit,
                note: 10i32,
                powered: false,
            }),
            1253u32 => Ok(NoteBlock {
                instrument: NoteBlockInstrument::Bit,
                note: 11i32,
                powered: true,
            }),
            1254u32 => Ok(NoteBlock {
                instrument: NoteBlockInstrument::Bit,
                note: 11i32,
                powered: false,
            }),
            1255u32 => Ok(NoteBlock {
                instrument: NoteBlockInstrument::Bit,
                note: 12i32,
                powered: true,
            }),
            1256u32 => Ok(NoteBlock {
                instrument: NoteBlockInstrument::Bit,
                note: 12i32,
                powered: false,
            }),
            1257u32 => Ok(NoteBlock {
                instrument: NoteBlockInstrument::Bit,
                note: 13i32,
                powered: true,
            }),
            1258u32 => Ok(NoteBlock {
                instrument: NoteBlockInstrument::Bit,
                note: 13i32,
                powered: false,
            }),
            1259u32 => Ok(NoteBlock {
                instrument: NoteBlockInstrument::Bit,
                note: 14i32,
                powered: true,
            }),
            1260u32 => Ok(NoteBlock {
                instrument: NoteBlockInstrument::Bit,
                note: 14i32,
                powered: false,
            }),
            1261u32 => Ok(NoteBlock {
                instrument: NoteBlockInstrument::Bit,
                note: 15i32,
                powered: true,
            }),
            1262u32 => Ok(NoteBlock {
                instrument: NoteBlockInstrument::Bit,
                note: 15i32,
                powered: false,
            }),
            1263u32 => Ok(NoteBlock {
                instrument: NoteBlockInstrument::Bit,
                note: 16i32,
                powered: true,
            }),
            1264u32 => Ok(NoteBlock {
                instrument: NoteBlockInstrument::Bit,
                note: 16i32,
                powered: false,
            }),
            1265u32 => Ok(NoteBlock {
                instrument: NoteBlockInstrument::Bit,
                note: 17i32,
                powered: true,
            }),
            1266u32 => Ok(NoteBlock {
                instrument: NoteBlockInstrument::Bit,
                note: 17i32,
                powered: false,
            }),
            1267u32 => Ok(NoteBlock {
                instrument: NoteBlockInstrument::Bit,
                note: 18i32,
                powered: true,
            }),
            1268u32 => Ok(NoteBlock {
                instrument: NoteBlockInstrument::Bit,
                note: 18i32,
                powered: false,
            }),
            1269u32 => Ok(NoteBlock {
                instrument: NoteBlockInstrument::Bit,
                note: 19i32,
                powered: true,
            }),
            1270u32 => Ok(NoteBlock {
                instrument: NoteBlockInstrument::Bit,
                note: 19i32,
                powered: false,
            }),
            1271u32 => Ok(NoteBlock {
                instrument: NoteBlockInstrument::Bit,
                note: 20i32,
                powered: true,
            }),
            1272u32 => Ok(NoteBlock {
                instrument: NoteBlockInstrument::Bit,
                note: 20i32,
                powered: false,
            }),
            1273u32 => Ok(NoteBlock {
                instrument: NoteBlockInstrument::Bit,
                note: 21i32,
                powered: true,
            }),
            1274u32 => Ok(NoteBlock {
                instrument: NoteBlockInstrument::Bit,
                note: 21i32,
                powered: false,
            }),
            1275u32 => Ok(NoteBlock {
                instrument: NoteBlockInstrument::Bit,
                note: 22i32,
                powered: true,
            }),
            1276u32 => Ok(NoteBlock {
                instrument: NoteBlockInstrument::Bit,
                note: 22i32,
                powered: false,
            }),
            1277u32 => Ok(NoteBlock {
                instrument: NoteBlockInstrument::Bit,
                note: 23i32,
                powered: true,
            }),
            1278u32 => Ok(NoteBlock {
                instrument: NoteBlockInstrument::Bit,
                note: 23i32,
                powered: false,
            }),
            1279u32 => Ok(NoteBlock {
                instrument: NoteBlockInstrument::Bit,
                note: 24i32,
                powered: true,
            }),
            1280u32 => Ok(NoteBlock {
                instrument: NoteBlockInstrument::Bit,
                note: 24i32,
                powered: false,
            }),
            1281u32 => Ok(NoteBlock {
                instrument: NoteBlockInstrument::Banjo,
                note: 0i32,
                powered: true,
            }),
            1282u32 => Ok(NoteBlock {
                instrument: NoteBlockInstrument::Banjo,
                note: 0i32,
                powered: false,
            }),
            1283u32 => Ok(NoteBlock {
                instrument: NoteBlockInstrument::Banjo,
                note: 1i32,
                powered: true,
            }),
            1284u32 => Ok(NoteBlock {
                instrument: NoteBlockInstrument::Banjo,
                note: 1i32,
                powered: false,
            }),
            1285u32 => Ok(NoteBlock {
                instrument: NoteBlockInstrument::Banjo,
                note: 2i32,
                powered: true,
            }),
            1286u32 => Ok(NoteBlock {
                instrument: NoteBlockInstrument::Banjo,
                note: 2i32,
                powered: false,
            }),
            1287u32 => Ok(NoteBlock {
                instrument: NoteBlockInstrument::Banjo,
                note: 3i32,
                powered: true,
            }),
            1288u32 => Ok(NoteBlock {
                instrument: NoteBlockInstrument::Banjo,
                note: 3i32,
                powered: false,
            }),
            1289u32 => Ok(NoteBlock {
                instrument: NoteBlockInstrument::Banjo,
                note: 4i32,
                powered: true,
            }),
            1290u32 => Ok(NoteBlock {
                instrument: NoteBlockInstrument::Banjo,
                note: 4i32,
                powered: false,
            }),
            1291u32 => Ok(NoteBlock {
                instrument: NoteBlockInstrument::Banjo,
                note: 5i32,
                powered: true,
            }),
            1292u32 => Ok(NoteBlock {
                instrument: NoteBlockInstrument::Banjo,
                note: 5i32,
                powered: false,
            }),
            1293u32 => Ok(NoteBlock {
                instrument: NoteBlockInstrument::Banjo,
                note: 6i32,
                powered: true,
            }),
            1294u32 => Ok(NoteBlock {
                instrument: NoteBlockInstrument::Banjo,
                note: 6i32,
                powered: false,
            }),
            1295u32 => Ok(NoteBlock {
                instrument: NoteBlockInstrument::Banjo,
                note: 7i32,
                powered: true,
            }),
            1296u32 => Ok(NoteBlock {
                instrument: NoteBlockInstrument::Banjo,
                note: 7i32,
                powered: false,
            }),
            1297u32 => Ok(NoteBlock {
                instrument: NoteBlockInstrument::Banjo,
                note: 8i32,
                powered: true,
            }),
            1298u32 => Ok(NoteBlock {
                instrument: NoteBlockInstrument::Banjo,
                note: 8i32,
                powered: false,
            }),
            1299u32 => Ok(NoteBlock {
                instrument: NoteBlockInstrument::Banjo,
                note: 9i32,
                powered: true,
            }),
            1300u32 => Ok(NoteBlock {
                instrument: NoteBlockInstrument::Banjo,
                note: 9i32,
                powered: false,
            }),
            1301u32 => Ok(NoteBlock {
                instrument: NoteBlockInstrument::Banjo,
                note: 10i32,
                powered: true,
            }),
            1302u32 => Ok(NoteBlock {
                instrument: NoteBlockInstrument::Banjo,
                note: 10i32,
                powered: false,
            }),
            1303u32 => Ok(NoteBlock {
                instrument: NoteBlockInstrument::Banjo,
                note: 11i32,
                powered: true,
            }),
            1304u32 => Ok(NoteBlock {
                instrument: NoteBlockInstrument::Banjo,
                note: 11i32,
                powered: false,
            }),
            1305u32 => Ok(NoteBlock {
                instrument: NoteBlockInstrument::Banjo,
                note: 12i32,
                powered: true,
            }),
            1306u32 => Ok(NoteBlock {
                instrument: NoteBlockInstrument::Banjo,
                note: 12i32,
                powered: false,
            }),
            1307u32 => Ok(NoteBlock {
                instrument: NoteBlockInstrument::Banjo,
                note: 13i32,
                powered: true,
            }),
            1308u32 => Ok(NoteBlock {
                instrument: NoteBlockInstrument::Banjo,
                note: 13i32,
                powered: false,
            }),
            1309u32 => Ok(NoteBlock {
                instrument: NoteBlockInstrument::Banjo,
                note: 14i32,
                powered: true,
            }),
            1310u32 => Ok(NoteBlock {
                instrument: NoteBlockInstrument::Banjo,
                note: 14i32,
                powered: false,
            }),
            1311u32 => Ok(NoteBlock {
                instrument: NoteBlockInstrument::Banjo,
                note: 15i32,
                powered: true,
            }),
            1312u32 => Ok(NoteBlock {
                instrument: NoteBlockInstrument::Banjo,
                note: 15i32,
                powered: false,
            }),
            1313u32 => Ok(NoteBlock {
                instrument: NoteBlockInstrument::Banjo,
                note: 16i32,
                powered: true,
            }),
            1314u32 => Ok(NoteBlock {
                instrument: NoteBlockInstrument::Banjo,
                note: 16i32,
                powered: false,
            }),
            1315u32 => Ok(NoteBlock {
                instrument: NoteBlockInstrument::Banjo,
                note: 17i32,
                powered: true,
            }),
            1316u32 => Ok(NoteBlock {
                instrument: NoteBlockInstrument::Banjo,
                note: 17i32,
                powered: false,
            }),
            1317u32 => Ok(NoteBlock {
                instrument: NoteBlockInstrument::Banjo,
                note: 18i32,
                powered: true,
            }),
            1318u32 => Ok(NoteBlock {
                instrument: NoteBlockInstrument::Banjo,
                note: 18i32,
                powered: false,
            }),
            1319u32 => Ok(NoteBlock {
                instrument: NoteBlockInstrument::Banjo,
                note: 19i32,
                powered: true,
            }),
            1320u32 => Ok(NoteBlock {
                instrument: NoteBlockInstrument::Banjo,
                note: 19i32,
                powered: false,
            }),
            1321u32 => Ok(NoteBlock {
                instrument: NoteBlockInstrument::Banjo,
                note: 20i32,
                powered: true,
            }),
            1322u32 => Ok(NoteBlock {
                instrument: NoteBlockInstrument::Banjo,
                note: 20i32,
                powered: false,
            }),
            1323u32 => Ok(NoteBlock {
                instrument: NoteBlockInstrument::Banjo,
                note: 21i32,
                powered: true,
            }),
            1324u32 => Ok(NoteBlock {
                instrument: NoteBlockInstrument::Banjo,
                note: 21i32,
                powered: false,
            }),
            1325u32 => Ok(NoteBlock {
                instrument: NoteBlockInstrument::Banjo,
                note: 22i32,
                powered: true,
            }),
            1326u32 => Ok(NoteBlock {
                instrument: NoteBlockInstrument::Banjo,
                note: 22i32,
                powered: false,
            }),
            1327u32 => Ok(NoteBlock {
                instrument: NoteBlockInstrument::Banjo,
                note: 23i32,
                powered: true,
            }),
            1328u32 => Ok(NoteBlock {
                instrument: NoteBlockInstrument::Banjo,
                note: 23i32,
                powered: false,
            }),
            1329u32 => Ok(NoteBlock {
                instrument: NoteBlockInstrument::Banjo,
                note: 24i32,
                powered: true,
            }),
            1330u32 => Ok(NoteBlock {
                instrument: NoteBlockInstrument::Banjo,
                note: 24i32,
                powered: false,
            }),
            1331u32 => Ok(NoteBlock {
                instrument: NoteBlockInstrument::Pling,
                note: 0i32,
                powered: true,
            }),
            1332u32 => Ok(NoteBlock {
                instrument: NoteBlockInstrument::Pling,
                note: 0i32,
                powered: false,
            }),
            1333u32 => Ok(NoteBlock {
                instrument: NoteBlockInstrument::Pling,
                note: 1i32,
                powered: true,
            }),
            1334u32 => Ok(NoteBlock {
                instrument: NoteBlockInstrument::Pling,
                note: 1i32,
                powered: false,
            }),
            1335u32 => Ok(NoteBlock {
                instrument: NoteBlockInstrument::Pling,
                note: 2i32,
                powered: true,
            }),
            1336u32 => Ok(NoteBlock {
                instrument: NoteBlockInstrument::Pling,
                note: 2i32,
                powered: false,
            }),
            1337u32 => Ok(NoteBlock {
                instrument: NoteBlockInstrument::Pling,
                note: 3i32,
                powered: true,
            }),
            1338u32 => Ok(NoteBlock {
                instrument: NoteBlockInstrument::Pling,
                note: 3i32,
                powered: false,
            }),
            1339u32 => Ok(NoteBlock {
                instrument: NoteBlockInstrument::Pling,
                note: 4i32,
                powered: true,
            }),
            1340u32 => Ok(NoteBlock {
                instrument: NoteBlockInstrument::Pling,
                note: 4i32,
                powered: false,
            }),
            1341u32 => Ok(NoteBlock {
                instrument: NoteBlockInstrument::Pling,
                note: 5i32,
                powered: true,
            }),
            1342u32 => Ok(NoteBlock {
                instrument: NoteBlockInstrument::Pling,
                note: 5i32,
                powered: false,
            }),
            1343u32 => Ok(NoteBlock {
                instrument: NoteBlockInstrument::Pling,
                note: 6i32,
                powered: true,
            }),
            1344u32 => Ok(NoteBlock {
                instrument: NoteBlockInstrument::Pling,
                note: 6i32,
                powered: false,
            }),
            1345u32 => Ok(NoteBlock {
                instrument: NoteBlockInstrument::Pling,
                note: 7i32,
                powered: true,
            }),
            1346u32 => Ok(NoteBlock {
                instrument: NoteBlockInstrument::Pling,
                note: 7i32,
                powered: false,
            }),
            1347u32 => Ok(NoteBlock {
                instrument: NoteBlockInstrument::Pling,
                note: 8i32,
                powered: true,
            }),
            1348u32 => Ok(NoteBlock {
                instrument: NoteBlockInstrument::Pling,
                note: 8i32,
                powered: false,
            }),
            1349u32 => Ok(NoteBlock {
                instrument: NoteBlockInstrument::Pling,
                note: 9i32,
                powered: true,
            }),
            1350u32 => Ok(NoteBlock {
                instrument: NoteBlockInstrument::Pling,
                note: 9i32,
                powered: false,
            }),
            1351u32 => Ok(NoteBlock {
                instrument: NoteBlockInstrument::Pling,
                note: 10i32,
                powered: true,
            }),
            1352u32 => Ok(NoteBlock {
                instrument: NoteBlockInstrument::Pling,
                note: 10i32,
                powered: false,
            }),
            1353u32 => Ok(NoteBlock {
                instrument: NoteBlockInstrument::Pling,
                note: 11i32,
                powered: true,
            }),
            1354u32 => Ok(NoteBlock {
                instrument: NoteBlockInstrument::Pling,
                note: 11i32,
                powered: false,
            }),
            1355u32 => Ok(NoteBlock {
                instrument: NoteBlockInstrument::Pling,
                note: 12i32,
                powered: true,
            }),
            1356u32 => Ok(NoteBlock {
                instrument: NoteBlockInstrument::Pling,
                note: 12i32,
                powered: false,
            }),
            1357u32 => Ok(NoteBlock {
                instrument: NoteBlockInstrument::Pling,
                note: 13i32,
                powered: true,
            }),
            1358u32 => Ok(NoteBlock {
                instrument: NoteBlockInstrument::Pling,
                note: 13i32,
                powered: false,
            }),
            1359u32 => Ok(NoteBlock {
                instrument: NoteBlockInstrument::Pling,
                note: 14i32,
                powered: true,
            }),
            1360u32 => Ok(NoteBlock {
                instrument: NoteBlockInstrument::Pling,
                note: 14i32,
                powered: false,
            }),
            1361u32 => Ok(NoteBlock {
                instrument: NoteBlockInstrument::Pling,
                note: 15i32,
                powered: true,
            }),
            1362u32 => Ok(NoteBlock {
                instrument: NoteBlockInstrument::Pling,
                note: 15i32,
                powered: false,
            }),
            1363u32 => Ok(NoteBlock {
                instrument: NoteBlockInstrument::Pling,
                note: 16i32,
                powered: true,
            }),
            1364u32 => Ok(NoteBlock {
                instrument: NoteBlockInstrument::Pling,
                note: 16i32,
                powered: false,
            }),
            1365u32 => Ok(NoteBlock {
                instrument: NoteBlockInstrument::Pling,
                note: 17i32,
                powered: true,
            }),
            1366u32 => Ok(NoteBlock {
                instrument: NoteBlockInstrument::Pling,
                note: 17i32,
                powered: false,
            }),
            1367u32 => Ok(NoteBlock {
                instrument: NoteBlockInstrument::Pling,
                note: 18i32,
                powered: true,
            }),
            1368u32 => Ok(NoteBlock {
                instrument: NoteBlockInstrument::Pling,
                note: 18i32,
                powered: false,
            }),
            1369u32 => Ok(NoteBlock {
                instrument: NoteBlockInstrument::Pling,
                note: 19i32,
                powered: true,
            }),
            1370u32 => Ok(NoteBlock {
                instrument: NoteBlockInstrument::Pling,
                note: 19i32,
                powered: false,
            }),
            1371u32 => Ok(NoteBlock {
                instrument: NoteBlockInstrument::Pling,
                note: 20i32,
                powered: true,
            }),
            1372u32 => Ok(NoteBlock {
                instrument: NoteBlockInstrument::Pling,
                note: 20i32,
                powered: false,
            }),
            1373u32 => Ok(NoteBlock {
                instrument: NoteBlockInstrument::Pling,
                note: 21i32,
                powered: true,
            }),
            1374u32 => Ok(NoteBlock {
                instrument: NoteBlockInstrument::Pling,
                note: 21i32,
                powered: false,
            }),
            1375u32 => Ok(NoteBlock {
                instrument: NoteBlockInstrument::Pling,
                note: 22i32,
                powered: true,
            }),
            1376u32 => Ok(NoteBlock {
                instrument: NoteBlockInstrument::Pling,
                note: 22i32,
                powered: false,
            }),
            1377u32 => Ok(NoteBlock {
                instrument: NoteBlockInstrument::Pling,
                note: 23i32,
                powered: true,
            }),
            1378u32 => Ok(NoteBlock {
                instrument: NoteBlockInstrument::Pling,
                note: 23i32,
                powered: false,
            }),
            1379u32 => Ok(NoteBlock {
                instrument: NoteBlockInstrument::Pling,
                note: 24i32,
                powered: true,
            }),
            1380u32 => Ok(NoteBlock {
                instrument: NoteBlockInstrument::Pling,
                note: 24i32,
                powered: false,
            }),
            1381u32 => Ok(NoteBlock {
                instrument: NoteBlockInstrument::Zombie,
                note: 0i32,
                powered: true,
            }),
            1382u32 => Ok(NoteBlock {
                instrument: NoteBlockInstrument::Zombie,
                note: 0i32,
                powered: false,
            }),
            1383u32 => Ok(NoteBlock {
                instrument: NoteBlockInstrument::Zombie,
                note: 1i32,
                powered: true,
            }),
            1384u32 => Ok(NoteBlock {
                instrument: NoteBlockInstrument::Zombie,
                note: 1i32,
                powered: false,
            }),
            1385u32 => Ok(NoteBlock {
                instrument: NoteBlockInstrument::Zombie,
                note: 2i32,
                powered: true,
            }),
            1386u32 => Ok(NoteBlock {
                instrument: NoteBlockInstrument::Zombie,
                note: 2i32,
                powered: false,
            }),
            1387u32 => Ok(NoteBlock {
                instrument: NoteBlockInstrument::Zombie,
                note: 3i32,
                powered: true,
            }),
            1388u32 => Ok(NoteBlock {
                instrument: NoteBlockInstrument::Zombie,
                note: 3i32,
                powered: false,
            }),
            1389u32 => Ok(NoteBlock {
                instrument: NoteBlockInstrument::Zombie,
                note: 4i32,
                powered: true,
            }),
            1390u32 => Ok(NoteBlock {
                instrument: NoteBlockInstrument::Zombie,
                note: 4i32,
                powered: false,
            }),
            1391u32 => Ok(NoteBlock {
                instrument: NoteBlockInstrument::Zombie,
                note: 5i32,
                powered: true,
            }),
            1392u32 => Ok(NoteBlock {
                instrument: NoteBlockInstrument::Zombie,
                note: 5i32,
                powered: false,
            }),
            1393u32 => Ok(NoteBlock {
                instrument: NoteBlockInstrument::Zombie,
                note: 6i32,
                powered: true,
            }),
            1394u32 => Ok(NoteBlock {
                instrument: NoteBlockInstrument::Zombie,
                note: 6i32,
                powered: false,
            }),
            1395u32 => Ok(NoteBlock {
                instrument: NoteBlockInstrument::Zombie,
                note: 7i32,
                powered: true,
            }),
            1396u32 => Ok(NoteBlock {
                instrument: NoteBlockInstrument::Zombie,
                note: 7i32,
                powered: false,
            }),
            1397u32 => Ok(NoteBlock {
                instrument: NoteBlockInstrument::Zombie,
                note: 8i32,
                powered: true,
            }),
            1398u32 => Ok(NoteBlock {
                instrument: NoteBlockInstrument::Zombie,
                note: 8i32,
                powered: false,
            }),
            1399u32 => Ok(NoteBlock {
                instrument: NoteBlockInstrument::Zombie,
                note: 9i32,
                powered: true,
            }),
            1400u32 => Ok(NoteBlock {
                instrument: NoteBlockInstrument::Zombie,
                note: 9i32,
                powered: false,
            }),
            1401u32 => Ok(NoteBlock {
                instrument: NoteBlockInstrument::Zombie,
                note: 10i32,
                powered: true,
            }),
            1402u32 => Ok(NoteBlock {
                instrument: NoteBlockInstrument::Zombie,
                note: 10i32,
                powered: false,
            }),
            1403u32 => Ok(NoteBlock {
                instrument: NoteBlockInstrument::Zombie,
                note: 11i32,
                powered: true,
            }),
            1404u32 => Ok(NoteBlock {
                instrument: NoteBlockInstrument::Zombie,
                note: 11i32,
                powered: false,
            }),
            1405u32 => Ok(NoteBlock {
                instrument: NoteBlockInstrument::Zombie,
                note: 12i32,
                powered: true,
            }),
            1406u32 => Ok(NoteBlock {
                instrument: NoteBlockInstrument::Zombie,
                note: 12i32,
                powered: false,
            }),
            1407u32 => Ok(NoteBlock {
                instrument: NoteBlockInstrument::Zombie,
                note: 13i32,
                powered: true,
            }),
            1408u32 => Ok(NoteBlock {
                instrument: NoteBlockInstrument::Zombie,
                note: 13i32,
                powered: false,
            }),
            1409u32 => Ok(NoteBlock {
                instrument: NoteBlockInstrument::Zombie,
                note: 14i32,
                powered: true,
            }),
            1410u32 => Ok(NoteBlock {
                instrument: NoteBlockInstrument::Zombie,
                note: 14i32,
                powered: false,
            }),
            1411u32 => Ok(NoteBlock {
                instrument: NoteBlockInstrument::Zombie,
                note: 15i32,
                powered: true,
            }),
            1412u32 => Ok(NoteBlock {
                instrument: NoteBlockInstrument::Zombie,
                note: 15i32,
                powered: false,
            }),
            1413u32 => Ok(NoteBlock {
                instrument: NoteBlockInstrument::Zombie,
                note: 16i32,
                powered: true,
            }),
            1414u32 => Ok(NoteBlock {
                instrument: NoteBlockInstrument::Zombie,
                note: 16i32,
                powered: false,
            }),
            1415u32 => Ok(NoteBlock {
                instrument: NoteBlockInstrument::Zombie,
                note: 17i32,
                powered: true,
            }),
            1416u32 => Ok(NoteBlock {
                instrument: NoteBlockInstrument::Zombie,
                note: 17i32,
                powered: false,
            }),
            1417u32 => Ok(NoteBlock {
                instrument: NoteBlockInstrument::Zombie,
                note: 18i32,
                powered: true,
            }),
            1418u32 => Ok(NoteBlock {
                instrument: NoteBlockInstrument::Zombie,
                note: 18i32,
                powered: false,
            }),
            1419u32 => Ok(NoteBlock {
                instrument: NoteBlockInstrument::Zombie,
                note: 19i32,
                powered: true,
            }),
            1420u32 => Ok(NoteBlock {
                instrument: NoteBlockInstrument::Zombie,
                note: 19i32,
                powered: false,
            }),
            1421u32 => Ok(NoteBlock {
                instrument: NoteBlockInstrument::Zombie,
                note: 20i32,
                powered: true,
            }),
            1422u32 => Ok(NoteBlock {
                instrument: NoteBlockInstrument::Zombie,
                note: 20i32,
                powered: false,
            }),
            1423u32 => Ok(NoteBlock {
                instrument: NoteBlockInstrument::Zombie,
                note: 21i32,
                powered: true,
            }),
            1424u32 => Ok(NoteBlock {
                instrument: NoteBlockInstrument::Zombie,
                note: 21i32,
                powered: false,
            }),
            1425u32 => Ok(NoteBlock {
                instrument: NoteBlockInstrument::Zombie,
                note: 22i32,
                powered: true,
            }),
            1426u32 => Ok(NoteBlock {
                instrument: NoteBlockInstrument::Zombie,
                note: 22i32,
                powered: false,
            }),
            1427u32 => Ok(NoteBlock {
                instrument: NoteBlockInstrument::Zombie,
                note: 23i32,
                powered: true,
            }),
            1428u32 => Ok(NoteBlock {
                instrument: NoteBlockInstrument::Zombie,
                note: 23i32,
                powered: false,
            }),
            1429u32 => Ok(NoteBlock {
                instrument: NoteBlockInstrument::Zombie,
                note: 24i32,
                powered: true,
            }),
            1430u32 => Ok(NoteBlock {
                instrument: NoteBlockInstrument::Zombie,
                note: 24i32,
                powered: false,
            }),
            1431u32 => Ok(NoteBlock {
                instrument: NoteBlockInstrument::Skeleton,
                note: 0i32,
                powered: true,
            }),
            1432u32 => Ok(NoteBlock {
                instrument: NoteBlockInstrument::Skeleton,
                note: 0i32,
                powered: false,
            }),
            1433u32 => Ok(NoteBlock {
                instrument: NoteBlockInstrument::Skeleton,
                note: 1i32,
                powered: true,
            }),
            1434u32 => Ok(NoteBlock {
                instrument: NoteBlockInstrument::Skeleton,
                note: 1i32,
                powered: false,
            }),
            1435u32 => Ok(NoteBlock {
                instrument: NoteBlockInstrument::Skeleton,
                note: 2i32,
                powered: true,
            }),
            1436u32 => Ok(NoteBlock {
                instrument: NoteBlockInstrument::Skeleton,
                note: 2i32,
                powered: false,
            }),
            1437u32 => Ok(NoteBlock {
                instrument: NoteBlockInstrument::Skeleton,
                note: 3i32,
                powered: true,
            }),
            1438u32 => Ok(NoteBlock {
                instrument: NoteBlockInstrument::Skeleton,
                note: 3i32,
                powered: false,
            }),
            1439u32 => Ok(NoteBlock {
                instrument: NoteBlockInstrument::Skeleton,
                note: 4i32,
                powered: true,
            }),
            1440u32 => Ok(NoteBlock {
                instrument: NoteBlockInstrument::Skeleton,
                note: 4i32,
                powered: false,
            }),
            1441u32 => Ok(NoteBlock {
                instrument: NoteBlockInstrument::Skeleton,
                note: 5i32,
                powered: true,
            }),
            1442u32 => Ok(NoteBlock {
                instrument: NoteBlockInstrument::Skeleton,
                note: 5i32,
                powered: false,
            }),
            1443u32 => Ok(NoteBlock {
                instrument: NoteBlockInstrument::Skeleton,
                note: 6i32,
                powered: true,
            }),
            1444u32 => Ok(NoteBlock {
                instrument: NoteBlockInstrument::Skeleton,
                note: 6i32,
                powered: false,
            }),
            1445u32 => Ok(NoteBlock {
                instrument: NoteBlockInstrument::Skeleton,
                note: 7i32,
                powered: true,
            }),
            1446u32 => Ok(NoteBlock {
                instrument: NoteBlockInstrument::Skeleton,
                note: 7i32,
                powered: false,
            }),
            1447u32 => Ok(NoteBlock {
                instrument: NoteBlockInstrument::Skeleton,
                note: 8i32,
                powered: true,
            }),
            1448u32 => Ok(NoteBlock {
                instrument: NoteBlockInstrument::Skeleton,
                note: 8i32,
                powered: false,
            }),
            1449u32 => Ok(NoteBlock {
                instrument: NoteBlockInstrument::Skeleton,
                note: 9i32,
                powered: true,
            }),
            1450u32 => Ok(NoteBlock {
                instrument: NoteBlockInstrument::Skeleton,
                note: 9i32,
                powered: false,
            }),
            1451u32 => Ok(NoteBlock {
                instrument: NoteBlockInstrument::Skeleton,
                note: 10i32,
                powered: true,
            }),
            1452u32 => Ok(NoteBlock {
                instrument: NoteBlockInstrument::Skeleton,
                note: 10i32,
                powered: false,
            }),
            1453u32 => Ok(NoteBlock {
                instrument: NoteBlockInstrument::Skeleton,
                note: 11i32,
                powered: true,
            }),
            1454u32 => Ok(NoteBlock {
                instrument: NoteBlockInstrument::Skeleton,
                note: 11i32,
                powered: false,
            }),
            1455u32 => Ok(NoteBlock {
                instrument: NoteBlockInstrument::Skeleton,
                note: 12i32,
                powered: true,
            }),
            1456u32 => Ok(NoteBlock {
                instrument: NoteBlockInstrument::Skeleton,
                note: 12i32,
                powered: false,
            }),
            1457u32 => Ok(NoteBlock {
                instrument: NoteBlockInstrument::Skeleton,
                note: 13i32,
                powered: true,
            }),
            1458u32 => Ok(NoteBlock {
                instrument: NoteBlockInstrument::Skeleton,
                note: 13i32,
                powered: false,
            }),
            1459u32 => Ok(NoteBlock {
                instrument: NoteBlockInstrument::Skeleton,
                note: 14i32,
                powered: true,
            }),
            1460u32 => Ok(NoteBlock {
                instrument: NoteBlockInstrument::Skeleton,
                note: 14i32,
                powered: false,
            }),
            1461u32 => Ok(NoteBlock {
                instrument: NoteBlockInstrument::Skeleton,
                note: 15i32,
                powered: true,
            }),
            1462u32 => Ok(NoteBlock {
                instrument: NoteBlockInstrument::Skeleton,
                note: 15i32,
                powered: false,
            }),
            1463u32 => Ok(NoteBlock {
                instrument: NoteBlockInstrument::Skeleton,
                note: 16i32,
                powered: true,
            }),
            1464u32 => Ok(NoteBlock {
                instrument: NoteBlockInstrument::Skeleton,
                note: 16i32,
                powered: false,
            }),
            1465u32 => Ok(NoteBlock {
                instrument: NoteBlockInstrument::Skeleton,
                note: 17i32,
                powered: true,
            }),
            1466u32 => Ok(NoteBlock {
                instrument: NoteBlockInstrument::Skeleton,
                note: 17i32,
                powered: false,
            }),
            1467u32 => Ok(NoteBlock {
                instrument: NoteBlockInstrument::Skeleton,
                note: 18i32,
                powered: true,
            }),
            1468u32 => Ok(NoteBlock {
                instrument: NoteBlockInstrument::Skeleton,
                note: 18i32,
                powered: false,
            }),
            1469u32 => Ok(NoteBlock {
                instrument: NoteBlockInstrument::Skeleton,
                note: 19i32,
                powered: true,
            }),
            1470u32 => Ok(NoteBlock {
                instrument: NoteBlockInstrument::Skeleton,
                note: 19i32,
                powered: false,
            }),
            1471u32 => Ok(NoteBlock {
                instrument: NoteBlockInstrument::Skeleton,
                note: 20i32,
                powered: true,
            }),
            1472u32 => Ok(NoteBlock {
                instrument: NoteBlockInstrument::Skeleton,
                note: 20i32,
                powered: false,
            }),
            1473u32 => Ok(NoteBlock {
                instrument: NoteBlockInstrument::Skeleton,
                note: 21i32,
                powered: true,
            }),
            1474u32 => Ok(NoteBlock {
                instrument: NoteBlockInstrument::Skeleton,
                note: 21i32,
                powered: false,
            }),
            1475u32 => Ok(NoteBlock {
                instrument: NoteBlockInstrument::Skeleton,
                note: 22i32,
                powered: true,
            }),
            1476u32 => Ok(NoteBlock {
                instrument: NoteBlockInstrument::Skeleton,
                note: 22i32,
                powered: false,
            }),
            1477u32 => Ok(NoteBlock {
                instrument: NoteBlockInstrument::Skeleton,
                note: 23i32,
                powered: true,
            }),
            1478u32 => Ok(NoteBlock {
                instrument: NoteBlockInstrument::Skeleton,
                note: 23i32,
                powered: false,
            }),
            1479u32 => Ok(NoteBlock {
                instrument: NoteBlockInstrument::Skeleton,
                note: 24i32,
                powered: true,
            }),
            1480u32 => Ok(NoteBlock {
                instrument: NoteBlockInstrument::Skeleton,
                note: 24i32,
                powered: false,
            }),
            1481u32 => Ok(NoteBlock {
                instrument: NoteBlockInstrument::Creeper,
                note: 0i32,
                powered: true,
            }),
            1482u32 => Ok(NoteBlock {
                instrument: NoteBlockInstrument::Creeper,
                note: 0i32,
                powered: false,
            }),
            1483u32 => Ok(NoteBlock {
                instrument: NoteBlockInstrument::Creeper,
                note: 1i32,
                powered: true,
            }),
            1484u32 => Ok(NoteBlock {
                instrument: NoteBlockInstrument::Creeper,
                note: 1i32,
                powered: false,
            }),
            1485u32 => Ok(NoteBlock {
                instrument: NoteBlockInstrument::Creeper,
                note: 2i32,
                powered: true,
            }),
            1486u32 => Ok(NoteBlock {
                instrument: NoteBlockInstrument::Creeper,
                note: 2i32,
                powered: false,
            }),
            1487u32 => Ok(NoteBlock {
                instrument: NoteBlockInstrument::Creeper,
                note: 3i32,
                powered: true,
            }),
            1488u32 => Ok(NoteBlock {
                instrument: NoteBlockInstrument::Creeper,
                note: 3i32,
                powered: false,
            }),
            1489u32 => Ok(NoteBlock {
                instrument: NoteBlockInstrument::Creeper,
                note: 4i32,
                powered: true,
            }),
            1490u32 => Ok(NoteBlock {
                instrument: NoteBlockInstrument::Creeper,
                note: 4i32,
                powered: false,
            }),
            1491u32 => Ok(NoteBlock {
                instrument: NoteBlockInstrument::Creeper,
                note: 5i32,
                powered: true,
            }),
            1492u32 => Ok(NoteBlock {
                instrument: NoteBlockInstrument::Creeper,
                note: 5i32,
                powered: false,
            }),
            1493u32 => Ok(NoteBlock {
                instrument: NoteBlockInstrument::Creeper,
                note: 6i32,
                powered: true,
            }),
            1494u32 => Ok(NoteBlock {
                instrument: NoteBlockInstrument::Creeper,
                note: 6i32,
                powered: false,
            }),
            1495u32 => Ok(NoteBlock {
                instrument: NoteBlockInstrument::Creeper,
                note: 7i32,
                powered: true,
            }),
            1496u32 => Ok(NoteBlock {
                instrument: NoteBlockInstrument::Creeper,
                note: 7i32,
                powered: false,
            }),
            1497u32 => Ok(NoteBlock {
                instrument: NoteBlockInstrument::Creeper,
                note: 8i32,
                powered: true,
            }),
            1498u32 => Ok(NoteBlock {
                instrument: NoteBlockInstrument::Creeper,
                note: 8i32,
                powered: false,
            }),
            1499u32 => Ok(NoteBlock {
                instrument: NoteBlockInstrument::Creeper,
                note: 9i32,
                powered: true,
            }),
            1500u32 => Ok(NoteBlock {
                instrument: NoteBlockInstrument::Creeper,
                note: 9i32,
                powered: false,
            }),
            1501u32 => Ok(NoteBlock {
                instrument: NoteBlockInstrument::Creeper,
                note: 10i32,
                powered: true,
            }),
            1502u32 => Ok(NoteBlock {
                instrument: NoteBlockInstrument::Creeper,
                note: 10i32,
                powered: false,
            }),
            1503u32 => Ok(NoteBlock {
                instrument: NoteBlockInstrument::Creeper,
                note: 11i32,
                powered: true,
            }),
            1504u32 => Ok(NoteBlock {
                instrument: NoteBlockInstrument::Creeper,
                note: 11i32,
                powered: false,
            }),
            1505u32 => Ok(NoteBlock {
                instrument: NoteBlockInstrument::Creeper,
                note: 12i32,
                powered: true,
            }),
            1506u32 => Ok(NoteBlock {
                instrument: NoteBlockInstrument::Creeper,
                note: 12i32,
                powered: false,
            }),
            1507u32 => Ok(NoteBlock {
                instrument: NoteBlockInstrument::Creeper,
                note: 13i32,
                powered: true,
            }),
            1508u32 => Ok(NoteBlock {
                instrument: NoteBlockInstrument::Creeper,
                note: 13i32,
                powered: false,
            }),
            1509u32 => Ok(NoteBlock {
                instrument: NoteBlockInstrument::Creeper,
                note: 14i32,
                powered: true,
            }),
            1510u32 => Ok(NoteBlock {
                instrument: NoteBlockInstrument::Creeper,
                note: 14i32,
                powered: false,
            }),
            1511u32 => Ok(NoteBlock {
                instrument: NoteBlockInstrument::Creeper,
                note: 15i32,
                powered: true,
            }),
            1512u32 => Ok(NoteBlock {
                instrument: NoteBlockInstrument::Creeper,
                note: 15i32,
                powered: false,
            }),
            1513u32 => Ok(NoteBlock {
                instrument: NoteBlockInstrument::Creeper,
                note: 16i32,
                powered: true,
            }),
            1514u32 => Ok(NoteBlock {
                instrument: NoteBlockInstrument::Creeper,
                note: 16i32,
                powered: false,
            }),
            1515u32 => Ok(NoteBlock {
                instrument: NoteBlockInstrument::Creeper,
                note: 17i32,
                powered: true,
            }),
            1516u32 => Ok(NoteBlock {
                instrument: NoteBlockInstrument::Creeper,
                note: 17i32,
                powered: false,
            }),
            1517u32 => Ok(NoteBlock {
                instrument: NoteBlockInstrument::Creeper,
                note: 18i32,
                powered: true,
            }),
            1518u32 => Ok(NoteBlock {
                instrument: NoteBlockInstrument::Creeper,
                note: 18i32,
                powered: false,
            }),
            1519u32 => Ok(NoteBlock {
                instrument: NoteBlockInstrument::Creeper,
                note: 19i32,
                powered: true,
            }),
            1520u32 => Ok(NoteBlock {
                instrument: NoteBlockInstrument::Creeper,
                note: 19i32,
                powered: false,
            }),
            1521u32 => Ok(NoteBlock {
                instrument: NoteBlockInstrument::Creeper,
                note: 20i32,
                powered: true,
            }),
            1522u32 => Ok(NoteBlock {
                instrument: NoteBlockInstrument::Creeper,
                note: 20i32,
                powered: false,
            }),
            1523u32 => Ok(NoteBlock {
                instrument: NoteBlockInstrument::Creeper,
                note: 21i32,
                powered: true,
            }),
            1524u32 => Ok(NoteBlock {
                instrument: NoteBlockInstrument::Creeper,
                note: 21i32,
                powered: false,
            }),
            1525u32 => Ok(NoteBlock {
                instrument: NoteBlockInstrument::Creeper,
                note: 22i32,
                powered: true,
            }),
            1526u32 => Ok(NoteBlock {
                instrument: NoteBlockInstrument::Creeper,
                note: 22i32,
                powered: false,
            }),
            1527u32 => Ok(NoteBlock {
                instrument: NoteBlockInstrument::Creeper,
                note: 23i32,
                powered: true,
            }),
            1528u32 => Ok(NoteBlock {
                instrument: NoteBlockInstrument::Creeper,
                note: 23i32,
                powered: false,
            }),
            1529u32 => Ok(NoteBlock {
                instrument: NoteBlockInstrument::Creeper,
                note: 24i32,
                powered: true,
            }),
            1530u32 => Ok(NoteBlock {
                instrument: NoteBlockInstrument::Creeper,
                note: 24i32,
                powered: false,
            }),
            1531u32 => Ok(NoteBlock {
                instrument: NoteBlockInstrument::Dragon,
                note: 0i32,
                powered: true,
            }),
            1532u32 => Ok(NoteBlock {
                instrument: NoteBlockInstrument::Dragon,
                note: 0i32,
                powered: false,
            }),
            1533u32 => Ok(NoteBlock {
                instrument: NoteBlockInstrument::Dragon,
                note: 1i32,
                powered: true,
            }),
            1534u32 => Ok(NoteBlock {
                instrument: NoteBlockInstrument::Dragon,
                note: 1i32,
                powered: false,
            }),
            1535u32 => Ok(NoteBlock {
                instrument: NoteBlockInstrument::Dragon,
                note: 2i32,
                powered: true,
            }),
            1536u32 => Ok(NoteBlock {
                instrument: NoteBlockInstrument::Dragon,
                note: 2i32,
                powered: false,
            }),
            1537u32 => Ok(NoteBlock {
                instrument: NoteBlockInstrument::Dragon,
                note: 3i32,
                powered: true,
            }),
            1538u32 => Ok(NoteBlock {
                instrument: NoteBlockInstrument::Dragon,
                note: 3i32,
                powered: false,
            }),
            1539u32 => Ok(NoteBlock {
                instrument: NoteBlockInstrument::Dragon,
                note: 4i32,
                powered: true,
            }),
            1540u32 => Ok(NoteBlock {
                instrument: NoteBlockInstrument::Dragon,
                note: 4i32,
                powered: false,
            }),
            1541u32 => Ok(NoteBlock {
                instrument: NoteBlockInstrument::Dragon,
                note: 5i32,
                powered: true,
            }),
            1542u32 => Ok(NoteBlock {
                instrument: NoteBlockInstrument::Dragon,
                note: 5i32,
                powered: false,
            }),
            1543u32 => Ok(NoteBlock {
                instrument: NoteBlockInstrument::Dragon,
                note: 6i32,
                powered: true,
            }),
            1544u32 => Ok(NoteBlock {
                instrument: NoteBlockInstrument::Dragon,
                note: 6i32,
                powered: false,
            }),
            1545u32 => Ok(NoteBlock {
                instrument: NoteBlockInstrument::Dragon,
                note: 7i32,
                powered: true,
            }),
            1546u32 => Ok(NoteBlock {
                instrument: NoteBlockInstrument::Dragon,
                note: 7i32,
                powered: false,
            }),
            1547u32 => Ok(NoteBlock {
                instrument: NoteBlockInstrument::Dragon,
                note: 8i32,
                powered: true,
            }),
            1548u32 => Ok(NoteBlock {
                instrument: NoteBlockInstrument::Dragon,
                note: 8i32,
                powered: false,
            }),
            1549u32 => Ok(NoteBlock {
                instrument: NoteBlockInstrument::Dragon,
                note: 9i32,
                powered: true,
            }),
            1550u32 => Ok(NoteBlock {
                instrument: NoteBlockInstrument::Dragon,
                note: 9i32,
                powered: false,
            }),
            1551u32 => Ok(NoteBlock {
                instrument: NoteBlockInstrument::Dragon,
                note: 10i32,
                powered: true,
            }),
            1552u32 => Ok(NoteBlock {
                instrument: NoteBlockInstrument::Dragon,
                note: 10i32,
                powered: false,
            }),
            1553u32 => Ok(NoteBlock {
                instrument: NoteBlockInstrument::Dragon,
                note: 11i32,
                powered: true,
            }),
            1554u32 => Ok(NoteBlock {
                instrument: NoteBlockInstrument::Dragon,
                note: 11i32,
                powered: false,
            }),
            1555u32 => Ok(NoteBlock {
                instrument: NoteBlockInstrument::Dragon,
                note: 12i32,
                powered: true,
            }),
            1556u32 => Ok(NoteBlock {
                instrument: NoteBlockInstrument::Dragon,
                note: 12i32,
                powered: false,
            }),
            1557u32 => Ok(NoteBlock {
                instrument: NoteBlockInstrument::Dragon,
                note: 13i32,
                powered: true,
            }),
            1558u32 => Ok(NoteBlock {
                instrument: NoteBlockInstrument::Dragon,
                note: 13i32,
                powered: false,
            }),
            1559u32 => Ok(NoteBlock {
                instrument: NoteBlockInstrument::Dragon,
                note: 14i32,
                powered: true,
            }),
            1560u32 => Ok(NoteBlock {
                instrument: NoteBlockInstrument::Dragon,
                note: 14i32,
                powered: false,
            }),
            1561u32 => Ok(NoteBlock {
                instrument: NoteBlockInstrument::Dragon,
                note: 15i32,
                powered: true,
            }),
            1562u32 => Ok(NoteBlock {
                instrument: NoteBlockInstrument::Dragon,
                note: 15i32,
                powered: false,
            }),
            1563u32 => Ok(NoteBlock {
                instrument: NoteBlockInstrument::Dragon,
                note: 16i32,
                powered: true,
            }),
            1564u32 => Ok(NoteBlock {
                instrument: NoteBlockInstrument::Dragon,
                note: 16i32,
                powered: false,
            }),
            1565u32 => Ok(NoteBlock {
                instrument: NoteBlockInstrument::Dragon,
                note: 17i32,
                powered: true,
            }),
            1566u32 => Ok(NoteBlock {
                instrument: NoteBlockInstrument::Dragon,
                note: 17i32,
                powered: false,
            }),
            1567u32 => Ok(NoteBlock {
                instrument: NoteBlockInstrument::Dragon,
                note: 18i32,
                powered: true,
            }),
            1568u32 => Ok(NoteBlock {
                instrument: NoteBlockInstrument::Dragon,
                note: 18i32,
                powered: false,
            }),
            1569u32 => Ok(NoteBlock {
                instrument: NoteBlockInstrument::Dragon,
                note: 19i32,
                powered: true,
            }),
            1570u32 => Ok(NoteBlock {
                instrument: NoteBlockInstrument::Dragon,
                note: 19i32,
                powered: false,
            }),
            1571u32 => Ok(NoteBlock {
                instrument: NoteBlockInstrument::Dragon,
                note: 20i32,
                powered: true,
            }),
            1572u32 => Ok(NoteBlock {
                instrument: NoteBlockInstrument::Dragon,
                note: 20i32,
                powered: false,
            }),
            1573u32 => Ok(NoteBlock {
                instrument: NoteBlockInstrument::Dragon,
                note: 21i32,
                powered: true,
            }),
            1574u32 => Ok(NoteBlock {
                instrument: NoteBlockInstrument::Dragon,
                note: 21i32,
                powered: false,
            }),
            1575u32 => Ok(NoteBlock {
                instrument: NoteBlockInstrument::Dragon,
                note: 22i32,
                powered: true,
            }),
            1576u32 => Ok(NoteBlock {
                instrument: NoteBlockInstrument::Dragon,
                note: 22i32,
                powered: false,
            }),
            1577u32 => Ok(NoteBlock {
                instrument: NoteBlockInstrument::Dragon,
                note: 23i32,
                powered: true,
            }),
            1578u32 => Ok(NoteBlock {
                instrument: NoteBlockInstrument::Dragon,
                note: 23i32,
                powered: false,
            }),
            1579u32 => Ok(NoteBlock {
                instrument: NoteBlockInstrument::Dragon,
                note: 24i32,
                powered: true,
            }),
            1580u32 => Ok(NoteBlock {
                instrument: NoteBlockInstrument::Dragon,
                note: 24i32,
                powered: false,
            }),
            1581u32 => Ok(NoteBlock {
                instrument: NoteBlockInstrument::WitherSkeleton,
                note: 0i32,
                powered: true,
            }),
            1582u32 => Ok(NoteBlock {
                instrument: NoteBlockInstrument::WitherSkeleton,
                note: 0i32,
                powered: false,
            }),
            1583u32 => Ok(NoteBlock {
                instrument: NoteBlockInstrument::WitherSkeleton,
                note: 1i32,
                powered: true,
            }),
            1584u32 => Ok(NoteBlock {
                instrument: NoteBlockInstrument::WitherSkeleton,
                note: 1i32,
                powered: false,
            }),
            1585u32 => Ok(NoteBlock {
                instrument: NoteBlockInstrument::WitherSkeleton,
                note: 2i32,
                powered: true,
            }),
            1586u32 => Ok(NoteBlock {
                instrument: NoteBlockInstrument::WitherSkeleton,
                note: 2i32,
                powered: false,
            }),
            1587u32 => Ok(NoteBlock {
                instrument: NoteBlockInstrument::WitherSkeleton,
                note: 3i32,
                powered: true,
            }),
            1588u32 => Ok(NoteBlock {
                instrument: NoteBlockInstrument::WitherSkeleton,
                note: 3i32,
                powered: false,
            }),
            1589u32 => Ok(NoteBlock {
                instrument: NoteBlockInstrument::WitherSkeleton,
                note: 4i32,
                powered: true,
            }),
            1590u32 => Ok(NoteBlock {
                instrument: NoteBlockInstrument::WitherSkeleton,
                note: 4i32,
                powered: false,
            }),
            1591u32 => Ok(NoteBlock {
                instrument: NoteBlockInstrument::WitherSkeleton,
                note: 5i32,
                powered: true,
            }),
            1592u32 => Ok(NoteBlock {
                instrument: NoteBlockInstrument::WitherSkeleton,
                note: 5i32,
                powered: false,
            }),
            1593u32 => Ok(NoteBlock {
                instrument: NoteBlockInstrument::WitherSkeleton,
                note: 6i32,
                powered: true,
            }),
            1594u32 => Ok(NoteBlock {
                instrument: NoteBlockInstrument::WitherSkeleton,
                note: 6i32,
                powered: false,
            }),
            1595u32 => Ok(NoteBlock {
                instrument: NoteBlockInstrument::WitherSkeleton,
                note: 7i32,
                powered: true,
            }),
            1596u32 => Ok(NoteBlock {
                instrument: NoteBlockInstrument::WitherSkeleton,
                note: 7i32,
                powered: false,
            }),
            1597u32 => Ok(NoteBlock {
                instrument: NoteBlockInstrument::WitherSkeleton,
                note: 8i32,
                powered: true,
            }),
            1598u32 => Ok(NoteBlock {
                instrument: NoteBlockInstrument::WitherSkeleton,
                note: 8i32,
                powered: false,
            }),
            1599u32 => Ok(NoteBlock {
                instrument: NoteBlockInstrument::WitherSkeleton,
                note: 9i32,
                powered: true,
            }),
            1600u32 => Ok(NoteBlock {
                instrument: NoteBlockInstrument::WitherSkeleton,
                note: 9i32,
                powered: false,
            }),
            1601u32 => Ok(NoteBlock {
                instrument: NoteBlockInstrument::WitherSkeleton,
                note: 10i32,
                powered: true,
            }),
            1602u32 => Ok(NoteBlock {
                instrument: NoteBlockInstrument::WitherSkeleton,
                note: 10i32,
                powered: false,
            }),
            1603u32 => Ok(NoteBlock {
                instrument: NoteBlockInstrument::WitherSkeleton,
                note: 11i32,
                powered: true,
            }),
            1604u32 => Ok(NoteBlock {
                instrument: NoteBlockInstrument::WitherSkeleton,
                note: 11i32,
                powered: false,
            }),
            1605u32 => Ok(NoteBlock {
                instrument: NoteBlockInstrument::WitherSkeleton,
                note: 12i32,
                powered: true,
            }),
            1606u32 => Ok(NoteBlock {
                instrument: NoteBlockInstrument::WitherSkeleton,
                note: 12i32,
                powered: false,
            }),
            1607u32 => Ok(NoteBlock {
                instrument: NoteBlockInstrument::WitherSkeleton,
                note: 13i32,
                powered: true,
            }),
            1608u32 => Ok(NoteBlock {
                instrument: NoteBlockInstrument::WitherSkeleton,
                note: 13i32,
                powered: false,
            }),
            1609u32 => Ok(NoteBlock {
                instrument: NoteBlockInstrument::WitherSkeleton,
                note: 14i32,
                powered: true,
            }),
            1610u32 => Ok(NoteBlock {
                instrument: NoteBlockInstrument::WitherSkeleton,
                note: 14i32,
                powered: false,
            }),
            1611u32 => Ok(NoteBlock {
                instrument: NoteBlockInstrument::WitherSkeleton,
                note: 15i32,
                powered: true,
            }),
            1612u32 => Ok(NoteBlock {
                instrument: NoteBlockInstrument::WitherSkeleton,
                note: 15i32,
                powered: false,
            }),
            1613u32 => Ok(NoteBlock {
                instrument: NoteBlockInstrument::WitherSkeleton,
                note: 16i32,
                powered: true,
            }),
            1614u32 => Ok(NoteBlock {
                instrument: NoteBlockInstrument::WitherSkeleton,
                note: 16i32,
                powered: false,
            }),
            1615u32 => Ok(NoteBlock {
                instrument: NoteBlockInstrument::WitherSkeleton,
                note: 17i32,
                powered: true,
            }),
            1616u32 => Ok(NoteBlock {
                instrument: NoteBlockInstrument::WitherSkeleton,
                note: 17i32,
                powered: false,
            }),
            1617u32 => Ok(NoteBlock {
                instrument: NoteBlockInstrument::WitherSkeleton,
                note: 18i32,
                powered: true,
            }),
            1618u32 => Ok(NoteBlock {
                instrument: NoteBlockInstrument::WitherSkeleton,
                note: 18i32,
                powered: false,
            }),
            1619u32 => Ok(NoteBlock {
                instrument: NoteBlockInstrument::WitherSkeleton,
                note: 19i32,
                powered: true,
            }),
            1620u32 => Ok(NoteBlock {
                instrument: NoteBlockInstrument::WitherSkeleton,
                note: 19i32,
                powered: false,
            }),
            1621u32 => Ok(NoteBlock {
                instrument: NoteBlockInstrument::WitherSkeleton,
                note: 20i32,
                powered: true,
            }),
            1622u32 => Ok(NoteBlock {
                instrument: NoteBlockInstrument::WitherSkeleton,
                note: 20i32,
                powered: false,
            }),
            1623u32 => Ok(NoteBlock {
                instrument: NoteBlockInstrument::WitherSkeleton,
                note: 21i32,
                powered: true,
            }),
            1624u32 => Ok(NoteBlock {
                instrument: NoteBlockInstrument::WitherSkeleton,
                note: 21i32,
                powered: false,
            }),
            1625u32 => Ok(NoteBlock {
                instrument: NoteBlockInstrument::WitherSkeleton,
                note: 22i32,
                powered: true,
            }),
            1626u32 => Ok(NoteBlock {
                instrument: NoteBlockInstrument::WitherSkeleton,
                note: 22i32,
                powered: false,
            }),
            1627u32 => Ok(NoteBlock {
                instrument: NoteBlockInstrument::WitherSkeleton,
                note: 23i32,
                powered: true,
            }),
            1628u32 => Ok(NoteBlock {
                instrument: NoteBlockInstrument::WitherSkeleton,
                note: 23i32,
                powered: false,
            }),
            1629u32 => Ok(NoteBlock {
                instrument: NoteBlockInstrument::WitherSkeleton,
                note: 24i32,
                powered: true,
            }),
            1630u32 => Ok(NoteBlock {
                instrument: NoteBlockInstrument::WitherSkeleton,
                note: 24i32,
                powered: false,
            }),
            1631u32 => Ok(NoteBlock {
                instrument: NoteBlockInstrument::Piglin,
                note: 0i32,
                powered: true,
            }),
            1632u32 => Ok(NoteBlock {
                instrument: NoteBlockInstrument::Piglin,
                note: 0i32,
                powered: false,
            }),
            1633u32 => Ok(NoteBlock {
                instrument: NoteBlockInstrument::Piglin,
                note: 1i32,
                powered: true,
            }),
            1634u32 => Ok(NoteBlock {
                instrument: NoteBlockInstrument::Piglin,
                note: 1i32,
                powered: false,
            }),
            1635u32 => Ok(NoteBlock {
                instrument: NoteBlockInstrument::Piglin,
                note: 2i32,
                powered: true,
            }),
            1636u32 => Ok(NoteBlock {
                instrument: NoteBlockInstrument::Piglin,
                note: 2i32,
                powered: false,
            }),
            1637u32 => Ok(NoteBlock {
                instrument: NoteBlockInstrument::Piglin,
                note: 3i32,
                powered: true,
            }),
            1638u32 => Ok(NoteBlock {
                instrument: NoteBlockInstrument::Piglin,
                note: 3i32,
                powered: false,
            }),
            1639u32 => Ok(NoteBlock {
                instrument: NoteBlockInstrument::Piglin,
                note: 4i32,
                powered: true,
            }),
            1640u32 => Ok(NoteBlock {
                instrument: NoteBlockInstrument::Piglin,
                note: 4i32,
                powered: false,
            }),
            1641u32 => Ok(NoteBlock {
                instrument: NoteBlockInstrument::Piglin,
                note: 5i32,
                powered: true,
            }),
            1642u32 => Ok(NoteBlock {
                instrument: NoteBlockInstrument::Piglin,
                note: 5i32,
                powered: false,
            }),
            1643u32 => Ok(NoteBlock {
                instrument: NoteBlockInstrument::Piglin,
                note: 6i32,
                powered: true,
            }),
            1644u32 => Ok(NoteBlock {
                instrument: NoteBlockInstrument::Piglin,
                note: 6i32,
                powered: false,
            }),
            1645u32 => Ok(NoteBlock {
                instrument: NoteBlockInstrument::Piglin,
                note: 7i32,
                powered: true,
            }),
            1646u32 => Ok(NoteBlock {
                instrument: NoteBlockInstrument::Piglin,
                note: 7i32,
                powered: false,
            }),
            1647u32 => Ok(NoteBlock {
                instrument: NoteBlockInstrument::Piglin,
                note: 8i32,
                powered: true,
            }),
            1648u32 => Ok(NoteBlock {
                instrument: NoteBlockInstrument::Piglin,
                note: 8i32,
                powered: false,
            }),
            1649u32 => Ok(NoteBlock {
                instrument: NoteBlockInstrument::Piglin,
                note: 9i32,
                powered: true,
            }),
            1650u32 => Ok(NoteBlock {
                instrument: NoteBlockInstrument::Piglin,
                note: 9i32,
                powered: false,
            }),
            1651u32 => Ok(NoteBlock {
                instrument: NoteBlockInstrument::Piglin,
                note: 10i32,
                powered: true,
            }),
            1652u32 => Ok(NoteBlock {
                instrument: NoteBlockInstrument::Piglin,
                note: 10i32,
                powered: false,
            }),
            1653u32 => Ok(NoteBlock {
                instrument: NoteBlockInstrument::Piglin,
                note: 11i32,
                powered: true,
            }),
            1654u32 => Ok(NoteBlock {
                instrument: NoteBlockInstrument::Piglin,
                note: 11i32,
                powered: false,
            }),
            1655u32 => Ok(NoteBlock {
                instrument: NoteBlockInstrument::Piglin,
                note: 12i32,
                powered: true,
            }),
            1656u32 => Ok(NoteBlock {
                instrument: NoteBlockInstrument::Piglin,
                note: 12i32,
                powered: false,
            }),
            1657u32 => Ok(NoteBlock {
                instrument: NoteBlockInstrument::Piglin,
                note: 13i32,
                powered: true,
            }),
            1658u32 => Ok(NoteBlock {
                instrument: NoteBlockInstrument::Piglin,
                note: 13i32,
                powered: false,
            }),
            1659u32 => Ok(NoteBlock {
                instrument: NoteBlockInstrument::Piglin,
                note: 14i32,
                powered: true,
            }),
            1660u32 => Ok(NoteBlock {
                instrument: NoteBlockInstrument::Piglin,
                note: 14i32,
                powered: false,
            }),
            1661u32 => Ok(NoteBlock {
                instrument: NoteBlockInstrument::Piglin,
                note: 15i32,
                powered: true,
            }),
            1662u32 => Ok(NoteBlock {
                instrument: NoteBlockInstrument::Piglin,
                note: 15i32,
                powered: false,
            }),
            1663u32 => Ok(NoteBlock {
                instrument: NoteBlockInstrument::Piglin,
                note: 16i32,
                powered: true,
            }),
            1664u32 => Ok(NoteBlock {
                instrument: NoteBlockInstrument::Piglin,
                note: 16i32,
                powered: false,
            }),
            1665u32 => Ok(NoteBlock {
                instrument: NoteBlockInstrument::Piglin,
                note: 17i32,
                powered: true,
            }),
            1666u32 => Ok(NoteBlock {
                instrument: NoteBlockInstrument::Piglin,
                note: 17i32,
                powered: false,
            }),
            1667u32 => Ok(NoteBlock {
                instrument: NoteBlockInstrument::Piglin,
                note: 18i32,
                powered: true,
            }),
            1668u32 => Ok(NoteBlock {
                instrument: NoteBlockInstrument::Piglin,
                note: 18i32,
                powered: false,
            }),
            1669u32 => Ok(NoteBlock {
                instrument: NoteBlockInstrument::Piglin,
                note: 19i32,
                powered: true,
            }),
            1670u32 => Ok(NoteBlock {
                instrument: NoteBlockInstrument::Piglin,
                note: 19i32,
                powered: false,
            }),
            1671u32 => Ok(NoteBlock {
                instrument: NoteBlockInstrument::Piglin,
                note: 20i32,
                powered: true,
            }),
            1672u32 => Ok(NoteBlock {
                instrument: NoteBlockInstrument::Piglin,
                note: 20i32,
                powered: false,
            }),
            1673u32 => Ok(NoteBlock {
                instrument: NoteBlockInstrument::Piglin,
                note: 21i32,
                powered: true,
            }),
            1674u32 => Ok(NoteBlock {
                instrument: NoteBlockInstrument::Piglin,
                note: 21i32,
                powered: false,
            }),
            1675u32 => Ok(NoteBlock {
                instrument: NoteBlockInstrument::Piglin,
                note: 22i32,
                powered: true,
            }),
            1676u32 => Ok(NoteBlock {
                instrument: NoteBlockInstrument::Piglin,
                note: 22i32,
                powered: false,
            }),
            1677u32 => Ok(NoteBlock {
                instrument: NoteBlockInstrument::Piglin,
                note: 23i32,
                powered: true,
            }),
            1678u32 => Ok(NoteBlock {
                instrument: NoteBlockInstrument::Piglin,
                note: 23i32,
                powered: false,
            }),
            1679u32 => Ok(NoteBlock {
                instrument: NoteBlockInstrument::Piglin,
                note: 24i32,
                powered: true,
            }),
            1680u32 => Ok(NoteBlock {
                instrument: NoteBlockInstrument::Piglin,
                note: 24i32,
                powered: false,
            }),
            1681u32 => Ok(NoteBlock {
                instrument: NoteBlockInstrument::CustomHead,
                note: 0i32,
                powered: true,
            }),
            1682u32 => Ok(NoteBlock {
                instrument: NoteBlockInstrument::CustomHead,
                note: 0i32,
                powered: false,
            }),
            1683u32 => Ok(NoteBlock {
                instrument: NoteBlockInstrument::CustomHead,
                note: 1i32,
                powered: true,
            }),
            1684u32 => Ok(NoteBlock {
                instrument: NoteBlockInstrument::CustomHead,
                note: 1i32,
                powered: false,
            }),
            1685u32 => Ok(NoteBlock {
                instrument: NoteBlockInstrument::CustomHead,
                note: 2i32,
                powered: true,
            }),
            1686u32 => Ok(NoteBlock {
                instrument: NoteBlockInstrument::CustomHead,
                note: 2i32,
                powered: false,
            }),
            1687u32 => Ok(NoteBlock {
                instrument: NoteBlockInstrument::CustomHead,
                note: 3i32,
                powered: true,
            }),
            1688u32 => Ok(NoteBlock {
                instrument: NoteBlockInstrument::CustomHead,
                note: 3i32,
                powered: false,
            }),
            1689u32 => Ok(NoteBlock {
                instrument: NoteBlockInstrument::CustomHead,
                note: 4i32,
                powered: true,
            }),
            1690u32 => Ok(NoteBlock {
                instrument: NoteBlockInstrument::CustomHead,
                note: 4i32,
                powered: false,
            }),
            1691u32 => Ok(NoteBlock {
                instrument: NoteBlockInstrument::CustomHead,
                note: 5i32,
                powered: true,
            }),
            1692u32 => Ok(NoteBlock {
                instrument: NoteBlockInstrument::CustomHead,
                note: 5i32,
                powered: false,
            }),
            1693u32 => Ok(NoteBlock {
                instrument: NoteBlockInstrument::CustomHead,
                note: 6i32,
                powered: true,
            }),
            1694u32 => Ok(NoteBlock {
                instrument: NoteBlockInstrument::CustomHead,
                note: 6i32,
                powered: false,
            }),
            1695u32 => Ok(NoteBlock {
                instrument: NoteBlockInstrument::CustomHead,
                note: 7i32,
                powered: true,
            }),
            1696u32 => Ok(NoteBlock {
                instrument: NoteBlockInstrument::CustomHead,
                note: 7i32,
                powered: false,
            }),
            1697u32 => Ok(NoteBlock {
                instrument: NoteBlockInstrument::CustomHead,
                note: 8i32,
                powered: true,
            }),
            1698u32 => Ok(NoteBlock {
                instrument: NoteBlockInstrument::CustomHead,
                note: 8i32,
                powered: false,
            }),
            1699u32 => Ok(NoteBlock {
                instrument: NoteBlockInstrument::CustomHead,
                note: 9i32,
                powered: true,
            }),
            1700u32 => Ok(NoteBlock {
                instrument: NoteBlockInstrument::CustomHead,
                note: 9i32,
                powered: false,
            }),
            1701u32 => Ok(NoteBlock {
                instrument: NoteBlockInstrument::CustomHead,
                note: 10i32,
                powered: true,
            }),
            1702u32 => Ok(NoteBlock {
                instrument: NoteBlockInstrument::CustomHead,
                note: 10i32,
                powered: false,
            }),
            1703u32 => Ok(NoteBlock {
                instrument: NoteBlockInstrument::CustomHead,
                note: 11i32,
                powered: true,
            }),
            1704u32 => Ok(NoteBlock {
                instrument: NoteBlockInstrument::CustomHead,
                note: 11i32,
                powered: false,
            }),
            1705u32 => Ok(NoteBlock {
                instrument: NoteBlockInstrument::CustomHead,
                note: 12i32,
                powered: true,
            }),
            1706u32 => Ok(NoteBlock {
                instrument: NoteBlockInstrument::CustomHead,
                note: 12i32,
                powered: false,
            }),
            1707u32 => Ok(NoteBlock {
                instrument: NoteBlockInstrument::CustomHead,
                note: 13i32,
                powered: true,
            }),
            1708u32 => Ok(NoteBlock {
                instrument: NoteBlockInstrument::CustomHead,
                note: 13i32,
                powered: false,
            }),
            1709u32 => Ok(NoteBlock {
                instrument: NoteBlockInstrument::CustomHead,
                note: 14i32,
                powered: true,
            }),
            1710u32 => Ok(NoteBlock {
                instrument: NoteBlockInstrument::CustomHead,
                note: 14i32,
                powered: false,
            }),
            1711u32 => Ok(NoteBlock {
                instrument: NoteBlockInstrument::CustomHead,
                note: 15i32,
                powered: true,
            }),
            1712u32 => Ok(NoteBlock {
                instrument: NoteBlockInstrument::CustomHead,
                note: 15i32,
                powered: false,
            }),
            1713u32 => Ok(NoteBlock {
                instrument: NoteBlockInstrument::CustomHead,
                note: 16i32,
                powered: true,
            }),
            1714u32 => Ok(NoteBlock {
                instrument: NoteBlockInstrument::CustomHead,
                note: 16i32,
                powered: false,
            }),
            1715u32 => Ok(NoteBlock {
                instrument: NoteBlockInstrument::CustomHead,
                note: 17i32,
                powered: true,
            }),
            1716u32 => Ok(NoteBlock {
                instrument: NoteBlockInstrument::CustomHead,
                note: 17i32,
                powered: false,
            }),
            1717u32 => Ok(NoteBlock {
                instrument: NoteBlockInstrument::CustomHead,
                note: 18i32,
                powered: true,
            }),
            1718u32 => Ok(NoteBlock {
                instrument: NoteBlockInstrument::CustomHead,
                note: 18i32,
                powered: false,
            }),
            1719u32 => Ok(NoteBlock {
                instrument: NoteBlockInstrument::CustomHead,
                note: 19i32,
                powered: true,
            }),
            1720u32 => Ok(NoteBlock {
                instrument: NoteBlockInstrument::CustomHead,
                note: 19i32,
                powered: false,
            }),
            1721u32 => Ok(NoteBlock {
                instrument: NoteBlockInstrument::CustomHead,
                note: 20i32,
                powered: true,
            }),
            1722u32 => Ok(NoteBlock {
                instrument: NoteBlockInstrument::CustomHead,
                note: 20i32,
                powered: false,
            }),
            1723u32 => Ok(NoteBlock {
                instrument: NoteBlockInstrument::CustomHead,
                note: 21i32,
                powered: true,
            }),
            1724u32 => Ok(NoteBlock {
                instrument: NoteBlockInstrument::CustomHead,
                note: 21i32,
                powered: false,
            }),
            1725u32 => Ok(NoteBlock {
                instrument: NoteBlockInstrument::CustomHead,
                note: 22i32,
                powered: true,
            }),
            1726u32 => Ok(NoteBlock {
                instrument: NoteBlockInstrument::CustomHead,
                note: 22i32,
                powered: false,
            }),
            1727u32 => Ok(NoteBlock {
                instrument: NoteBlockInstrument::CustomHead,
                note: 23i32,
                powered: true,
            }),
            1728u32 => Ok(NoteBlock {
                instrument: NoteBlockInstrument::CustomHead,
                note: 23i32,
                powered: false,
            }),
            1729u32 => Ok(NoteBlock {
                instrument: NoteBlockInstrument::CustomHead,
                note: 24i32,
                powered: true,
            }),
            1730u32 => Ok(NoteBlock {
                instrument: NoteBlockInstrument::CustomHead,
                note: 24i32,
                powered: false,
            }),
            _ => Err(()),
        }
    }
}
impl TryInto<u32> for NoteBlock {
    type Error = ();
    fn try_into(self) -> Result<u32, Self::Error> {
        #[allow(unreachable_patterns)]
        match self {
            NoteBlock {
                instrument: NoteBlockInstrument::Harp,
                note: 0i32,
                powered: true,
            } => Ok(581u32),
            NoteBlock {
                instrument: NoteBlockInstrument::Harp,
                note: 0i32,
                powered: false,
            } => Ok(582u32),
            NoteBlock {
                instrument: NoteBlockInstrument::Harp,
                note: 1i32,
                powered: true,
            } => Ok(583u32),
            NoteBlock {
                instrument: NoteBlockInstrument::Harp,
                note: 1i32,
                powered: false,
            } => Ok(584u32),
            NoteBlock {
                instrument: NoteBlockInstrument::Harp,
                note: 2i32,
                powered: true,
            } => Ok(585u32),
            NoteBlock {
                instrument: NoteBlockInstrument::Harp,
                note: 2i32,
                powered: false,
            } => Ok(586u32),
            NoteBlock {
                instrument: NoteBlockInstrument::Harp,
                note: 3i32,
                powered: true,
            } => Ok(587u32),
            NoteBlock {
                instrument: NoteBlockInstrument::Harp,
                note: 3i32,
                powered: false,
            } => Ok(588u32),
            NoteBlock {
                instrument: NoteBlockInstrument::Harp,
                note: 4i32,
                powered: true,
            } => Ok(589u32),
            NoteBlock {
                instrument: NoteBlockInstrument::Harp,
                note: 4i32,
                powered: false,
            } => Ok(590u32),
            NoteBlock {
                instrument: NoteBlockInstrument::Harp,
                note: 5i32,
                powered: true,
            } => Ok(591u32),
            NoteBlock {
                instrument: NoteBlockInstrument::Harp,
                note: 5i32,
                powered: false,
            } => Ok(592u32),
            NoteBlock {
                instrument: NoteBlockInstrument::Harp,
                note: 6i32,
                powered: true,
            } => Ok(593u32),
            NoteBlock {
                instrument: NoteBlockInstrument::Harp,
                note: 6i32,
                powered: false,
            } => Ok(594u32),
            NoteBlock {
                instrument: NoteBlockInstrument::Harp,
                note: 7i32,
                powered: true,
            } => Ok(595u32),
            NoteBlock {
                instrument: NoteBlockInstrument::Harp,
                note: 7i32,
                powered: false,
            } => Ok(596u32),
            NoteBlock {
                instrument: NoteBlockInstrument::Harp,
                note: 8i32,
                powered: true,
            } => Ok(597u32),
            NoteBlock {
                instrument: NoteBlockInstrument::Harp,
                note: 8i32,
                powered: false,
            } => Ok(598u32),
            NoteBlock {
                instrument: NoteBlockInstrument::Harp,
                note: 9i32,
                powered: true,
            } => Ok(599u32),
            NoteBlock {
                instrument: NoteBlockInstrument::Harp,
                note: 9i32,
                powered: false,
            } => Ok(600u32),
            NoteBlock {
                instrument: NoteBlockInstrument::Harp,
                note: 10i32,
                powered: true,
            } => Ok(601u32),
            NoteBlock {
                instrument: NoteBlockInstrument::Harp,
                note: 10i32,
                powered: false,
            } => Ok(602u32),
            NoteBlock {
                instrument: NoteBlockInstrument::Harp,
                note: 11i32,
                powered: true,
            } => Ok(603u32),
            NoteBlock {
                instrument: NoteBlockInstrument::Harp,
                note: 11i32,
                powered: false,
            } => Ok(604u32),
            NoteBlock {
                instrument: NoteBlockInstrument::Harp,
                note: 12i32,
                powered: true,
            } => Ok(605u32),
            NoteBlock {
                instrument: NoteBlockInstrument::Harp,
                note: 12i32,
                powered: false,
            } => Ok(606u32),
            NoteBlock {
                instrument: NoteBlockInstrument::Harp,
                note: 13i32,
                powered: true,
            } => Ok(607u32),
            NoteBlock {
                instrument: NoteBlockInstrument::Harp,
                note: 13i32,
                powered: false,
            } => Ok(608u32),
            NoteBlock {
                instrument: NoteBlockInstrument::Harp,
                note: 14i32,
                powered: true,
            } => Ok(609u32),
            NoteBlock {
                instrument: NoteBlockInstrument::Harp,
                note: 14i32,
                powered: false,
            } => Ok(610u32),
            NoteBlock {
                instrument: NoteBlockInstrument::Harp,
                note: 15i32,
                powered: true,
            } => Ok(611u32),
            NoteBlock {
                instrument: NoteBlockInstrument::Harp,
                note: 15i32,
                powered: false,
            } => Ok(612u32),
            NoteBlock {
                instrument: NoteBlockInstrument::Harp,
                note: 16i32,
                powered: true,
            } => Ok(613u32),
            NoteBlock {
                instrument: NoteBlockInstrument::Harp,
                note: 16i32,
                powered: false,
            } => Ok(614u32),
            NoteBlock {
                instrument: NoteBlockInstrument::Harp,
                note: 17i32,
                powered: true,
            } => Ok(615u32),
            NoteBlock {
                instrument: NoteBlockInstrument::Harp,
                note: 17i32,
                powered: false,
            } => Ok(616u32),
            NoteBlock {
                instrument: NoteBlockInstrument::Harp,
                note: 18i32,
                powered: true,
            } => Ok(617u32),
            NoteBlock {
                instrument: NoteBlockInstrument::Harp,
                note: 18i32,
                powered: false,
            } => Ok(618u32),
            NoteBlock {
                instrument: NoteBlockInstrument::Harp,
                note: 19i32,
                powered: true,
            } => Ok(619u32),
            NoteBlock {
                instrument: NoteBlockInstrument::Harp,
                note: 19i32,
                powered: false,
            } => Ok(620u32),
            NoteBlock {
                instrument: NoteBlockInstrument::Harp,
                note: 20i32,
                powered: true,
            } => Ok(621u32),
            NoteBlock {
                instrument: NoteBlockInstrument::Harp,
                note: 20i32,
                powered: false,
            } => Ok(622u32),
            NoteBlock {
                instrument: NoteBlockInstrument::Harp,
                note: 21i32,
                powered: true,
            } => Ok(623u32),
            NoteBlock {
                instrument: NoteBlockInstrument::Harp,
                note: 21i32,
                powered: false,
            } => Ok(624u32),
            NoteBlock {
                instrument: NoteBlockInstrument::Harp,
                note: 22i32,
                powered: true,
            } => Ok(625u32),
            NoteBlock {
                instrument: NoteBlockInstrument::Harp,
                note: 22i32,
                powered: false,
            } => Ok(626u32),
            NoteBlock {
                instrument: NoteBlockInstrument::Harp,
                note: 23i32,
                powered: true,
            } => Ok(627u32),
            NoteBlock {
                instrument: NoteBlockInstrument::Harp,
                note: 23i32,
                powered: false,
            } => Ok(628u32),
            NoteBlock {
                instrument: NoteBlockInstrument::Harp,
                note: 24i32,
                powered: true,
            } => Ok(629u32),
            NoteBlock {
                instrument: NoteBlockInstrument::Harp,
                note: 24i32,
                powered: false,
            } => Ok(630u32),
            NoteBlock {
                instrument: NoteBlockInstrument::BaseDrum,
                note: 0i32,
                powered: true,
            } => Ok(631u32),
            NoteBlock {
                instrument: NoteBlockInstrument::BaseDrum,
                note: 0i32,
                powered: false,
            } => Ok(632u32),
            NoteBlock {
                instrument: NoteBlockInstrument::BaseDrum,
                note: 1i32,
                powered: true,
            } => Ok(633u32),
            NoteBlock {
                instrument: NoteBlockInstrument::BaseDrum,
                note: 1i32,
                powered: false,
            } => Ok(634u32),
            NoteBlock {
                instrument: NoteBlockInstrument::BaseDrum,
                note: 2i32,
                powered: true,
            } => Ok(635u32),
            NoteBlock {
                instrument: NoteBlockInstrument::BaseDrum,
                note: 2i32,
                powered: false,
            } => Ok(636u32),
            NoteBlock {
                instrument: NoteBlockInstrument::BaseDrum,
                note: 3i32,
                powered: true,
            } => Ok(637u32),
            NoteBlock {
                instrument: NoteBlockInstrument::BaseDrum,
                note: 3i32,
                powered: false,
            } => Ok(638u32),
            NoteBlock {
                instrument: NoteBlockInstrument::BaseDrum,
                note: 4i32,
                powered: true,
            } => Ok(639u32),
            NoteBlock {
                instrument: NoteBlockInstrument::BaseDrum,
                note: 4i32,
                powered: false,
            } => Ok(640u32),
            NoteBlock {
                instrument: NoteBlockInstrument::BaseDrum,
                note: 5i32,
                powered: true,
            } => Ok(641u32),
            NoteBlock {
                instrument: NoteBlockInstrument::BaseDrum,
                note: 5i32,
                powered: false,
            } => Ok(642u32),
            NoteBlock {
                instrument: NoteBlockInstrument::BaseDrum,
                note: 6i32,
                powered: true,
            } => Ok(643u32),
            NoteBlock {
                instrument: NoteBlockInstrument::BaseDrum,
                note: 6i32,
                powered: false,
            } => Ok(644u32),
            NoteBlock {
                instrument: NoteBlockInstrument::BaseDrum,
                note: 7i32,
                powered: true,
            } => Ok(645u32),
            NoteBlock {
                instrument: NoteBlockInstrument::BaseDrum,
                note: 7i32,
                powered: false,
            } => Ok(646u32),
            NoteBlock {
                instrument: NoteBlockInstrument::BaseDrum,
                note: 8i32,
                powered: true,
            } => Ok(647u32),
            NoteBlock {
                instrument: NoteBlockInstrument::BaseDrum,
                note: 8i32,
                powered: false,
            } => Ok(648u32),
            NoteBlock {
                instrument: NoteBlockInstrument::BaseDrum,
                note: 9i32,
                powered: true,
            } => Ok(649u32),
            NoteBlock {
                instrument: NoteBlockInstrument::BaseDrum,
                note: 9i32,
                powered: false,
            } => Ok(650u32),
            NoteBlock {
                instrument: NoteBlockInstrument::BaseDrum,
                note: 10i32,
                powered: true,
            } => Ok(651u32),
            NoteBlock {
                instrument: NoteBlockInstrument::BaseDrum,
                note: 10i32,
                powered: false,
            } => Ok(652u32),
            NoteBlock {
                instrument: NoteBlockInstrument::BaseDrum,
                note: 11i32,
                powered: true,
            } => Ok(653u32),
            NoteBlock {
                instrument: NoteBlockInstrument::BaseDrum,
                note: 11i32,
                powered: false,
            } => Ok(654u32),
            NoteBlock {
                instrument: NoteBlockInstrument::BaseDrum,
                note: 12i32,
                powered: true,
            } => Ok(655u32),
            NoteBlock {
                instrument: NoteBlockInstrument::BaseDrum,
                note: 12i32,
                powered: false,
            } => Ok(656u32),
            NoteBlock {
                instrument: NoteBlockInstrument::BaseDrum,
                note: 13i32,
                powered: true,
            } => Ok(657u32),
            NoteBlock {
                instrument: NoteBlockInstrument::BaseDrum,
                note: 13i32,
                powered: false,
            } => Ok(658u32),
            NoteBlock {
                instrument: NoteBlockInstrument::BaseDrum,
                note: 14i32,
                powered: true,
            } => Ok(659u32),
            NoteBlock {
                instrument: NoteBlockInstrument::BaseDrum,
                note: 14i32,
                powered: false,
            } => Ok(660u32),
            NoteBlock {
                instrument: NoteBlockInstrument::BaseDrum,
                note: 15i32,
                powered: true,
            } => Ok(661u32),
            NoteBlock {
                instrument: NoteBlockInstrument::BaseDrum,
                note: 15i32,
                powered: false,
            } => Ok(662u32),
            NoteBlock {
                instrument: NoteBlockInstrument::BaseDrum,
                note: 16i32,
                powered: true,
            } => Ok(663u32),
            NoteBlock {
                instrument: NoteBlockInstrument::BaseDrum,
                note: 16i32,
                powered: false,
            } => Ok(664u32),
            NoteBlock {
                instrument: NoteBlockInstrument::BaseDrum,
                note: 17i32,
                powered: true,
            } => Ok(665u32),
            NoteBlock {
                instrument: NoteBlockInstrument::BaseDrum,
                note: 17i32,
                powered: false,
            } => Ok(666u32),
            NoteBlock {
                instrument: NoteBlockInstrument::BaseDrum,
                note: 18i32,
                powered: true,
            } => Ok(667u32),
            NoteBlock {
                instrument: NoteBlockInstrument::BaseDrum,
                note: 18i32,
                powered: false,
            } => Ok(668u32),
            NoteBlock {
                instrument: NoteBlockInstrument::BaseDrum,
                note: 19i32,
                powered: true,
            } => Ok(669u32),
            NoteBlock {
                instrument: NoteBlockInstrument::BaseDrum,
                note: 19i32,
                powered: false,
            } => Ok(670u32),
            NoteBlock {
                instrument: NoteBlockInstrument::BaseDrum,
                note: 20i32,
                powered: true,
            } => Ok(671u32),
            NoteBlock {
                instrument: NoteBlockInstrument::BaseDrum,
                note: 20i32,
                powered: false,
            } => Ok(672u32),
            NoteBlock {
                instrument: NoteBlockInstrument::BaseDrum,
                note: 21i32,
                powered: true,
            } => Ok(673u32),
            NoteBlock {
                instrument: NoteBlockInstrument::BaseDrum,
                note: 21i32,
                powered: false,
            } => Ok(674u32),
            NoteBlock {
                instrument: NoteBlockInstrument::BaseDrum,
                note: 22i32,
                powered: true,
            } => Ok(675u32),
            NoteBlock {
                instrument: NoteBlockInstrument::BaseDrum,
                note: 22i32,
                powered: false,
            } => Ok(676u32),
            NoteBlock {
                instrument: NoteBlockInstrument::BaseDrum,
                note: 23i32,
                powered: true,
            } => Ok(677u32),
            NoteBlock {
                instrument: NoteBlockInstrument::BaseDrum,
                note: 23i32,
                powered: false,
            } => Ok(678u32),
            NoteBlock {
                instrument: NoteBlockInstrument::BaseDrum,
                note: 24i32,
                powered: true,
            } => Ok(679u32),
            NoteBlock {
                instrument: NoteBlockInstrument::BaseDrum,
                note: 24i32,
                powered: false,
            } => Ok(680u32),
            NoteBlock {
                instrument: NoteBlockInstrument::Snare,
                note: 0i32,
                powered: true,
            } => Ok(681u32),
            NoteBlock {
                instrument: NoteBlockInstrument::Snare,
                note: 0i32,
                powered: false,
            } => Ok(682u32),
            NoteBlock {
                instrument: NoteBlockInstrument::Snare,
                note: 1i32,
                powered: true,
            } => Ok(683u32),
            NoteBlock {
                instrument: NoteBlockInstrument::Snare,
                note: 1i32,
                powered: false,
            } => Ok(684u32),
            NoteBlock {
                instrument: NoteBlockInstrument::Snare,
                note: 2i32,
                powered: true,
            } => Ok(685u32),
            NoteBlock {
                instrument: NoteBlockInstrument::Snare,
                note: 2i32,
                powered: false,
            } => Ok(686u32),
            NoteBlock {
                instrument: NoteBlockInstrument::Snare,
                note: 3i32,
                powered: true,
            } => Ok(687u32),
            NoteBlock {
                instrument: NoteBlockInstrument::Snare,
                note: 3i32,
                powered: false,
            } => Ok(688u32),
            NoteBlock {
                instrument: NoteBlockInstrument::Snare,
                note: 4i32,
                powered: true,
            } => Ok(689u32),
            NoteBlock {
                instrument: NoteBlockInstrument::Snare,
                note: 4i32,
                powered: false,
            } => Ok(690u32),
            NoteBlock {
                instrument: NoteBlockInstrument::Snare,
                note: 5i32,
                powered: true,
            } => Ok(691u32),
            NoteBlock {
                instrument: NoteBlockInstrument::Snare,
                note: 5i32,
                powered: false,
            } => Ok(692u32),
            NoteBlock {
                instrument: NoteBlockInstrument::Snare,
                note: 6i32,
                powered: true,
            } => Ok(693u32),
            NoteBlock {
                instrument: NoteBlockInstrument::Snare,
                note: 6i32,
                powered: false,
            } => Ok(694u32),
            NoteBlock {
                instrument: NoteBlockInstrument::Snare,
                note: 7i32,
                powered: true,
            } => Ok(695u32),
            NoteBlock {
                instrument: NoteBlockInstrument::Snare,
                note: 7i32,
                powered: false,
            } => Ok(696u32),
            NoteBlock {
                instrument: NoteBlockInstrument::Snare,
                note: 8i32,
                powered: true,
            } => Ok(697u32),
            NoteBlock {
                instrument: NoteBlockInstrument::Snare,
                note: 8i32,
                powered: false,
            } => Ok(698u32),
            NoteBlock {
                instrument: NoteBlockInstrument::Snare,
                note: 9i32,
                powered: true,
            } => Ok(699u32),
            NoteBlock {
                instrument: NoteBlockInstrument::Snare,
                note: 9i32,
                powered: false,
            } => Ok(700u32),
            NoteBlock {
                instrument: NoteBlockInstrument::Snare,
                note: 10i32,
                powered: true,
            } => Ok(701u32),
            NoteBlock {
                instrument: NoteBlockInstrument::Snare,
                note: 10i32,
                powered: false,
            } => Ok(702u32),
            NoteBlock {
                instrument: NoteBlockInstrument::Snare,
                note: 11i32,
                powered: true,
            } => Ok(703u32),
            NoteBlock {
                instrument: NoteBlockInstrument::Snare,
                note: 11i32,
                powered: false,
            } => Ok(704u32),
            NoteBlock {
                instrument: NoteBlockInstrument::Snare,
                note: 12i32,
                powered: true,
            } => Ok(705u32),
            NoteBlock {
                instrument: NoteBlockInstrument::Snare,
                note: 12i32,
                powered: false,
            } => Ok(706u32),
            NoteBlock {
                instrument: NoteBlockInstrument::Snare,
                note: 13i32,
                powered: true,
            } => Ok(707u32),
            NoteBlock {
                instrument: NoteBlockInstrument::Snare,
                note: 13i32,
                powered: false,
            } => Ok(708u32),
            NoteBlock {
                instrument: NoteBlockInstrument::Snare,
                note: 14i32,
                powered: true,
            } => Ok(709u32),
            NoteBlock {
                instrument: NoteBlockInstrument::Snare,
                note: 14i32,
                powered: false,
            } => Ok(710u32),
            NoteBlock {
                instrument: NoteBlockInstrument::Snare,
                note: 15i32,
                powered: true,
            } => Ok(711u32),
            NoteBlock {
                instrument: NoteBlockInstrument::Snare,
                note: 15i32,
                powered: false,
            } => Ok(712u32),
            NoteBlock {
                instrument: NoteBlockInstrument::Snare,
                note: 16i32,
                powered: true,
            } => Ok(713u32),
            NoteBlock {
                instrument: NoteBlockInstrument::Snare,
                note: 16i32,
                powered: false,
            } => Ok(714u32),
            NoteBlock {
                instrument: NoteBlockInstrument::Snare,
                note: 17i32,
                powered: true,
            } => Ok(715u32),
            NoteBlock {
                instrument: NoteBlockInstrument::Snare,
                note: 17i32,
                powered: false,
            } => Ok(716u32),
            NoteBlock {
                instrument: NoteBlockInstrument::Snare,
                note: 18i32,
                powered: true,
            } => Ok(717u32),
            NoteBlock {
                instrument: NoteBlockInstrument::Snare,
                note: 18i32,
                powered: false,
            } => Ok(718u32),
            NoteBlock {
                instrument: NoteBlockInstrument::Snare,
                note: 19i32,
                powered: true,
            } => Ok(719u32),
            NoteBlock {
                instrument: NoteBlockInstrument::Snare,
                note: 19i32,
                powered: false,
            } => Ok(720u32),
            NoteBlock {
                instrument: NoteBlockInstrument::Snare,
                note: 20i32,
                powered: true,
            } => Ok(721u32),
            NoteBlock {
                instrument: NoteBlockInstrument::Snare,
                note: 20i32,
                powered: false,
            } => Ok(722u32),
            NoteBlock {
                instrument: NoteBlockInstrument::Snare,
                note: 21i32,
                powered: true,
            } => Ok(723u32),
            NoteBlock {
                instrument: NoteBlockInstrument::Snare,
                note: 21i32,
                powered: false,
            } => Ok(724u32),
            NoteBlock {
                instrument: NoteBlockInstrument::Snare,
                note: 22i32,
                powered: true,
            } => Ok(725u32),
            NoteBlock {
                instrument: NoteBlockInstrument::Snare,
                note: 22i32,
                powered: false,
            } => Ok(726u32),
            NoteBlock {
                instrument: NoteBlockInstrument::Snare,
                note: 23i32,
                powered: true,
            } => Ok(727u32),
            NoteBlock {
                instrument: NoteBlockInstrument::Snare,
                note: 23i32,
                powered: false,
            } => Ok(728u32),
            NoteBlock {
                instrument: NoteBlockInstrument::Snare,
                note: 24i32,
                powered: true,
            } => Ok(729u32),
            NoteBlock {
                instrument: NoteBlockInstrument::Snare,
                note: 24i32,
                powered: false,
            } => Ok(730u32),
            NoteBlock {
                instrument: NoteBlockInstrument::Hat,
                note: 0i32,
                powered: true,
            } => Ok(731u32),
            NoteBlock {
                instrument: NoteBlockInstrument::Hat,
                note: 0i32,
                powered: false,
            } => Ok(732u32),
            NoteBlock {
                instrument: NoteBlockInstrument::Hat,
                note: 1i32,
                powered: true,
            } => Ok(733u32),
            NoteBlock {
                instrument: NoteBlockInstrument::Hat,
                note: 1i32,
                powered: false,
            } => Ok(734u32),
            NoteBlock {
                instrument: NoteBlockInstrument::Hat,
                note: 2i32,
                powered: true,
            } => Ok(735u32),
            NoteBlock {
                instrument: NoteBlockInstrument::Hat,
                note: 2i32,
                powered: false,
            } => Ok(736u32),
            NoteBlock {
                instrument: NoteBlockInstrument::Hat,
                note: 3i32,
                powered: true,
            } => Ok(737u32),
            NoteBlock {
                instrument: NoteBlockInstrument::Hat,
                note: 3i32,
                powered: false,
            } => Ok(738u32),
            NoteBlock {
                instrument: NoteBlockInstrument::Hat,
                note: 4i32,
                powered: true,
            } => Ok(739u32),
            NoteBlock {
                instrument: NoteBlockInstrument::Hat,
                note: 4i32,
                powered: false,
            } => Ok(740u32),
            NoteBlock {
                instrument: NoteBlockInstrument::Hat,
                note: 5i32,
                powered: true,
            } => Ok(741u32),
            NoteBlock {
                instrument: NoteBlockInstrument::Hat,
                note: 5i32,
                powered: false,
            } => Ok(742u32),
            NoteBlock {
                instrument: NoteBlockInstrument::Hat,
                note: 6i32,
                powered: true,
            } => Ok(743u32),
            NoteBlock {
                instrument: NoteBlockInstrument::Hat,
                note: 6i32,
                powered: false,
            } => Ok(744u32),
            NoteBlock {
                instrument: NoteBlockInstrument::Hat,
                note: 7i32,
                powered: true,
            } => Ok(745u32),
            NoteBlock {
                instrument: NoteBlockInstrument::Hat,
                note: 7i32,
                powered: false,
            } => Ok(746u32),
            NoteBlock {
                instrument: NoteBlockInstrument::Hat,
                note: 8i32,
                powered: true,
            } => Ok(747u32),
            NoteBlock {
                instrument: NoteBlockInstrument::Hat,
                note: 8i32,
                powered: false,
            } => Ok(748u32),
            NoteBlock {
                instrument: NoteBlockInstrument::Hat,
                note: 9i32,
                powered: true,
            } => Ok(749u32),
            NoteBlock {
                instrument: NoteBlockInstrument::Hat,
                note: 9i32,
                powered: false,
            } => Ok(750u32),
            NoteBlock {
                instrument: NoteBlockInstrument::Hat,
                note: 10i32,
                powered: true,
            } => Ok(751u32),
            NoteBlock {
                instrument: NoteBlockInstrument::Hat,
                note: 10i32,
                powered: false,
            } => Ok(752u32),
            NoteBlock {
                instrument: NoteBlockInstrument::Hat,
                note: 11i32,
                powered: true,
            } => Ok(753u32),
            NoteBlock {
                instrument: NoteBlockInstrument::Hat,
                note: 11i32,
                powered: false,
            } => Ok(754u32),
            NoteBlock {
                instrument: NoteBlockInstrument::Hat,
                note: 12i32,
                powered: true,
            } => Ok(755u32),
            NoteBlock {
                instrument: NoteBlockInstrument::Hat,
                note: 12i32,
                powered: false,
            } => Ok(756u32),
            NoteBlock {
                instrument: NoteBlockInstrument::Hat,
                note: 13i32,
                powered: true,
            } => Ok(757u32),
            NoteBlock {
                instrument: NoteBlockInstrument::Hat,
                note: 13i32,
                powered: false,
            } => Ok(758u32),
            NoteBlock {
                instrument: NoteBlockInstrument::Hat,
                note: 14i32,
                powered: true,
            } => Ok(759u32),
            NoteBlock {
                instrument: NoteBlockInstrument::Hat,
                note: 14i32,
                powered: false,
            } => Ok(760u32),
            NoteBlock {
                instrument: NoteBlockInstrument::Hat,
                note: 15i32,
                powered: true,
            } => Ok(761u32),
            NoteBlock {
                instrument: NoteBlockInstrument::Hat,
                note: 15i32,
                powered: false,
            } => Ok(762u32),
            NoteBlock {
                instrument: NoteBlockInstrument::Hat,
                note: 16i32,
                powered: true,
            } => Ok(763u32),
            NoteBlock {
                instrument: NoteBlockInstrument::Hat,
                note: 16i32,
                powered: false,
            } => Ok(764u32),
            NoteBlock {
                instrument: NoteBlockInstrument::Hat,
                note: 17i32,
                powered: true,
            } => Ok(765u32),
            NoteBlock {
                instrument: NoteBlockInstrument::Hat,
                note: 17i32,
                powered: false,
            } => Ok(766u32),
            NoteBlock {
                instrument: NoteBlockInstrument::Hat,
                note: 18i32,
                powered: true,
            } => Ok(767u32),
            NoteBlock {
                instrument: NoteBlockInstrument::Hat,
                note: 18i32,
                powered: false,
            } => Ok(768u32),
            NoteBlock {
                instrument: NoteBlockInstrument::Hat,
                note: 19i32,
                powered: true,
            } => Ok(769u32),
            NoteBlock {
                instrument: NoteBlockInstrument::Hat,
                note: 19i32,
                powered: false,
            } => Ok(770u32),
            NoteBlock {
                instrument: NoteBlockInstrument::Hat,
                note: 20i32,
                powered: true,
            } => Ok(771u32),
            NoteBlock {
                instrument: NoteBlockInstrument::Hat,
                note: 20i32,
                powered: false,
            } => Ok(772u32),
            NoteBlock {
                instrument: NoteBlockInstrument::Hat,
                note: 21i32,
                powered: true,
            } => Ok(773u32),
            NoteBlock {
                instrument: NoteBlockInstrument::Hat,
                note: 21i32,
                powered: false,
            } => Ok(774u32),
            NoteBlock {
                instrument: NoteBlockInstrument::Hat,
                note: 22i32,
                powered: true,
            } => Ok(775u32),
            NoteBlock {
                instrument: NoteBlockInstrument::Hat,
                note: 22i32,
                powered: false,
            } => Ok(776u32),
            NoteBlock {
                instrument: NoteBlockInstrument::Hat,
                note: 23i32,
                powered: true,
            } => Ok(777u32),
            NoteBlock {
                instrument: NoteBlockInstrument::Hat,
                note: 23i32,
                powered: false,
            } => Ok(778u32),
            NoteBlock {
                instrument: NoteBlockInstrument::Hat,
                note: 24i32,
                powered: true,
            } => Ok(779u32),
            NoteBlock {
                instrument: NoteBlockInstrument::Hat,
                note: 24i32,
                powered: false,
            } => Ok(780u32),
            NoteBlock {
                instrument: NoteBlockInstrument::Bass,
                note: 0i32,
                powered: true,
            } => Ok(781u32),
            NoteBlock {
                instrument: NoteBlockInstrument::Bass,
                note: 0i32,
                powered: false,
            } => Ok(782u32),
            NoteBlock {
                instrument: NoteBlockInstrument::Bass,
                note: 1i32,
                powered: true,
            } => Ok(783u32),
            NoteBlock {
                instrument: NoteBlockInstrument::Bass,
                note: 1i32,
                powered: false,
            } => Ok(784u32),
            NoteBlock {
                instrument: NoteBlockInstrument::Bass,
                note: 2i32,
                powered: true,
            } => Ok(785u32),
            NoteBlock {
                instrument: NoteBlockInstrument::Bass,
                note: 2i32,
                powered: false,
            } => Ok(786u32),
            NoteBlock {
                instrument: NoteBlockInstrument::Bass,
                note: 3i32,
                powered: true,
            } => Ok(787u32),
            NoteBlock {
                instrument: NoteBlockInstrument::Bass,
                note: 3i32,
                powered: false,
            } => Ok(788u32),
            NoteBlock {
                instrument: NoteBlockInstrument::Bass,
                note: 4i32,
                powered: true,
            } => Ok(789u32),
            NoteBlock {
                instrument: NoteBlockInstrument::Bass,
                note: 4i32,
                powered: false,
            } => Ok(790u32),
            NoteBlock {
                instrument: NoteBlockInstrument::Bass,
                note: 5i32,
                powered: true,
            } => Ok(791u32),
            NoteBlock {
                instrument: NoteBlockInstrument::Bass,
                note: 5i32,
                powered: false,
            } => Ok(792u32),
            NoteBlock {
                instrument: NoteBlockInstrument::Bass,
                note: 6i32,
                powered: true,
            } => Ok(793u32),
            NoteBlock {
                instrument: NoteBlockInstrument::Bass,
                note: 6i32,
                powered: false,
            } => Ok(794u32),
            NoteBlock {
                instrument: NoteBlockInstrument::Bass,
                note: 7i32,
                powered: true,
            } => Ok(795u32),
            NoteBlock {
                instrument: NoteBlockInstrument::Bass,
                note: 7i32,
                powered: false,
            } => Ok(796u32),
            NoteBlock {
                instrument: NoteBlockInstrument::Bass,
                note: 8i32,
                powered: true,
            } => Ok(797u32),
            NoteBlock {
                instrument: NoteBlockInstrument::Bass,
                note: 8i32,
                powered: false,
            } => Ok(798u32),
            NoteBlock {
                instrument: NoteBlockInstrument::Bass,
                note: 9i32,
                powered: true,
            } => Ok(799u32),
            NoteBlock {
                instrument: NoteBlockInstrument::Bass,
                note: 9i32,
                powered: false,
            } => Ok(800u32),
            NoteBlock {
                instrument: NoteBlockInstrument::Bass,
                note: 10i32,
                powered: true,
            } => Ok(801u32),
            NoteBlock {
                instrument: NoteBlockInstrument::Bass,
                note: 10i32,
                powered: false,
            } => Ok(802u32),
            NoteBlock {
                instrument: NoteBlockInstrument::Bass,
                note: 11i32,
                powered: true,
            } => Ok(803u32),
            NoteBlock {
                instrument: NoteBlockInstrument::Bass,
                note: 11i32,
                powered: false,
            } => Ok(804u32),
            NoteBlock {
                instrument: NoteBlockInstrument::Bass,
                note: 12i32,
                powered: true,
            } => Ok(805u32),
            NoteBlock {
                instrument: NoteBlockInstrument::Bass,
                note: 12i32,
                powered: false,
            } => Ok(806u32),
            NoteBlock {
                instrument: NoteBlockInstrument::Bass,
                note: 13i32,
                powered: true,
            } => Ok(807u32),
            NoteBlock {
                instrument: NoteBlockInstrument::Bass,
                note: 13i32,
                powered: false,
            } => Ok(808u32),
            NoteBlock {
                instrument: NoteBlockInstrument::Bass,
                note: 14i32,
                powered: true,
            } => Ok(809u32),
            NoteBlock {
                instrument: NoteBlockInstrument::Bass,
                note: 14i32,
                powered: false,
            } => Ok(810u32),
            NoteBlock {
                instrument: NoteBlockInstrument::Bass,
                note: 15i32,
                powered: true,
            } => Ok(811u32),
            NoteBlock {
                instrument: NoteBlockInstrument::Bass,
                note: 15i32,
                powered: false,
            } => Ok(812u32),
            NoteBlock {
                instrument: NoteBlockInstrument::Bass,
                note: 16i32,
                powered: true,
            } => Ok(813u32),
            NoteBlock {
                instrument: NoteBlockInstrument::Bass,
                note: 16i32,
                powered: false,
            } => Ok(814u32),
            NoteBlock {
                instrument: NoteBlockInstrument::Bass,
                note: 17i32,
                powered: true,
            } => Ok(815u32),
            NoteBlock {
                instrument: NoteBlockInstrument::Bass,
                note: 17i32,
                powered: false,
            } => Ok(816u32),
            NoteBlock {
                instrument: NoteBlockInstrument::Bass,
                note: 18i32,
                powered: true,
            } => Ok(817u32),
            NoteBlock {
                instrument: NoteBlockInstrument::Bass,
                note: 18i32,
                powered: false,
            } => Ok(818u32),
            NoteBlock {
                instrument: NoteBlockInstrument::Bass,
                note: 19i32,
                powered: true,
            } => Ok(819u32),
            NoteBlock {
                instrument: NoteBlockInstrument::Bass,
                note: 19i32,
                powered: false,
            } => Ok(820u32),
            NoteBlock {
                instrument: NoteBlockInstrument::Bass,
                note: 20i32,
                powered: true,
            } => Ok(821u32),
            NoteBlock {
                instrument: NoteBlockInstrument::Bass,
                note: 20i32,
                powered: false,
            } => Ok(822u32),
            NoteBlock {
                instrument: NoteBlockInstrument::Bass,
                note: 21i32,
                powered: true,
            } => Ok(823u32),
            NoteBlock {
                instrument: NoteBlockInstrument::Bass,
                note: 21i32,
                powered: false,
            } => Ok(824u32),
            NoteBlock {
                instrument: NoteBlockInstrument::Bass,
                note: 22i32,
                powered: true,
            } => Ok(825u32),
            NoteBlock {
                instrument: NoteBlockInstrument::Bass,
                note: 22i32,
                powered: false,
            } => Ok(826u32),
            NoteBlock {
                instrument: NoteBlockInstrument::Bass,
                note: 23i32,
                powered: true,
            } => Ok(827u32),
            NoteBlock {
                instrument: NoteBlockInstrument::Bass,
                note: 23i32,
                powered: false,
            } => Ok(828u32),
            NoteBlock {
                instrument: NoteBlockInstrument::Bass,
                note: 24i32,
                powered: true,
            } => Ok(829u32),
            NoteBlock {
                instrument: NoteBlockInstrument::Bass,
                note: 24i32,
                powered: false,
            } => Ok(830u32),
            NoteBlock {
                instrument: NoteBlockInstrument::Flute,
                note: 0i32,
                powered: true,
            } => Ok(831u32),
            NoteBlock {
                instrument: NoteBlockInstrument::Flute,
                note: 0i32,
                powered: false,
            } => Ok(832u32),
            NoteBlock {
                instrument: NoteBlockInstrument::Flute,
                note: 1i32,
                powered: true,
            } => Ok(833u32),
            NoteBlock {
                instrument: NoteBlockInstrument::Flute,
                note: 1i32,
                powered: false,
            } => Ok(834u32),
            NoteBlock {
                instrument: NoteBlockInstrument::Flute,
                note: 2i32,
                powered: true,
            } => Ok(835u32),
            NoteBlock {
                instrument: NoteBlockInstrument::Flute,
                note: 2i32,
                powered: false,
            } => Ok(836u32),
            NoteBlock {
                instrument: NoteBlockInstrument::Flute,
                note: 3i32,
                powered: true,
            } => Ok(837u32),
            NoteBlock {
                instrument: NoteBlockInstrument::Flute,
                note: 3i32,
                powered: false,
            } => Ok(838u32),
            NoteBlock {
                instrument: NoteBlockInstrument::Flute,
                note: 4i32,
                powered: true,
            } => Ok(839u32),
            NoteBlock {
                instrument: NoteBlockInstrument::Flute,
                note: 4i32,
                powered: false,
            } => Ok(840u32),
            NoteBlock {
                instrument: NoteBlockInstrument::Flute,
                note: 5i32,
                powered: true,
            } => Ok(841u32),
            NoteBlock {
                instrument: NoteBlockInstrument::Flute,
                note: 5i32,
                powered: false,
            } => Ok(842u32),
            NoteBlock {
                instrument: NoteBlockInstrument::Flute,
                note: 6i32,
                powered: true,
            } => Ok(843u32),
            NoteBlock {
                instrument: NoteBlockInstrument::Flute,
                note: 6i32,
                powered: false,
            } => Ok(844u32),
            NoteBlock {
                instrument: NoteBlockInstrument::Flute,
                note: 7i32,
                powered: true,
            } => Ok(845u32),
            NoteBlock {
                instrument: NoteBlockInstrument::Flute,
                note: 7i32,
                powered: false,
            } => Ok(846u32),
            NoteBlock {
                instrument: NoteBlockInstrument::Flute,
                note: 8i32,
                powered: true,
            } => Ok(847u32),
            NoteBlock {
                instrument: NoteBlockInstrument::Flute,
                note: 8i32,
                powered: false,
            } => Ok(848u32),
            NoteBlock {
                instrument: NoteBlockInstrument::Flute,
                note: 9i32,
                powered: true,
            } => Ok(849u32),
            NoteBlock {
                instrument: NoteBlockInstrument::Flute,
                note: 9i32,
                powered: false,
            } => Ok(850u32),
            NoteBlock {
                instrument: NoteBlockInstrument::Flute,
                note: 10i32,
                powered: true,
            } => Ok(851u32),
            NoteBlock {
                instrument: NoteBlockInstrument::Flute,
                note: 10i32,
                powered: false,
            } => Ok(852u32),
            NoteBlock {
                instrument: NoteBlockInstrument::Flute,
                note: 11i32,
                powered: true,
            } => Ok(853u32),
            NoteBlock {
                instrument: NoteBlockInstrument::Flute,
                note: 11i32,
                powered: false,
            } => Ok(854u32),
            NoteBlock {
                instrument: NoteBlockInstrument::Flute,
                note: 12i32,
                powered: true,
            } => Ok(855u32),
            NoteBlock {
                instrument: NoteBlockInstrument::Flute,
                note: 12i32,
                powered: false,
            } => Ok(856u32),
            NoteBlock {
                instrument: NoteBlockInstrument::Flute,
                note: 13i32,
                powered: true,
            } => Ok(857u32),
            NoteBlock {
                instrument: NoteBlockInstrument::Flute,
                note: 13i32,
                powered: false,
            } => Ok(858u32),
            NoteBlock {
                instrument: NoteBlockInstrument::Flute,
                note: 14i32,
                powered: true,
            } => Ok(859u32),
            NoteBlock {
                instrument: NoteBlockInstrument::Flute,
                note: 14i32,
                powered: false,
            } => Ok(860u32),
            NoteBlock {
                instrument: NoteBlockInstrument::Flute,
                note: 15i32,
                powered: true,
            } => Ok(861u32),
            NoteBlock {
                instrument: NoteBlockInstrument::Flute,
                note: 15i32,
                powered: false,
            } => Ok(862u32),
            NoteBlock {
                instrument: NoteBlockInstrument::Flute,
                note: 16i32,
                powered: true,
            } => Ok(863u32),
            NoteBlock {
                instrument: NoteBlockInstrument::Flute,
                note: 16i32,
                powered: false,
            } => Ok(864u32),
            NoteBlock {
                instrument: NoteBlockInstrument::Flute,
                note: 17i32,
                powered: true,
            } => Ok(865u32),
            NoteBlock {
                instrument: NoteBlockInstrument::Flute,
                note: 17i32,
                powered: false,
            } => Ok(866u32),
            NoteBlock {
                instrument: NoteBlockInstrument::Flute,
                note: 18i32,
                powered: true,
            } => Ok(867u32),
            NoteBlock {
                instrument: NoteBlockInstrument::Flute,
                note: 18i32,
                powered: false,
            } => Ok(868u32),
            NoteBlock {
                instrument: NoteBlockInstrument::Flute,
                note: 19i32,
                powered: true,
            } => Ok(869u32),
            NoteBlock {
                instrument: NoteBlockInstrument::Flute,
                note: 19i32,
                powered: false,
            } => Ok(870u32),
            NoteBlock {
                instrument: NoteBlockInstrument::Flute,
                note: 20i32,
                powered: true,
            } => Ok(871u32),
            NoteBlock {
                instrument: NoteBlockInstrument::Flute,
                note: 20i32,
                powered: false,
            } => Ok(872u32),
            NoteBlock {
                instrument: NoteBlockInstrument::Flute,
                note: 21i32,
                powered: true,
            } => Ok(873u32),
            NoteBlock {
                instrument: NoteBlockInstrument::Flute,
                note: 21i32,
                powered: false,
            } => Ok(874u32),
            NoteBlock {
                instrument: NoteBlockInstrument::Flute,
                note: 22i32,
                powered: true,
            } => Ok(875u32),
            NoteBlock {
                instrument: NoteBlockInstrument::Flute,
                note: 22i32,
                powered: false,
            } => Ok(876u32),
            NoteBlock {
                instrument: NoteBlockInstrument::Flute,
                note: 23i32,
                powered: true,
            } => Ok(877u32),
            NoteBlock {
                instrument: NoteBlockInstrument::Flute,
                note: 23i32,
                powered: false,
            } => Ok(878u32),
            NoteBlock {
                instrument: NoteBlockInstrument::Flute,
                note: 24i32,
                powered: true,
            } => Ok(879u32),
            NoteBlock {
                instrument: NoteBlockInstrument::Flute,
                note: 24i32,
                powered: false,
            } => Ok(880u32),
            NoteBlock {
                instrument: NoteBlockInstrument::Bell,
                note: 0i32,
                powered: true,
            } => Ok(881u32),
            NoteBlock {
                instrument: NoteBlockInstrument::Bell,
                note: 0i32,
                powered: false,
            } => Ok(882u32),
            NoteBlock {
                instrument: NoteBlockInstrument::Bell,
                note: 1i32,
                powered: true,
            } => Ok(883u32),
            NoteBlock {
                instrument: NoteBlockInstrument::Bell,
                note: 1i32,
                powered: false,
            } => Ok(884u32),
            NoteBlock {
                instrument: NoteBlockInstrument::Bell,
                note: 2i32,
                powered: true,
            } => Ok(885u32),
            NoteBlock {
                instrument: NoteBlockInstrument::Bell,
                note: 2i32,
                powered: false,
            } => Ok(886u32),
            NoteBlock {
                instrument: NoteBlockInstrument::Bell,
                note: 3i32,
                powered: true,
            } => Ok(887u32),
            NoteBlock {
                instrument: NoteBlockInstrument::Bell,
                note: 3i32,
                powered: false,
            } => Ok(888u32),
            NoteBlock {
                instrument: NoteBlockInstrument::Bell,
                note: 4i32,
                powered: true,
            } => Ok(889u32),
            NoteBlock {
                instrument: NoteBlockInstrument::Bell,
                note: 4i32,
                powered: false,
            } => Ok(890u32),
            NoteBlock {
                instrument: NoteBlockInstrument::Bell,
                note: 5i32,
                powered: true,
            } => Ok(891u32),
            NoteBlock {
                instrument: NoteBlockInstrument::Bell,
                note: 5i32,
                powered: false,
            } => Ok(892u32),
            NoteBlock {
                instrument: NoteBlockInstrument::Bell,
                note: 6i32,
                powered: true,
            } => Ok(893u32),
            NoteBlock {
                instrument: NoteBlockInstrument::Bell,
                note: 6i32,
                powered: false,
            } => Ok(894u32),
            NoteBlock {
                instrument: NoteBlockInstrument::Bell,
                note: 7i32,
                powered: true,
            } => Ok(895u32),
            NoteBlock {
                instrument: NoteBlockInstrument::Bell,
                note: 7i32,
                powered: false,
            } => Ok(896u32),
            NoteBlock {
                instrument: NoteBlockInstrument::Bell,
                note: 8i32,
                powered: true,
            } => Ok(897u32),
            NoteBlock {
                instrument: NoteBlockInstrument::Bell,
                note: 8i32,
                powered: false,
            } => Ok(898u32),
            NoteBlock {
                instrument: NoteBlockInstrument::Bell,
                note: 9i32,
                powered: true,
            } => Ok(899u32),
            NoteBlock {
                instrument: NoteBlockInstrument::Bell,
                note: 9i32,
                powered: false,
            } => Ok(900u32),
            NoteBlock {
                instrument: NoteBlockInstrument::Bell,
                note: 10i32,
                powered: true,
            } => Ok(901u32),
            NoteBlock {
                instrument: NoteBlockInstrument::Bell,
                note: 10i32,
                powered: false,
            } => Ok(902u32),
            NoteBlock {
                instrument: NoteBlockInstrument::Bell,
                note: 11i32,
                powered: true,
            } => Ok(903u32),
            NoteBlock {
                instrument: NoteBlockInstrument::Bell,
                note: 11i32,
                powered: false,
            } => Ok(904u32),
            NoteBlock {
                instrument: NoteBlockInstrument::Bell,
                note: 12i32,
                powered: true,
            } => Ok(905u32),
            NoteBlock {
                instrument: NoteBlockInstrument::Bell,
                note: 12i32,
                powered: false,
            } => Ok(906u32),
            NoteBlock {
                instrument: NoteBlockInstrument::Bell,
                note: 13i32,
                powered: true,
            } => Ok(907u32),
            NoteBlock {
                instrument: NoteBlockInstrument::Bell,
                note: 13i32,
                powered: false,
            } => Ok(908u32),
            NoteBlock {
                instrument: NoteBlockInstrument::Bell,
                note: 14i32,
                powered: true,
            } => Ok(909u32),
            NoteBlock {
                instrument: NoteBlockInstrument::Bell,
                note: 14i32,
                powered: false,
            } => Ok(910u32),
            NoteBlock {
                instrument: NoteBlockInstrument::Bell,
                note: 15i32,
                powered: true,
            } => Ok(911u32),
            NoteBlock {
                instrument: NoteBlockInstrument::Bell,
                note: 15i32,
                powered: false,
            } => Ok(912u32),
            NoteBlock {
                instrument: NoteBlockInstrument::Bell,
                note: 16i32,
                powered: true,
            } => Ok(913u32),
            NoteBlock {
                instrument: NoteBlockInstrument::Bell,
                note: 16i32,
                powered: false,
            } => Ok(914u32),
            NoteBlock {
                instrument: NoteBlockInstrument::Bell,
                note: 17i32,
                powered: true,
            } => Ok(915u32),
            NoteBlock {
                instrument: NoteBlockInstrument::Bell,
                note: 17i32,
                powered: false,
            } => Ok(916u32),
            NoteBlock {
                instrument: NoteBlockInstrument::Bell,
                note: 18i32,
                powered: true,
            } => Ok(917u32),
            NoteBlock {
                instrument: NoteBlockInstrument::Bell,
                note: 18i32,
                powered: false,
            } => Ok(918u32),
            NoteBlock {
                instrument: NoteBlockInstrument::Bell,
                note: 19i32,
                powered: true,
            } => Ok(919u32),
            NoteBlock {
                instrument: NoteBlockInstrument::Bell,
                note: 19i32,
                powered: false,
            } => Ok(920u32),
            NoteBlock {
                instrument: NoteBlockInstrument::Bell,
                note: 20i32,
                powered: true,
            } => Ok(921u32),
            NoteBlock {
                instrument: NoteBlockInstrument::Bell,
                note: 20i32,
                powered: false,
            } => Ok(922u32),
            NoteBlock {
                instrument: NoteBlockInstrument::Bell,
                note: 21i32,
                powered: true,
            } => Ok(923u32),
            NoteBlock {
                instrument: NoteBlockInstrument::Bell,
                note: 21i32,
                powered: false,
            } => Ok(924u32),
            NoteBlock {
                instrument: NoteBlockInstrument::Bell,
                note: 22i32,
                powered: true,
            } => Ok(925u32),
            NoteBlock {
                instrument: NoteBlockInstrument::Bell,
                note: 22i32,
                powered: false,
            } => Ok(926u32),
            NoteBlock {
                instrument: NoteBlockInstrument::Bell,
                note: 23i32,
                powered: true,
            } => Ok(927u32),
            NoteBlock {
                instrument: NoteBlockInstrument::Bell,
                note: 23i32,
                powered: false,
            } => Ok(928u32),
            NoteBlock {
                instrument: NoteBlockInstrument::Bell,
                note: 24i32,
                powered: true,
            } => Ok(929u32),
            NoteBlock {
                instrument: NoteBlockInstrument::Bell,
                note: 24i32,
                powered: false,
            } => Ok(930u32),
            NoteBlock {
                instrument: NoteBlockInstrument::Guitar,
                note: 0i32,
                powered: true,
            } => Ok(931u32),
            NoteBlock {
                instrument: NoteBlockInstrument::Guitar,
                note: 0i32,
                powered: false,
            } => Ok(932u32),
            NoteBlock {
                instrument: NoteBlockInstrument::Guitar,
                note: 1i32,
                powered: true,
            } => Ok(933u32),
            NoteBlock {
                instrument: NoteBlockInstrument::Guitar,
                note: 1i32,
                powered: false,
            } => Ok(934u32),
            NoteBlock {
                instrument: NoteBlockInstrument::Guitar,
                note: 2i32,
                powered: true,
            } => Ok(935u32),
            NoteBlock {
                instrument: NoteBlockInstrument::Guitar,
                note: 2i32,
                powered: false,
            } => Ok(936u32),
            NoteBlock {
                instrument: NoteBlockInstrument::Guitar,
                note: 3i32,
                powered: true,
            } => Ok(937u32),
            NoteBlock {
                instrument: NoteBlockInstrument::Guitar,
                note: 3i32,
                powered: false,
            } => Ok(938u32),
            NoteBlock {
                instrument: NoteBlockInstrument::Guitar,
                note: 4i32,
                powered: true,
            } => Ok(939u32),
            NoteBlock {
                instrument: NoteBlockInstrument::Guitar,
                note: 4i32,
                powered: false,
            } => Ok(940u32),
            NoteBlock {
                instrument: NoteBlockInstrument::Guitar,
                note: 5i32,
                powered: true,
            } => Ok(941u32),
            NoteBlock {
                instrument: NoteBlockInstrument::Guitar,
                note: 5i32,
                powered: false,
            } => Ok(942u32),
            NoteBlock {
                instrument: NoteBlockInstrument::Guitar,
                note: 6i32,
                powered: true,
            } => Ok(943u32),
            NoteBlock {
                instrument: NoteBlockInstrument::Guitar,
                note: 6i32,
                powered: false,
            } => Ok(944u32),
            NoteBlock {
                instrument: NoteBlockInstrument::Guitar,
                note: 7i32,
                powered: true,
            } => Ok(945u32),
            NoteBlock {
                instrument: NoteBlockInstrument::Guitar,
                note: 7i32,
                powered: false,
            } => Ok(946u32),
            NoteBlock {
                instrument: NoteBlockInstrument::Guitar,
                note: 8i32,
                powered: true,
            } => Ok(947u32),
            NoteBlock {
                instrument: NoteBlockInstrument::Guitar,
                note: 8i32,
                powered: false,
            } => Ok(948u32),
            NoteBlock {
                instrument: NoteBlockInstrument::Guitar,
                note: 9i32,
                powered: true,
            } => Ok(949u32),
            NoteBlock {
                instrument: NoteBlockInstrument::Guitar,
                note: 9i32,
                powered: false,
            } => Ok(950u32),
            NoteBlock {
                instrument: NoteBlockInstrument::Guitar,
                note: 10i32,
                powered: true,
            } => Ok(951u32),
            NoteBlock {
                instrument: NoteBlockInstrument::Guitar,
                note: 10i32,
                powered: false,
            } => Ok(952u32),
            NoteBlock {
                instrument: NoteBlockInstrument::Guitar,
                note: 11i32,
                powered: true,
            } => Ok(953u32),
            NoteBlock {
                instrument: NoteBlockInstrument::Guitar,
                note: 11i32,
                powered: false,
            } => Ok(954u32),
            NoteBlock {
                instrument: NoteBlockInstrument::Guitar,
                note: 12i32,
                powered: true,
            } => Ok(955u32),
            NoteBlock {
                instrument: NoteBlockInstrument::Guitar,
                note: 12i32,
                powered: false,
            } => Ok(956u32),
            NoteBlock {
                instrument: NoteBlockInstrument::Guitar,
                note: 13i32,
                powered: true,
            } => Ok(957u32),
            NoteBlock {
                instrument: NoteBlockInstrument::Guitar,
                note: 13i32,
                powered: false,
            } => Ok(958u32),
            NoteBlock {
                instrument: NoteBlockInstrument::Guitar,
                note: 14i32,
                powered: true,
            } => Ok(959u32),
            NoteBlock {
                instrument: NoteBlockInstrument::Guitar,
                note: 14i32,
                powered: false,
            } => Ok(960u32),
            NoteBlock {
                instrument: NoteBlockInstrument::Guitar,
                note: 15i32,
                powered: true,
            } => Ok(961u32),
            NoteBlock {
                instrument: NoteBlockInstrument::Guitar,
                note: 15i32,
                powered: false,
            } => Ok(962u32),
            NoteBlock {
                instrument: NoteBlockInstrument::Guitar,
                note: 16i32,
                powered: true,
            } => Ok(963u32),
            NoteBlock {
                instrument: NoteBlockInstrument::Guitar,
                note: 16i32,
                powered: false,
            } => Ok(964u32),
            NoteBlock {
                instrument: NoteBlockInstrument::Guitar,
                note: 17i32,
                powered: true,
            } => Ok(965u32),
            NoteBlock {
                instrument: NoteBlockInstrument::Guitar,
                note: 17i32,
                powered: false,
            } => Ok(966u32),
            NoteBlock {
                instrument: NoteBlockInstrument::Guitar,
                note: 18i32,
                powered: true,
            } => Ok(967u32),
            NoteBlock {
                instrument: NoteBlockInstrument::Guitar,
                note: 18i32,
                powered: false,
            } => Ok(968u32),
            NoteBlock {
                instrument: NoteBlockInstrument::Guitar,
                note: 19i32,
                powered: true,
            } => Ok(969u32),
            NoteBlock {
                instrument: NoteBlockInstrument::Guitar,
                note: 19i32,
                powered: false,
            } => Ok(970u32),
            NoteBlock {
                instrument: NoteBlockInstrument::Guitar,
                note: 20i32,
                powered: true,
            } => Ok(971u32),
            NoteBlock {
                instrument: NoteBlockInstrument::Guitar,
                note: 20i32,
                powered: false,
            } => Ok(972u32),
            NoteBlock {
                instrument: NoteBlockInstrument::Guitar,
                note: 21i32,
                powered: true,
            } => Ok(973u32),
            NoteBlock {
                instrument: NoteBlockInstrument::Guitar,
                note: 21i32,
                powered: false,
            } => Ok(974u32),
            NoteBlock {
                instrument: NoteBlockInstrument::Guitar,
                note: 22i32,
                powered: true,
            } => Ok(975u32),
            NoteBlock {
                instrument: NoteBlockInstrument::Guitar,
                note: 22i32,
                powered: false,
            } => Ok(976u32),
            NoteBlock {
                instrument: NoteBlockInstrument::Guitar,
                note: 23i32,
                powered: true,
            } => Ok(977u32),
            NoteBlock {
                instrument: NoteBlockInstrument::Guitar,
                note: 23i32,
                powered: false,
            } => Ok(978u32),
            NoteBlock {
                instrument: NoteBlockInstrument::Guitar,
                note: 24i32,
                powered: true,
            } => Ok(979u32),
            NoteBlock {
                instrument: NoteBlockInstrument::Guitar,
                note: 24i32,
                powered: false,
            } => Ok(980u32),
            NoteBlock {
                instrument: NoteBlockInstrument::Chime,
                note: 0i32,
                powered: true,
            } => Ok(981u32),
            NoteBlock {
                instrument: NoteBlockInstrument::Chime,
                note: 0i32,
                powered: false,
            } => Ok(982u32),
            NoteBlock {
                instrument: NoteBlockInstrument::Chime,
                note: 1i32,
                powered: true,
            } => Ok(983u32),
            NoteBlock {
                instrument: NoteBlockInstrument::Chime,
                note: 1i32,
                powered: false,
            } => Ok(984u32),
            NoteBlock {
                instrument: NoteBlockInstrument::Chime,
                note: 2i32,
                powered: true,
            } => Ok(985u32),
            NoteBlock {
                instrument: NoteBlockInstrument::Chime,
                note: 2i32,
                powered: false,
            } => Ok(986u32),
            NoteBlock {
                instrument: NoteBlockInstrument::Chime,
                note: 3i32,
                powered: true,
            } => Ok(987u32),
            NoteBlock {
                instrument: NoteBlockInstrument::Chime,
                note: 3i32,
                powered: false,
            } => Ok(988u32),
            NoteBlock {
                instrument: NoteBlockInstrument::Chime,
                note: 4i32,
                powered: true,
            } => Ok(989u32),
            NoteBlock {
                instrument: NoteBlockInstrument::Chime,
                note: 4i32,
                powered: false,
            } => Ok(990u32),
            NoteBlock {
                instrument: NoteBlockInstrument::Chime,
                note: 5i32,
                powered: true,
            } => Ok(991u32),
            NoteBlock {
                instrument: NoteBlockInstrument::Chime,
                note: 5i32,
                powered: false,
            } => Ok(992u32),
            NoteBlock {
                instrument: NoteBlockInstrument::Chime,
                note: 6i32,
                powered: true,
            } => Ok(993u32),
            NoteBlock {
                instrument: NoteBlockInstrument::Chime,
                note: 6i32,
                powered: false,
            } => Ok(994u32),
            NoteBlock {
                instrument: NoteBlockInstrument::Chime,
                note: 7i32,
                powered: true,
            } => Ok(995u32),
            NoteBlock {
                instrument: NoteBlockInstrument::Chime,
                note: 7i32,
                powered: false,
            } => Ok(996u32),
            NoteBlock {
                instrument: NoteBlockInstrument::Chime,
                note: 8i32,
                powered: true,
            } => Ok(997u32),
            NoteBlock {
                instrument: NoteBlockInstrument::Chime,
                note: 8i32,
                powered: false,
            } => Ok(998u32),
            NoteBlock {
                instrument: NoteBlockInstrument::Chime,
                note: 9i32,
                powered: true,
            } => Ok(999u32),
            NoteBlock {
                instrument: NoteBlockInstrument::Chime,
                note: 9i32,
                powered: false,
            } => Ok(1000u32),
            NoteBlock {
                instrument: NoteBlockInstrument::Chime,
                note: 10i32,
                powered: true,
            } => Ok(1001u32),
            NoteBlock {
                instrument: NoteBlockInstrument::Chime,
                note: 10i32,
                powered: false,
            } => Ok(1002u32),
            NoteBlock {
                instrument: NoteBlockInstrument::Chime,
                note: 11i32,
                powered: true,
            } => Ok(1003u32),
            NoteBlock {
                instrument: NoteBlockInstrument::Chime,
                note: 11i32,
                powered: false,
            } => Ok(1004u32),
            NoteBlock {
                instrument: NoteBlockInstrument::Chime,
                note: 12i32,
                powered: true,
            } => Ok(1005u32),
            NoteBlock {
                instrument: NoteBlockInstrument::Chime,
                note: 12i32,
                powered: false,
            } => Ok(1006u32),
            NoteBlock {
                instrument: NoteBlockInstrument::Chime,
                note: 13i32,
                powered: true,
            } => Ok(1007u32),
            NoteBlock {
                instrument: NoteBlockInstrument::Chime,
                note: 13i32,
                powered: false,
            } => Ok(1008u32),
            NoteBlock {
                instrument: NoteBlockInstrument::Chime,
                note: 14i32,
                powered: true,
            } => Ok(1009u32),
            NoteBlock {
                instrument: NoteBlockInstrument::Chime,
                note: 14i32,
                powered: false,
            } => Ok(1010u32),
            NoteBlock {
                instrument: NoteBlockInstrument::Chime,
                note: 15i32,
                powered: true,
            } => Ok(1011u32),
            NoteBlock {
                instrument: NoteBlockInstrument::Chime,
                note: 15i32,
                powered: false,
            } => Ok(1012u32),
            NoteBlock {
                instrument: NoteBlockInstrument::Chime,
                note: 16i32,
                powered: true,
            } => Ok(1013u32),
            NoteBlock {
                instrument: NoteBlockInstrument::Chime,
                note: 16i32,
                powered: false,
            } => Ok(1014u32),
            NoteBlock {
                instrument: NoteBlockInstrument::Chime,
                note: 17i32,
                powered: true,
            } => Ok(1015u32),
            NoteBlock {
                instrument: NoteBlockInstrument::Chime,
                note: 17i32,
                powered: false,
            } => Ok(1016u32),
            NoteBlock {
                instrument: NoteBlockInstrument::Chime,
                note: 18i32,
                powered: true,
            } => Ok(1017u32),
            NoteBlock {
                instrument: NoteBlockInstrument::Chime,
                note: 18i32,
                powered: false,
            } => Ok(1018u32),
            NoteBlock {
                instrument: NoteBlockInstrument::Chime,
                note: 19i32,
                powered: true,
            } => Ok(1019u32),
            NoteBlock {
                instrument: NoteBlockInstrument::Chime,
                note: 19i32,
                powered: false,
            } => Ok(1020u32),
            NoteBlock {
                instrument: NoteBlockInstrument::Chime,
                note: 20i32,
                powered: true,
            } => Ok(1021u32),
            NoteBlock {
                instrument: NoteBlockInstrument::Chime,
                note: 20i32,
                powered: false,
            } => Ok(1022u32),
            NoteBlock {
                instrument: NoteBlockInstrument::Chime,
                note: 21i32,
                powered: true,
            } => Ok(1023u32),
            NoteBlock {
                instrument: NoteBlockInstrument::Chime,
                note: 21i32,
                powered: false,
            } => Ok(1024u32),
            NoteBlock {
                instrument: NoteBlockInstrument::Chime,
                note: 22i32,
                powered: true,
            } => Ok(1025u32),
            NoteBlock {
                instrument: NoteBlockInstrument::Chime,
                note: 22i32,
                powered: false,
            } => Ok(1026u32),
            NoteBlock {
                instrument: NoteBlockInstrument::Chime,
                note: 23i32,
                powered: true,
            } => Ok(1027u32),
            NoteBlock {
                instrument: NoteBlockInstrument::Chime,
                note: 23i32,
                powered: false,
            } => Ok(1028u32),
            NoteBlock {
                instrument: NoteBlockInstrument::Chime,
                note: 24i32,
                powered: true,
            } => Ok(1029u32),
            NoteBlock {
                instrument: NoteBlockInstrument::Chime,
                note: 24i32,
                powered: false,
            } => Ok(1030u32),
            NoteBlock {
                instrument: NoteBlockInstrument::Xylophone,
                note: 0i32,
                powered: true,
            } => Ok(1031u32),
            NoteBlock {
                instrument: NoteBlockInstrument::Xylophone,
                note: 0i32,
                powered: false,
            } => Ok(1032u32),
            NoteBlock {
                instrument: NoteBlockInstrument::Xylophone,
                note: 1i32,
                powered: true,
            } => Ok(1033u32),
            NoteBlock {
                instrument: NoteBlockInstrument::Xylophone,
                note: 1i32,
                powered: false,
            } => Ok(1034u32),
            NoteBlock {
                instrument: NoteBlockInstrument::Xylophone,
                note: 2i32,
                powered: true,
            } => Ok(1035u32),
            NoteBlock {
                instrument: NoteBlockInstrument::Xylophone,
                note: 2i32,
                powered: false,
            } => Ok(1036u32),
            NoteBlock {
                instrument: NoteBlockInstrument::Xylophone,
                note: 3i32,
                powered: true,
            } => Ok(1037u32),
            NoteBlock {
                instrument: NoteBlockInstrument::Xylophone,
                note: 3i32,
                powered: false,
            } => Ok(1038u32),
            NoteBlock {
                instrument: NoteBlockInstrument::Xylophone,
                note: 4i32,
                powered: true,
            } => Ok(1039u32),
            NoteBlock {
                instrument: NoteBlockInstrument::Xylophone,
                note: 4i32,
                powered: false,
            } => Ok(1040u32),
            NoteBlock {
                instrument: NoteBlockInstrument::Xylophone,
                note: 5i32,
                powered: true,
            } => Ok(1041u32),
            NoteBlock {
                instrument: NoteBlockInstrument::Xylophone,
                note: 5i32,
                powered: false,
            } => Ok(1042u32),
            NoteBlock {
                instrument: NoteBlockInstrument::Xylophone,
                note: 6i32,
                powered: true,
            } => Ok(1043u32),
            NoteBlock {
                instrument: NoteBlockInstrument::Xylophone,
                note: 6i32,
                powered: false,
            } => Ok(1044u32),
            NoteBlock {
                instrument: NoteBlockInstrument::Xylophone,
                note: 7i32,
                powered: true,
            } => Ok(1045u32),
            NoteBlock {
                instrument: NoteBlockInstrument::Xylophone,
                note: 7i32,
                powered: false,
            } => Ok(1046u32),
            NoteBlock {
                instrument: NoteBlockInstrument::Xylophone,
                note: 8i32,
                powered: true,
            } => Ok(1047u32),
            NoteBlock {
                instrument: NoteBlockInstrument::Xylophone,
                note: 8i32,
                powered: false,
            } => Ok(1048u32),
            NoteBlock {
                instrument: NoteBlockInstrument::Xylophone,
                note: 9i32,
                powered: true,
            } => Ok(1049u32),
            NoteBlock {
                instrument: NoteBlockInstrument::Xylophone,
                note: 9i32,
                powered: false,
            } => Ok(1050u32),
            NoteBlock {
                instrument: NoteBlockInstrument::Xylophone,
                note: 10i32,
                powered: true,
            } => Ok(1051u32),
            NoteBlock {
                instrument: NoteBlockInstrument::Xylophone,
                note: 10i32,
                powered: false,
            } => Ok(1052u32),
            NoteBlock {
                instrument: NoteBlockInstrument::Xylophone,
                note: 11i32,
                powered: true,
            } => Ok(1053u32),
            NoteBlock {
                instrument: NoteBlockInstrument::Xylophone,
                note: 11i32,
                powered: false,
            } => Ok(1054u32),
            NoteBlock {
                instrument: NoteBlockInstrument::Xylophone,
                note: 12i32,
                powered: true,
            } => Ok(1055u32),
            NoteBlock {
                instrument: NoteBlockInstrument::Xylophone,
                note: 12i32,
                powered: false,
            } => Ok(1056u32),
            NoteBlock {
                instrument: NoteBlockInstrument::Xylophone,
                note: 13i32,
                powered: true,
            } => Ok(1057u32),
            NoteBlock {
                instrument: NoteBlockInstrument::Xylophone,
                note: 13i32,
                powered: false,
            } => Ok(1058u32),
            NoteBlock {
                instrument: NoteBlockInstrument::Xylophone,
                note: 14i32,
                powered: true,
            } => Ok(1059u32),
            NoteBlock {
                instrument: NoteBlockInstrument::Xylophone,
                note: 14i32,
                powered: false,
            } => Ok(1060u32),
            NoteBlock {
                instrument: NoteBlockInstrument::Xylophone,
                note: 15i32,
                powered: true,
            } => Ok(1061u32),
            NoteBlock {
                instrument: NoteBlockInstrument::Xylophone,
                note: 15i32,
                powered: false,
            } => Ok(1062u32),
            NoteBlock {
                instrument: NoteBlockInstrument::Xylophone,
                note: 16i32,
                powered: true,
            } => Ok(1063u32),
            NoteBlock {
                instrument: NoteBlockInstrument::Xylophone,
                note: 16i32,
                powered: false,
            } => Ok(1064u32),
            NoteBlock {
                instrument: NoteBlockInstrument::Xylophone,
                note: 17i32,
                powered: true,
            } => Ok(1065u32),
            NoteBlock {
                instrument: NoteBlockInstrument::Xylophone,
                note: 17i32,
                powered: false,
            } => Ok(1066u32),
            NoteBlock {
                instrument: NoteBlockInstrument::Xylophone,
                note: 18i32,
                powered: true,
            } => Ok(1067u32),
            NoteBlock {
                instrument: NoteBlockInstrument::Xylophone,
                note: 18i32,
                powered: false,
            } => Ok(1068u32),
            NoteBlock {
                instrument: NoteBlockInstrument::Xylophone,
                note: 19i32,
                powered: true,
            } => Ok(1069u32),
            NoteBlock {
                instrument: NoteBlockInstrument::Xylophone,
                note: 19i32,
                powered: false,
            } => Ok(1070u32),
            NoteBlock {
                instrument: NoteBlockInstrument::Xylophone,
                note: 20i32,
                powered: true,
            } => Ok(1071u32),
            NoteBlock {
                instrument: NoteBlockInstrument::Xylophone,
                note: 20i32,
                powered: false,
            } => Ok(1072u32),
            NoteBlock {
                instrument: NoteBlockInstrument::Xylophone,
                note: 21i32,
                powered: true,
            } => Ok(1073u32),
            NoteBlock {
                instrument: NoteBlockInstrument::Xylophone,
                note: 21i32,
                powered: false,
            } => Ok(1074u32),
            NoteBlock {
                instrument: NoteBlockInstrument::Xylophone,
                note: 22i32,
                powered: true,
            } => Ok(1075u32),
            NoteBlock {
                instrument: NoteBlockInstrument::Xylophone,
                note: 22i32,
                powered: false,
            } => Ok(1076u32),
            NoteBlock {
                instrument: NoteBlockInstrument::Xylophone,
                note: 23i32,
                powered: true,
            } => Ok(1077u32),
            NoteBlock {
                instrument: NoteBlockInstrument::Xylophone,
                note: 23i32,
                powered: false,
            } => Ok(1078u32),
            NoteBlock {
                instrument: NoteBlockInstrument::Xylophone,
                note: 24i32,
                powered: true,
            } => Ok(1079u32),
            NoteBlock {
                instrument: NoteBlockInstrument::Xylophone,
                note: 24i32,
                powered: false,
            } => Ok(1080u32),
            NoteBlock {
                instrument: NoteBlockInstrument::IronXylophone,
                note: 0i32,
                powered: true,
            } => Ok(1081u32),
            NoteBlock {
                instrument: NoteBlockInstrument::IronXylophone,
                note: 0i32,
                powered: false,
            } => Ok(1082u32),
            NoteBlock {
                instrument: NoteBlockInstrument::IronXylophone,
                note: 1i32,
                powered: true,
            } => Ok(1083u32),
            NoteBlock {
                instrument: NoteBlockInstrument::IronXylophone,
                note: 1i32,
                powered: false,
            } => Ok(1084u32),
            NoteBlock {
                instrument: NoteBlockInstrument::IronXylophone,
                note: 2i32,
                powered: true,
            } => Ok(1085u32),
            NoteBlock {
                instrument: NoteBlockInstrument::IronXylophone,
                note: 2i32,
                powered: false,
            } => Ok(1086u32),
            NoteBlock {
                instrument: NoteBlockInstrument::IronXylophone,
                note: 3i32,
                powered: true,
            } => Ok(1087u32),
            NoteBlock {
                instrument: NoteBlockInstrument::IronXylophone,
                note: 3i32,
                powered: false,
            } => Ok(1088u32),
            NoteBlock {
                instrument: NoteBlockInstrument::IronXylophone,
                note: 4i32,
                powered: true,
            } => Ok(1089u32),
            NoteBlock {
                instrument: NoteBlockInstrument::IronXylophone,
                note: 4i32,
                powered: false,
            } => Ok(1090u32),
            NoteBlock {
                instrument: NoteBlockInstrument::IronXylophone,
                note: 5i32,
                powered: true,
            } => Ok(1091u32),
            NoteBlock {
                instrument: NoteBlockInstrument::IronXylophone,
                note: 5i32,
                powered: false,
            } => Ok(1092u32),
            NoteBlock {
                instrument: NoteBlockInstrument::IronXylophone,
                note: 6i32,
                powered: true,
            } => Ok(1093u32),
            NoteBlock {
                instrument: NoteBlockInstrument::IronXylophone,
                note: 6i32,
                powered: false,
            } => Ok(1094u32),
            NoteBlock {
                instrument: NoteBlockInstrument::IronXylophone,
                note: 7i32,
                powered: true,
            } => Ok(1095u32),
            NoteBlock {
                instrument: NoteBlockInstrument::IronXylophone,
                note: 7i32,
                powered: false,
            } => Ok(1096u32),
            NoteBlock {
                instrument: NoteBlockInstrument::IronXylophone,
                note: 8i32,
                powered: true,
            } => Ok(1097u32),
            NoteBlock {
                instrument: NoteBlockInstrument::IronXylophone,
                note: 8i32,
                powered: false,
            } => Ok(1098u32),
            NoteBlock {
                instrument: NoteBlockInstrument::IronXylophone,
                note: 9i32,
                powered: true,
            } => Ok(1099u32),
            NoteBlock {
                instrument: NoteBlockInstrument::IronXylophone,
                note: 9i32,
                powered: false,
            } => Ok(1100u32),
            NoteBlock {
                instrument: NoteBlockInstrument::IronXylophone,
                note: 10i32,
                powered: true,
            } => Ok(1101u32),
            NoteBlock {
                instrument: NoteBlockInstrument::IronXylophone,
                note: 10i32,
                powered: false,
            } => Ok(1102u32),
            NoteBlock {
                instrument: NoteBlockInstrument::IronXylophone,
                note: 11i32,
                powered: true,
            } => Ok(1103u32),
            NoteBlock {
                instrument: NoteBlockInstrument::IronXylophone,
                note: 11i32,
                powered: false,
            } => Ok(1104u32),
            NoteBlock {
                instrument: NoteBlockInstrument::IronXylophone,
                note: 12i32,
                powered: true,
            } => Ok(1105u32),
            NoteBlock {
                instrument: NoteBlockInstrument::IronXylophone,
                note: 12i32,
                powered: false,
            } => Ok(1106u32),
            NoteBlock {
                instrument: NoteBlockInstrument::IronXylophone,
                note: 13i32,
                powered: true,
            } => Ok(1107u32),
            NoteBlock {
                instrument: NoteBlockInstrument::IronXylophone,
                note: 13i32,
                powered: false,
            } => Ok(1108u32),
            NoteBlock {
                instrument: NoteBlockInstrument::IronXylophone,
                note: 14i32,
                powered: true,
            } => Ok(1109u32),
            NoteBlock {
                instrument: NoteBlockInstrument::IronXylophone,
                note: 14i32,
                powered: false,
            } => Ok(1110u32),
            NoteBlock {
                instrument: NoteBlockInstrument::IronXylophone,
                note: 15i32,
                powered: true,
            } => Ok(1111u32),
            NoteBlock {
                instrument: NoteBlockInstrument::IronXylophone,
                note: 15i32,
                powered: false,
            } => Ok(1112u32),
            NoteBlock {
                instrument: NoteBlockInstrument::IronXylophone,
                note: 16i32,
                powered: true,
            } => Ok(1113u32),
            NoteBlock {
                instrument: NoteBlockInstrument::IronXylophone,
                note: 16i32,
                powered: false,
            } => Ok(1114u32),
            NoteBlock {
                instrument: NoteBlockInstrument::IronXylophone,
                note: 17i32,
                powered: true,
            } => Ok(1115u32),
            NoteBlock {
                instrument: NoteBlockInstrument::IronXylophone,
                note: 17i32,
                powered: false,
            } => Ok(1116u32),
            NoteBlock {
                instrument: NoteBlockInstrument::IronXylophone,
                note: 18i32,
                powered: true,
            } => Ok(1117u32),
            NoteBlock {
                instrument: NoteBlockInstrument::IronXylophone,
                note: 18i32,
                powered: false,
            } => Ok(1118u32),
            NoteBlock {
                instrument: NoteBlockInstrument::IronXylophone,
                note: 19i32,
                powered: true,
            } => Ok(1119u32),
            NoteBlock {
                instrument: NoteBlockInstrument::IronXylophone,
                note: 19i32,
                powered: false,
            } => Ok(1120u32),
            NoteBlock {
                instrument: NoteBlockInstrument::IronXylophone,
                note: 20i32,
                powered: true,
            } => Ok(1121u32),
            NoteBlock {
                instrument: NoteBlockInstrument::IronXylophone,
                note: 20i32,
                powered: false,
            } => Ok(1122u32),
            NoteBlock {
                instrument: NoteBlockInstrument::IronXylophone,
                note: 21i32,
                powered: true,
            } => Ok(1123u32),
            NoteBlock {
                instrument: NoteBlockInstrument::IronXylophone,
                note: 21i32,
                powered: false,
            } => Ok(1124u32),
            NoteBlock {
                instrument: NoteBlockInstrument::IronXylophone,
                note: 22i32,
                powered: true,
            } => Ok(1125u32),
            NoteBlock {
                instrument: NoteBlockInstrument::IronXylophone,
                note: 22i32,
                powered: false,
            } => Ok(1126u32),
            NoteBlock {
                instrument: NoteBlockInstrument::IronXylophone,
                note: 23i32,
                powered: true,
            } => Ok(1127u32),
            NoteBlock {
                instrument: NoteBlockInstrument::IronXylophone,
                note: 23i32,
                powered: false,
            } => Ok(1128u32),
            NoteBlock {
                instrument: NoteBlockInstrument::IronXylophone,
                note: 24i32,
                powered: true,
            } => Ok(1129u32),
            NoteBlock {
                instrument: NoteBlockInstrument::IronXylophone,
                note: 24i32,
                powered: false,
            } => Ok(1130u32),
            NoteBlock {
                instrument: NoteBlockInstrument::CowBell,
                note: 0i32,
                powered: true,
            } => Ok(1131u32),
            NoteBlock {
                instrument: NoteBlockInstrument::CowBell,
                note: 0i32,
                powered: false,
            } => Ok(1132u32),
            NoteBlock {
                instrument: NoteBlockInstrument::CowBell,
                note: 1i32,
                powered: true,
            } => Ok(1133u32),
            NoteBlock {
                instrument: NoteBlockInstrument::CowBell,
                note: 1i32,
                powered: false,
            } => Ok(1134u32),
            NoteBlock {
                instrument: NoteBlockInstrument::CowBell,
                note: 2i32,
                powered: true,
            } => Ok(1135u32),
            NoteBlock {
                instrument: NoteBlockInstrument::CowBell,
                note: 2i32,
                powered: false,
            } => Ok(1136u32),
            NoteBlock {
                instrument: NoteBlockInstrument::CowBell,
                note: 3i32,
                powered: true,
            } => Ok(1137u32),
            NoteBlock {
                instrument: NoteBlockInstrument::CowBell,
                note: 3i32,
                powered: false,
            } => Ok(1138u32),
            NoteBlock {
                instrument: NoteBlockInstrument::CowBell,
                note: 4i32,
                powered: true,
            } => Ok(1139u32),
            NoteBlock {
                instrument: NoteBlockInstrument::CowBell,
                note: 4i32,
                powered: false,
            } => Ok(1140u32),
            NoteBlock {
                instrument: NoteBlockInstrument::CowBell,
                note: 5i32,
                powered: true,
            } => Ok(1141u32),
            NoteBlock {
                instrument: NoteBlockInstrument::CowBell,
                note: 5i32,
                powered: false,
            } => Ok(1142u32),
            NoteBlock {
                instrument: NoteBlockInstrument::CowBell,
                note: 6i32,
                powered: true,
            } => Ok(1143u32),
            NoteBlock {
                instrument: NoteBlockInstrument::CowBell,
                note: 6i32,
                powered: false,
            } => Ok(1144u32),
            NoteBlock {
                instrument: NoteBlockInstrument::CowBell,
                note: 7i32,
                powered: true,
            } => Ok(1145u32),
            NoteBlock {
                instrument: NoteBlockInstrument::CowBell,
                note: 7i32,
                powered: false,
            } => Ok(1146u32),
            NoteBlock {
                instrument: NoteBlockInstrument::CowBell,
                note: 8i32,
                powered: true,
            } => Ok(1147u32),
            NoteBlock {
                instrument: NoteBlockInstrument::CowBell,
                note: 8i32,
                powered: false,
            } => Ok(1148u32),
            NoteBlock {
                instrument: NoteBlockInstrument::CowBell,
                note: 9i32,
                powered: true,
            } => Ok(1149u32),
            NoteBlock {
                instrument: NoteBlockInstrument::CowBell,
                note: 9i32,
                powered: false,
            } => Ok(1150u32),
            NoteBlock {
                instrument: NoteBlockInstrument::CowBell,
                note: 10i32,
                powered: true,
            } => Ok(1151u32),
            NoteBlock {
                instrument: NoteBlockInstrument::CowBell,
                note: 10i32,
                powered: false,
            } => Ok(1152u32),
            NoteBlock {
                instrument: NoteBlockInstrument::CowBell,
                note: 11i32,
                powered: true,
            } => Ok(1153u32),
            NoteBlock {
                instrument: NoteBlockInstrument::CowBell,
                note: 11i32,
                powered: false,
            } => Ok(1154u32),
            NoteBlock {
                instrument: NoteBlockInstrument::CowBell,
                note: 12i32,
                powered: true,
            } => Ok(1155u32),
            NoteBlock {
                instrument: NoteBlockInstrument::CowBell,
                note: 12i32,
                powered: false,
            } => Ok(1156u32),
            NoteBlock {
                instrument: NoteBlockInstrument::CowBell,
                note: 13i32,
                powered: true,
            } => Ok(1157u32),
            NoteBlock {
                instrument: NoteBlockInstrument::CowBell,
                note: 13i32,
                powered: false,
            } => Ok(1158u32),
            NoteBlock {
                instrument: NoteBlockInstrument::CowBell,
                note: 14i32,
                powered: true,
            } => Ok(1159u32),
            NoteBlock {
                instrument: NoteBlockInstrument::CowBell,
                note: 14i32,
                powered: false,
            } => Ok(1160u32),
            NoteBlock {
                instrument: NoteBlockInstrument::CowBell,
                note: 15i32,
                powered: true,
            } => Ok(1161u32),
            NoteBlock {
                instrument: NoteBlockInstrument::CowBell,
                note: 15i32,
                powered: false,
            } => Ok(1162u32),
            NoteBlock {
                instrument: NoteBlockInstrument::CowBell,
                note: 16i32,
                powered: true,
            } => Ok(1163u32),
            NoteBlock {
                instrument: NoteBlockInstrument::CowBell,
                note: 16i32,
                powered: false,
            } => Ok(1164u32),
            NoteBlock {
                instrument: NoteBlockInstrument::CowBell,
                note: 17i32,
                powered: true,
            } => Ok(1165u32),
            NoteBlock {
                instrument: NoteBlockInstrument::CowBell,
                note: 17i32,
                powered: false,
            } => Ok(1166u32),
            NoteBlock {
                instrument: NoteBlockInstrument::CowBell,
                note: 18i32,
                powered: true,
            } => Ok(1167u32),
            NoteBlock {
                instrument: NoteBlockInstrument::CowBell,
                note: 18i32,
                powered: false,
            } => Ok(1168u32),
            NoteBlock {
                instrument: NoteBlockInstrument::CowBell,
                note: 19i32,
                powered: true,
            } => Ok(1169u32),
            NoteBlock {
                instrument: NoteBlockInstrument::CowBell,
                note: 19i32,
                powered: false,
            } => Ok(1170u32),
            NoteBlock {
                instrument: NoteBlockInstrument::CowBell,
                note: 20i32,
                powered: true,
            } => Ok(1171u32),
            NoteBlock {
                instrument: NoteBlockInstrument::CowBell,
                note: 20i32,
                powered: false,
            } => Ok(1172u32),
            NoteBlock {
                instrument: NoteBlockInstrument::CowBell,
                note: 21i32,
                powered: true,
            } => Ok(1173u32),
            NoteBlock {
                instrument: NoteBlockInstrument::CowBell,
                note: 21i32,
                powered: false,
            } => Ok(1174u32),
            NoteBlock {
                instrument: NoteBlockInstrument::CowBell,
                note: 22i32,
                powered: true,
            } => Ok(1175u32),
            NoteBlock {
                instrument: NoteBlockInstrument::CowBell,
                note: 22i32,
                powered: false,
            } => Ok(1176u32),
            NoteBlock {
                instrument: NoteBlockInstrument::CowBell,
                note: 23i32,
                powered: true,
            } => Ok(1177u32),
            NoteBlock {
                instrument: NoteBlockInstrument::CowBell,
                note: 23i32,
                powered: false,
            } => Ok(1178u32),
            NoteBlock {
                instrument: NoteBlockInstrument::CowBell,
                note: 24i32,
                powered: true,
            } => Ok(1179u32),
            NoteBlock {
                instrument: NoteBlockInstrument::CowBell,
                note: 24i32,
                powered: false,
            } => Ok(1180u32),
            NoteBlock {
                instrument: NoteBlockInstrument::Didgeridoo,
                note: 0i32,
                powered: true,
            } => Ok(1181u32),
            NoteBlock {
                instrument: NoteBlockInstrument::Didgeridoo,
                note: 0i32,
                powered: false,
            } => Ok(1182u32),
            NoteBlock {
                instrument: NoteBlockInstrument::Didgeridoo,
                note: 1i32,
                powered: true,
            } => Ok(1183u32),
            NoteBlock {
                instrument: NoteBlockInstrument::Didgeridoo,
                note: 1i32,
                powered: false,
            } => Ok(1184u32),
            NoteBlock {
                instrument: NoteBlockInstrument::Didgeridoo,
                note: 2i32,
                powered: true,
            } => Ok(1185u32),
            NoteBlock {
                instrument: NoteBlockInstrument::Didgeridoo,
                note: 2i32,
                powered: false,
            } => Ok(1186u32),
            NoteBlock {
                instrument: NoteBlockInstrument::Didgeridoo,
                note: 3i32,
                powered: true,
            } => Ok(1187u32),
            NoteBlock {
                instrument: NoteBlockInstrument::Didgeridoo,
                note: 3i32,
                powered: false,
            } => Ok(1188u32),
            NoteBlock {
                instrument: NoteBlockInstrument::Didgeridoo,
                note: 4i32,
                powered: true,
            } => Ok(1189u32),
            NoteBlock {
                instrument: NoteBlockInstrument::Didgeridoo,
                note: 4i32,
                powered: false,
            } => Ok(1190u32),
            NoteBlock {
                instrument: NoteBlockInstrument::Didgeridoo,
                note: 5i32,
                powered: true,
            } => Ok(1191u32),
            NoteBlock {
                instrument: NoteBlockInstrument::Didgeridoo,
                note: 5i32,
                powered: false,
            } => Ok(1192u32),
            NoteBlock {
                instrument: NoteBlockInstrument::Didgeridoo,
                note: 6i32,
                powered: true,
            } => Ok(1193u32),
            NoteBlock {
                instrument: NoteBlockInstrument::Didgeridoo,
                note: 6i32,
                powered: false,
            } => Ok(1194u32),
            NoteBlock {
                instrument: NoteBlockInstrument::Didgeridoo,
                note: 7i32,
                powered: true,
            } => Ok(1195u32),
            NoteBlock {
                instrument: NoteBlockInstrument::Didgeridoo,
                note: 7i32,
                powered: false,
            } => Ok(1196u32),
            NoteBlock {
                instrument: NoteBlockInstrument::Didgeridoo,
                note: 8i32,
                powered: true,
            } => Ok(1197u32),
            NoteBlock {
                instrument: NoteBlockInstrument::Didgeridoo,
                note: 8i32,
                powered: false,
            } => Ok(1198u32),
            NoteBlock {
                instrument: NoteBlockInstrument::Didgeridoo,
                note: 9i32,
                powered: true,
            } => Ok(1199u32),
            NoteBlock {
                instrument: NoteBlockInstrument::Didgeridoo,
                note: 9i32,
                powered: false,
            } => Ok(1200u32),
            NoteBlock {
                instrument: NoteBlockInstrument::Didgeridoo,
                note: 10i32,
                powered: true,
            } => Ok(1201u32),
            NoteBlock {
                instrument: NoteBlockInstrument::Didgeridoo,
                note: 10i32,
                powered: false,
            } => Ok(1202u32),
            NoteBlock {
                instrument: NoteBlockInstrument::Didgeridoo,
                note: 11i32,
                powered: true,
            } => Ok(1203u32),
            NoteBlock {
                instrument: NoteBlockInstrument::Didgeridoo,
                note: 11i32,
                powered: false,
            } => Ok(1204u32),
            NoteBlock {
                instrument: NoteBlockInstrument::Didgeridoo,
                note: 12i32,
                powered: true,
            } => Ok(1205u32),
            NoteBlock {
                instrument: NoteBlockInstrument::Didgeridoo,
                note: 12i32,
                powered: false,
            } => Ok(1206u32),
            NoteBlock {
                instrument: NoteBlockInstrument::Didgeridoo,
                note: 13i32,
                powered: true,
            } => Ok(1207u32),
            NoteBlock {
                instrument: NoteBlockInstrument::Didgeridoo,
                note: 13i32,
                powered: false,
            } => Ok(1208u32),
            NoteBlock {
                instrument: NoteBlockInstrument::Didgeridoo,
                note: 14i32,
                powered: true,
            } => Ok(1209u32),
            NoteBlock {
                instrument: NoteBlockInstrument::Didgeridoo,
                note: 14i32,
                powered: false,
            } => Ok(1210u32),
            NoteBlock {
                instrument: NoteBlockInstrument::Didgeridoo,
                note: 15i32,
                powered: true,
            } => Ok(1211u32),
            NoteBlock {
                instrument: NoteBlockInstrument::Didgeridoo,
                note: 15i32,
                powered: false,
            } => Ok(1212u32),
            NoteBlock {
                instrument: NoteBlockInstrument::Didgeridoo,
                note: 16i32,
                powered: true,
            } => Ok(1213u32),
            NoteBlock {
                instrument: NoteBlockInstrument::Didgeridoo,
                note: 16i32,
                powered: false,
            } => Ok(1214u32),
            NoteBlock {
                instrument: NoteBlockInstrument::Didgeridoo,
                note: 17i32,
                powered: true,
            } => Ok(1215u32),
            NoteBlock {
                instrument: NoteBlockInstrument::Didgeridoo,
                note: 17i32,
                powered: false,
            } => Ok(1216u32),
            NoteBlock {
                instrument: NoteBlockInstrument::Didgeridoo,
                note: 18i32,
                powered: true,
            } => Ok(1217u32),
            NoteBlock {
                instrument: NoteBlockInstrument::Didgeridoo,
                note: 18i32,
                powered: false,
            } => Ok(1218u32),
            NoteBlock {
                instrument: NoteBlockInstrument::Didgeridoo,
                note: 19i32,
                powered: true,
            } => Ok(1219u32),
            NoteBlock {
                instrument: NoteBlockInstrument::Didgeridoo,
                note: 19i32,
                powered: false,
            } => Ok(1220u32),
            NoteBlock {
                instrument: NoteBlockInstrument::Didgeridoo,
                note: 20i32,
                powered: true,
            } => Ok(1221u32),
            NoteBlock {
                instrument: NoteBlockInstrument::Didgeridoo,
                note: 20i32,
                powered: false,
            } => Ok(1222u32),
            NoteBlock {
                instrument: NoteBlockInstrument::Didgeridoo,
                note: 21i32,
                powered: true,
            } => Ok(1223u32),
            NoteBlock {
                instrument: NoteBlockInstrument::Didgeridoo,
                note: 21i32,
                powered: false,
            } => Ok(1224u32),
            NoteBlock {
                instrument: NoteBlockInstrument::Didgeridoo,
                note: 22i32,
                powered: true,
            } => Ok(1225u32),
            NoteBlock {
                instrument: NoteBlockInstrument::Didgeridoo,
                note: 22i32,
                powered: false,
            } => Ok(1226u32),
            NoteBlock {
                instrument: NoteBlockInstrument::Didgeridoo,
                note: 23i32,
                powered: true,
            } => Ok(1227u32),
            NoteBlock {
                instrument: NoteBlockInstrument::Didgeridoo,
                note: 23i32,
                powered: false,
            } => Ok(1228u32),
            NoteBlock {
                instrument: NoteBlockInstrument::Didgeridoo,
                note: 24i32,
                powered: true,
            } => Ok(1229u32),
            NoteBlock {
                instrument: NoteBlockInstrument::Didgeridoo,
                note: 24i32,
                powered: false,
            } => Ok(1230u32),
            NoteBlock {
                instrument: NoteBlockInstrument::Bit,
                note: 0i32,
                powered: true,
            } => Ok(1231u32),
            NoteBlock {
                instrument: NoteBlockInstrument::Bit,
                note: 0i32,
                powered: false,
            } => Ok(1232u32),
            NoteBlock {
                instrument: NoteBlockInstrument::Bit,
                note: 1i32,
                powered: true,
            } => Ok(1233u32),
            NoteBlock {
                instrument: NoteBlockInstrument::Bit,
                note: 1i32,
                powered: false,
            } => Ok(1234u32),
            NoteBlock {
                instrument: NoteBlockInstrument::Bit,
                note: 2i32,
                powered: true,
            } => Ok(1235u32),
            NoteBlock {
                instrument: NoteBlockInstrument::Bit,
                note: 2i32,
                powered: false,
            } => Ok(1236u32),
            NoteBlock {
                instrument: NoteBlockInstrument::Bit,
                note: 3i32,
                powered: true,
            } => Ok(1237u32),
            NoteBlock {
                instrument: NoteBlockInstrument::Bit,
                note: 3i32,
                powered: false,
            } => Ok(1238u32),
            NoteBlock {
                instrument: NoteBlockInstrument::Bit,
                note: 4i32,
                powered: true,
            } => Ok(1239u32),
            NoteBlock {
                instrument: NoteBlockInstrument::Bit,
                note: 4i32,
                powered: false,
            } => Ok(1240u32),
            NoteBlock {
                instrument: NoteBlockInstrument::Bit,
                note: 5i32,
                powered: true,
            } => Ok(1241u32),
            NoteBlock {
                instrument: NoteBlockInstrument::Bit,
                note: 5i32,
                powered: false,
            } => Ok(1242u32),
            NoteBlock {
                instrument: NoteBlockInstrument::Bit,
                note: 6i32,
                powered: true,
            } => Ok(1243u32),
            NoteBlock {
                instrument: NoteBlockInstrument::Bit,
                note: 6i32,
                powered: false,
            } => Ok(1244u32),
            NoteBlock {
                instrument: NoteBlockInstrument::Bit,
                note: 7i32,
                powered: true,
            } => Ok(1245u32),
            NoteBlock {
                instrument: NoteBlockInstrument::Bit,
                note: 7i32,
                powered: false,
            } => Ok(1246u32),
            NoteBlock {
                instrument: NoteBlockInstrument::Bit,
                note: 8i32,
                powered: true,
            } => Ok(1247u32),
            NoteBlock {
                instrument: NoteBlockInstrument::Bit,
                note: 8i32,
                powered: false,
            } => Ok(1248u32),
            NoteBlock {
                instrument: NoteBlockInstrument::Bit,
                note: 9i32,
                powered: true,
            } => Ok(1249u32),
            NoteBlock {
                instrument: NoteBlockInstrument::Bit,
                note: 9i32,
                powered: false,
            } => Ok(1250u32),
            NoteBlock {
                instrument: NoteBlockInstrument::Bit,
                note: 10i32,
                powered: true,
            } => Ok(1251u32),
            NoteBlock {
                instrument: NoteBlockInstrument::Bit,
                note: 10i32,
                powered: false,
            } => Ok(1252u32),
            NoteBlock {
                instrument: NoteBlockInstrument::Bit,
                note: 11i32,
                powered: true,
            } => Ok(1253u32),
            NoteBlock {
                instrument: NoteBlockInstrument::Bit,
                note: 11i32,
                powered: false,
            } => Ok(1254u32),
            NoteBlock {
                instrument: NoteBlockInstrument::Bit,
                note: 12i32,
                powered: true,
            } => Ok(1255u32),
            NoteBlock {
                instrument: NoteBlockInstrument::Bit,
                note: 12i32,
                powered: false,
            } => Ok(1256u32),
            NoteBlock {
                instrument: NoteBlockInstrument::Bit,
                note: 13i32,
                powered: true,
            } => Ok(1257u32),
            NoteBlock {
                instrument: NoteBlockInstrument::Bit,
                note: 13i32,
                powered: false,
            } => Ok(1258u32),
            NoteBlock {
                instrument: NoteBlockInstrument::Bit,
                note: 14i32,
                powered: true,
            } => Ok(1259u32),
            NoteBlock {
                instrument: NoteBlockInstrument::Bit,
                note: 14i32,
                powered: false,
            } => Ok(1260u32),
            NoteBlock {
                instrument: NoteBlockInstrument::Bit,
                note: 15i32,
                powered: true,
            } => Ok(1261u32),
            NoteBlock {
                instrument: NoteBlockInstrument::Bit,
                note: 15i32,
                powered: false,
            } => Ok(1262u32),
            NoteBlock {
                instrument: NoteBlockInstrument::Bit,
                note: 16i32,
                powered: true,
            } => Ok(1263u32),
            NoteBlock {
                instrument: NoteBlockInstrument::Bit,
                note: 16i32,
                powered: false,
            } => Ok(1264u32),
            NoteBlock {
                instrument: NoteBlockInstrument::Bit,
                note: 17i32,
                powered: true,
            } => Ok(1265u32),
            NoteBlock {
                instrument: NoteBlockInstrument::Bit,
                note: 17i32,
                powered: false,
            } => Ok(1266u32),
            NoteBlock {
                instrument: NoteBlockInstrument::Bit,
                note: 18i32,
                powered: true,
            } => Ok(1267u32),
            NoteBlock {
                instrument: NoteBlockInstrument::Bit,
                note: 18i32,
                powered: false,
            } => Ok(1268u32),
            NoteBlock {
                instrument: NoteBlockInstrument::Bit,
                note: 19i32,
                powered: true,
            } => Ok(1269u32),
            NoteBlock {
                instrument: NoteBlockInstrument::Bit,
                note: 19i32,
                powered: false,
            } => Ok(1270u32),
            NoteBlock {
                instrument: NoteBlockInstrument::Bit,
                note: 20i32,
                powered: true,
            } => Ok(1271u32),
            NoteBlock {
                instrument: NoteBlockInstrument::Bit,
                note: 20i32,
                powered: false,
            } => Ok(1272u32),
            NoteBlock {
                instrument: NoteBlockInstrument::Bit,
                note: 21i32,
                powered: true,
            } => Ok(1273u32),
            NoteBlock {
                instrument: NoteBlockInstrument::Bit,
                note: 21i32,
                powered: false,
            } => Ok(1274u32),
            NoteBlock {
                instrument: NoteBlockInstrument::Bit,
                note: 22i32,
                powered: true,
            } => Ok(1275u32),
            NoteBlock {
                instrument: NoteBlockInstrument::Bit,
                note: 22i32,
                powered: false,
            } => Ok(1276u32),
            NoteBlock {
                instrument: NoteBlockInstrument::Bit,
                note: 23i32,
                powered: true,
            } => Ok(1277u32),
            NoteBlock {
                instrument: NoteBlockInstrument::Bit,
                note: 23i32,
                powered: false,
            } => Ok(1278u32),
            NoteBlock {
                instrument: NoteBlockInstrument::Bit,
                note: 24i32,
                powered: true,
            } => Ok(1279u32),
            NoteBlock {
                instrument: NoteBlockInstrument::Bit,
                note: 24i32,
                powered: false,
            } => Ok(1280u32),
            NoteBlock {
                instrument: NoteBlockInstrument::Banjo,
                note: 0i32,
                powered: true,
            } => Ok(1281u32),
            NoteBlock {
                instrument: NoteBlockInstrument::Banjo,
                note: 0i32,
                powered: false,
            } => Ok(1282u32),
            NoteBlock {
                instrument: NoteBlockInstrument::Banjo,
                note: 1i32,
                powered: true,
            } => Ok(1283u32),
            NoteBlock {
                instrument: NoteBlockInstrument::Banjo,
                note: 1i32,
                powered: false,
            } => Ok(1284u32),
            NoteBlock {
                instrument: NoteBlockInstrument::Banjo,
                note: 2i32,
                powered: true,
            } => Ok(1285u32),
            NoteBlock {
                instrument: NoteBlockInstrument::Banjo,
                note: 2i32,
                powered: false,
            } => Ok(1286u32),
            NoteBlock {
                instrument: NoteBlockInstrument::Banjo,
                note: 3i32,
                powered: true,
            } => Ok(1287u32),
            NoteBlock {
                instrument: NoteBlockInstrument::Banjo,
                note: 3i32,
                powered: false,
            } => Ok(1288u32),
            NoteBlock {
                instrument: NoteBlockInstrument::Banjo,
                note: 4i32,
                powered: true,
            } => Ok(1289u32),
            NoteBlock {
                instrument: NoteBlockInstrument::Banjo,
                note: 4i32,
                powered: false,
            } => Ok(1290u32),
            NoteBlock {
                instrument: NoteBlockInstrument::Banjo,
                note: 5i32,
                powered: true,
            } => Ok(1291u32),
            NoteBlock {
                instrument: NoteBlockInstrument::Banjo,
                note: 5i32,
                powered: false,
            } => Ok(1292u32),
            NoteBlock {
                instrument: NoteBlockInstrument::Banjo,
                note: 6i32,
                powered: true,
            } => Ok(1293u32),
            NoteBlock {
                instrument: NoteBlockInstrument::Banjo,
                note: 6i32,
                powered: false,
            } => Ok(1294u32),
            NoteBlock {
                instrument: NoteBlockInstrument::Banjo,
                note: 7i32,
                powered: true,
            } => Ok(1295u32),
            NoteBlock {
                instrument: NoteBlockInstrument::Banjo,
                note: 7i32,
                powered: false,
            } => Ok(1296u32),
            NoteBlock {
                instrument: NoteBlockInstrument::Banjo,
                note: 8i32,
                powered: true,
            } => Ok(1297u32),
            NoteBlock {
                instrument: NoteBlockInstrument::Banjo,
                note: 8i32,
                powered: false,
            } => Ok(1298u32),
            NoteBlock {
                instrument: NoteBlockInstrument::Banjo,
                note: 9i32,
                powered: true,
            } => Ok(1299u32),
            NoteBlock {
                instrument: NoteBlockInstrument::Banjo,
                note: 9i32,
                powered: false,
            } => Ok(1300u32),
            NoteBlock {
                instrument: NoteBlockInstrument::Banjo,
                note: 10i32,
                powered: true,
            } => Ok(1301u32),
            NoteBlock {
                instrument: NoteBlockInstrument::Banjo,
                note: 10i32,
                powered: false,
            } => Ok(1302u32),
            NoteBlock {
                instrument: NoteBlockInstrument::Banjo,
                note: 11i32,
                powered: true,
            } => Ok(1303u32),
            NoteBlock {
                instrument: NoteBlockInstrument::Banjo,
                note: 11i32,
                powered: false,
            } => Ok(1304u32),
            NoteBlock {
                instrument: NoteBlockInstrument::Banjo,
                note: 12i32,
                powered: true,
            } => Ok(1305u32),
            NoteBlock {
                instrument: NoteBlockInstrument::Banjo,
                note: 12i32,
                powered: false,
            } => Ok(1306u32),
            NoteBlock {
                instrument: NoteBlockInstrument::Banjo,
                note: 13i32,
                powered: true,
            } => Ok(1307u32),
            NoteBlock {
                instrument: NoteBlockInstrument::Banjo,
                note: 13i32,
                powered: false,
            } => Ok(1308u32),
            NoteBlock {
                instrument: NoteBlockInstrument::Banjo,
                note: 14i32,
                powered: true,
            } => Ok(1309u32),
            NoteBlock {
                instrument: NoteBlockInstrument::Banjo,
                note: 14i32,
                powered: false,
            } => Ok(1310u32),
            NoteBlock {
                instrument: NoteBlockInstrument::Banjo,
                note: 15i32,
                powered: true,
            } => Ok(1311u32),
            NoteBlock {
                instrument: NoteBlockInstrument::Banjo,
                note: 15i32,
                powered: false,
            } => Ok(1312u32),
            NoteBlock {
                instrument: NoteBlockInstrument::Banjo,
                note: 16i32,
                powered: true,
            } => Ok(1313u32),
            NoteBlock {
                instrument: NoteBlockInstrument::Banjo,
                note: 16i32,
                powered: false,
            } => Ok(1314u32),
            NoteBlock {
                instrument: NoteBlockInstrument::Banjo,
                note: 17i32,
                powered: true,
            } => Ok(1315u32),
            NoteBlock {
                instrument: NoteBlockInstrument::Banjo,
                note: 17i32,
                powered: false,
            } => Ok(1316u32),
            NoteBlock {
                instrument: NoteBlockInstrument::Banjo,
                note: 18i32,
                powered: true,
            } => Ok(1317u32),
            NoteBlock {
                instrument: NoteBlockInstrument::Banjo,
                note: 18i32,
                powered: false,
            } => Ok(1318u32),
            NoteBlock {
                instrument: NoteBlockInstrument::Banjo,
                note: 19i32,
                powered: true,
            } => Ok(1319u32),
            NoteBlock {
                instrument: NoteBlockInstrument::Banjo,
                note: 19i32,
                powered: false,
            } => Ok(1320u32),
            NoteBlock {
                instrument: NoteBlockInstrument::Banjo,
                note: 20i32,
                powered: true,
            } => Ok(1321u32),
            NoteBlock {
                instrument: NoteBlockInstrument::Banjo,
                note: 20i32,
                powered: false,
            } => Ok(1322u32),
            NoteBlock {
                instrument: NoteBlockInstrument::Banjo,
                note: 21i32,
                powered: true,
            } => Ok(1323u32),
            NoteBlock {
                instrument: NoteBlockInstrument::Banjo,
                note: 21i32,
                powered: false,
            } => Ok(1324u32),
            NoteBlock {
                instrument: NoteBlockInstrument::Banjo,
                note: 22i32,
                powered: true,
            } => Ok(1325u32),
            NoteBlock {
                instrument: NoteBlockInstrument::Banjo,
                note: 22i32,
                powered: false,
            } => Ok(1326u32),
            NoteBlock {
                instrument: NoteBlockInstrument::Banjo,
                note: 23i32,
                powered: true,
            } => Ok(1327u32),
            NoteBlock {
                instrument: NoteBlockInstrument::Banjo,
                note: 23i32,
                powered: false,
            } => Ok(1328u32),
            NoteBlock {
                instrument: NoteBlockInstrument::Banjo,
                note: 24i32,
                powered: true,
            } => Ok(1329u32),
            NoteBlock {
                instrument: NoteBlockInstrument::Banjo,
                note: 24i32,
                powered: false,
            } => Ok(1330u32),
            NoteBlock {
                instrument: NoteBlockInstrument::Pling,
                note: 0i32,
                powered: true,
            } => Ok(1331u32),
            NoteBlock {
                instrument: NoteBlockInstrument::Pling,
                note: 0i32,
                powered: false,
            } => Ok(1332u32),
            NoteBlock {
                instrument: NoteBlockInstrument::Pling,
                note: 1i32,
                powered: true,
            } => Ok(1333u32),
            NoteBlock {
                instrument: NoteBlockInstrument::Pling,
                note: 1i32,
                powered: false,
            } => Ok(1334u32),
            NoteBlock {
                instrument: NoteBlockInstrument::Pling,
                note: 2i32,
                powered: true,
            } => Ok(1335u32),
            NoteBlock {
                instrument: NoteBlockInstrument::Pling,
                note: 2i32,
                powered: false,
            } => Ok(1336u32),
            NoteBlock {
                instrument: NoteBlockInstrument::Pling,
                note: 3i32,
                powered: true,
            } => Ok(1337u32),
            NoteBlock {
                instrument: NoteBlockInstrument::Pling,
                note: 3i32,
                powered: false,
            } => Ok(1338u32),
            NoteBlock {
                instrument: NoteBlockInstrument::Pling,
                note: 4i32,
                powered: true,
            } => Ok(1339u32),
            NoteBlock {
                instrument: NoteBlockInstrument::Pling,
                note: 4i32,
                powered: false,
            } => Ok(1340u32),
            NoteBlock {
                instrument: NoteBlockInstrument::Pling,
                note: 5i32,
                powered: true,
            } => Ok(1341u32),
            NoteBlock {
                instrument: NoteBlockInstrument::Pling,
                note: 5i32,
                powered: false,
            } => Ok(1342u32),
            NoteBlock {
                instrument: NoteBlockInstrument::Pling,
                note: 6i32,
                powered: true,
            } => Ok(1343u32),
            NoteBlock {
                instrument: NoteBlockInstrument::Pling,
                note: 6i32,
                powered: false,
            } => Ok(1344u32),
            NoteBlock {
                instrument: NoteBlockInstrument::Pling,
                note: 7i32,
                powered: true,
            } => Ok(1345u32),
            NoteBlock {
                instrument: NoteBlockInstrument::Pling,
                note: 7i32,
                powered: false,
            } => Ok(1346u32),
            NoteBlock {
                instrument: NoteBlockInstrument::Pling,
                note: 8i32,
                powered: true,
            } => Ok(1347u32),
            NoteBlock {
                instrument: NoteBlockInstrument::Pling,
                note: 8i32,
                powered: false,
            } => Ok(1348u32),
            NoteBlock {
                instrument: NoteBlockInstrument::Pling,
                note: 9i32,
                powered: true,
            } => Ok(1349u32),
            NoteBlock {
                instrument: NoteBlockInstrument::Pling,
                note: 9i32,
                powered: false,
            } => Ok(1350u32),
            NoteBlock {
                instrument: NoteBlockInstrument::Pling,
                note: 10i32,
                powered: true,
            } => Ok(1351u32),
            NoteBlock {
                instrument: NoteBlockInstrument::Pling,
                note: 10i32,
                powered: false,
            } => Ok(1352u32),
            NoteBlock {
                instrument: NoteBlockInstrument::Pling,
                note: 11i32,
                powered: true,
            } => Ok(1353u32),
            NoteBlock {
                instrument: NoteBlockInstrument::Pling,
                note: 11i32,
                powered: false,
            } => Ok(1354u32),
            NoteBlock {
                instrument: NoteBlockInstrument::Pling,
                note: 12i32,
                powered: true,
            } => Ok(1355u32),
            NoteBlock {
                instrument: NoteBlockInstrument::Pling,
                note: 12i32,
                powered: false,
            } => Ok(1356u32),
            NoteBlock {
                instrument: NoteBlockInstrument::Pling,
                note: 13i32,
                powered: true,
            } => Ok(1357u32),
            NoteBlock {
                instrument: NoteBlockInstrument::Pling,
                note: 13i32,
                powered: false,
            } => Ok(1358u32),
            NoteBlock {
                instrument: NoteBlockInstrument::Pling,
                note: 14i32,
                powered: true,
            } => Ok(1359u32),
            NoteBlock {
                instrument: NoteBlockInstrument::Pling,
                note: 14i32,
                powered: false,
            } => Ok(1360u32),
            NoteBlock {
                instrument: NoteBlockInstrument::Pling,
                note: 15i32,
                powered: true,
            } => Ok(1361u32),
            NoteBlock {
                instrument: NoteBlockInstrument::Pling,
                note: 15i32,
                powered: false,
            } => Ok(1362u32),
            NoteBlock {
                instrument: NoteBlockInstrument::Pling,
                note: 16i32,
                powered: true,
            } => Ok(1363u32),
            NoteBlock {
                instrument: NoteBlockInstrument::Pling,
                note: 16i32,
                powered: false,
            } => Ok(1364u32),
            NoteBlock {
                instrument: NoteBlockInstrument::Pling,
                note: 17i32,
                powered: true,
            } => Ok(1365u32),
            NoteBlock {
                instrument: NoteBlockInstrument::Pling,
                note: 17i32,
                powered: false,
            } => Ok(1366u32),
            NoteBlock {
                instrument: NoteBlockInstrument::Pling,
                note: 18i32,
                powered: true,
            } => Ok(1367u32),
            NoteBlock {
                instrument: NoteBlockInstrument::Pling,
                note: 18i32,
                powered: false,
            } => Ok(1368u32),
            NoteBlock {
                instrument: NoteBlockInstrument::Pling,
                note: 19i32,
                powered: true,
            } => Ok(1369u32),
            NoteBlock {
                instrument: NoteBlockInstrument::Pling,
                note: 19i32,
                powered: false,
            } => Ok(1370u32),
            NoteBlock {
                instrument: NoteBlockInstrument::Pling,
                note: 20i32,
                powered: true,
            } => Ok(1371u32),
            NoteBlock {
                instrument: NoteBlockInstrument::Pling,
                note: 20i32,
                powered: false,
            } => Ok(1372u32),
            NoteBlock {
                instrument: NoteBlockInstrument::Pling,
                note: 21i32,
                powered: true,
            } => Ok(1373u32),
            NoteBlock {
                instrument: NoteBlockInstrument::Pling,
                note: 21i32,
                powered: false,
            } => Ok(1374u32),
            NoteBlock {
                instrument: NoteBlockInstrument::Pling,
                note: 22i32,
                powered: true,
            } => Ok(1375u32),
            NoteBlock {
                instrument: NoteBlockInstrument::Pling,
                note: 22i32,
                powered: false,
            } => Ok(1376u32),
            NoteBlock {
                instrument: NoteBlockInstrument::Pling,
                note: 23i32,
                powered: true,
            } => Ok(1377u32),
            NoteBlock {
                instrument: NoteBlockInstrument::Pling,
                note: 23i32,
                powered: false,
            } => Ok(1378u32),
            NoteBlock {
                instrument: NoteBlockInstrument::Pling,
                note: 24i32,
                powered: true,
            } => Ok(1379u32),
            NoteBlock {
                instrument: NoteBlockInstrument::Pling,
                note: 24i32,
                powered: false,
            } => Ok(1380u32),
            NoteBlock {
                instrument: NoteBlockInstrument::Zombie,
                note: 0i32,
                powered: true,
            } => Ok(1381u32),
            NoteBlock {
                instrument: NoteBlockInstrument::Zombie,
                note: 0i32,
                powered: false,
            } => Ok(1382u32),
            NoteBlock {
                instrument: NoteBlockInstrument::Zombie,
                note: 1i32,
                powered: true,
            } => Ok(1383u32),
            NoteBlock {
                instrument: NoteBlockInstrument::Zombie,
                note: 1i32,
                powered: false,
            } => Ok(1384u32),
            NoteBlock {
                instrument: NoteBlockInstrument::Zombie,
                note: 2i32,
                powered: true,
            } => Ok(1385u32),
            NoteBlock {
                instrument: NoteBlockInstrument::Zombie,
                note: 2i32,
                powered: false,
            } => Ok(1386u32),
            NoteBlock {
                instrument: NoteBlockInstrument::Zombie,
                note: 3i32,
                powered: true,
            } => Ok(1387u32),
            NoteBlock {
                instrument: NoteBlockInstrument::Zombie,
                note: 3i32,
                powered: false,
            } => Ok(1388u32),
            NoteBlock {
                instrument: NoteBlockInstrument::Zombie,
                note: 4i32,
                powered: true,
            } => Ok(1389u32),
            NoteBlock {
                instrument: NoteBlockInstrument::Zombie,
                note: 4i32,
                powered: false,
            } => Ok(1390u32),
            NoteBlock {
                instrument: NoteBlockInstrument::Zombie,
                note: 5i32,
                powered: true,
            } => Ok(1391u32),
            NoteBlock {
                instrument: NoteBlockInstrument::Zombie,
                note: 5i32,
                powered: false,
            } => Ok(1392u32),
            NoteBlock {
                instrument: NoteBlockInstrument::Zombie,
                note: 6i32,
                powered: true,
            } => Ok(1393u32),
            NoteBlock {
                instrument: NoteBlockInstrument::Zombie,
                note: 6i32,
                powered: false,
            } => Ok(1394u32),
            NoteBlock {
                instrument: NoteBlockInstrument::Zombie,
                note: 7i32,
                powered: true,
            } => Ok(1395u32),
            NoteBlock {
                instrument: NoteBlockInstrument::Zombie,
                note: 7i32,
                powered: false,
            } => Ok(1396u32),
            NoteBlock {
                instrument: NoteBlockInstrument::Zombie,
                note: 8i32,
                powered: true,
            } => Ok(1397u32),
            NoteBlock {
                instrument: NoteBlockInstrument::Zombie,
                note: 8i32,
                powered: false,
            } => Ok(1398u32),
            NoteBlock {
                instrument: NoteBlockInstrument::Zombie,
                note: 9i32,
                powered: true,
            } => Ok(1399u32),
            NoteBlock {
                instrument: NoteBlockInstrument::Zombie,
                note: 9i32,
                powered: false,
            } => Ok(1400u32),
            NoteBlock {
                instrument: NoteBlockInstrument::Zombie,
                note: 10i32,
                powered: true,
            } => Ok(1401u32),
            NoteBlock {
                instrument: NoteBlockInstrument::Zombie,
                note: 10i32,
                powered: false,
            } => Ok(1402u32),
            NoteBlock {
                instrument: NoteBlockInstrument::Zombie,
                note: 11i32,
                powered: true,
            } => Ok(1403u32),
            NoteBlock {
                instrument: NoteBlockInstrument::Zombie,
                note: 11i32,
                powered: false,
            } => Ok(1404u32),
            NoteBlock {
                instrument: NoteBlockInstrument::Zombie,
                note: 12i32,
                powered: true,
            } => Ok(1405u32),
            NoteBlock {
                instrument: NoteBlockInstrument::Zombie,
                note: 12i32,
                powered: false,
            } => Ok(1406u32),
            NoteBlock {
                instrument: NoteBlockInstrument::Zombie,
                note: 13i32,
                powered: true,
            } => Ok(1407u32),
            NoteBlock {
                instrument: NoteBlockInstrument::Zombie,
                note: 13i32,
                powered: false,
            } => Ok(1408u32),
            NoteBlock {
                instrument: NoteBlockInstrument::Zombie,
                note: 14i32,
                powered: true,
            } => Ok(1409u32),
            NoteBlock {
                instrument: NoteBlockInstrument::Zombie,
                note: 14i32,
                powered: false,
            } => Ok(1410u32),
            NoteBlock {
                instrument: NoteBlockInstrument::Zombie,
                note: 15i32,
                powered: true,
            } => Ok(1411u32),
            NoteBlock {
                instrument: NoteBlockInstrument::Zombie,
                note: 15i32,
                powered: false,
            } => Ok(1412u32),
            NoteBlock {
                instrument: NoteBlockInstrument::Zombie,
                note: 16i32,
                powered: true,
            } => Ok(1413u32),
            NoteBlock {
                instrument: NoteBlockInstrument::Zombie,
                note: 16i32,
                powered: false,
            } => Ok(1414u32),
            NoteBlock {
                instrument: NoteBlockInstrument::Zombie,
                note: 17i32,
                powered: true,
            } => Ok(1415u32),
            NoteBlock {
                instrument: NoteBlockInstrument::Zombie,
                note: 17i32,
                powered: false,
            } => Ok(1416u32),
            NoteBlock {
                instrument: NoteBlockInstrument::Zombie,
                note: 18i32,
                powered: true,
            } => Ok(1417u32),
            NoteBlock {
                instrument: NoteBlockInstrument::Zombie,
                note: 18i32,
                powered: false,
            } => Ok(1418u32),
            NoteBlock {
                instrument: NoteBlockInstrument::Zombie,
                note: 19i32,
                powered: true,
            } => Ok(1419u32),
            NoteBlock {
                instrument: NoteBlockInstrument::Zombie,
                note: 19i32,
                powered: false,
            } => Ok(1420u32),
            NoteBlock {
                instrument: NoteBlockInstrument::Zombie,
                note: 20i32,
                powered: true,
            } => Ok(1421u32),
            NoteBlock {
                instrument: NoteBlockInstrument::Zombie,
                note: 20i32,
                powered: false,
            } => Ok(1422u32),
            NoteBlock {
                instrument: NoteBlockInstrument::Zombie,
                note: 21i32,
                powered: true,
            } => Ok(1423u32),
            NoteBlock {
                instrument: NoteBlockInstrument::Zombie,
                note: 21i32,
                powered: false,
            } => Ok(1424u32),
            NoteBlock {
                instrument: NoteBlockInstrument::Zombie,
                note: 22i32,
                powered: true,
            } => Ok(1425u32),
            NoteBlock {
                instrument: NoteBlockInstrument::Zombie,
                note: 22i32,
                powered: false,
            } => Ok(1426u32),
            NoteBlock {
                instrument: NoteBlockInstrument::Zombie,
                note: 23i32,
                powered: true,
            } => Ok(1427u32),
            NoteBlock {
                instrument: NoteBlockInstrument::Zombie,
                note: 23i32,
                powered: false,
            } => Ok(1428u32),
            NoteBlock {
                instrument: NoteBlockInstrument::Zombie,
                note: 24i32,
                powered: true,
            } => Ok(1429u32),
            NoteBlock {
                instrument: NoteBlockInstrument::Zombie,
                note: 24i32,
                powered: false,
            } => Ok(1430u32),
            NoteBlock {
                instrument: NoteBlockInstrument::Skeleton,
                note: 0i32,
                powered: true,
            } => Ok(1431u32),
            NoteBlock {
                instrument: NoteBlockInstrument::Skeleton,
                note: 0i32,
                powered: false,
            } => Ok(1432u32),
            NoteBlock {
                instrument: NoteBlockInstrument::Skeleton,
                note: 1i32,
                powered: true,
            } => Ok(1433u32),
            NoteBlock {
                instrument: NoteBlockInstrument::Skeleton,
                note: 1i32,
                powered: false,
            } => Ok(1434u32),
            NoteBlock {
                instrument: NoteBlockInstrument::Skeleton,
                note: 2i32,
                powered: true,
            } => Ok(1435u32),
            NoteBlock {
                instrument: NoteBlockInstrument::Skeleton,
                note: 2i32,
                powered: false,
            } => Ok(1436u32),
            NoteBlock {
                instrument: NoteBlockInstrument::Skeleton,
                note: 3i32,
                powered: true,
            } => Ok(1437u32),
            NoteBlock {
                instrument: NoteBlockInstrument::Skeleton,
                note: 3i32,
                powered: false,
            } => Ok(1438u32),
            NoteBlock {
                instrument: NoteBlockInstrument::Skeleton,
                note: 4i32,
                powered: true,
            } => Ok(1439u32),
            NoteBlock {
                instrument: NoteBlockInstrument::Skeleton,
                note: 4i32,
                powered: false,
            } => Ok(1440u32),
            NoteBlock {
                instrument: NoteBlockInstrument::Skeleton,
                note: 5i32,
                powered: true,
            } => Ok(1441u32),
            NoteBlock {
                instrument: NoteBlockInstrument::Skeleton,
                note: 5i32,
                powered: false,
            } => Ok(1442u32),
            NoteBlock {
                instrument: NoteBlockInstrument::Skeleton,
                note: 6i32,
                powered: true,
            } => Ok(1443u32),
            NoteBlock {
                instrument: NoteBlockInstrument::Skeleton,
                note: 6i32,
                powered: false,
            } => Ok(1444u32),
            NoteBlock {
                instrument: NoteBlockInstrument::Skeleton,
                note: 7i32,
                powered: true,
            } => Ok(1445u32),
            NoteBlock {
                instrument: NoteBlockInstrument::Skeleton,
                note: 7i32,
                powered: false,
            } => Ok(1446u32),
            NoteBlock {
                instrument: NoteBlockInstrument::Skeleton,
                note: 8i32,
                powered: true,
            } => Ok(1447u32),
            NoteBlock {
                instrument: NoteBlockInstrument::Skeleton,
                note: 8i32,
                powered: false,
            } => Ok(1448u32),
            NoteBlock {
                instrument: NoteBlockInstrument::Skeleton,
                note: 9i32,
                powered: true,
            } => Ok(1449u32),
            NoteBlock {
                instrument: NoteBlockInstrument::Skeleton,
                note: 9i32,
                powered: false,
            } => Ok(1450u32),
            NoteBlock {
                instrument: NoteBlockInstrument::Skeleton,
                note: 10i32,
                powered: true,
            } => Ok(1451u32),
            NoteBlock {
                instrument: NoteBlockInstrument::Skeleton,
                note: 10i32,
                powered: false,
            } => Ok(1452u32),
            NoteBlock {
                instrument: NoteBlockInstrument::Skeleton,
                note: 11i32,
                powered: true,
            } => Ok(1453u32),
            NoteBlock {
                instrument: NoteBlockInstrument::Skeleton,
                note: 11i32,
                powered: false,
            } => Ok(1454u32),
            NoteBlock {
                instrument: NoteBlockInstrument::Skeleton,
                note: 12i32,
                powered: true,
            } => Ok(1455u32),
            NoteBlock {
                instrument: NoteBlockInstrument::Skeleton,
                note: 12i32,
                powered: false,
            } => Ok(1456u32),
            NoteBlock {
                instrument: NoteBlockInstrument::Skeleton,
                note: 13i32,
                powered: true,
            } => Ok(1457u32),
            NoteBlock {
                instrument: NoteBlockInstrument::Skeleton,
                note: 13i32,
                powered: false,
            } => Ok(1458u32),
            NoteBlock {
                instrument: NoteBlockInstrument::Skeleton,
                note: 14i32,
                powered: true,
            } => Ok(1459u32),
            NoteBlock {
                instrument: NoteBlockInstrument::Skeleton,
                note: 14i32,
                powered: false,
            } => Ok(1460u32),
            NoteBlock {
                instrument: NoteBlockInstrument::Skeleton,
                note: 15i32,
                powered: true,
            } => Ok(1461u32),
            NoteBlock {
                instrument: NoteBlockInstrument::Skeleton,
                note: 15i32,
                powered: false,
            } => Ok(1462u32),
            NoteBlock {
                instrument: NoteBlockInstrument::Skeleton,
                note: 16i32,
                powered: true,
            } => Ok(1463u32),
            NoteBlock {
                instrument: NoteBlockInstrument::Skeleton,
                note: 16i32,
                powered: false,
            } => Ok(1464u32),
            NoteBlock {
                instrument: NoteBlockInstrument::Skeleton,
                note: 17i32,
                powered: true,
            } => Ok(1465u32),
            NoteBlock {
                instrument: NoteBlockInstrument::Skeleton,
                note: 17i32,
                powered: false,
            } => Ok(1466u32),
            NoteBlock {
                instrument: NoteBlockInstrument::Skeleton,
                note: 18i32,
                powered: true,
            } => Ok(1467u32),
            NoteBlock {
                instrument: NoteBlockInstrument::Skeleton,
                note: 18i32,
                powered: false,
            } => Ok(1468u32),
            NoteBlock {
                instrument: NoteBlockInstrument::Skeleton,
                note: 19i32,
                powered: true,
            } => Ok(1469u32),
            NoteBlock {
                instrument: NoteBlockInstrument::Skeleton,
                note: 19i32,
                powered: false,
            } => Ok(1470u32),
            NoteBlock {
                instrument: NoteBlockInstrument::Skeleton,
                note: 20i32,
                powered: true,
            } => Ok(1471u32),
            NoteBlock {
                instrument: NoteBlockInstrument::Skeleton,
                note: 20i32,
                powered: false,
            } => Ok(1472u32),
            NoteBlock {
                instrument: NoteBlockInstrument::Skeleton,
                note: 21i32,
                powered: true,
            } => Ok(1473u32),
            NoteBlock {
                instrument: NoteBlockInstrument::Skeleton,
                note: 21i32,
                powered: false,
            } => Ok(1474u32),
            NoteBlock {
                instrument: NoteBlockInstrument::Skeleton,
                note: 22i32,
                powered: true,
            } => Ok(1475u32),
            NoteBlock {
                instrument: NoteBlockInstrument::Skeleton,
                note: 22i32,
                powered: false,
            } => Ok(1476u32),
            NoteBlock {
                instrument: NoteBlockInstrument::Skeleton,
                note: 23i32,
                powered: true,
            } => Ok(1477u32),
            NoteBlock {
                instrument: NoteBlockInstrument::Skeleton,
                note: 23i32,
                powered: false,
            } => Ok(1478u32),
            NoteBlock {
                instrument: NoteBlockInstrument::Skeleton,
                note: 24i32,
                powered: true,
            } => Ok(1479u32),
            NoteBlock {
                instrument: NoteBlockInstrument::Skeleton,
                note: 24i32,
                powered: false,
            } => Ok(1480u32),
            NoteBlock {
                instrument: NoteBlockInstrument::Creeper,
                note: 0i32,
                powered: true,
            } => Ok(1481u32),
            NoteBlock {
                instrument: NoteBlockInstrument::Creeper,
                note: 0i32,
                powered: false,
            } => Ok(1482u32),
            NoteBlock {
                instrument: NoteBlockInstrument::Creeper,
                note: 1i32,
                powered: true,
            } => Ok(1483u32),
            NoteBlock {
                instrument: NoteBlockInstrument::Creeper,
                note: 1i32,
                powered: false,
            } => Ok(1484u32),
            NoteBlock {
                instrument: NoteBlockInstrument::Creeper,
                note: 2i32,
                powered: true,
            } => Ok(1485u32),
            NoteBlock {
                instrument: NoteBlockInstrument::Creeper,
                note: 2i32,
                powered: false,
            } => Ok(1486u32),
            NoteBlock {
                instrument: NoteBlockInstrument::Creeper,
                note: 3i32,
                powered: true,
            } => Ok(1487u32),
            NoteBlock {
                instrument: NoteBlockInstrument::Creeper,
                note: 3i32,
                powered: false,
            } => Ok(1488u32),
            NoteBlock {
                instrument: NoteBlockInstrument::Creeper,
                note: 4i32,
                powered: true,
            } => Ok(1489u32),
            NoteBlock {
                instrument: NoteBlockInstrument::Creeper,
                note: 4i32,
                powered: false,
            } => Ok(1490u32),
            NoteBlock {
                instrument: NoteBlockInstrument::Creeper,
                note: 5i32,
                powered: true,
            } => Ok(1491u32),
            NoteBlock {
                instrument: NoteBlockInstrument::Creeper,
                note: 5i32,
                powered: false,
            } => Ok(1492u32),
            NoteBlock {
                instrument: NoteBlockInstrument::Creeper,
                note: 6i32,
                powered: true,
            } => Ok(1493u32),
            NoteBlock {
                instrument: NoteBlockInstrument::Creeper,
                note: 6i32,
                powered: false,
            } => Ok(1494u32),
            NoteBlock {
                instrument: NoteBlockInstrument::Creeper,
                note: 7i32,
                powered: true,
            } => Ok(1495u32),
            NoteBlock {
                instrument: NoteBlockInstrument::Creeper,
                note: 7i32,
                powered: false,
            } => Ok(1496u32),
            NoteBlock {
                instrument: NoteBlockInstrument::Creeper,
                note: 8i32,
                powered: true,
            } => Ok(1497u32),
            NoteBlock {
                instrument: NoteBlockInstrument::Creeper,
                note: 8i32,
                powered: false,
            } => Ok(1498u32),
            NoteBlock {
                instrument: NoteBlockInstrument::Creeper,
                note: 9i32,
                powered: true,
            } => Ok(1499u32),
            NoteBlock {
                instrument: NoteBlockInstrument::Creeper,
                note: 9i32,
                powered: false,
            } => Ok(1500u32),
            NoteBlock {
                instrument: NoteBlockInstrument::Creeper,
                note: 10i32,
                powered: true,
            } => Ok(1501u32),
            NoteBlock {
                instrument: NoteBlockInstrument::Creeper,
                note: 10i32,
                powered: false,
            } => Ok(1502u32),
            NoteBlock {
                instrument: NoteBlockInstrument::Creeper,
                note: 11i32,
                powered: true,
            } => Ok(1503u32),
            NoteBlock {
                instrument: NoteBlockInstrument::Creeper,
                note: 11i32,
                powered: false,
            } => Ok(1504u32),
            NoteBlock {
                instrument: NoteBlockInstrument::Creeper,
                note: 12i32,
                powered: true,
            } => Ok(1505u32),
            NoteBlock {
                instrument: NoteBlockInstrument::Creeper,
                note: 12i32,
                powered: false,
            } => Ok(1506u32),
            NoteBlock {
                instrument: NoteBlockInstrument::Creeper,
                note: 13i32,
                powered: true,
            } => Ok(1507u32),
            NoteBlock {
                instrument: NoteBlockInstrument::Creeper,
                note: 13i32,
                powered: false,
            } => Ok(1508u32),
            NoteBlock {
                instrument: NoteBlockInstrument::Creeper,
                note: 14i32,
                powered: true,
            } => Ok(1509u32),
            NoteBlock {
                instrument: NoteBlockInstrument::Creeper,
                note: 14i32,
                powered: false,
            } => Ok(1510u32),
            NoteBlock {
                instrument: NoteBlockInstrument::Creeper,
                note: 15i32,
                powered: true,
            } => Ok(1511u32),
            NoteBlock {
                instrument: NoteBlockInstrument::Creeper,
                note: 15i32,
                powered: false,
            } => Ok(1512u32),
            NoteBlock {
                instrument: NoteBlockInstrument::Creeper,
                note: 16i32,
                powered: true,
            } => Ok(1513u32),
            NoteBlock {
                instrument: NoteBlockInstrument::Creeper,
                note: 16i32,
                powered: false,
            } => Ok(1514u32),
            NoteBlock {
                instrument: NoteBlockInstrument::Creeper,
                note: 17i32,
                powered: true,
            } => Ok(1515u32),
            NoteBlock {
                instrument: NoteBlockInstrument::Creeper,
                note: 17i32,
                powered: false,
            } => Ok(1516u32),
            NoteBlock {
                instrument: NoteBlockInstrument::Creeper,
                note: 18i32,
                powered: true,
            } => Ok(1517u32),
            NoteBlock {
                instrument: NoteBlockInstrument::Creeper,
                note: 18i32,
                powered: false,
            } => Ok(1518u32),
            NoteBlock {
                instrument: NoteBlockInstrument::Creeper,
                note: 19i32,
                powered: true,
            } => Ok(1519u32),
            NoteBlock {
                instrument: NoteBlockInstrument::Creeper,
                note: 19i32,
                powered: false,
            } => Ok(1520u32),
            NoteBlock {
                instrument: NoteBlockInstrument::Creeper,
                note: 20i32,
                powered: true,
            } => Ok(1521u32),
            NoteBlock {
                instrument: NoteBlockInstrument::Creeper,
                note: 20i32,
                powered: false,
            } => Ok(1522u32),
            NoteBlock {
                instrument: NoteBlockInstrument::Creeper,
                note: 21i32,
                powered: true,
            } => Ok(1523u32),
            NoteBlock {
                instrument: NoteBlockInstrument::Creeper,
                note: 21i32,
                powered: false,
            } => Ok(1524u32),
            NoteBlock {
                instrument: NoteBlockInstrument::Creeper,
                note: 22i32,
                powered: true,
            } => Ok(1525u32),
            NoteBlock {
                instrument: NoteBlockInstrument::Creeper,
                note: 22i32,
                powered: false,
            } => Ok(1526u32),
            NoteBlock {
                instrument: NoteBlockInstrument::Creeper,
                note: 23i32,
                powered: true,
            } => Ok(1527u32),
            NoteBlock {
                instrument: NoteBlockInstrument::Creeper,
                note: 23i32,
                powered: false,
            } => Ok(1528u32),
            NoteBlock {
                instrument: NoteBlockInstrument::Creeper,
                note: 24i32,
                powered: true,
            } => Ok(1529u32),
            NoteBlock {
                instrument: NoteBlockInstrument::Creeper,
                note: 24i32,
                powered: false,
            } => Ok(1530u32),
            NoteBlock {
                instrument: NoteBlockInstrument::Dragon,
                note: 0i32,
                powered: true,
            } => Ok(1531u32),
            NoteBlock {
                instrument: NoteBlockInstrument::Dragon,
                note: 0i32,
                powered: false,
            } => Ok(1532u32),
            NoteBlock {
                instrument: NoteBlockInstrument::Dragon,
                note: 1i32,
                powered: true,
            } => Ok(1533u32),
            NoteBlock {
                instrument: NoteBlockInstrument::Dragon,
                note: 1i32,
                powered: false,
            } => Ok(1534u32),
            NoteBlock {
                instrument: NoteBlockInstrument::Dragon,
                note: 2i32,
                powered: true,
            } => Ok(1535u32),
            NoteBlock {
                instrument: NoteBlockInstrument::Dragon,
                note: 2i32,
                powered: false,
            } => Ok(1536u32),
            NoteBlock {
                instrument: NoteBlockInstrument::Dragon,
                note: 3i32,
                powered: true,
            } => Ok(1537u32),
            NoteBlock {
                instrument: NoteBlockInstrument::Dragon,
                note: 3i32,
                powered: false,
            } => Ok(1538u32),
            NoteBlock {
                instrument: NoteBlockInstrument::Dragon,
                note: 4i32,
                powered: true,
            } => Ok(1539u32),
            NoteBlock {
                instrument: NoteBlockInstrument::Dragon,
                note: 4i32,
                powered: false,
            } => Ok(1540u32),
            NoteBlock {
                instrument: NoteBlockInstrument::Dragon,
                note: 5i32,
                powered: true,
            } => Ok(1541u32),
            NoteBlock {
                instrument: NoteBlockInstrument::Dragon,
                note: 5i32,
                powered: false,
            } => Ok(1542u32),
            NoteBlock {
                instrument: NoteBlockInstrument::Dragon,
                note: 6i32,
                powered: true,
            } => Ok(1543u32),
            NoteBlock {
                instrument: NoteBlockInstrument::Dragon,
                note: 6i32,
                powered: false,
            } => Ok(1544u32),
            NoteBlock {
                instrument: NoteBlockInstrument::Dragon,
                note: 7i32,
                powered: true,
            } => Ok(1545u32),
            NoteBlock {
                instrument: NoteBlockInstrument::Dragon,
                note: 7i32,
                powered: false,
            } => Ok(1546u32),
            NoteBlock {
                instrument: NoteBlockInstrument::Dragon,
                note: 8i32,
                powered: true,
            } => Ok(1547u32),
            NoteBlock {
                instrument: NoteBlockInstrument::Dragon,
                note: 8i32,
                powered: false,
            } => Ok(1548u32),
            NoteBlock {
                instrument: NoteBlockInstrument::Dragon,
                note: 9i32,
                powered: true,
            } => Ok(1549u32),
            NoteBlock {
                instrument: NoteBlockInstrument::Dragon,
                note: 9i32,
                powered: false,
            } => Ok(1550u32),
            NoteBlock {
                instrument: NoteBlockInstrument::Dragon,
                note: 10i32,
                powered: true,
            } => Ok(1551u32),
            NoteBlock {
                instrument: NoteBlockInstrument::Dragon,
                note: 10i32,
                powered: false,
            } => Ok(1552u32),
            NoteBlock {
                instrument: NoteBlockInstrument::Dragon,
                note: 11i32,
                powered: true,
            } => Ok(1553u32),
            NoteBlock {
                instrument: NoteBlockInstrument::Dragon,
                note: 11i32,
                powered: false,
            } => Ok(1554u32),
            NoteBlock {
                instrument: NoteBlockInstrument::Dragon,
                note: 12i32,
                powered: true,
            } => Ok(1555u32),
            NoteBlock {
                instrument: NoteBlockInstrument::Dragon,
                note: 12i32,
                powered: false,
            } => Ok(1556u32),
            NoteBlock {
                instrument: NoteBlockInstrument::Dragon,
                note: 13i32,
                powered: true,
            } => Ok(1557u32),
            NoteBlock {
                instrument: NoteBlockInstrument::Dragon,
                note: 13i32,
                powered: false,
            } => Ok(1558u32),
            NoteBlock {
                instrument: NoteBlockInstrument::Dragon,
                note: 14i32,
                powered: true,
            } => Ok(1559u32),
            NoteBlock {
                instrument: NoteBlockInstrument::Dragon,
                note: 14i32,
                powered: false,
            } => Ok(1560u32),
            NoteBlock {
                instrument: NoteBlockInstrument::Dragon,
                note: 15i32,
                powered: true,
            } => Ok(1561u32),
            NoteBlock {
                instrument: NoteBlockInstrument::Dragon,
                note: 15i32,
                powered: false,
            } => Ok(1562u32),
            NoteBlock {
                instrument: NoteBlockInstrument::Dragon,
                note: 16i32,
                powered: true,
            } => Ok(1563u32),
            NoteBlock {
                instrument: NoteBlockInstrument::Dragon,
                note: 16i32,
                powered: false,
            } => Ok(1564u32),
            NoteBlock {
                instrument: NoteBlockInstrument::Dragon,
                note: 17i32,
                powered: true,
            } => Ok(1565u32),
            NoteBlock {
                instrument: NoteBlockInstrument::Dragon,
                note: 17i32,
                powered: false,
            } => Ok(1566u32),
            NoteBlock {
                instrument: NoteBlockInstrument::Dragon,
                note: 18i32,
                powered: true,
            } => Ok(1567u32),
            NoteBlock {
                instrument: NoteBlockInstrument::Dragon,
                note: 18i32,
                powered: false,
            } => Ok(1568u32),
            NoteBlock {
                instrument: NoteBlockInstrument::Dragon,
                note: 19i32,
                powered: true,
            } => Ok(1569u32),
            NoteBlock {
                instrument: NoteBlockInstrument::Dragon,
                note: 19i32,
                powered: false,
            } => Ok(1570u32),
            NoteBlock {
                instrument: NoteBlockInstrument::Dragon,
                note: 20i32,
                powered: true,
            } => Ok(1571u32),
            NoteBlock {
                instrument: NoteBlockInstrument::Dragon,
                note: 20i32,
                powered: false,
            } => Ok(1572u32),
            NoteBlock {
                instrument: NoteBlockInstrument::Dragon,
                note: 21i32,
                powered: true,
            } => Ok(1573u32),
            NoteBlock {
                instrument: NoteBlockInstrument::Dragon,
                note: 21i32,
                powered: false,
            } => Ok(1574u32),
            NoteBlock {
                instrument: NoteBlockInstrument::Dragon,
                note: 22i32,
                powered: true,
            } => Ok(1575u32),
            NoteBlock {
                instrument: NoteBlockInstrument::Dragon,
                note: 22i32,
                powered: false,
            } => Ok(1576u32),
            NoteBlock {
                instrument: NoteBlockInstrument::Dragon,
                note: 23i32,
                powered: true,
            } => Ok(1577u32),
            NoteBlock {
                instrument: NoteBlockInstrument::Dragon,
                note: 23i32,
                powered: false,
            } => Ok(1578u32),
            NoteBlock {
                instrument: NoteBlockInstrument::Dragon,
                note: 24i32,
                powered: true,
            } => Ok(1579u32),
            NoteBlock {
                instrument: NoteBlockInstrument::Dragon,
                note: 24i32,
                powered: false,
            } => Ok(1580u32),
            NoteBlock {
                instrument: NoteBlockInstrument::WitherSkeleton,
                note: 0i32,
                powered: true,
            } => Ok(1581u32),
            NoteBlock {
                instrument: NoteBlockInstrument::WitherSkeleton,
                note: 0i32,
                powered: false,
            } => Ok(1582u32),
            NoteBlock {
                instrument: NoteBlockInstrument::WitherSkeleton,
                note: 1i32,
                powered: true,
            } => Ok(1583u32),
            NoteBlock {
                instrument: NoteBlockInstrument::WitherSkeleton,
                note: 1i32,
                powered: false,
            } => Ok(1584u32),
            NoteBlock {
                instrument: NoteBlockInstrument::WitherSkeleton,
                note: 2i32,
                powered: true,
            } => Ok(1585u32),
            NoteBlock {
                instrument: NoteBlockInstrument::WitherSkeleton,
                note: 2i32,
                powered: false,
            } => Ok(1586u32),
            NoteBlock {
                instrument: NoteBlockInstrument::WitherSkeleton,
                note: 3i32,
                powered: true,
            } => Ok(1587u32),
            NoteBlock {
                instrument: NoteBlockInstrument::WitherSkeleton,
                note: 3i32,
                powered: false,
            } => Ok(1588u32),
            NoteBlock {
                instrument: NoteBlockInstrument::WitherSkeleton,
                note: 4i32,
                powered: true,
            } => Ok(1589u32),
            NoteBlock {
                instrument: NoteBlockInstrument::WitherSkeleton,
                note: 4i32,
                powered: false,
            } => Ok(1590u32),
            NoteBlock {
                instrument: NoteBlockInstrument::WitherSkeleton,
                note: 5i32,
                powered: true,
            } => Ok(1591u32),
            NoteBlock {
                instrument: NoteBlockInstrument::WitherSkeleton,
                note: 5i32,
                powered: false,
            } => Ok(1592u32),
            NoteBlock {
                instrument: NoteBlockInstrument::WitherSkeleton,
                note: 6i32,
                powered: true,
            } => Ok(1593u32),
            NoteBlock {
                instrument: NoteBlockInstrument::WitherSkeleton,
                note: 6i32,
                powered: false,
            } => Ok(1594u32),
            NoteBlock {
                instrument: NoteBlockInstrument::WitherSkeleton,
                note: 7i32,
                powered: true,
            } => Ok(1595u32),
            NoteBlock {
                instrument: NoteBlockInstrument::WitherSkeleton,
                note: 7i32,
                powered: false,
            } => Ok(1596u32),
            NoteBlock {
                instrument: NoteBlockInstrument::WitherSkeleton,
                note: 8i32,
                powered: true,
            } => Ok(1597u32),
            NoteBlock {
                instrument: NoteBlockInstrument::WitherSkeleton,
                note: 8i32,
                powered: false,
            } => Ok(1598u32),
            NoteBlock {
                instrument: NoteBlockInstrument::WitherSkeleton,
                note: 9i32,
                powered: true,
            } => Ok(1599u32),
            NoteBlock {
                instrument: NoteBlockInstrument::WitherSkeleton,
                note: 9i32,
                powered: false,
            } => Ok(1600u32),
            NoteBlock {
                instrument: NoteBlockInstrument::WitherSkeleton,
                note: 10i32,
                powered: true,
            } => Ok(1601u32),
            NoteBlock {
                instrument: NoteBlockInstrument::WitherSkeleton,
                note: 10i32,
                powered: false,
            } => Ok(1602u32),
            NoteBlock {
                instrument: NoteBlockInstrument::WitherSkeleton,
                note: 11i32,
                powered: true,
            } => Ok(1603u32),
            NoteBlock {
                instrument: NoteBlockInstrument::WitherSkeleton,
                note: 11i32,
                powered: false,
            } => Ok(1604u32),
            NoteBlock {
                instrument: NoteBlockInstrument::WitherSkeleton,
                note: 12i32,
                powered: true,
            } => Ok(1605u32),
            NoteBlock {
                instrument: NoteBlockInstrument::WitherSkeleton,
                note: 12i32,
                powered: false,
            } => Ok(1606u32),
            NoteBlock {
                instrument: NoteBlockInstrument::WitherSkeleton,
                note: 13i32,
                powered: true,
            } => Ok(1607u32),
            NoteBlock {
                instrument: NoteBlockInstrument::WitherSkeleton,
                note: 13i32,
                powered: false,
            } => Ok(1608u32),
            NoteBlock {
                instrument: NoteBlockInstrument::WitherSkeleton,
                note: 14i32,
                powered: true,
            } => Ok(1609u32),
            NoteBlock {
                instrument: NoteBlockInstrument::WitherSkeleton,
                note: 14i32,
                powered: false,
            } => Ok(1610u32),
            NoteBlock {
                instrument: NoteBlockInstrument::WitherSkeleton,
                note: 15i32,
                powered: true,
            } => Ok(1611u32),
            NoteBlock {
                instrument: NoteBlockInstrument::WitherSkeleton,
                note: 15i32,
                powered: false,
            } => Ok(1612u32),
            NoteBlock {
                instrument: NoteBlockInstrument::WitherSkeleton,
                note: 16i32,
                powered: true,
            } => Ok(1613u32),
            NoteBlock {
                instrument: NoteBlockInstrument::WitherSkeleton,
                note: 16i32,
                powered: false,
            } => Ok(1614u32),
            NoteBlock {
                instrument: NoteBlockInstrument::WitherSkeleton,
                note: 17i32,
                powered: true,
            } => Ok(1615u32),
            NoteBlock {
                instrument: NoteBlockInstrument::WitherSkeleton,
                note: 17i32,
                powered: false,
            } => Ok(1616u32),
            NoteBlock {
                instrument: NoteBlockInstrument::WitherSkeleton,
                note: 18i32,
                powered: true,
            } => Ok(1617u32),
            NoteBlock {
                instrument: NoteBlockInstrument::WitherSkeleton,
                note: 18i32,
                powered: false,
            } => Ok(1618u32),
            NoteBlock {
                instrument: NoteBlockInstrument::WitherSkeleton,
                note: 19i32,
                powered: true,
            } => Ok(1619u32),
            NoteBlock {
                instrument: NoteBlockInstrument::WitherSkeleton,
                note: 19i32,
                powered: false,
            } => Ok(1620u32),
            NoteBlock {
                instrument: NoteBlockInstrument::WitherSkeleton,
                note: 20i32,
                powered: true,
            } => Ok(1621u32),
            NoteBlock {
                instrument: NoteBlockInstrument::WitherSkeleton,
                note: 20i32,
                powered: false,
            } => Ok(1622u32),
            NoteBlock {
                instrument: NoteBlockInstrument::WitherSkeleton,
                note: 21i32,
                powered: true,
            } => Ok(1623u32),
            NoteBlock {
                instrument: NoteBlockInstrument::WitherSkeleton,
                note: 21i32,
                powered: false,
            } => Ok(1624u32),
            NoteBlock {
                instrument: NoteBlockInstrument::WitherSkeleton,
                note: 22i32,
                powered: true,
            } => Ok(1625u32),
            NoteBlock {
                instrument: NoteBlockInstrument::WitherSkeleton,
                note: 22i32,
                powered: false,
            } => Ok(1626u32),
            NoteBlock {
                instrument: NoteBlockInstrument::WitherSkeleton,
                note: 23i32,
                powered: true,
            } => Ok(1627u32),
            NoteBlock {
                instrument: NoteBlockInstrument::WitherSkeleton,
                note: 23i32,
                powered: false,
            } => Ok(1628u32),
            NoteBlock {
                instrument: NoteBlockInstrument::WitherSkeleton,
                note: 24i32,
                powered: true,
            } => Ok(1629u32),
            NoteBlock {
                instrument: NoteBlockInstrument::WitherSkeleton,
                note: 24i32,
                powered: false,
            } => Ok(1630u32),
            NoteBlock {
                instrument: NoteBlockInstrument::Piglin,
                note: 0i32,
                powered: true,
            } => Ok(1631u32),
            NoteBlock {
                instrument: NoteBlockInstrument::Piglin,
                note: 0i32,
                powered: false,
            } => Ok(1632u32),
            NoteBlock {
                instrument: NoteBlockInstrument::Piglin,
                note: 1i32,
                powered: true,
            } => Ok(1633u32),
            NoteBlock {
                instrument: NoteBlockInstrument::Piglin,
                note: 1i32,
                powered: false,
            } => Ok(1634u32),
            NoteBlock {
                instrument: NoteBlockInstrument::Piglin,
                note: 2i32,
                powered: true,
            } => Ok(1635u32),
            NoteBlock {
                instrument: NoteBlockInstrument::Piglin,
                note: 2i32,
                powered: false,
            } => Ok(1636u32),
            NoteBlock {
                instrument: NoteBlockInstrument::Piglin,
                note: 3i32,
                powered: true,
            } => Ok(1637u32),
            NoteBlock {
                instrument: NoteBlockInstrument::Piglin,
                note: 3i32,
                powered: false,
            } => Ok(1638u32),
            NoteBlock {
                instrument: NoteBlockInstrument::Piglin,
                note: 4i32,
                powered: true,
            } => Ok(1639u32),
            NoteBlock {
                instrument: NoteBlockInstrument::Piglin,
                note: 4i32,
                powered: false,
            } => Ok(1640u32),
            NoteBlock {
                instrument: NoteBlockInstrument::Piglin,
                note: 5i32,
                powered: true,
            } => Ok(1641u32),
            NoteBlock {
                instrument: NoteBlockInstrument::Piglin,
                note: 5i32,
                powered: false,
            } => Ok(1642u32),
            NoteBlock {
                instrument: NoteBlockInstrument::Piglin,
                note: 6i32,
                powered: true,
            } => Ok(1643u32),
            NoteBlock {
                instrument: NoteBlockInstrument::Piglin,
                note: 6i32,
                powered: false,
            } => Ok(1644u32),
            NoteBlock {
                instrument: NoteBlockInstrument::Piglin,
                note: 7i32,
                powered: true,
            } => Ok(1645u32),
            NoteBlock {
                instrument: NoteBlockInstrument::Piglin,
                note: 7i32,
                powered: false,
            } => Ok(1646u32),
            NoteBlock {
                instrument: NoteBlockInstrument::Piglin,
                note: 8i32,
                powered: true,
            } => Ok(1647u32),
            NoteBlock {
                instrument: NoteBlockInstrument::Piglin,
                note: 8i32,
                powered: false,
            } => Ok(1648u32),
            NoteBlock {
                instrument: NoteBlockInstrument::Piglin,
                note: 9i32,
                powered: true,
            } => Ok(1649u32),
            NoteBlock {
                instrument: NoteBlockInstrument::Piglin,
                note: 9i32,
                powered: false,
            } => Ok(1650u32),
            NoteBlock {
                instrument: NoteBlockInstrument::Piglin,
                note: 10i32,
                powered: true,
            } => Ok(1651u32),
            NoteBlock {
                instrument: NoteBlockInstrument::Piglin,
                note: 10i32,
                powered: false,
            } => Ok(1652u32),
            NoteBlock {
                instrument: NoteBlockInstrument::Piglin,
                note: 11i32,
                powered: true,
            } => Ok(1653u32),
            NoteBlock {
                instrument: NoteBlockInstrument::Piglin,
                note: 11i32,
                powered: false,
            } => Ok(1654u32),
            NoteBlock {
                instrument: NoteBlockInstrument::Piglin,
                note: 12i32,
                powered: true,
            } => Ok(1655u32),
            NoteBlock {
                instrument: NoteBlockInstrument::Piglin,
                note: 12i32,
                powered: false,
            } => Ok(1656u32),
            NoteBlock {
                instrument: NoteBlockInstrument::Piglin,
                note: 13i32,
                powered: true,
            } => Ok(1657u32),
            NoteBlock {
                instrument: NoteBlockInstrument::Piglin,
                note: 13i32,
                powered: false,
            } => Ok(1658u32),
            NoteBlock {
                instrument: NoteBlockInstrument::Piglin,
                note: 14i32,
                powered: true,
            } => Ok(1659u32),
            NoteBlock {
                instrument: NoteBlockInstrument::Piglin,
                note: 14i32,
                powered: false,
            } => Ok(1660u32),
            NoteBlock {
                instrument: NoteBlockInstrument::Piglin,
                note: 15i32,
                powered: true,
            } => Ok(1661u32),
            NoteBlock {
                instrument: NoteBlockInstrument::Piglin,
                note: 15i32,
                powered: false,
            } => Ok(1662u32),
            NoteBlock {
                instrument: NoteBlockInstrument::Piglin,
                note: 16i32,
                powered: true,
            } => Ok(1663u32),
            NoteBlock {
                instrument: NoteBlockInstrument::Piglin,
                note: 16i32,
                powered: false,
            } => Ok(1664u32),
            NoteBlock {
                instrument: NoteBlockInstrument::Piglin,
                note: 17i32,
                powered: true,
            } => Ok(1665u32),
            NoteBlock {
                instrument: NoteBlockInstrument::Piglin,
                note: 17i32,
                powered: false,
            } => Ok(1666u32),
            NoteBlock {
                instrument: NoteBlockInstrument::Piglin,
                note: 18i32,
                powered: true,
            } => Ok(1667u32),
            NoteBlock {
                instrument: NoteBlockInstrument::Piglin,
                note: 18i32,
                powered: false,
            } => Ok(1668u32),
            NoteBlock {
                instrument: NoteBlockInstrument::Piglin,
                note: 19i32,
                powered: true,
            } => Ok(1669u32),
            NoteBlock {
                instrument: NoteBlockInstrument::Piglin,
                note: 19i32,
                powered: false,
            } => Ok(1670u32),
            NoteBlock {
                instrument: NoteBlockInstrument::Piglin,
                note: 20i32,
                powered: true,
            } => Ok(1671u32),
            NoteBlock {
                instrument: NoteBlockInstrument::Piglin,
                note: 20i32,
                powered: false,
            } => Ok(1672u32),
            NoteBlock {
                instrument: NoteBlockInstrument::Piglin,
                note: 21i32,
                powered: true,
            } => Ok(1673u32),
            NoteBlock {
                instrument: NoteBlockInstrument::Piglin,
                note: 21i32,
                powered: false,
            } => Ok(1674u32),
            NoteBlock {
                instrument: NoteBlockInstrument::Piglin,
                note: 22i32,
                powered: true,
            } => Ok(1675u32),
            NoteBlock {
                instrument: NoteBlockInstrument::Piglin,
                note: 22i32,
                powered: false,
            } => Ok(1676u32),
            NoteBlock {
                instrument: NoteBlockInstrument::Piglin,
                note: 23i32,
                powered: true,
            } => Ok(1677u32),
            NoteBlock {
                instrument: NoteBlockInstrument::Piglin,
                note: 23i32,
                powered: false,
            } => Ok(1678u32),
            NoteBlock {
                instrument: NoteBlockInstrument::Piglin,
                note: 24i32,
                powered: true,
            } => Ok(1679u32),
            NoteBlock {
                instrument: NoteBlockInstrument::Piglin,
                note: 24i32,
                powered: false,
            } => Ok(1680u32),
            NoteBlock {
                instrument: NoteBlockInstrument::CustomHead,
                note: 0i32,
                powered: true,
            } => Ok(1681u32),
            NoteBlock {
                instrument: NoteBlockInstrument::CustomHead,
                note: 0i32,
                powered: false,
            } => Ok(1682u32),
            NoteBlock {
                instrument: NoteBlockInstrument::CustomHead,
                note: 1i32,
                powered: true,
            } => Ok(1683u32),
            NoteBlock {
                instrument: NoteBlockInstrument::CustomHead,
                note: 1i32,
                powered: false,
            } => Ok(1684u32),
            NoteBlock {
                instrument: NoteBlockInstrument::CustomHead,
                note: 2i32,
                powered: true,
            } => Ok(1685u32),
            NoteBlock {
                instrument: NoteBlockInstrument::CustomHead,
                note: 2i32,
                powered: false,
            } => Ok(1686u32),
            NoteBlock {
                instrument: NoteBlockInstrument::CustomHead,
                note: 3i32,
                powered: true,
            } => Ok(1687u32),
            NoteBlock {
                instrument: NoteBlockInstrument::CustomHead,
                note: 3i32,
                powered: false,
            } => Ok(1688u32),
            NoteBlock {
                instrument: NoteBlockInstrument::CustomHead,
                note: 4i32,
                powered: true,
            } => Ok(1689u32),
            NoteBlock {
                instrument: NoteBlockInstrument::CustomHead,
                note: 4i32,
                powered: false,
            } => Ok(1690u32),
            NoteBlock {
                instrument: NoteBlockInstrument::CustomHead,
                note: 5i32,
                powered: true,
            } => Ok(1691u32),
            NoteBlock {
                instrument: NoteBlockInstrument::CustomHead,
                note: 5i32,
                powered: false,
            } => Ok(1692u32),
            NoteBlock {
                instrument: NoteBlockInstrument::CustomHead,
                note: 6i32,
                powered: true,
            } => Ok(1693u32),
            NoteBlock {
                instrument: NoteBlockInstrument::CustomHead,
                note: 6i32,
                powered: false,
            } => Ok(1694u32),
            NoteBlock {
                instrument: NoteBlockInstrument::CustomHead,
                note: 7i32,
                powered: true,
            } => Ok(1695u32),
            NoteBlock {
                instrument: NoteBlockInstrument::CustomHead,
                note: 7i32,
                powered: false,
            } => Ok(1696u32),
            NoteBlock {
                instrument: NoteBlockInstrument::CustomHead,
                note: 8i32,
                powered: true,
            } => Ok(1697u32),
            NoteBlock {
                instrument: NoteBlockInstrument::CustomHead,
                note: 8i32,
                powered: false,
            } => Ok(1698u32),
            NoteBlock {
                instrument: NoteBlockInstrument::CustomHead,
                note: 9i32,
                powered: true,
            } => Ok(1699u32),
            NoteBlock {
                instrument: NoteBlockInstrument::CustomHead,
                note: 9i32,
                powered: false,
            } => Ok(1700u32),
            NoteBlock {
                instrument: NoteBlockInstrument::CustomHead,
                note: 10i32,
                powered: true,
            } => Ok(1701u32),
            NoteBlock {
                instrument: NoteBlockInstrument::CustomHead,
                note: 10i32,
                powered: false,
            } => Ok(1702u32),
            NoteBlock {
                instrument: NoteBlockInstrument::CustomHead,
                note: 11i32,
                powered: true,
            } => Ok(1703u32),
            NoteBlock {
                instrument: NoteBlockInstrument::CustomHead,
                note: 11i32,
                powered: false,
            } => Ok(1704u32),
            NoteBlock {
                instrument: NoteBlockInstrument::CustomHead,
                note: 12i32,
                powered: true,
            } => Ok(1705u32),
            NoteBlock {
                instrument: NoteBlockInstrument::CustomHead,
                note: 12i32,
                powered: false,
            } => Ok(1706u32),
            NoteBlock {
                instrument: NoteBlockInstrument::CustomHead,
                note: 13i32,
                powered: true,
            } => Ok(1707u32),
            NoteBlock {
                instrument: NoteBlockInstrument::CustomHead,
                note: 13i32,
                powered: false,
            } => Ok(1708u32),
            NoteBlock {
                instrument: NoteBlockInstrument::CustomHead,
                note: 14i32,
                powered: true,
            } => Ok(1709u32),
            NoteBlock {
                instrument: NoteBlockInstrument::CustomHead,
                note: 14i32,
                powered: false,
            } => Ok(1710u32),
            NoteBlock {
                instrument: NoteBlockInstrument::CustomHead,
                note: 15i32,
                powered: true,
            } => Ok(1711u32),
            NoteBlock {
                instrument: NoteBlockInstrument::CustomHead,
                note: 15i32,
                powered: false,
            } => Ok(1712u32),
            NoteBlock {
                instrument: NoteBlockInstrument::CustomHead,
                note: 16i32,
                powered: true,
            } => Ok(1713u32),
            NoteBlock {
                instrument: NoteBlockInstrument::CustomHead,
                note: 16i32,
                powered: false,
            } => Ok(1714u32),
            NoteBlock {
                instrument: NoteBlockInstrument::CustomHead,
                note: 17i32,
                powered: true,
            } => Ok(1715u32),
            NoteBlock {
                instrument: NoteBlockInstrument::CustomHead,
                note: 17i32,
                powered: false,
            } => Ok(1716u32),
            NoteBlock {
                instrument: NoteBlockInstrument::CustomHead,
                note: 18i32,
                powered: true,
            } => Ok(1717u32),
            NoteBlock {
                instrument: NoteBlockInstrument::CustomHead,
                note: 18i32,
                powered: false,
            } => Ok(1718u32),
            NoteBlock {
                instrument: NoteBlockInstrument::CustomHead,
                note: 19i32,
                powered: true,
            } => Ok(1719u32),
            NoteBlock {
                instrument: NoteBlockInstrument::CustomHead,
                note: 19i32,
                powered: false,
            } => Ok(1720u32),
            NoteBlock {
                instrument: NoteBlockInstrument::CustomHead,
                note: 20i32,
                powered: true,
            } => Ok(1721u32),
            NoteBlock {
                instrument: NoteBlockInstrument::CustomHead,
                note: 20i32,
                powered: false,
            } => Ok(1722u32),
            NoteBlock {
                instrument: NoteBlockInstrument::CustomHead,
                note: 21i32,
                powered: true,
            } => Ok(1723u32),
            NoteBlock {
                instrument: NoteBlockInstrument::CustomHead,
                note: 21i32,
                powered: false,
            } => Ok(1724u32),
            NoteBlock {
                instrument: NoteBlockInstrument::CustomHead,
                note: 22i32,
                powered: true,
            } => Ok(1725u32),
            NoteBlock {
                instrument: NoteBlockInstrument::CustomHead,
                note: 22i32,
                powered: false,
            } => Ok(1726u32),
            NoteBlock {
                instrument: NoteBlockInstrument::CustomHead,
                note: 23i32,
                powered: true,
            } => Ok(1727u32),
            NoteBlock {
                instrument: NoteBlockInstrument::CustomHead,
                note: 23i32,
                powered: false,
            } => Ok(1728u32),
            NoteBlock {
                instrument: NoteBlockInstrument::CustomHead,
                note: 24i32,
                powered: true,
            } => Ok(1729u32),
            NoteBlock {
                instrument: NoteBlockInstrument::CustomHead,
                note: 24i32,
                powered: false,
            } => Ok(1730u32),
            _ => Err(()),
        }
    }
}
