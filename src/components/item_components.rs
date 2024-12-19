// src/components/item_components.rs
use bevy::prelude::*;
use std::fmt;

#[derive(Component, Clone, Copy, PartialEq, Eq, Hash, Debug)]
pub enum ItemCategory {
    BodyArmor,
    Helmet,
    Weapon,
    Shield,
    Potion,
    Equipment,
}

impl fmt::Display for ItemCategory {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

#[derive(Component, Clone, Copy, PartialEq, Eq, Debug)]
pub enum ItemRarity {
    Common,
    Uncommon,
    Rare,
    Epic,
    Legendary,
}

impl fmt::Display for ItemRarity {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

#[derive(Component)]
pub struct Item {
    pub rarity: ItemRarity,
    pub value: u32,
    pub score_bonus: u32,
    pub category: ItemCategory,
}