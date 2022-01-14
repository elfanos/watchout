use crate::components::Movement;
use bevy::prelude::*;

// 0.6
#[derive(Component)]
pub struct Player {
    pub direction: Movement,
}
