use bevy::prelude::*;
use crate::components::item_components::ItemCategory;
use std::collections::HashMap;

#[derive(Component, Resource)]
pub struct Player {
    pub level: u32,
    pub money: u32,
    pub score: u32,
}

#[derive(Component, Resource)]
pub struct EquippedItems {
    pub items: HashMap<ItemCategory, Entity>,
}
