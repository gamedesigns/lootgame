use bevy::prelude::*;
use crate::systems::update_hall_of_fame::update_hall_of_fame_system;

pub struct UiPlugin;

impl Plugin for UiPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(update_hall_of_fame_system);
        // Add UI-related systems
    }
}
