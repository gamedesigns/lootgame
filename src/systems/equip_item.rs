use bevy::prelude::*;
use crate::components::item_components::Item;
use crate::components::player_components::{Player, EquippedItems};

pub fn equip_item_system(
    _commands: Commands,
    mut player_query: Query<(&mut Player, &mut EquippedItems)>,
    item_query: Query<(Entity, &Item)>,
) {
    if let Ok((mut player, mut equipped_items)) = player_query.get_single_mut() {
        let items: Vec<(Entity, &Item)> = item_query.iter().collect();

        for (item_entity, item) in items {
            if let Some(old_item_entity) = equipped_items.items.get(&item.category) {
                if let Ok(old_item_entry) = item_query.get(*old_item_entity) {
                    player.score -= old_item_entry.1.score_bonus;
                }
                equipped_items.items.insert(item.category, item_entity);
            } else {
                equipped_items.items.insert(item.category, item_entity);
            }
            player.score += item.score_bonus;
        }
    }
}
