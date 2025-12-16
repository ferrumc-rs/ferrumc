use crate::{biome::Biome, biome_chunk::NoisePoint};
// reference: net.minecraft.world.level.biome.OverworldBiomeBuilder
const VALLEY_SIZE: f32 = 0.05;
const LOW_START: f32 = 0.26666668;
pub const HIGH_START: f32 = 0.4;
pub const HIGH_END: f32 = 0.93333334;
pub const PEAK_START: f32 = 0.56666666;
pub const PEAK_END: f32 = 0.7666667;
pub const NEAR_INLAND_START: f32 = -0.11;
pub const MID_INLAND_START: f32 = 0.03;
pub const FAR_INLAND_START: f32 = 0.3;
pub const EROSION_INDEX_1_START: f32 = -0.78;
pub const EROSION_INDEX_2_START: f32 = -0.375;
pub const EROSION_DEEP_DARK_DRYNESS_THRESHOLD: f64 = -0.225;
pub const DEPTH_DEEP_DARK_DRYNESS_THRESHOLD: f64 = 0.9;

const FULL_RANGE: (f32, f32) = (-1.0, 1.0);

const TEMPERATURES: [(f32, f32); 5] = [
    (-1.0, -0.45),
    (-0.45, -0.15),
    (-0.15, 0.2),
    (0.2, 0.55),
    (0.55, 1.0),
];

const HUMIDITIES: [(f32, f32); 5] = [
    (-1.0, -0.35),
    (-0.35, -0.1),
    (-0.1, 0.1),
    (0.1, 0.3),
    (0.3, 1.0),
];

const EROSIONS: [(f32, f32); 7] = [
    (-1.0, EROSION_INDEX_1_START),
    (EROSION_INDEX_1_START, EROSION_INDEX_2_START),
    (EROSION_INDEX_2_START, -0.2225),
    (-0.2225, 0.05),
    (0.05, 0.45),
    (0.45, 0.55),
    (0.55, 1.0),
];

const FROZEN_RANGE: (f32, f32) = TEMPERATURES[0];
const UNFROZEN_RANGE: (f32, f32) = (TEMPERATURES[1].0, TEMPERATURES[4].1);

const MUSHROOM_FIELDS_CONTINENTALNESS: (f32, f32) = (-1.2, -1.05);
const DEEP_OCEAN_CONTINENTALNESS: (f32, f32) = (-1.05, -0.455);
const OCEAN_CONTINENTALNESS: (f32, f32) = (-0.455, -0.19);
const COAST_CONTINENTALNESS: (f32, f32) = (-0.19, NEAR_INLAND_START);
const INLAND_CONTINENTALNESS: (f32, f32) = (NEAR_INLAND_START, 0.55);
const NEAR_INLAND_CONTINENTALNESS: (f32, f32) = (NEAR_INLAND_START, MID_INLAND_START);
const MID_INLAND_CONTINENTALNESS: (f32, f32) = (MID_INLAND_START, FAR_INLAND_START);
const FAR_INLAND_CONTINENTALNESS: (f32, f32) = (FAR_INLAND_START, 1.0);

// --- biome grids ---
const OCEANS: [[Biome; 5]; 2] = [
    [
        Biome::DeepFrozenOcean,
        Biome::DeepColdOcean,
        Biome::DeepOcean,
        Biome::DeepLukewarmOcean,
        Biome::WarmOcean,
    ],
    [
        Biome::FrozenOcean,
        Biome::ColdOcean,
        Biome::Ocean,
        Biome::LukewarmOcean,
        Biome::WarmOcean,
    ],
];

const MIDDLE_BIOMES: [[Biome; 5]; 5] = [
    [
        Biome::SnowyPlains,
        Biome::SnowyPlains,
        Biome::SnowyPlains,
        Biome::SnowyTaiga,
        Biome::Taiga,
    ],
    [
        Biome::Plains,
        Biome::Plains,
        Biome::Forest,
        Biome::Taiga,
        Biome::OldGrowthSpruceTaiga,
    ],
    [
        Biome::FlowerForest,
        Biome::Plains,
        Biome::Forest,
        Biome::BirchForest,
        Biome::DarkForest,
    ],
    [
        Biome::Savanna,
        Biome::Savanna,
        Biome::Forest,
        Biome::Jungle,
        Biome::Jungle,
    ],
    [
        Biome::Desert,
        Biome::Desert,
        Biome::Desert,
        Biome::Desert,
        Biome::Desert,
    ],
];

const MIDDLE_BIOMES_VARIANT: [[Option<Biome>; 5]; 5] = [
    [
        Some(Biome::IceSpikes),
        None,
        Some(Biome::SnowyTaiga),
        None,
        None,
    ],
    [None, None, None, None, Some(Biome::OldGrowthPineTaiga)],
    [
        Some(Biome::SunflowerPlains),
        None,
        None,
        Some(Biome::OldGrowthBirchForest),
        None,
    ],
    [
        None,
        None,
        Some(Biome::Plains),
        Some(Biome::SparseJungle),
        Some(Biome::BambooJungle),
    ],
    [None, None, None, None, None],
];

const PLATEAU_BIOMES: [[Biome; 5]; 5] = [
    [
        Biome::SnowyPlains,
        Biome::SnowyPlains,
        Biome::SnowyPlains,
        Biome::SnowyTaiga,
        Biome::SnowyTaiga,
    ],
    [
        Biome::Meadow,
        Biome::Meadow,
        Biome::Forest,
        Biome::Taiga,
        Biome::OldGrowthSpruceTaiga,
    ],
    [
        Biome::Meadow,
        Biome::Meadow,
        Biome::Meadow,
        Biome::Meadow,
        Biome::PaleGarden,
    ],
    [
        Biome::SavannaPlateau,
        Biome::SavannaPlateau,
        Biome::Forest,
        Biome::Forest,
        Biome::Jungle,
    ],
    [
        Biome::Badlands,
        Biome::Badlands,
        Biome::Badlands,
        Biome::WoodedBadlands,
        Biome::WoodedBadlands,
    ],
];

const PLATEAU_BIOMES_VARIANT: [[Option<Biome>; 5]; 5] = [
    [Some(Biome::IceSpikes), None, None, None, None],
    [
        Some(Biome::CherryGrove),
        None,
        Some(Biome::Meadow),
        Some(Biome::Meadow),
        Some(Biome::OldGrowthPineTaiga),
    ],
    [
        Some(Biome::CherryGrove),
        Some(Biome::CherryGrove),
        Some(Biome::Forest),
        Some(Biome::BirchForest),
        None,
    ],
    [None, None, None, None, None],
    [
        Some(Biome::ErodedBadlands),
        Some(Biome::ErodedBadlands),
        None,
        None,
        None,
    ],
];

const SHATTERED_BIOMES: [[Option<Biome>; 5]; 5] = [
    [
        Some(Biome::WindsweptGravellyHills),
        Some(Biome::WindsweptGravellyHills),
        Some(Biome::WindsweptHills),
        Some(Biome::WindsweptForest),
        Some(Biome::WindsweptForest),
    ],
    [
        Some(Biome::WindsweptGravellyHills),
        Some(Biome::WindsweptGravellyHills),
        Some(Biome::WindsweptHills),
        Some(Biome::WindsweptForest),
        Some(Biome::WindsweptForest),
    ],
    [
        Some(Biome::WindsweptHills),
        Some(Biome::WindsweptHills),
        Some(Biome::WindsweptHills),
        Some(Biome::WindsweptForest),
        Some(Biome::WindsweptForest),
    ],
    [None, None, None, None, None],
    [None, None, None, None, None],
];

fn surface(
    temperature: (f32, f32),
    humidity: (f32, f32),
    continentalness: (f32, f32),
    erosion: (f32, f32),
    peaks_and_valleys: (f32, f32),
    biome: Biome,
) -> [(NoisePoint, Biome); 2] {
    [
        NoisePoint::new(
            temperature,
            humidity,
            continentalness,
            erosion,
            (0.0, 0.0),
            peaks_and_valleys,
            biome,
        ),
        NoisePoint::new(
            temperature,
            humidity,
            continentalness,
            erosion,
            (1.0, 1.0),
            peaks_and_valleys,
            biome,
        ),
    ]
}

pub fn overworld_biomes() -> Vec<(NoisePoint, Biome)> {
    let mut vec = add_ocean_biomes();
    vec.extend(add_inland_biomes());
    vec.extend(add_underground_biomes());
    vec
}

fn add_ocean_biomes() -> Vec<(NoisePoint, Biome)> {
    let ocean_surface = |temperature: (f32, f32),
                         continentalness: (f32, f32),
                         biome: Biome|
     -> [(NoisePoint, Biome); 2] {
        surface(
            temperature,
            FULL_RANGE,
            continentalness,
            FULL_RANGE,
            FULL_RANGE,
            biome,
        )
    };

    ocean_surface(
        FULL_RANGE,
        MUSHROOM_FIELDS_CONTINENTALNESS,
        Biome::MushroomFields,
    );
    TEMPERATURES
        .iter()
        .enumerate()
        .flat_map(|(i, temp)| {
            [
                ocean_surface(*temp, DEEP_OCEAN_CONTINENTALNESS, OCEANS[0][i]),
                ocean_surface(*temp, OCEAN_CONTINENTALNESS, OCEANS[1][i]),
            ]
            .into_iter()
            .flat_map(|a| a.into_iter())
        })
        .chain(ocean_surface(
            FULL_RANGE,
            MUSHROOM_FIELDS_CONTINENTALNESS,
            Biome::MushroomFields,
        ))
        .collect()
}

fn add_underground_biomes() -> [(NoisePoint, Biome); 3] {
    let underground = |humidity: (f32, f32),
                       continentalness: (f32, f32),
                       erosion: (f32, f32),
                       depth: (f32, f32),
                       biome: Biome|
     -> (NoisePoint, Biome) {
        NoisePoint::new(
            FULL_RANGE,
            humidity,
            continentalness,
            erosion,
            depth,
            FULL_RANGE,
            biome,
        )
    };
    [
        underground(
            FULL_RANGE,
            (0.8, 1.0),
            FULL_RANGE,
            (0.2, 0.9),
            Biome::DripstoneCaves,
        ),
        underground(
            (0.7, 1.0),
            FULL_RANGE,
            FULL_RANGE,
            (0.2, 0.9),
            Biome::LushCaves,
        ),
        underground(
            FULL_RANGE,
            FULL_RANGE,
            (EROSIONS[0].0, EROSIONS[1].1),
            (1.1, 1.1),
            Biome::DeepDark,
        ),
    ]
}

fn add_inland_biomes() -> Vec<(NoisePoint, Biome)> {
    let mut res = Vec::new();
    add_mid_slice(&mut res, (-1.0, -HIGH_END));
    add_high_slice(&mut res, (-HIGH_END, -PEAK_END));
    add_peaks(&mut res, (-PEAK_END, -PEAK_START));
    add_high_slice(&mut res, (-PEAK_START, -HIGH_START));
    add_mid_slice(&mut res, (-HIGH_START, -LOW_START));
    add_low_slice(&mut res, (-LOW_START, -VALLEY_SIZE));
    add_valleys(&mut res, (-VALLEY_SIZE, VALLEY_SIZE));
    add_low_slice(&mut res, (VALLEY_SIZE, LOW_START));
    add_mid_slice(&mut res, (LOW_START, HIGH_START));
    add_high_slice(&mut res, (HIGH_START, PEAK_START));
    add_peaks(&mut res, (PEAK_START, PEAK_END));
    add_high_slice(&mut res, (PEAK_END, HIGH_END));
    add_mid_slice(&mut res, (HIGH_END, 1.0));
    res
}
fn add_peaks(vec: &mut Vec<(NoisePoint, Biome)>, param: (f32, f32)) {
    for (i, &temperature_param) in TEMPERATURES.iter().enumerate() {
        for (j, &humidity_param) in HUMIDITIES.iter().enumerate() {
            let mut peaks_surface =
                |continentalness: (f32, f32), erosion: (f32, f32), biome: Biome| {
                    vec.extend(surface(
                        temperature_param,
                        humidity_param,
                        continentalness,
                        erosion,
                        param,
                        biome,
                    ))
                };
            let middle_biome = pick_middle_biome(i, j, param);
            let middle_or_badlands = pick_middle_biome_or_badlands_if_hot(i, j, param);
            let middle_or_badlands_or_slope =
                pick_middle_biome_or_badlands_if_hot_or_slope_if_cold(i, j, param);
            let plateau = pick_plateau_biome(i, j, param);
            let shattered = pick_shattered_biome(i, j, param);
            let shattered_or_savanna = maybe_pick_windswept_savanna_biome(i, j, param, shattered);
            let peak = pick_peak_biome(i, j, param);

            peaks_surface(
                (COAST_CONTINENTALNESS.0, FAR_INLAND_CONTINENTALNESS.1),
                EROSIONS[0],
                peak,
            );
            peaks_surface(
                (COAST_CONTINENTALNESS.0, NEAR_INLAND_CONTINENTALNESS.1),
                EROSIONS[1],
                middle_or_badlands_or_slope,
            );
            peaks_surface(
                (MID_INLAND_CONTINENTALNESS.0, FAR_INLAND_CONTINENTALNESS.1),
                EROSIONS[1],
                peak,
            );
            peaks_surface(
                (COAST_CONTINENTALNESS.0, NEAR_INLAND_CONTINENTALNESS.1),
                (EROSIONS[2].0, EROSIONS[3].1),
                middle_biome,
            );
            peaks_surface(
                (MID_INLAND_CONTINENTALNESS.0, FAR_INLAND_CONTINENTALNESS.1),
                EROSIONS[2],
                plateau,
            );
            peaks_surface(MID_INLAND_CONTINENTALNESS, EROSIONS[3], middle_or_badlands);
            peaks_surface(FAR_INLAND_CONTINENTALNESS, EROSIONS[3], plateau);
            peaks_surface(
                (COAST_CONTINENTALNESS.0, FAR_INLAND_CONTINENTALNESS.1),
                EROSIONS[4],
                middle_biome,
            );
            peaks_surface(
                (COAST_CONTINENTALNESS.0, NEAR_INLAND_CONTINENTALNESS.1),
                EROSIONS[5],
                shattered_or_savanna,
            );
            peaks_surface(
                (MID_INLAND_CONTINENTALNESS.0, FAR_INLAND_CONTINENTALNESS.1),
                EROSIONS[5],
                shattered,
            );
            peaks_surface(
                (COAST_CONTINENTALNESS.0, FAR_INLAND_CONTINENTALNESS.1),
                EROSIONS[6],
                middle_biome,
            );
        }
    }
}
fn add_high_slice(vec: &mut Vec<(NoisePoint, Biome)>, param: (f32, f32)) {
    for (i, &temperature_param) in TEMPERATURES.iter().enumerate() {
        for (j, &humidity_param) in HUMIDITIES.iter().enumerate() {
            let mut high_surface =
                |continentalness: (f32, f32), erosion: (f32, f32), biome: Biome| {
                    vec.extend(surface(
                        temperature_param,
                        humidity_param,
                        continentalness,
                        erosion,
                        param,
                        biome,
                    ));
                };
            let resource_key = pick_middle_biome(i, j, param);
            let resource_key1 = pick_middle_biome_or_badlands_if_hot(i, j, param);
            let resource_key2 = pick_middle_biome_or_badlands_if_hot_or_slope_if_cold(i, j, param);
            let resource_key3 = pick_plateau_biome(i, j, param);
            let resource_key4 = pick_shattered_biome(i, j, param);
            let resource_key5 = maybe_pick_windswept_savanna_biome(i, j, param, resource_key);
            let resource_key6 = pick_slope_biome(i, j, param);
            let resource_key7 = pick_peak_biome(i, j, param);

            high_surface(
                COAST_CONTINENTALNESS,
                (EROSIONS[0].0, EROSIONS[1].1),
                resource_key,
            );
            high_surface(NEAR_INLAND_CONTINENTALNESS, EROSIONS[0], resource_key6);
            high_surface(
                (MID_INLAND_CONTINENTALNESS.0, FAR_INLAND_CONTINENTALNESS.1),
                EROSIONS[0],
                resource_key7,
            );
            high_surface(NEAR_INLAND_CONTINENTALNESS, EROSIONS[1], resource_key2);
            high_surface(
                (MID_INLAND_CONTINENTALNESS.0, FAR_INLAND_CONTINENTALNESS.1),
                EROSIONS[1],
                resource_key6,
            );
            high_surface(
                (COAST_CONTINENTALNESS.0, NEAR_INLAND_CONTINENTALNESS.1),
                (EROSIONS[2].0, EROSIONS[3].1),
                resource_key,
            );
            high_surface(
                (MID_INLAND_CONTINENTALNESS.0, FAR_INLAND_CONTINENTALNESS.1),
                EROSIONS[2],
                resource_key3,
            );
            high_surface(MID_INLAND_CONTINENTALNESS, EROSIONS[3], resource_key1);
            high_surface(FAR_INLAND_CONTINENTALNESS, EROSIONS[3], resource_key3);
            high_surface(
                (COAST_CONTINENTALNESS.0, FAR_INLAND_CONTINENTALNESS.1),
                EROSIONS[4],
                resource_key,
            );
            high_surface(
                (COAST_CONTINENTALNESS.0, NEAR_INLAND_CONTINENTALNESS.1),
                EROSIONS[5],
                resource_key5,
            );
            high_surface(
                (MID_INLAND_CONTINENTALNESS.0, FAR_INLAND_CONTINENTALNESS.1),
                EROSIONS[5],
                resource_key4,
            );
            high_surface(
                (COAST_CONTINENTALNESS.0, FAR_INLAND_CONTINENTALNESS.1),
                EROSIONS[6],
                resource_key,
            );
        }
    }
}

fn add_mid_slice(vec: &mut Vec<(NoisePoint, Biome)>, param: (f32, f32)) {
    vec.extend(surface(
        FULL_RANGE,
        FULL_RANGE,
        COAST_CONTINENTALNESS,
        (EROSIONS[0].0, EROSIONS[2].1),
        param,
        Biome::StonyShore,
    ));
    vec.extend(surface(
        (TEMPERATURES[1].0, TEMPERATURES[2].1),
        FULL_RANGE,
        (NEAR_INLAND_CONTINENTALNESS.0, FAR_INLAND_CONTINENTALNESS.1),
        EROSIONS[6],
        param,
        Biome::Swamp,
    ));
    vec.extend(surface(
        (TEMPERATURES[3].0, TEMPERATURES[4].1),
        FULL_RANGE,
        (NEAR_INLAND_CONTINENTALNESS.0, FAR_INLAND_CONTINENTALNESS.1),
        EROSIONS[6],
        param,
        Biome::MangroveSwamp,
    ));

    // loop over temperature × humidity grid
    for (i, &temperature_param) in TEMPERATURES.iter().enumerate() {
        for (j, &humidity_param) in HUMIDITIES.iter().enumerate() {
            let mut mid_surface =
                |continentalness: (f32, f32), erosion: (f32, f32), biome: Biome| {
                    vec.extend(surface(
                        temperature_param,
                        humidity_param,
                        continentalness,
                        erosion,
                        param,
                        biome,
                    ));
                };

            let middle = pick_middle_biome(i, j, param);
            let middle_or_badlands = pick_middle_biome_or_badlands_if_hot(i, j, param);
            let middle_or_badlands_or_slope =
                pick_middle_biome_or_badlands_if_hot_or_slope_if_cold(i, j, param);
            let shattered = pick_shattered_biome(i, j, param);
            let plateau = pick_plateau_biome(i, j, param);
            let beach = pick_beach_biome(i, j);
            let savanna = maybe_pick_windswept_savanna_biome(i, j, param, middle);
            let shattered_coast = pick_shattered_coast_biome(i, j, param);
            let slope = pick_slope_biome(i, j, param);

            mid_surface(
                (NEAR_INLAND_CONTINENTALNESS.0, FAR_INLAND_CONTINENTALNESS.1),
                EROSIONS[0],
                slope,
            );
            mid_surface(
                (NEAR_INLAND_CONTINENTALNESS.0, MID_INLAND_CONTINENTALNESS.1),
                EROSIONS[1],
                middle_or_badlands_or_slope,
            );
            mid_surface(
                FAR_INLAND_CONTINENTALNESS,
                EROSIONS[1],
                if i == 0 { slope } else { plateau },
            );
            mid_surface(NEAR_INLAND_CONTINENTALNESS, EROSIONS[2], middle);
            mid_surface(MID_INLAND_CONTINENTALNESS, EROSIONS[2], middle_or_badlands);
            mid_surface(FAR_INLAND_CONTINENTALNESS, EROSIONS[2], plateau);
            mid_surface(
                (COAST_CONTINENTALNESS.0, NEAR_INLAND_CONTINENTALNESS.1),
                EROSIONS[3],
                middle,
            );
            mid_surface(
                (MID_INLAND_CONTINENTALNESS.0, FAR_INLAND_CONTINENTALNESS.1),
                EROSIONS[3],
                middle_or_badlands,
            );

            if param.1 < 0.0 {
                mid_surface(COAST_CONTINENTALNESS, EROSIONS[4], beach);
                mid_surface(
                    (NEAR_INLAND_CONTINENTALNESS.0, FAR_INLAND_CONTINENTALNESS.1),
                    EROSIONS[4],
                    middle,
                );
            } else {
                mid_surface(
                    (COAST_CONTINENTALNESS.0, FAR_INLAND_CONTINENTALNESS.1),
                    EROSIONS[4],
                    middle,
                );
            }

            mid_surface(COAST_CONTINENTALNESS, EROSIONS[5], shattered_coast);
            mid_surface(NEAR_INLAND_CONTINENTALNESS, EROSIONS[5], savanna);
            mid_surface(
                (MID_INLAND_CONTINENTALNESS.0, FAR_INLAND_CONTINENTALNESS.1),
                EROSIONS[5],
                shattered,
            );

            if param.1 < 0.0 {
                mid_surface(COAST_CONTINENTALNESS, EROSIONS[6], beach);
            } else {
                mid_surface(COAST_CONTINENTALNESS, EROSIONS[6], middle);
            }

            if i == 0 {
                mid_surface(
                    (NEAR_INLAND_CONTINENTALNESS.0, FAR_INLAND_CONTINENTALNESS.1),
                    EROSIONS[6],
                    middle,
                );
            }
        }
    }
}

fn add_low_slice(vec: &mut Vec<(NoisePoint, Biome)>, param: (f32, f32)) {
    // special cases before looping
    vec.extend(surface(
        FULL_RANGE,
        FULL_RANGE,
        COAST_CONTINENTALNESS,
        (EROSIONS[0].0, EROSIONS[2].1),
        param,
        Biome::StonyShore,
    ));
    vec.extend(surface(
        (TEMPERATURES[1].0, TEMPERATURES[2].1),
        FULL_RANGE,
        (NEAR_INLAND_CONTINENTALNESS.0, FAR_INLAND_CONTINENTALNESS.1),
        EROSIONS[6],
        param,
        Biome::Swamp,
    ));
    vec.extend(surface(
        (TEMPERATURES[3].0, TEMPERATURES[4].1),
        FULL_RANGE,
        (NEAR_INLAND_CONTINENTALNESS.0, FAR_INLAND_CONTINENTALNESS.1),
        EROSIONS[6],
        param,
        Biome::MangroveSwamp,
    ));

    // loop over temperature × humidity grid
    for (i, &temperature_param) in TEMPERATURES.iter().enumerate() {
        for (j, &humidity_param) in HUMIDITIES.iter().enumerate() {
            let mut low_surface =
                |continentalness: (f32, f32), erosion: (f32, f32), biome: Biome| {
                    vec.extend(surface(
                        temperature_param,
                        humidity_param,
                        continentalness,
                        erosion,
                        param,
                        biome,
                    ));
                };

            let middle = pick_middle_biome(i, j, param);
            let middle_or_badlands = pick_middle_biome_or_badlands_if_hot(i, j, param);
            let middle_or_badlands_or_slope =
                pick_middle_biome_or_badlands_if_hot_or_slope_if_cold(i, j, param);
            let beach = pick_beach_biome(i, j);
            let savanna = maybe_pick_windswept_savanna_biome(i, j, param, middle);
            let shattered_coast = pick_shattered_coast_biome(i, j, param);

            low_surface(
                NEAR_INLAND_CONTINENTALNESS,
                (EROSIONS[0].0, EROSIONS[1].1),
                middle_or_badlands,
            );
            low_surface(
                (MID_INLAND_CONTINENTALNESS.0, FAR_INLAND_CONTINENTALNESS.1),
                (EROSIONS[0].0, EROSIONS[1].1),
                middle_or_badlands_or_slope,
            );
            low_surface(
                NEAR_INLAND_CONTINENTALNESS,
                (EROSIONS[2].0, EROSIONS[3].1),
                middle,
            );
            low_surface(
                (MID_INLAND_CONTINENTALNESS.0, FAR_INLAND_CONTINENTALNESS.1),
                (EROSIONS[2].0, EROSIONS[3].1),
                middle_or_badlands,
            );
            low_surface(COAST_CONTINENTALNESS, (EROSIONS[3].0, EROSIONS[4].1), beach);
            low_surface(
                (NEAR_INLAND_CONTINENTALNESS.0, FAR_INLAND_CONTINENTALNESS.1),
                EROSIONS[4],
                middle,
            );
            low_surface(COAST_CONTINENTALNESS, EROSIONS[5], shattered_coast);
            low_surface(NEAR_INLAND_CONTINENTALNESS, EROSIONS[5], savanna);
            low_surface(
                (MID_INLAND_CONTINENTALNESS.0, FAR_INLAND_CONTINENTALNESS.1),
                EROSIONS[5],
                middle,
            );
            low_surface(COAST_CONTINENTALNESS, EROSIONS[6], beach);

            if i == 0 {
                low_surface(
                    (NEAR_INLAND_CONTINENTALNESS.0, FAR_INLAND_CONTINENTALNESS.1),
                    EROSIONS[6],
                    middle,
                );
            }
        }
    }
}

fn add_valleys(vec: &mut Vec<(NoisePoint, Biome)>, param: (f32, f32)) {
    let mut valleys_surface = |temperature: (f32, f32),
                               continentalness: (f32, f32),
                               erosion: (f32, f32),
                               biome: Biome| {
        vec.extend(surface(
            temperature,
            FULL_RANGE,
            continentalness,
            erosion,
            param,
            biome,
        ));
    };

    let stony_or_frozen = if param.1 < 0.0 {
        Biome::StonyShore
    } else {
        Biome::FrozenRiver
    };
    let stony_or_river = if param.1 < 0.0 {
        Biome::StonyShore
    } else {
        Biome::River
    };

    valleys_surface(
        FROZEN_RANGE,
        COAST_CONTINENTALNESS,
        (EROSIONS[0].0, EROSIONS[1].1),
        stony_or_frozen,
    );
    valleys_surface(
        UNFROZEN_RANGE,
        COAST_CONTINENTALNESS,
        (EROSIONS[0].0, EROSIONS[1].1),
        stony_or_river,
    );
    valleys_surface(
        FROZEN_RANGE,
        NEAR_INLAND_CONTINENTALNESS,
        (EROSIONS[0].0, EROSIONS[1].1),
        Biome::FrozenRiver,
    );
    valleys_surface(
        UNFROZEN_RANGE,
        NEAR_INLAND_CONTINENTALNESS,
        (EROSIONS[0].0, EROSIONS[1].1),
        Biome::River,
    );
    valleys_surface(
        FROZEN_RANGE,
        (COAST_CONTINENTALNESS.0, FAR_INLAND_CONTINENTALNESS.1),
        (EROSIONS[2].0, EROSIONS[5].1),
        Biome::FrozenRiver,
    );
    valleys_surface(
        UNFROZEN_RANGE,
        (COAST_CONTINENTALNESS.0, FAR_INLAND_CONTINENTALNESS.1),
        (EROSIONS[2].0, EROSIONS[5].1),
        Biome::River,
    );
    valleys_surface(
        FROZEN_RANGE,
        COAST_CONTINENTALNESS,
        EROSIONS[6],
        Biome::FrozenRiver,
    );
    valleys_surface(
        UNFROZEN_RANGE,
        COAST_CONTINENTALNESS,
        EROSIONS[6],
        Biome::River,
    );
    valleys_surface(
        (TEMPERATURES[1].0, TEMPERATURES[2].1),
        (INLAND_CONTINENTALNESS.0, FAR_INLAND_CONTINENTALNESS.1),
        EROSIONS[6],
        Biome::Swamp,
    );
    valleys_surface(
        (TEMPERATURES[3].0, TEMPERATURES[4].1),
        (INLAND_CONTINENTALNESS.0, FAR_INLAND_CONTINENTALNESS.1),
        EROSIONS[6],
        Biome::MangroveSwamp,
    );
    valleys_surface(
        FROZEN_RANGE,
        (INLAND_CONTINENTALNESS.0, FAR_INLAND_CONTINENTALNESS.1),
        EROSIONS[6],
        Biome::FrozenRiver,
    );

    for (i, &temperature_param) in TEMPERATURES.iter().enumerate() {
        for (j, &humidity_param) in HUMIDITIES.iter().enumerate() {
            vec.extend(surface(
                temperature_param,
                humidity_param,
                (MID_INLAND_CONTINENTALNESS.0, FAR_INLAND_CONTINENTALNESS.1),
                (EROSIONS[0].0, EROSIONS[1].1),
                param,
                pick_middle_biome_or_badlands_if_hot(i, j, param),
            ));
        }
    }
}

fn pick_middle_biome(temperature: usize, humidity: usize, pandv: (f32, f32)) -> Biome {
    if pandv.1 < 0.0 {
        MIDDLE_BIOMES[temperature][humidity]
    } else {
        match MIDDLE_BIOMES_VARIANT[temperature][humidity] {
            Some(biome) => biome,
            None => MIDDLE_BIOMES[temperature][humidity],
        }
    }
}

fn pick_middle_biome_or_badlands_if_hot(
    temperature: usize,
    humidity: usize,
    pandv: (f32, f32),
) -> Biome {
    if temperature == 4 {
        pick_badlands_biome(humidity, pandv)
    } else {
        pick_middle_biome(temperature, humidity, pandv)
    }
}

fn pick_middle_biome_or_badlands_if_hot_or_slope_if_cold(
    temperature: usize,
    humidity: usize,
    pandv: (f32, f32),
) -> Biome {
    if temperature == 0 {
        pick_slope_biome(temperature, humidity, pandv)
    } else {
        pick_middle_biome_or_badlands_if_hot(temperature, humidity, pandv)
    }
}

fn maybe_pick_windswept_savanna_biome(
    temperature: usize,
    humidity: usize,
    pandv: (f32, f32),
    key: Biome,
) -> Biome {
    if temperature > 1 && humidity < 4 && pandv.1 >= 0.0 {
        Biome::WindsweptSavanna
    } else {
        key
    }
}

fn pick_shattered_coast_biome(temperature: usize, humidity: usize, pandv: (f32, f32)) -> Biome {
    let resource_key = if pandv.1 >= 0.0 {
        pick_middle_biome(temperature, humidity, pandv)
    } else {
        pick_beach_biome(temperature, humidity)
    };
    maybe_pick_windswept_savanna_biome(temperature, humidity, pandv, resource_key)
}

fn pick_beach_biome(temperature: usize, _humidity: usize) -> Biome {
    if temperature == 0 {
        Biome::SnowyBeach
    } else if temperature == 4 {
        Biome::Desert
    } else {
        Biome::Beach
    }
}

fn pick_badlands_biome(humidity: usize, pandv: (f32, f32)) -> Biome {
    if humidity < 2 {
        if pandv.1 < 0.0 {
            Biome::Badlands
        } else {
            Biome::ErodedBadlands
        }
    } else if humidity < 3 {
        Biome::Badlands
    } else {
        Biome::WoodedBadlands
    }
}

fn pick_plateau_biome(temperature: usize, humidity: usize, pandv: (f32, f32)) -> Biome {
    if pandv.1 >= 0.0
        && let Some(biome) = PLATEAU_BIOMES_VARIANT[temperature][humidity]
    {
        return biome;
    }
    PLATEAU_BIOMES[temperature][humidity]
}

fn pick_peak_biome(temperature: usize, humidity: usize, pandv: (f32, f32)) -> Biome {
    if temperature <= 2 {
        if pandv.1 < 0.0 {
            Biome::JaggedPeaks
        } else {
            Biome::FrozenPeaks
        }
    } else if temperature == 3 {
        Biome::StonyPeaks
    } else {
        pick_badlands_biome(humidity, pandv)
    }
}

fn pick_slope_biome(temperature: usize, humidity: usize, pandv: (f32, f32)) -> Biome {
    if temperature >= 3 {
        pick_plateau_biome(temperature, humidity, pandv)
    } else if humidity <= 1 {
        Biome::SnowySlopes
    } else {
        Biome::Grove
    }
}

fn pick_shattered_biome(temperature: usize, humidity: usize, pandv: (f32, f32)) -> Biome {
    match SHATTERED_BIOMES[temperature][humidity] {
        Some(biome) => biome,
        None => pick_middle_biome(temperature, humidity, pandv),
    }
}
