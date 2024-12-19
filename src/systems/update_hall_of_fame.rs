use bevy::prelude::*;
use crate::components::player_components::Player;
use crate::resources::hall_of_fame::HallOfFame;

pub fn update_hall_of_fame_system(
    player_query: Query<&Player>,
    mut hall_of_fame: ResMut<HallOfFame>,
) {
    if let Ok(player) = player_query.get_single() {
        hall_of_fame.scores.push(player.score);
        hall_of_fame.scores.sort_by(|a, b| b.cmp(a));
        hall_of_fame.scores.truncate(10);
    }
}
