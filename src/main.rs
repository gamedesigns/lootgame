use bevy::prelude::*;
use plugins::{PlayerPlugin, LootBoxPlugin, ItemPlugin, LevelingPlugin, UiPlugin};
use systems::update_hall_of_fame::update_hall_of_fame_system;
use crate::resources::hall_of_fame::HallOfFame;
use crate::resources::loot_box_pool::LootBoxPool;

mod components;
mod resources;
mod systems;
mod plugins;
mod ui;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(PlayerPlugin)
        .add_plugins(LootBoxPlugin)
        .add_plugins(ItemPlugin)
        .add_plugins(LevelingPlugin)
        .add_plugins(UiPlugin)
        .insert_resource(HallOfFame { scores: Vec::new() })
        .insert_resource(LootBoxPool)
        .add_systems(Update, update_hall_of_fame_system)
        .run();
}
