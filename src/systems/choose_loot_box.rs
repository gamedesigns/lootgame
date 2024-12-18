use bevy::prelude::*;
use crate::components::loot_box_components::LootBox;
use crate::components::player_components::Player;

pub fn choose_loot_box_system(
    mut commands: Commands,
    mut player_query: Query<&mut Player>,
    loot_box_query: Query<(Entity, &LootBox)>,
) {
    let mut player = player_query.single_mut();
    let loot_boxes: Vec<(Entity, &LootBox)> = loot_box_query.iter().collect();

    // For simplicity, choose the first loot box
    if let Some((loot_box_entity, _)) = loot_boxes.first() {
        // Open the chosen loot box
        commands.entity(*loot_box_entity).despawn();
        // Logic to add items to the player's inventory
    }
}
