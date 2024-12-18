use bevy::prelude::*;
use crate::systems::equip_item::equip_item_system;
use crate::systems::sell_item::sell_item_system;

pub struct ItemPlugin;

impl Plugin for ItemPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(equip_item_system)
            .add_system(sell_item_system);
    }
}
