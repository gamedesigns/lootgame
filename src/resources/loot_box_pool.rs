use bevy::prelude::*;
use crate::components::loot_box_components::LootBoxRarity;

#[derive(Resource)]
pub struct LootBoxPool {
    pub pool: Vec<LootBoxRarity>,
}
