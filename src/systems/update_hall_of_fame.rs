use bevy::prelude::*;
use crate::components::player_components::Player;
use crate::resources::hall_of_fame::HallOfFame;

pub fn update_hall_of_fame_system(
    mut commands: Commands,
    player_query: Query<&Player>,
    mut hall_of_fame: ResMut<HallOfFame>,
) {
    let player = player_query.single();
    hall_of_fame.scores.push(player.score);
    hall_of_fame.scores.sort_by(|a, b| b.cmp(a));
    hall_of_fame.scores.truncate(10); // Keep only top 10 scores
}
