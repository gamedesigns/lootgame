use bevy::prelude::*;

#[derive(Component)]
pub struct Item {
    pub rarity: ItemRarity,
    pub value: u32,
    pub score_bonus: u32,
    pub category: ItemCategory,
}

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum ItemRarity {
    Common,
    Uncommon,
    Rare,
    Epic,
    Legendary,
}

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum ItemCategory {
    BodyArmor,
    Helmet,
    Weapon,
    Shield,
    Potion,
    Equipment,
}
