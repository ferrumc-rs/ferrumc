use ferrumc_macros::{packet, NetEncode};
use ferrumc_net_codec::encode::{NetEncode, NetEncodeOpts, NetEncodeResult};
use std::io::Write;
use tokio::io::AsyncWrite;

#[derive(Debug, Copy, Clone)]
pub enum ContainerProperty {
    // Furnace
    FurnaceFireIcon(u16),
    FurnaceMaxFuelTime(u16),
    FurnaceProgressArrow(u16),
    FurnaceMaxProgress(u16),

    // Enchantment
    EnchantmentLevelTop(u16),
    EnchantmentLevelMiddle(u16),
    EnchantmentLevelBottom(u16),
    EnchantmentSeed(u16),
    EnchantmentIdTop(u16),
    EnchantmentIdMiddle(u16),
    EnchantmentIdBottom(u16),
    EnchantmentLevelIdTop(u16),
    EnchantmentLevelIdMiddle(u16),
    EnchantmentLevelIdBottom(u16),

    // Beacon
    BeaconPowerLevel(u16),
    BeaconFirstPotionEffect(u16),
    BeaconSecondPotionEffect(u16),

    // Anvil
    AnvilRepairCost(u16),

    // Brewing Stand
    BrewingStandBrewTime(u16),
    BrewingStandFuelTime(u16),

    // Stonecutter
    StonecutterSelectedRecipe(u16),

    // Loom
    LoomSelectedPattern(u16),

    // Lectern
    LecternPageNumber(u16),
}

impl ContainerProperty {
    pub fn get_property_id(&self) -> u16 {
        match self {
            // Furnace
            ContainerProperty::FurnaceFireIcon(_) => 0,
            ContainerProperty::FurnaceMaxFuelTime(_) => 1,
            ContainerProperty::FurnaceProgressArrow(_) => 2,
            ContainerProperty::FurnaceMaxProgress(_) => 3,

            // Enchantment Table
            ContainerProperty::EnchantmentLevelTop(_) => 0,
            ContainerProperty::EnchantmentLevelMiddle(_) => 1,
            ContainerProperty::EnchantmentLevelBottom(_) => 2,
            ContainerProperty::EnchantmentSeed(_) => 3,
            ContainerProperty::EnchantmentIdTop(_) => 4,
            ContainerProperty::EnchantmentIdMiddle(_) => 5,
            ContainerProperty::EnchantmentIdBottom(_) => 6,
            ContainerProperty::EnchantmentLevelIdTop(_) => 7,
            ContainerProperty::EnchantmentLevelIdMiddle(_) => 8,
            ContainerProperty::EnchantmentLevelIdBottom(_) => 9,

            // Beacon
            ContainerProperty::BeaconPowerLevel(_) => 0,
            ContainerProperty::BeaconFirstPotionEffect(_) => 1,
            ContainerProperty::BeaconSecondPotionEffect(_) => 2,

            // Anvil
            ContainerProperty::AnvilRepairCost(_) => 0,

            // Brewing Stand
            ContainerProperty::BrewingStandBrewTime(_) => 0,
            ContainerProperty::BrewingStandFuelTime(_) => 1,

            // Stonecutter
            ContainerProperty::StonecutterSelectedRecipe(_) => 0,

            // Loom
            ContainerProperty::LoomSelectedPattern(_) => 0,

            // Lectern
            ContainerProperty::LecternPageNumber(_) => 0,
        }
    }
}

impl NetEncode for ContainerProperty {
    fn encode<W: Write>(&self, writer: &mut W, opts: &NetEncodeOpts) -> NetEncodeResult<()> {
        self.get_property_id().encode(writer, opts)?;
        match self {
            Self::FurnaceFireIcon(value)
            | Self::FurnaceMaxFuelTime(value)
            | Self::FurnaceProgressArrow(value)
            | Self::FurnaceMaxProgress(value)
            | Self::EnchantmentLevelTop(value)
            | Self::EnchantmentLevelMiddle(value)
            | Self::EnchantmentLevelBottom(value)
            | Self::EnchantmentSeed(value)
            | Self::EnchantmentIdTop(value)
            | Self::EnchantmentIdMiddle(value)
            | Self::EnchantmentIdBottom(value)
            | Self::EnchantmentLevelIdTop(value)
            | Self::EnchantmentLevelIdMiddle(value)
            | Self::EnchantmentLevelIdBottom(value)
            | Self::BeaconPowerLevel(value)
            | Self::BeaconFirstPotionEffect(value)
            | Self::BeaconSecondPotionEffect(value)
            | Self::AnvilRepairCost(value)
            | Self::BrewingStandBrewTime(value)
            | Self::BrewingStandFuelTime(value)
            | Self::StonecutterSelectedRecipe(value)
            | Self::LoomSelectedPattern(value)
            | Self::LecternPageNumber(value) => value.encode(writer, opts),
        }
    }

    async fn encode_async<W: AsyncWrite + Unpin>(
        &self,
        writer: &mut W,
        opts: &NetEncodeOpts,
    ) -> NetEncodeResult<()> {
        self.get_property_id().encode_async(writer, opts).await?;
        match self {
            Self::FurnaceFireIcon(value)
            | Self::FurnaceMaxFuelTime(value)
            | Self::FurnaceProgressArrow(value)
            | Self::FurnaceMaxProgress(value)
            | Self::EnchantmentLevelTop(value)
            | Self::EnchantmentLevelMiddle(value)
            | Self::EnchantmentLevelBottom(value)
            | Self::EnchantmentSeed(value)
            | Self::EnchantmentIdTop(value)
            | Self::EnchantmentIdMiddle(value)
            | Self::EnchantmentIdBottom(value)
            | Self::EnchantmentLevelIdTop(value)
            | Self::EnchantmentLevelIdMiddle(value)
            | Self::EnchantmentLevelIdBottom(value)
            | Self::BeaconPowerLevel(value)
            | Self::BeaconFirstPotionEffect(value)
            | Self::BeaconSecondPotionEffect(value)
            | Self::AnvilRepairCost(value)
            | Self::BrewingStandBrewTime(value)
            | Self::BrewingStandFuelTime(value)
            | Self::StonecutterSelectedRecipe(value)
            | Self::LoomSelectedPattern(value)
            | Self::LecternPageNumber(value) => value.encode_async(writer, opts).await,
        }
    }
}

#[derive(NetEncode)]
#[packet(packet_id = "container_set_data", state_id = "play")]
pub struct SetContainerPropertyPacket {
    pub window_id: u8,
    pub property: ContainerProperty,
}

impl SetContainerPropertyPacket {
    pub fn new(window_id: u8, property: ContainerProperty) -> Self {
        Self {
            window_id,
            property,
        }
    }
}
