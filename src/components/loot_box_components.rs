use bevy::prelude::*;
use std::fmt;

#[derive(Component, Clone, Copy, PartialEq, Eq)]
pub enum LootBoxRarity {
    Common,
    Uncommon,
    Rare,
    Epic,
    Legendary,
}

impl fmt::Display for LootBoxRarity {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

#[derive(Component)]
pub struct LootBox {
    pub rarity: LootBoxRarity,
    pub items: Vec<Entity>,
}
