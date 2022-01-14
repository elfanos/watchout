use bevy::prelude::*;
use crate::components::Movement;
use bevy::prelude::*;

#[derive(Component)]
pub struct Player {
    pub direction: Movement,
}
