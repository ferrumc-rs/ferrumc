#![allow(non_snake_case)]

#[derive(Debug, serde::Deserialize)]
pub struct WorldData {
    Data: WorldDataDetails,
}

#[derive(Debug, serde::Deserialize)]
struct WorldDataDetails {
    Difficulty: i8,
    thunderTime: i64,
    BorderSize: f64,
    LastPlayed: i64,
    allowCommands: bool,
    BorderCenterX: f64,
    initialized: bool,
    BorderWarningBlocks: f64,
    hardcore: bool,
    version: i32,
    ServerBrands: Vec<String>,
    SpawnX: i32,
    GameType: i32,
    BorderSafeZone: f64,
    SpawnAngle: f32,
    LevelName: String,
    Time: i64,
    ScheduledEvents: Vec<String>,
    clearWeatherTime: i32,
    BorderDamagePerBlock: f64,
    WanderingTraderSpawnDelay: i32,
    //Bukkit: BukkitData,
    thundering: bool,
    WasModded: bool,
    BorderWarningTime: f64,
    WanderingTraderSpawnChance: i32,
    SpawnY: i32,
    SpawnZ: i32,
    BorderSizeLerpTime: i64,
    raining: bool,
    WorldGenSettings: WorldGenSettings,
    rainTime: i64,
    DataPacks: DataPacks,
    DataVersion: i32,
    GameRules: GameRules,
    DragonFight: DragonFight,
    DifficultyLocked: bool,
    DayTime: i64,
    BorderCenterZ: f64,
    BorderSizeLerpTarget: f64,
    Version: Version,
    CustomBossEvents: serde_json::Value, // Represents an empty object
}

#[derive(Debug, serde::Deserialize)]
struct BukkitData {
    Version: String,
}

#[derive(Debug, serde::Deserialize)]
struct WorldGenSettings {
    bonus_chest: bool,
    generate_features: bool,
    dimensions: Dimensions,
    seed: i64,
}

#[derive(Debug, serde::Deserialize)]
struct Dimensions {
    #[serde(rename = "minecraft:the_nether")]
    minecraft_the_nether: Dimension,
    #[serde(rename = "minecraft:overworld")]
    minecraft_overworld: Dimension,
    #[serde(rename = "minecraft:the_end")]
    minecraft_the_end: Dimension,
}

#[derive(Debug, serde::Deserialize)]
struct Dimension {
    r#type: String,
    generator: Generator,
}

#[derive(Debug, serde::Deserialize)]
struct Generator {
    r#type: String,
    biome_source: BiomeSource,
    settings: String,
}

#[derive(Debug, serde::Deserialize)]
struct BiomeSource {
    r#type: String,
    preset: Option<String>,
}

#[derive(Debug, serde::Deserialize)]
struct DataPacks {
    Enabled: Vec<String>,
    Disabled: Vec<String>,
}

#[derive(Debug, serde::Deserialize)]
struct GameRules {
    forgiveDeadPlayers: String,
    doInsomnia: String,
    fallDamage: String,
    doDaylightCycle: String,
    spawnRadius: String,
    doWeatherCycle: String,
    globalSoundEvents: String,
    doPatrolSpawning: String,
    maxCommandChainLength: String,
    universalAnger: String,
    doImmediateRespawn: String,
    fireDamage: String,
    playersSleepingPercentage: String,
    mobExplosionDropDecay: String,
    maxEntityCramming: String,
    doMobSpawning: String,
    lavaSourceConversion: String,
    tntExplosionDropDecay: String,
    showDeathMessages: String,
    announceAdvancements: String,
    disableRaids: String,
    sendCommandFeedback: String,
    naturalRegeneration: String,
    reducedDebugInfo: String,
    doFireTick: String,
    drowningDamage: String,
    blockExplosionDropDecay: String,
    doLimitedCrafting: String,
    commandBlockOutput: String,
    doTraderSpawning: String,
    doVinesSpread: String,
    spectatorsGenerateChunks: String,
    snowAccumulationHeight: String,
    mobGriefing: String,
    doEntityDrops: String,
    doTileDrops: String,
    keepInventory: String,
    randomTickSpeed: String,
    doWardenSpawning: String,
    freezeDamage: String,
    doMobLoot: String,
    commandModificationBlockLimit: String,
    waterSourceConversion: String,
    logAdminCommands: String,
    disableElytraMovementCheck: String,
}

#[derive(Debug, serde::Deserialize)]
struct DragonFight {
    PreviouslyKilled: bool,
    NeedsStateScanning: bool,
    DragonKilled: bool,
}

#[derive(Debug, serde::Deserialize)]
struct Version {
    Name: String,
    Series: String,
    Snapshot: bool,
    Id: i32,
}