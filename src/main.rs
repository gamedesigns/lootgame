use bevy::prelude::*;
use plugins::{PlayerPlugin, LootBoxPlugin, ItemPlugin, LevelingPlugin, UiPlugin};

mod components;
mod resources;
mod systems;
mod plugins;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugin(PlayerPlugin)
        .add_plugin(LootBoxPlugin)
        .add_plugin(ItemPlugin)
        .add_plugin(LevelingPlugin)
        .add_plugin(UiPlugin)
        .run();
}
