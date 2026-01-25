//! Fluent builder API for creating inventory slots with components.
//!
//! # Example
//! ```ignore
//! use ferrumc_inventories::{ItemBuilder, Component};
//! use ferrumc_inventories::components::Rarity;
//!
//! let item = ItemBuilder::new(862)  // Diamond ID
//!     .count(64)
//!     .custom_name("Epic Diamond")
//!     .rarity(Rarity::Epic)
//!     .enchantment_glint(true)
//!     .lore(["A legendary gem", "Forged in starfire"])
//!     .build();
//! ```

use crate::components::{Component, EnchantComponent, Rarity};
use crate::item::ItemID;
use crate::slot::InventorySlot;
use ferrumc_net_codec::net_types::var_int::VarInt;
use ferrumc_text::TextComponent;

/// A fluent builder for creating inventory slots with components.
///
/// # Example
/// ```ignore
/// let item = ItemBuilder::new(862)
///     .count(64)
///     .custom_name("Epic Diamond")
///     .rarity(Rarity::Epic)
///     .build();
/// ```
#[derive(Debug, Clone)]
pub struct ItemBuilder {
    item_id: ItemID,
    count: i32,
    components: Vec<Component>,
    components_to_remove: Vec<VarInt>,
}

impl ItemBuilder {
    /// Creates a new ItemBuilder from a raw item ID.
    ///
    /// # Example
    /// ```ignore
    /// let builder = ItemBuilder::new(862); // Diamond
    /// ```
    pub fn new(item_id: i32) -> Self {
        Self {
            item_id: ItemID::new(item_id),
            count: 1,
            components: Vec::new(),
            components_to_remove: Vec::new(),
        }
    }

    /// Creates a new ItemBuilder from an item name.
    ///
    /// Returns `None` if the item name is not found in the registry.
    ///
    /// # Example
    /// ```ignore
    /// let builder = ItemBuilder::from_name("minecraft:diamond")?;
    /// let builder = ItemBuilder::from_name("diamond")?; // Also works
    /// ```
    pub fn from_name(name: &str) -> Option<Self> {
        ItemID::from_name(name).map(|id| Self {
            item_id: id,
            count: 1,
            components: Vec::new(),
            components_to_remove: Vec::new(),
        })
    }

    /// Creates a new ItemBuilder from an ItemID.
    pub fn from_id(item_id: ItemID) -> Self {
        Self {
            item_id,
            count: 1,
            components: Vec::new(),
            components_to_remove: Vec::new(),
        }
    }

    /// Sets the item count.
    pub fn count(mut self, count: i32) -> Self {
        self.count = count;
        self
    }

    /// Adds a raw component to the item.
    pub fn component(mut self, component: Component) -> Self {
        self.components.push(component);
        self
    }

    /// Marks a component type ID for removal from the item's default components.
    pub fn remove_component(mut self, component_id: i32) -> Self {
        self.components_to_remove.push(VarInt(component_id));
        self
    }

    /// Builds the final InventorySlot.
    pub fn build(self) -> InventorySlot {
        InventorySlot {
            count: VarInt(self.count),
            item_id: Some(self.item_id),
            components_to_add: self.components,
            components_to_remove: self.components_to_remove,
        }
    }

    // =========================================================================
    // Fluent Component Methods
    // =========================================================================

    /// Sets a custom display name for the item.
    ///
    /// # Example
    /// ```ignore
    /// builder.custom_name("Epic Sword")
    /// builder.custom_name(TextComponent::from("Colored").color(NamedColor::Gold))
    /// ```
    pub fn custom_name(self, name: impl Into<TextComponent>) -> Self {
        self.component(Component::custom_name(name))
    }

    /// Sets the item name (different from custom_name - this is the base name).
    pub fn item_name(self, name: impl Into<TextComponent>) -> Self {
        self.component(Component::item_name(name))
    }

    /// Sets the item lore (description lines shown in tooltip).
    ///
    /// # Example
    /// ```ignore
    /// builder.lore(["Line 1", "Line 2"])
    /// ```
    pub fn lore<I, T>(self, lines: I) -> Self
    where
        I: IntoIterator<Item = T>,
        T: Into<TextComponent>,
    {
        self.component(Component::lore(lines))
    }

    /// Sets the item rarity (affects name color).
    pub fn rarity(self, rarity: Rarity) -> Self {
        self.component(Component::Rarity(rarity))
    }

    /// Sets the maximum stack size for this item.
    pub fn max_stack_size(self, size: i32) -> Self {
        self.component(Component::max_stack_size(size))
    }

    /// Sets the current damage value.
    pub fn damage(self, value: i32) -> Self {
        self.component(Component::damage(value))
    }

    /// Sets the maximum damage (durability) for this item.
    pub fn max_damage(self, value: i32) -> Self {
        self.component(Component::max_damage(value))
    }

    /// Makes the item unbreakable.
    pub fn unbreakable(self) -> Self {
        self.component(Component::Unbreakable)
    }

    /// Overrides the enchantment glint (shimmer effect).
    ///
    /// - `true` = always show glint
    /// - `false` = never show glint
    pub fn enchantment_glint(self, enabled: bool) -> Self {
        self.component(Component::EnchantmentGlintOverride(enabled))
    }

    /// Sets the repair cost in an anvil.
    pub fn repair_cost(self, cost: i32) -> Self {
        self.component(Component::repair_cost(cost))
    }

    /// Sets whether the item can be enchanted and at what level.
    pub fn enchantable(self, value: i32) -> Self {
        self.component(Component::enchantable(value))
    }

    /// Adds enchantments to the item.
    ///
    /// # Example
    /// ```ignore
    /// builder.enchantments([
    ///     Component::enchantment(0, 5),  // Sharpness V
    ///     Component::enchantment(9, 3),  // Unbreaking III
    /// ])
    /// ```
    pub fn enchantments<I>(self, enchants: I) -> Self
    where
        I: IntoIterator<Item = EnchantComponent>,
    {
        self.component(Component::enchantments(enchants))
    }

    /// Adds stored enchantments (for enchanted books).
    pub fn stored_enchantments<I>(self, enchants: I) -> Self
    where
        I: IntoIterator<Item = EnchantComponent>,
    {
        self.component(Component::stored_enchantments(enchants))
    }

    /// Makes this item a food item.
    pub fn food(self, nutrition: i32, saturation: f32, can_always_eat: bool) -> Self {
        self.component(Component::food(nutrition, saturation, can_always_eat))
    }

    /// Makes this item a weapon.
    pub fn weapon(self, damage: i32, disable_blocking_seconds: f32) -> Self {
        self.component(Component::weapon(damage, disable_blocking_seconds))
    }

    /// Sets the dyed color (for leather armor, etc.).
    pub fn dyed_color(self, color: i32) -> Self {
        self.component(Component::dyed_color(color))
    }

    /// Locks the item to creative mode only.
    pub fn creative_slot_lock(self) -> Self {
        self.component(Component::CreativeSlotLock)
    }

    /// Makes this item a glider (elytra-like behavior).
    pub fn glider(self) -> Self {
        self.component(Component::Glider)
    }
}

impl From<ItemBuilder> for InventorySlot {
    fn from(builder: ItemBuilder) -> Self {
        builder.build()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_simple_item() {
        let item = ItemBuilder::new(1).count(64).build();

        assert_eq!(item.count.0, 64);
        assert!(item.item_id.is_some());
        assert!(item.components_to_add.is_empty());
    }

    #[test]
    fn test_item_with_components() {
        let item = ItemBuilder::new(862) // Diamond
            .count(1)
            .rarity(Rarity::Epic)
            .enchantment_glint(true)
            .max_stack_size(99)
            .build();

        assert_eq!(item.count.0, 1);
        assert_eq!(item.components_to_add.len(), 3);
    }

    #[test]
    fn test_item_with_custom_name() {
        let item = ItemBuilder::new(862).custom_name("Epic Diamond").build();

        assert_eq!(item.components_to_add.len(), 1);
        // Verify it's a CustomName component
        assert_eq!(item.components_to_add[0].id().0, 5);
    }

    #[test]
    fn test_item_with_lore() {
        let item = ItemBuilder::new(862)
            .lore(["Line 1", "Line 2", "Line 3"])
            .build();

        assert_eq!(item.components_to_add.len(), 1);
        // Verify it's a Lore component
        assert_eq!(item.components_to_add[0].id().0, 8);
    }

    #[test]
    fn test_from_name() {
        // This test depends on the registry being available
        if let Some(builder) = ItemBuilder::from_name("diamond") {
            let item = builder.count(32).build();
            assert_eq!(item.count.0, 32);
        }
    }

    #[test]
    fn test_builder_into_slot() {
        let slot: InventorySlot = ItemBuilder::new(1).count(64).into();
        assert_eq!(slot.count.0, 64);
    }
}
