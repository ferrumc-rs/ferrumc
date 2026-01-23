/// All of these enums come directly from the enums found in the `net.minecraft.world.level.block.state.properties` package.

use crate::enum_property;

enum_property!(
    AttachFace,
    Floor => "floor",
    Wall => "wall",
    Ceiling => "ceiling",
);

enum_property!(
    BambooLeaves,
    None => "none",
    Small => "small",
    Large => "large",
);

enum_property!(
    BedPart,
    Head => "head",
    Foot => "foot",
);

enum_property!(
    BellAttachType,
    Floor => "floor",
    Ceiling => "ceiling",
    SingleWall => "single_wall",
    DoubleWall => "double_wall",
);

enum_property!(
    ChestType,
    Single => "single",
    Left => "left",
    Right => "right",
);

enum_property!(
    ComparatorMode,
    Compare => "compare",
    Subtract => "subtract",
);

enum_property!(
    CreakingHeartState,
    Uprooted => "uprooted",
    Dormant => "dormant",
    Awake => "awake",
);

enum_property!(
    DoorHingeSide,
    Left => "left",
    Right => "right",
);

enum_property!(
    DripstoneThickness,
    TipMerge => "tip_merge",
    Tip => "tip",
    Frustum => "frustum",
    Middle => "middle",
    Base => "base",
);

enum_property!(
    Half,
    Top => "top",
    Bottom => "bottom",
);

enum_property!(
    PistonType,
    Default => "normal",
    Sticky => "sticky",
);

enum_property!(
    RailShape,
    NorthSouth => "north_south",
    EastWest => "east_west",
    AscendingEast => "ascending_east",
    AscendingWest => "ascending_west",
    AscendingNorth => "ascending_north",
    AscendingSouth => "ascending_south",
    SouthEast => "south_east",
    SouthWest => "south_west",
    NorthWest => "north_west",
    NorthEast => "north_east",
);

enum_property!(
    RedstoneSide,
    Up => "up",
    Side => "side",
    None => "none",
);

enum_property!(
    SculkSensorPhase,
    Inactive => "inactive",
    Active => "active",
    Cooldown => "cooldown",
);

enum_property!(
    SideChainPart,
    Unconnected => "unconnected",
    Right => "right",
    Center => "center",
    Left => "left",
);

enum_property!(
    SlabType,
    Top => "top",
    Bottom => "bottom",
    Double => "double",
);

enum_property!(
    StairsShape,
    Straight => "straight",
    InnerLeft => "inner_left",
    InnerRight => "inner_right",
    OuterLeft => "outer_left",
    OuterRight => "outer_right",
);

enum_property!(
    StructureMode,
    Save => "save",
    Load => "load",
    Corner => "corner",
    Data => "data",
);

enum_property!(
    TestBlockMode,
    Start => "start",
    Log => "log",
    Fail => "fail",
    Accept => "accept",
);

enum_property!(
    Tilt,
    None => "none",
    Unstable => "unstable",
    Partial => "partial",
    Full => "full",
);

enum_property!(
    WallSide,
    None => "none",
    Low => "low",
    Tall => "tall",
);