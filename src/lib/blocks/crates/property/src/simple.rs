/// All of these enums come directly from the enums found in the `net.minecraft.world.level.block.state.properties` package.
use crate::enum_property;

enum_property!(
    AttachFace,
    Floor => "floor",
    Wall => "wall",
    Ceiling => "ceiling",
);

enum_property!(
    Axis,
    X => "x",
    Y => "y",
    Z => "z",
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
    Direction,
    Down => "down",
    Up => "up",
    North => "north",
    South => "south",
    East => "east",
    West => "west",
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
    FrontAndTop,
    DownEast => "down_east",
    DownNorth => "down_north",
    DownSouth => "down_south",
    DownWest => "down_west",
    UpEast => "up_east",
    UpNorth => "up_north",
    UpSouth => "up_south",
    UpWest => "up_west",
    WestUp => "west_up",
    EastUp => "east_up",
    NorthUp => "north_up",
    SouthUp => "south_up",
);

enum_property!(
    Half,
    Top => "top",
    Bottom => "bottom",
);

enum_property!(
    CopperGolemPose,
    Standing => "standing",
    Sitting => "sitting",
    Running => "running",
    Star => "star",
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
    TrialSpawnerState,
    Inactive => "inactive",
    WaitingForPlayers => "waiting_for_players",
    Active => "active",
    WaitingForRewardEjection => "waiting_for_reward_ejection",
    EjectingReward => "ejecting_reward",
    Cooldown => "cooldown",
);

enum_property!(
    VaultState,
    Inactive => "inactive",
    Active => "active",
    Unlocking => "unlocking",
    Ejecting => "ejecting",
);

enum_property!(
    WallSide,
    None => "none",
    Low => "low",
    Tall => "tall",
);
