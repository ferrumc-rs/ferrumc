use crate::enum_property;

enum_property!(
    NoteBlockInstrument,
    Harp => "harp",
    BaseDrum => "basedrum",
    Snare => "snare",
    Hat => "hat",
    Bass => "bass",
    Flute => "flute",
    Bell => "bell",
    Guitar => "guitar",
    Chime => "chime",
    Xylophone => "xylophone",
    IronXylophone => "iron_xylophone",
    CowBell => "cow_bell",
    Didgeridoo => "didgeridoo",
    Bit => "bit",
    Banjo => "banjo",
    Pling => "pling",
    Zombie => "zombie",
    Skeleton => "skeleton",
    Creeper => "creeper",
    Dragon => "dragon",
    WitherSkeleton => "wither_skeleton",
    Piglin => "piglin",
    CustomHead => "custom_head",
);

// TODO: return sound events based on the variant
