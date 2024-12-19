// src/systems/open_loot_box.rs
use bevy::prelude::*;
use crate::components::loot_box_components::LootBox;
use crate::components::player_components::Player;
use crate::components::item_components::Item;

pub fn open_loot_box_system(
    mut commands: Commands,
    mut player_query: Query<&mut Player>,
    loot_box_query: Query<(Entity, &LootBox)>,
    item_query: Query<&Item>,
) {
    let mut player = player_query.single_mut();
    let loot_boxes: Vec<(Entity, &LootBox)> = loot_box_query.iter().collect();

    for (loot_box_entity, loot_box) in loot_boxes {
        for item_entity in &loot_box.items {
            if let Ok(item) = item_query.get(*item_entity) {
                player.score += item.score_bonus;
            }
        }
        commands.entity(loot_box_entity).despawn();
    }
}