use bevy::prelude::*;
use crate::ui::ui_system::ui_system;

pub struct UiPlugin;

impl Plugin for UiPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, ui_system);
    }
}
