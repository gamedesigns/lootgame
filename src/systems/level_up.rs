use bevy::prelude::*;
use crate::components::player_components::Player;

pub fn level_up_system(mut player_query: Query<&mut Player>) {
    if let Ok(mut player) = player_query.get_single_mut() {
        if player.money >= 100 { // Example cost to level up
            player.money -= 100;
            player.level += 1;
        }
    }
}
