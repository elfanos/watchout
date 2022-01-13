use bevy::prelude::*;

#[derive(SystemLabel, Debug, Hash, PartialEq, Eq, Clone)]
pub enum Movement {
    UP,
    DOWN,
    LEFT,
    RIGHT,
}
