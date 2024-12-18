use bevy::prelude::*;
use crate::components::item_components::Item;
use crate::components::player_components::{Player, EquippedItems};

pub fn sell_item_system(
    mut commands: Commands,
    mut player_query: Query<(&mut Player, &mut EquippedItems)>,
    item_query: Query<(Entity, &Item)>,
) {
    let (mut player, mut equipped_items) = player_query.single_mut();
    let items: Vec<(Entity, &Item)> = item_query.iter().collect();

    for (item_entity, item) in items {
        player.money += item.value;
        equipped_items.items.retain(|&x, _| x != item_entity);
        commands.entity(item_entity).despawn();
    }
}
