use bevy::prelude::*;
use crate::components::item_components::ItemCategory;
use std::collections::HashMap;

#[derive(Component)]
pub struct Player {
    pub level: u32,
    pub money: u32,
    pub score: u32,
}

#[derive(Component)]
pub struct EquippedItems {
    pub items: HashMap<ItemCategory, Entity>,
}
