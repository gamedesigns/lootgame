use bevy::prelude::*;

#[derive(Component)]
pub struct LootBox {
    pub rarity: LootBoxRarity,
    pub items: Vec<Entity>,
}

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum LootBoxRarity {
    Common,
    Uncommon,
    Rare,
    Epic,
    Legendary,
}
