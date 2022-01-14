use bevy::prelude::*;
use crate::components::Movement;

#[derive(Component)]
pub struct Player {
    pub direction: Movement,
}
