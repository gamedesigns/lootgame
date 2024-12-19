// src/main.rs
use bevy::prelude::*;
use plugins::{PlayerPlugin, LootBoxPlugin, ItemPlugin, LevelingPlugin, UiPlugin};

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
        .run();
}