use bevy::prelude::*;

#[derive(SystemLabel, Debug, Hash, PartialEq, Eq, Clone, Component)]
pub enum ComponentInteraction {
    MOVING,
    IDLE,
}
