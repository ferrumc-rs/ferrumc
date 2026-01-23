use ferrumc_block_properties::*;
#[allow(dead_code, non_camel_case_types)]
pub enum GeneratedBlock_0_BlockType {
    Beetroots,
    Cactus,
    Carrots,
    ChorusFlower,
    FrostedIce,
    Kelp,
    MelonStem,
    NetherWart,
    Potatoes,
    PumpkinStem,
    SugarCane,
    SweetBerryBush,
    TorchflowerCrop,
    TwistingVines,
    WeepingVines,
    Wheat,
}
#[allow(dead_code, non_camel_case_types)]
pub struct GeneratedBlock_0 {
    pub block_type: GeneratedBlock_0_BlockType,
    pub age: i32,
}
#[allow(dead_code, non_camel_case_types)]
pub struct GeneratedBlock_CaveVines {
    pub age: i32,
    pub berries: bool,
}
#[allow(dead_code, non_camel_case_types)]
pub struct GeneratedBlock_Fire {
    pub age: i32,
    pub east: bool,
    pub north: bool,
    pub south: bool,
    pub up: bool,
    pub west: bool,
}
#[allow(dead_code, non_camel_case_types)]
pub struct GeneratedBlock_Cocoa {
    pub age: i32,
    pub facing: Direction,
}
#[allow(dead_code, non_camel_case_types)]
pub struct GeneratedBlock_PitcherCrop {
    pub age: i32,
    pub half: DoubleBlockHalf,
}
#[allow(dead_code, non_camel_case_types)]
pub struct GeneratedBlock_MangrovePropagule {
    pub age: i32,
    pub hanging: bool,
    pub stage: i32,
    pub waterlogged: bool,
}
#[allow(dead_code, non_camel_case_types)]
pub struct GeneratedBlock_Bamboo {
    pub age: i32,
    pub leaves: BambooLeaves,
    pub stage: i32,
}
#[allow(dead_code, non_camel_case_types)]
pub struct GeneratedBlock_Tripwire {
    pub attached: bool,
    pub disarmed: bool,
    pub east: bool,
    pub north: bool,
    pub powered: bool,
    pub south: bool,
    pub west: bool,
}
#[allow(dead_code, non_camel_case_types)]
pub struct GeneratedBlock_TripwireHook {
    pub attached: bool,
    pub facing: Direction,
    pub powered: bool,
}
#[allow(dead_code, non_camel_case_types)]
pub enum GeneratedBlock_9_BlockType {
    AcaciaHangingSign,
    BambooHangingSign,
    BirchHangingSign,
    CherryHangingSign,
    CrimsonHangingSign,
    DarkOakHangingSign,
    JungleHangingSign,
    MangroveHangingSign,
    OakHangingSign,
    PaleOakHangingSign,
    SpruceHangingSign,
    WarpedHangingSign,
}
#[allow(dead_code, non_camel_case_types)]
pub struct GeneratedBlock_9 {
    pub block_type: GeneratedBlock_9_BlockType,
    pub attached: bool,
    pub rotation: i32,
    pub waterlogged: bool,
}
#[allow(dead_code, non_camel_case_types)]
pub struct GeneratedBlock_Bell {
    pub attachment: BellAttachType,
    pub facing: Direction,
    pub powered: bool,
}
#[allow(dead_code, non_camel_case_types)]
pub enum GeneratedBlock_11_BlockType {
    AcaciaLog,
    AcaciaWood,
    BambooBlock,
    Basalt,
    BirchLog,
    BirchWood,
    BoneBlock,
    CherryLog,
    CherryWood,
    CrimsonHyphae,
    CrimsonStem,
    DarkOakLog,
    DarkOakWood,
    Deepslate,
    HayBlock,
    InfestedDeepslate,
    JungleLog,
    JungleWood,
    MangroveLog,
    MangroveWood,
    MuddyMangroveRoots,
    NetherPortal,
    OakLog,
    OakWood,
    OchreFroglight,
    PaleOakLog,
    PaleOakWood,
    PearlescentFroglight,
    PolishedBasalt,
    PurpurPillar,
    QuartzPillar,
    SpruceLog,
    SpruceWood,
    StrippedAcaciaLog,
    StrippedAcaciaWood,
    StrippedBambooBlock,
    StrippedBirchLog,
    StrippedBirchWood,
    StrippedCherryLog,
    StrippedCherryWood,
    StrippedCrimsonHyphae,
    StrippedCrimsonStem,
    StrippedDarkOakLog,
    StrippedDarkOakWood,
    StrippedJungleLog,
    StrippedJungleWood,
    StrippedMangroveLog,
    StrippedMangroveWood,
    StrippedOakLog,
    StrippedOakWood,
    StrippedPaleOakLog,
    StrippedPaleOakWood,
    StrippedSpruceLog,
    StrippedSpruceWood,
    StrippedWarpedHyphae,
    StrippedWarpedStem,
    VerdantFroglight,
    WarpedHyphae,
    WarpedStem,
}
#[allow(dead_code, non_camel_case_types)]
pub struct GeneratedBlock_11 {
    pub block_type: GeneratedBlock_11_BlockType,
    pub axis: Axis,
}
#[allow(dead_code, non_camel_case_types)]
pub struct GeneratedBlock_CreakingHeart {
    pub axis: Axis,
    pub creaking_heart_state: CreakingHeartState,
    pub natural: bool,
}
#[allow(dead_code, non_camel_case_types)]
pub struct GeneratedBlock_Chain {
    pub axis: Axis,
    pub waterlogged: bool,
}
#[allow(dead_code, non_camel_case_types)]
pub struct GeneratedBlock_CaveVinesPlant {
    pub berries: bool,
}
#[allow(dead_code, non_camel_case_types)]
pub struct GeneratedBlock_Cake {
    pub bites: i32,
}
#[allow(dead_code, non_camel_case_types)]
pub struct GeneratedBlock_SculkCatalyst {
    pub bloom: bool,
}
#[allow(dead_code, non_camel_case_types)]
pub struct GeneratedBlock_Scaffolding {
    pub bottom: bool,
    pub distance: i32,
    pub waterlogged: bool,
}
#[allow(dead_code, non_camel_case_types)]
pub struct GeneratedBlock_PaleMossCarpet {
    pub bottom: bool,
    pub east: WallSide,
    pub north: WallSide,
    pub south: WallSide,
    pub west: WallSide,
}
#[allow(dead_code, non_camel_case_types)]
pub struct GeneratedBlock_SculkShrieker {
    pub can_summon: bool,
    pub shrieking: bool,
    pub waterlogged: bool,
}
#[allow(dead_code, non_camel_case_types)]
pub enum GeneratedBlock_20_BlockType {
    BlackCandle,
    BlueCandle,
    BrownCandle,
    Candle,
    CyanCandle,
    GrayCandle,
    GreenCandle,
    LightBlueCandle,
    LightGrayCandle,
    LimeCandle,
    MagentaCandle,
    OrangeCandle,
    PinkCandle,
    PurpleCandle,
    RedCandle,
    WhiteCandle,
    YellowCandle,
}
#[allow(dead_code, non_camel_case_types)]
pub struct GeneratedBlock_20 {
    pub block_type: GeneratedBlock_20_BlockType,
    pub candles: i32,
    pub lit: bool,
    pub waterlogged: bool,
}
#[allow(dead_code, non_camel_case_types)]
pub struct GeneratedBlock_RespawnAnchor {
    pub charges: i32,
}
#[allow(dead_code, non_camel_case_types)]
pub enum GeneratedBlock_22_BlockType {
    ChainCommandBlock,
    CommandBlock,
    RepeatingCommandBlock,
}
#[allow(dead_code, non_camel_case_types)]
pub struct GeneratedBlock_22 {
    pub block_type: GeneratedBlock_22_BlockType,
    pub conditional: bool,
    pub facing: Direction,
}
#[allow(dead_code, non_camel_case_types)]
pub struct GeneratedBlock_DecoratedPot {
    pub cracked: bool,
    pub facing: Direction,
    pub waterlogged: bool,
}
#[allow(dead_code, non_camel_case_types)]
pub struct GeneratedBlock_Crafter {
    pub crafting: bool,
    pub orientation: FrontAndTop,
    pub triggered: bool,
}
#[allow(dead_code, non_camel_case_types)]
pub struct GeneratedBlock_Repeater {
    pub delay: i32,
    pub facing: Direction,
    pub locked: bool,
    pub powered: bool,
}
#[allow(dead_code, non_camel_case_types)]
pub enum GeneratedBlock_26_BlockType {
    AcaciaLeaves,
    AzaleaLeaves,
    BirchLeaves,
    CherryLeaves,
    DarkOakLeaves,
    FloweringAzaleaLeaves,
    JungleLeaves,
    MangroveLeaves,
    OakLeaves,
    PaleOakLeaves,
    SpruceLeaves,
}
#[allow(dead_code, non_camel_case_types)]
pub struct GeneratedBlock_26 {
    pub block_type: GeneratedBlock_26_BlockType,
    pub distance: i32,
    pub persistent: bool,
    pub waterlogged: bool,
}
#[allow(dead_code, non_camel_case_types)]
pub enum GeneratedBlock_27_BlockType {
    GlowLichen,
    ResinClump,
    SculkVein,
}
#[allow(dead_code, non_camel_case_types)]
pub struct GeneratedBlock_27 {
    pub block_type: GeneratedBlock_27_BlockType,
    pub down: bool,
    pub east: bool,
    pub north: bool,
    pub south: bool,
    pub up: bool,
    pub waterlogged: bool,
    pub west: bool,
}
#[allow(dead_code, non_camel_case_types)]
pub enum GeneratedBlock_28_BlockType {
    BrownMushroomBlock,
    ChorusPlant,
    MushroomStem,
    RedMushroomBlock,
}
#[allow(dead_code, non_camel_case_types)]
pub struct GeneratedBlock_28 {
    pub block_type: GeneratedBlock_28_BlockType,
    pub down: bool,
    pub east: bool,
    pub north: bool,
    pub south: bool,
    pub up: bool,
    pub west: bool,
}
#[allow(dead_code, non_camel_case_types)]
pub struct GeneratedBlock_BubbleColumn {
    pub drag: bool,
}
#[allow(dead_code, non_camel_case_types)]
pub enum GeneratedBlock_30_BlockType {
    SuspiciousGravel,
    SuspiciousSand,
}
#[allow(dead_code, non_camel_case_types)]
pub struct GeneratedBlock_30 {
    pub block_type: GeneratedBlock_30_BlockType,
    pub dusted: i32,
}
#[allow(dead_code, non_camel_case_types)]
pub struct GeneratedBlock_RedstoneWire {
    pub east: RedstoneSide,
    pub north: RedstoneSide,
    pub power: i32,
    pub south: RedstoneSide,
    pub west: RedstoneSide,
}
#[allow(dead_code, non_camel_case_types)]
pub enum GeneratedBlock_32_BlockType {
    AndesiteWall,
    BlackstoneWall,
    BrickWall,
    CobbledDeepslateWall,
    CobblestoneWall,
    DeepslateBrickWall,
    DeepslateTileWall,
    DioriteWall,
    EndStoneBrickWall,
    GraniteWall,
    MossyCobblestoneWall,
    MossyStoneBrickWall,
    MudBrickWall,
    NetherBrickWall,
    PolishedBlackstoneBrickWall,
    PolishedBlackstoneWall,
    PolishedDeepslateWall,
    PolishedTuffWall,
    PrismarineWall,
    RedNetherBrickWall,
    RedSandstoneWall,
    ResinBrickWall,
    SandstoneWall,
    StoneBrickWall,
    TuffBrickWall,
    TuffWall,
}
#[allow(dead_code, non_camel_case_types)]
pub struct GeneratedBlock_32 {
    pub block_type: GeneratedBlock_32_BlockType,
    pub east: WallSide,
    pub north: WallSide,
    pub south: WallSide,
    pub up: bool,
    pub waterlogged: bool,
    pub west: WallSide,
}
#[allow(dead_code, non_camel_case_types)]
pub struct GeneratedBlock_Vine {
    pub east: bool,
    pub north: bool,
    pub south: bool,
    pub up: bool,
    pub west: bool,
}
#[allow(dead_code, non_camel_case_types)]
pub enum GeneratedBlock_34_BlockType {
    AcaciaFence,
    BambooFence,
    BirchFence,
    BlackStainedGlassPane,
    BlueStainedGlassPane,
    BrownStainedGlassPane,
    CherryFence,
    CrimsonFence,
    CyanStainedGlassPane,
    DarkOakFence,
    GlassPane,
    GrayStainedGlassPane,
    GreenStainedGlassPane,
    IronBars,
    JungleFence,
    LightBlueStainedGlassPane,
    LightGrayStainedGlassPane,
    LimeStainedGlassPane,
    MagentaStainedGlassPane,
    MangroveFence,
    NetherBrickFence,
    OakFence,
    OrangeStainedGlassPane,
    PaleOakFence,
    PinkStainedGlassPane,
    PurpleStainedGlassPane,
    RedStainedGlassPane,
    SpruceFence,
    WarpedFence,
    WhiteStainedGlassPane,
    YellowStainedGlassPane,
}
#[allow(dead_code, non_camel_case_types)]
pub struct GeneratedBlock_34 {
    pub block_type: GeneratedBlock_34_BlockType,
    pub east: bool,
    pub north: bool,
    pub south: bool,
    pub waterlogged: bool,
    pub west: bool,
}
#[allow(dead_code, non_camel_case_types)]
pub struct GeneratedBlock_TurtleEgg {
    pub eggs: i32,
    pub hatch: i32,
}
#[allow(dead_code, non_camel_case_types)]
pub struct GeneratedBlock_Hopper {
    pub enabled: bool,
    pub facing: Direction,
}
#[allow(dead_code, non_camel_case_types)]
pub enum GeneratedBlock_37_BlockType {
    Piston,
    StickyPiston,
}
#[allow(dead_code, non_camel_case_types)]
pub struct GeneratedBlock_37 {
    pub block_type: GeneratedBlock_37_BlockType,
    pub extended: bool,
    pub facing: Direction,
}
#[allow(dead_code, non_camel_case_types)]
pub struct GeneratedBlock_EndPortalFrame {
    pub eye: bool,
    pub facing: Direction,
}
#[allow(dead_code, non_camel_case_types)]
pub struct GeneratedBlock_Grindstone {
    pub face: AttachFace,
    pub facing: Direction,
}
#[allow(dead_code, non_camel_case_types)]
pub enum GeneratedBlock_40_BlockType {
    AcaciaButton,
    BambooButton,
    BirchButton,
    CherryButton,
    CrimsonButton,
    DarkOakButton,
    JungleButton,
    Lever,
    MangroveButton,
    OakButton,
    PaleOakButton,
    PolishedBlackstoneButton,
    SpruceButton,
    StoneButton,
    WarpedButton,
}
#[allow(dead_code, non_camel_case_types)]
pub struct GeneratedBlock_40 {
    pub block_type: GeneratedBlock_40_BlockType,
    pub face: AttachFace,
    pub facing: Direction,
    pub powered: bool,
}
#[allow(dead_code, non_camel_case_types)]
pub enum GeneratedBlock_41_BlockType {
    Anvil,
    AttachedMelonStem,
    AttachedPumpkinStem,
    BlackGlazedTerracotta,
    BlackShulkerBox,
    BlackWallBanner,
    BlueGlazedTerracotta,
    BlueShulkerBox,
    BlueWallBanner,
    BrownGlazedTerracotta,
    BrownShulkerBox,
    BrownWallBanner,
    CarvedPumpkin,
    ChippedAnvil,
    CyanGlazedTerracotta,
    CyanShulkerBox,
    CyanWallBanner,
    DamagedAnvil,
    EndRod,
    GrayGlazedTerracotta,
    GrayShulkerBox,
    GrayWallBanner,
    GreenGlazedTerracotta,
    GreenShulkerBox,
    GreenWallBanner,
    JackOLantern,
    LightBlueGlazedTerracotta,
    LightBlueShulkerBox,
    LightBlueWallBanner,
    LightGrayGlazedTerracotta,
    LightGrayShulkerBox,
    LightGrayWallBanner,
    LimeGlazedTerracotta,
    LimeShulkerBox,
    LimeWallBanner,
    Loom,
    MagentaGlazedTerracotta,
    MagentaShulkerBox,
    MagentaWallBanner,
    OrangeGlazedTerracotta,
    OrangeShulkerBox,
    OrangeWallBanner,
    PinkGlazedTerracotta,
    PinkShulkerBox,
    PinkWallBanner,
    PurpleGlazedTerracotta,
    PurpleShulkerBox,
    PurpleWallBanner,
    RedGlazedTerracotta,
    RedShulkerBox,
    RedWallBanner,
    ShulkerBox,
    SoulWallTorch,
    Stonecutter,
    WallTorch,
    WhiteGlazedTerracotta,
    WhiteShulkerBox,
    WhiteWallBanner,
    YellowGlazedTerracotta,
    YellowShulkerBox,
    YellowWallBanner,
}
#[allow(dead_code, non_camel_case_types)]
pub struct GeneratedBlock_41 {
    pub block_type: GeneratedBlock_41_BlockType,
    pub facing: Direction,
}
#[allow(dead_code, non_camel_case_types)]
pub enum GeneratedBlock_42_BlockType {
    PinkPetals,
    Wildflowers,
}
#[allow(dead_code, non_camel_case_types)]
pub struct GeneratedBlock_42 {
    pub block_type: GeneratedBlock_42_BlockType,
    pub facing: Direction,
    pub flower_amount: i32,
}
#[allow(dead_code, non_camel_case_types)]
pub enum GeneratedBlock_43_BlockType {
    AcaciaDoor,
    BambooDoor,
    BirchDoor,
    CherryDoor,
    CopperDoor,
    CrimsonDoor,
    DarkOakDoor,
    ExposedCopperDoor,
    IronDoor,
    JungleDoor,
    MangroveDoor,
    OakDoor,
    OxidizedCopperDoor,
    PaleOakDoor,
    SpruceDoor,
    WarpedDoor,
    WaxedCopperDoor,
    WaxedExposedCopperDoor,
    WaxedOxidizedCopperDoor,
    WaxedWeatheredCopperDoor,
    WeatheredCopperDoor,
}
#[allow(dead_code, non_camel_case_types)]
pub struct GeneratedBlock_43 {
    pub block_type: GeneratedBlock_43_BlockType,
    pub facing: Direction,
    pub half: DoubleBlockHalf,
    pub hinge: DoorHingeSide,
    pub open: bool,
    pub powered: bool,
}
#[allow(dead_code, non_camel_case_types)]
pub struct GeneratedBlock_SmallDripleaf {
    pub facing: Direction,
    pub half: DoubleBlockHalf,
    pub waterlogged: bool,
}
#[allow(dead_code, non_camel_case_types)]
pub enum GeneratedBlock_45_BlockType {
    AcaciaTrapdoor,
    BambooTrapdoor,
    BirchTrapdoor,
    CherryTrapdoor,
    CopperTrapdoor,
    CrimsonTrapdoor,
    DarkOakTrapdoor,
    ExposedCopperTrapdoor,
    IronTrapdoor,
    JungleTrapdoor,
    MangroveTrapdoor,
    OakTrapdoor,
    OxidizedCopperTrapdoor,
    PaleOakTrapdoor,
    SpruceTrapdoor,
    WarpedTrapdoor,
    WaxedCopperTrapdoor,
    WaxedExposedCopperTrapdoor,
    WaxedOxidizedCopperTrapdoor,
    WaxedWeatheredCopperTrapdoor,
    WeatheredCopperTrapdoor,
}
#[allow(dead_code, non_camel_case_types)]
pub struct GeneratedBlock_45 {
    pub block_type: GeneratedBlock_45_BlockType,
    pub facing: Direction,
    pub half: Half,
    pub open: bool,
    pub powered: bool,
    pub waterlogged: bool,
}
#[allow(dead_code, non_camel_case_types)]
pub enum GeneratedBlock_46_BlockType {
    AcaciaStairs,
    AndesiteStairs,
    BambooMosaicStairs,
    BambooStairs,
    BirchStairs,
    BlackstoneStairs,
    BrickStairs,
    CherryStairs,
    CobbledDeepslateStairs,
    CobblestoneStairs,
    CrimsonStairs,
    CutCopperStairs,
    DarkOakStairs,
    DarkPrismarineStairs,
    DeepslateBrickStairs,
    DeepslateTileStairs,
    DioriteStairs,
    EndStoneBrickStairs,
    ExposedCutCopperStairs,
    GraniteStairs,
    JungleStairs,
    MangroveStairs,
    MossyCobblestoneStairs,
    MossyStoneBrickStairs,
    MudBrickStairs,
    NetherBrickStairs,
    OakStairs,
    OxidizedCutCopperStairs,
    PaleOakStairs,
    PolishedAndesiteStairs,
    PolishedBlackstoneBrickStairs,
    PolishedBlackstoneStairs,
    PolishedDeepslateStairs,
    PolishedDioriteStairs,
    PolishedGraniteStairs,
    PolishedTuffStairs,
    PrismarineBrickStairs,
    PrismarineStairs,
    PurpurStairs,
    QuartzStairs,
    RedNetherBrickStairs,
    RedSandstoneStairs,
    ResinBrickStairs,
    SandstoneStairs,
    SmoothQuartzStairs,
    SmoothRedSandstoneStairs,
    SmoothSandstoneStairs,
    SpruceStairs,
    StoneBrickStairs,
    StoneStairs,
    TuffBrickStairs,
    TuffStairs,
    WarpedStairs,
    WaxedCutCopperStairs,
    WaxedExposedCutCopperStairs,
    WaxedOxidizedCutCopperStairs,
    WaxedWeatheredCutCopperStairs,
    WeatheredCutCopperStairs,
}
#[allow(dead_code, non_camel_case_types)]
pub struct GeneratedBlock_46 {
    pub block_type: GeneratedBlock_46_BlockType,
    pub facing: Direction,
    pub half: Half,
    pub shape: StairsShape,
    pub waterlogged: bool,
}
#[allow(dead_code, non_camel_case_types)]
pub struct GeneratedBlock_Lectern {
    pub facing: Direction,
    pub has_book: bool,
    pub powered: bool,
}
#[allow(dead_code, non_camel_case_types)]
pub enum GeneratedBlock_48_BlockType {
    BeeNest,
    Beehive,
}
#[allow(dead_code, non_camel_case_types)]
pub struct GeneratedBlock_48 {
    pub block_type: GeneratedBlock_48_BlockType,
    pub facing: Direction,
    pub honey_level: i32,
}
#[allow(dead_code, non_camel_case_types)]
pub enum GeneratedBlock_49_BlockType {
    AcaciaFenceGate,
    BambooFenceGate,
    BirchFenceGate,
    CherryFenceGate,
    CrimsonFenceGate,
    DarkOakFenceGate,
    JungleFenceGate,
    MangroveFenceGate,
    OakFenceGate,
    PaleOakFenceGate,
    SpruceFenceGate,
    WarpedFenceGate,
}
#[allow(dead_code, non_camel_case_types)]
pub struct GeneratedBlock_49 {
    pub block_type: GeneratedBlock_49_BlockType,
    pub facing: Direction,
    pub in_wall: bool,
    pub open: bool,
    pub powered: bool,
}
#[allow(dead_code, non_camel_case_types)]
pub enum GeneratedBlock_50_BlockType {
    BlastFurnace,
    Furnace,
    RedstoneWallTorch,
    Smoker,
}
#[allow(dead_code, non_camel_case_types)]
pub struct GeneratedBlock_50 {
    pub block_type: GeneratedBlock_50_BlockType,
    pub facing: Direction,
    pub lit: bool,
}
#[allow(dead_code, non_camel_case_types)]
pub enum GeneratedBlock_51_BlockType {
    Campfire,
    SoulCampfire,
}
#[allow(dead_code, non_camel_case_types)]
pub struct GeneratedBlock_51 {
    pub block_type: GeneratedBlock_51_BlockType,
    pub facing: Direction,
    pub lit: bool,
    pub signal_fire: bool,
    pub waterlogged: bool,
}
#[allow(dead_code, non_camel_case_types)]
pub struct GeneratedBlock_Comparator {
    pub facing: Direction,
    pub mode: ComparatorMode,
    pub powered: bool,
}
#[allow(dead_code, non_camel_case_types)]
pub enum GeneratedBlock_53_BlockType {
    BlackBed,
    BlueBed,
    BrownBed,
    CyanBed,
    GrayBed,
    GreenBed,
    LightBlueBed,
    LightGrayBed,
    LimeBed,
    MagentaBed,
    OrangeBed,
    PinkBed,
    PurpleBed,
    RedBed,
    WhiteBed,
    YellowBed,
}
#[allow(dead_code, non_camel_case_types)]
pub struct GeneratedBlock_53 {
    pub block_type: GeneratedBlock_53_BlockType,
    pub facing: Direction,
    pub occupied: bool,
    pub part: BedPart,
}
#[allow(dead_code, non_camel_case_types)]
pub struct GeneratedBlock_Vault {
    pub facing: Direction,
    pub ominous: bool,
    pub vault_state: VaultState,
}
#[allow(dead_code, non_camel_case_types)]
pub struct GeneratedBlock_Barrel {
    pub facing: Direction,
    pub open: bool,
}
#[allow(dead_code, non_camel_case_types)]
pub struct GeneratedBlock_CalibratedSculkSensor {
    pub facing: Direction,
    pub power: i32,
    pub sculk_sensor_phase: SculkSensorPhase,
    pub waterlogged: bool,
}
#[allow(dead_code, non_camel_case_types)]
pub enum GeneratedBlock_57_BlockType {
    CreeperWallHead,
    DragonWallHead,
    Observer,
    PiglinWallHead,
    PlayerWallHead,
    SkeletonWallSkull,
    WitherSkeletonWallSkull,
    ZombieWallHead,
}
#[allow(dead_code, non_camel_case_types)]
pub struct GeneratedBlock_57 {
    pub block_type: GeneratedBlock_57_BlockType,
    pub facing: Direction,
    pub powered: bool,
}
#[allow(dead_code, non_camel_case_types)]
pub struct GeneratedBlock_LightningRod {
    pub facing: Direction,
    pub powered: bool,
    pub waterlogged: bool,
}
#[allow(dead_code, non_camel_case_types)]
pub struct GeneratedBlock_LeafLitter {
    pub facing: Direction,
    pub segment_amount: i32,
}
#[allow(dead_code, non_camel_case_types)]
pub struct GeneratedBlock_PistonHead {
    pub facing: Direction,
    pub short: bool,
    pub ty: PistonType,
}
#[allow(dead_code, non_camel_case_types)]
pub struct GeneratedBlock_ChiseledBookshelf {
    pub facing: Direction,
    pub slot_0_occupied: bool,
    pub slot_1_occupied: bool,
    pub slot_2_occupied: bool,
    pub slot_3_occupied: bool,
    pub slot_4_occupied: bool,
    pub slot_5_occupied: bool,
}
#[allow(dead_code, non_camel_case_types)]
pub struct GeneratedBlock_BigDripleaf {
    pub facing: Direction,
    pub tilt: Tilt,
    pub waterlogged: bool,
}
#[allow(dead_code, non_camel_case_types)]
pub enum GeneratedBlock_63_BlockType {
    Dispenser,
    Dropper,
}
#[allow(dead_code, non_camel_case_types)]
pub struct GeneratedBlock_63 {
    pub block_type: GeneratedBlock_63_BlockType,
    pub facing: Direction,
    pub triggered: bool,
}
#[allow(dead_code, non_camel_case_types)]
pub enum GeneratedBlock_64_BlockType {
    Chest,
    TrappedChest,
}
#[allow(dead_code, non_camel_case_types)]
pub struct GeneratedBlock_64 {
    pub block_type: GeneratedBlock_64_BlockType,
    pub facing: Direction,
    pub ty: ChestType,
    pub waterlogged: bool,
}
#[allow(dead_code, non_camel_case_types)]
pub struct GeneratedBlock_MovingPiston {
    pub facing: Direction,
    pub ty: PistonType,
}
#[allow(dead_code, non_camel_case_types)]
pub enum GeneratedBlock_66_BlockType {
    AcaciaWallHangingSign,
    AcaciaWallSign,
    AmethystCluster,
    BambooWallHangingSign,
    BambooWallSign,
    BigDripleafStem,
    BirchWallHangingSign,
    BirchWallSign,
    BrainCoralWallFan,
    BubbleCoralWallFan,
    CherryWallHangingSign,
    CherryWallSign,
    CrimsonWallHangingSign,
    CrimsonWallSign,
    DarkOakWallHangingSign,
    DarkOakWallSign,
    DeadBrainCoralWallFan,
    DeadBubbleCoralWallFan,
    DeadFireCoralWallFan,
    DeadHornCoralWallFan,
    DeadTubeCoralWallFan,
    EnderChest,
    FireCoralWallFan,
    HornCoralWallFan,
    JungleWallHangingSign,
    JungleWallSign,
    Ladder,
    LargeAmethystBud,
    MangroveWallHangingSign,
    MangroveWallSign,
    MediumAmethystBud,
    OakWallHangingSign,
    OakWallSign,
    PaleOakWallHangingSign,
    PaleOakWallSign,
    SmallAmethystBud,
    SpruceWallHangingSign,
    SpruceWallSign,
    TubeCoralWallFan,
    WarpedWallHangingSign,
    WarpedWallSign,
}
#[allow(dead_code, non_camel_case_types)]
pub struct GeneratedBlock_66 {
    pub block_type: GeneratedBlock_66_BlockType,
    pub facing: Direction,
    pub waterlogged: bool,
}
#[allow(dead_code, non_camel_case_types)]
pub enum GeneratedBlock_67_BlockType {
    LargeFern,
    Lilac,
    Peony,
    PitcherPlant,
    RoseBush,
    Sunflower,
    TallGrass,
    TallSeagrass,
}
#[allow(dead_code, non_camel_case_types)]
pub struct GeneratedBlock_67 {
    pub block_type: GeneratedBlock_67_BlockType,
    pub half: DoubleBlockHalf,
}
#[allow(dead_code, non_camel_case_types)]
pub enum GeneratedBlock_68_BlockType {
    Lantern,
    SoulLantern,
}
#[allow(dead_code, non_camel_case_types)]
pub struct GeneratedBlock_68 {
    pub block_type: GeneratedBlock_68_BlockType,
    pub hanging: bool,
    pub waterlogged: bool,
}
#[allow(dead_code, non_camel_case_types)]
pub struct GeneratedBlock_BrewingStand {
    pub has_bottle_0: bool,
    pub has_bottle_1: bool,
    pub has_bottle_2: bool,
}
#[allow(dead_code, non_camel_case_types)]
pub struct GeneratedBlock_Jukebox {
    pub has_record: bool,
}
#[allow(dead_code, non_camel_case_types)]
pub struct GeneratedBlock_SnifferEgg {
    pub hatch: i32,
}
#[allow(dead_code, non_camel_case_types)]
pub struct GeneratedBlock_NoteBlock {
    pub instrument: NoteBlockInstrument,
    pub note: i32,
    pub powered: bool,
}
#[allow(dead_code, non_camel_case_types)]
pub struct GeneratedBlock_DaylightDetector {
    pub inverted: bool,
    pub power: i32,
}
#[allow(dead_code, non_camel_case_types)]
pub struct GeneratedBlock_Snow {
    pub layers: i32,
}
#[allow(dead_code, non_camel_case_types)]
pub enum GeneratedBlock_75_BlockType {
    Composter,
    Lava,
    PowderSnowCauldron,
    Water,
    WaterCauldron,
}
#[allow(dead_code, non_camel_case_types)]
pub struct GeneratedBlock_75 {
    pub block_type: GeneratedBlock_75_BlockType,
    pub level: i32,
}
#[allow(dead_code, non_camel_case_types)]
pub struct GeneratedBlock_Light {
    pub level: i32,
    pub waterlogged: bool,
}
#[allow(dead_code, non_camel_case_types)]
pub enum GeneratedBlock_77_BlockType {
    BlackCandleCake,
    BlueCandleCake,
    BrownCandleCake,
    CandleCake,
    CyanCandleCake,
    DeepslateRedstoneOre,
    GrayCandleCake,
    GreenCandleCake,
    LightBlueCandleCake,
    LightGrayCandleCake,
    LimeCandleCake,
    MagentaCandleCake,
    OrangeCandleCake,
    PinkCandleCake,
    PurpleCandleCake,
    RedCandleCake,
    RedstoneLamp,
    RedstoneOre,
    RedstoneTorch,
    WhiteCandleCake,
    YellowCandleCake,
}
#[allow(dead_code, non_camel_case_types)]
pub struct GeneratedBlock_77 {
    pub block_type: GeneratedBlock_77_BlockType,
    pub lit: bool,
}
#[allow(dead_code, non_camel_case_types)]
pub enum GeneratedBlock_78_BlockType {
    CopperBulb,
    ExposedCopperBulb,
    OxidizedCopperBulb,
    WaxedCopperBulb,
    WaxedExposedCopperBulb,
    WaxedOxidizedCopperBulb,
    WaxedWeatheredCopperBulb,
    WeatheredCopperBulb,
}
#[allow(dead_code, non_camel_case_types)]
pub struct GeneratedBlock_78 {
    pub block_type: GeneratedBlock_78_BlockType,
    pub lit: bool,
    pub powered: bool,
}
#[allow(dead_code, non_camel_case_types)]
pub struct GeneratedBlock_StructureBlock {
    pub mode: StructureMode,
}
#[allow(dead_code, non_camel_case_types)]
pub struct GeneratedBlock_TestBlock {
    pub mode: TestBlockMode,
}
#[allow(dead_code, non_camel_case_types)]
pub struct GeneratedBlock_Farmland {
    pub moisture: i32,
}
#[allow(dead_code, non_camel_case_types)]
pub struct GeneratedBlock_TrialSpawner {
    pub ominous: bool,
    pub trial_spawner_state: TrialSpawnerState,
}
#[allow(dead_code, non_camel_case_types)]
pub struct GeneratedBlock_Jigsaw {
    pub orientation: FrontAndTop,
}
#[allow(dead_code, non_camel_case_types)]
pub struct GeneratedBlock_SeaPickle {
    pub pickles: i32,
    pub waterlogged: bool,
}
#[allow(dead_code, non_camel_case_types)]
pub enum GeneratedBlock_85_BlockType {
    HeavyWeightedPressurePlate,
    LightWeightedPressurePlate,
    Target,
}
#[allow(dead_code, non_camel_case_types)]
pub struct GeneratedBlock_85 {
    pub block_type: GeneratedBlock_85_BlockType,
    pub power: i32,
}
#[allow(dead_code, non_camel_case_types)]
pub struct GeneratedBlock_SculkSensor {
    pub power: i32,
    pub sculk_sensor_phase: SculkSensorPhase,
    pub waterlogged: bool,
}
#[allow(dead_code, non_camel_case_types)]
pub enum GeneratedBlock_87_BlockType {
    AcaciaPressurePlate,
    BambooPressurePlate,
    BirchPressurePlate,
    CherryPressurePlate,
    CrimsonPressurePlate,
    DarkOakPressurePlate,
    JunglePressurePlate,
    MangrovePressurePlate,
    OakPressurePlate,
    PaleOakPressurePlate,
    PolishedBlackstonePressurePlate,
    SprucePressurePlate,
    StonePressurePlate,
    WarpedPressurePlate,
}
#[allow(dead_code, non_camel_case_types)]
pub struct GeneratedBlock_87 {
    pub block_type: GeneratedBlock_87_BlockType,
    pub powered: bool,
}
#[allow(dead_code, non_camel_case_types)]
pub enum GeneratedBlock_88_BlockType {
    CreeperHead,
    DragonHead,
    PiglinHead,
    PlayerHead,
    SkeletonSkull,
    WitherSkeletonSkull,
    ZombieHead,
}
#[allow(dead_code, non_camel_case_types)]
pub struct GeneratedBlock_88 {
    pub block_type: GeneratedBlock_88_BlockType,
    pub powered: bool,
    pub rotation: i32,
}
#[allow(dead_code, non_camel_case_types)]
pub enum GeneratedBlock_89_BlockType {
    ActivatorRail,
    DetectorRail,
    PoweredRail,
}
#[allow(dead_code, non_camel_case_types)]
pub struct GeneratedBlock_89 {
    pub block_type: GeneratedBlock_89_BlockType,
    pub powered: bool,
    pub shape: RailShape,
    pub waterlogged: bool,
}
#[allow(dead_code, non_camel_case_types)]
pub enum GeneratedBlock_90_BlockType {
    BlackBanner,
    BlueBanner,
    BrownBanner,
    CyanBanner,
    GrayBanner,
    GreenBanner,
    LightBlueBanner,
    LightGrayBanner,
    LimeBanner,
    MagentaBanner,
    OrangeBanner,
    PinkBanner,
    PurpleBanner,
    RedBanner,
    WhiteBanner,
    YellowBanner,
}
#[allow(dead_code, non_camel_case_types)]
pub struct GeneratedBlock_90 {
    pub block_type: GeneratedBlock_90_BlockType,
    pub rotation: i32,
}
#[allow(dead_code, non_camel_case_types)]
pub enum GeneratedBlock_91_BlockType {
    AcaciaSign,
    BambooSign,
    BirchSign,
    CherrySign,
    CrimsonSign,
    DarkOakSign,
    JungleSign,
    MangroveSign,
    OakSign,
    PaleOakSign,
    SpruceSign,
    WarpedSign,
}
#[allow(dead_code, non_camel_case_types)]
pub struct GeneratedBlock_91 {
    pub block_type: GeneratedBlock_91_BlockType,
    pub rotation: i32,
    pub waterlogged: bool,
}
#[allow(dead_code, non_camel_case_types)]
pub struct GeneratedBlock_Rail {
    pub shape: RailShape,
    pub waterlogged: bool,
}
#[allow(dead_code, non_camel_case_types)]
pub enum GeneratedBlock_93_BlockType {
    GrassBlock,
    Mycelium,
    Podzol,
}
#[allow(dead_code, non_camel_case_types)]
pub struct GeneratedBlock_93 {
    pub block_type: GeneratedBlock_93_BlockType,
    pub snowy: bool,
}
#[allow(dead_code, non_camel_case_types)]
pub enum GeneratedBlock_94_BlockType {
    AcaciaSapling,
    BirchSapling,
    CherrySapling,
    DarkOakSapling,
    JungleSapling,
    OakSapling,
    PaleOakSapling,
    SpruceSapling,
}
#[allow(dead_code, non_camel_case_types)]
pub struct GeneratedBlock_94 {
    pub block_type: GeneratedBlock_94_BlockType,
    pub stage: i32,
}
#[allow(dead_code, non_camel_case_types)]
pub struct GeneratedBlock_PointedDripstone {
    pub thickness: DripstoneThickness,
    pub vertical_direction: Direction,
    pub waterlogged: bool,
}
#[allow(dead_code, non_camel_case_types)]
pub struct GeneratedBlock_PaleHangingMoss {
    pub tip: bool,
}
#[allow(dead_code, non_camel_case_types)]
pub enum GeneratedBlock_97_BlockType {
    AcaciaSlab,
    AndesiteSlab,
    BambooMosaicSlab,
    BambooSlab,
    BirchSlab,
    BlackstoneSlab,
    BrickSlab,
    CherrySlab,
    CobbledDeepslateSlab,
    CobblestoneSlab,
    CrimsonSlab,
    CutCopperSlab,
    CutRedSandstoneSlab,
    CutSandstoneSlab,
    DarkOakSlab,
    DarkPrismarineSlab,
    DeepslateBrickSlab,
    DeepslateTileSlab,
    DioriteSlab,
    EndStoneBrickSlab,
    ExposedCutCopperSlab,
    GraniteSlab,
    JungleSlab,
    MangroveSlab,
    MossyCobblestoneSlab,
    MossyStoneBrickSlab,
    MudBrickSlab,
    NetherBrickSlab,
    OakSlab,
    OxidizedCutCopperSlab,
    PaleOakSlab,
    PetrifiedOakSlab,
    PolishedAndesiteSlab,
    PolishedBlackstoneBrickSlab,
    PolishedBlackstoneSlab,
    PolishedDeepslateSlab,
    PolishedDioriteSlab,
    PolishedGraniteSlab,
    PolishedTuffSlab,
    PrismarineBrickSlab,
    PrismarineSlab,
    PurpurSlab,
    QuartzSlab,
    RedNetherBrickSlab,
    RedSandstoneSlab,
    ResinBrickSlab,
    SandstoneSlab,
    SmoothQuartzSlab,
    SmoothRedSandstoneSlab,
    SmoothSandstoneSlab,
    SmoothStoneSlab,
    SpruceSlab,
    StoneBrickSlab,
    StoneSlab,
    TuffBrickSlab,
    TuffSlab,
    WarpedSlab,
    WaxedCutCopperSlab,
    WaxedExposedCutCopperSlab,
    WaxedOxidizedCutCopperSlab,
    WaxedWeatheredCutCopperSlab,
    WeatheredCutCopperSlab,
}
#[allow(dead_code, non_camel_case_types)]
pub struct GeneratedBlock_97 {
    pub block_type: GeneratedBlock_97_BlockType,
    pub ty: SlabType,
    pub waterlogged: bool,
}
#[allow(dead_code, non_camel_case_types)]
pub struct GeneratedBlock_Tnt {
    pub unstable: bool,
}
#[allow(dead_code, non_camel_case_types)]
pub enum GeneratedBlock_99_BlockType {
    Barrier,
    BrainCoral,
    BrainCoralFan,
    BubbleCoral,
    BubbleCoralFan,
    Conduit,
    CopperGrate,
    DeadBrainCoral,
    DeadBrainCoralFan,
    DeadBubbleCoral,
    DeadBubbleCoralFan,
    DeadFireCoral,
    DeadFireCoralFan,
    DeadHornCoral,
    DeadHornCoralFan,
    DeadTubeCoral,
    DeadTubeCoralFan,
    ExposedCopperGrate,
    FireCoral,
    FireCoralFan,
    HangingRoots,
    HeavyCore,
    HornCoral,
    HornCoralFan,
    MangroveRoots,
    OxidizedCopperGrate,
    TubeCoral,
    TubeCoralFan,
    WaxedCopperGrate,
    WaxedExposedCopperGrate,
    WaxedOxidizedCopperGrate,
    WaxedWeatheredCopperGrate,
    WeatheredCopperGrate,
}
#[allow(dead_code, non_camel_case_types)]
pub struct GeneratedBlock_99 {
    pub block_type: GeneratedBlock_99_BlockType,
    pub waterlogged: bool,
}
