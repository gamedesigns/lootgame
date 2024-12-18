use bevy::prelude::*;

#[derive(Component, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ItemCategory {
    BodyArmor,
    Helmet,
    Weapon,
    Shield,
    Potion,
    Equipment,
}

#[derive(Component, Clone, Copy, PartialEq, Eq)]
pub enum ItemRarity {
    Common,
    Uncommon,
    Rare,
    Epic,
    Legendary,
}

#[derive(Component)]
pub struct Item {
    pub rarity: ItemRarity,
    pub value: u32,
    pub score_bonus: u32,
    pub category: ItemCategory,
}
