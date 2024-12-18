use bevy::prelude::*;
use crate::systems::level_up::level_up_system;

pub struct LevelingPlugin;

impl Plugin for LevelingPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, level_up_system);
    }
}
