use bevy::prelude::*;
use crate::components::loot_box_components::LootBox;
use crate::components::player_components::Player;

pub fn choose_loot_box_system(
    mut commands: Commands,
    player_query: Query<&Player>,
    loot_box_query: Query<(Entity, &LootBox)>,
) {
    if let Ok(_player) = player_query.get_single() {
        let loot_boxes: Vec<(Entity, &LootBox)> = loot_box_query.iter().collect();

        if let Some((loot_box_entity, _)) = loot_boxes.first() {
            commands.entity(*loot_box_entity).despawn();
        }
    }
}
