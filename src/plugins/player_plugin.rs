use bevy::prelude::*;
use crate::components::player_components::{Player, EquippedItems};
use std::collections::HashMap;

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(Player {
            level: 1,
            money: 0,
            score: 0,
        })
        .insert_resource(EquippedItems {
            items: HashMap::new(),
        });
    }
}
