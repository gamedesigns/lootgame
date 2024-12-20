use bevy::prelude::*;
use crate::systems::generate_loot_boxes::generate_loot_boxes_system;
use crate::systems::choose_loot_box::choose_loot_box_system;
use crate::systems::open_loot_box::open_loot_box_system;

pub struct LootBoxPlugin;

impl Plugin for LootBoxPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, generate_loot_boxes_system)
            .add_systems(Update, choose_loot_box_system)
            .add_systems(Update, open_loot_box_system);
    }
}
