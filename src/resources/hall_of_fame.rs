use bevy::prelude::*;

#[derive(Resource)]
pub struct HallOfFame {
    pub scores: Vec<u32>,
}
