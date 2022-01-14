use bevy::prelude::*;

use crate::{
    components::{Health, Movement, Player, Velocity},
    materials::Materials,
    meshes::Meshes,
    world::position::Position,
    PlayerCamera,
};

pub fn spawn_game(mut commands: Commands, materials: Res<Materials>, meshes: Res<Meshes>) {
    let world = commands
        .spawn_bundle(PbrBundle {
            mesh: meshes.ground_mesh.clone(),
            material: materials.ground.clone(),
            ..Default::default()
        })
        .id();

    let player = commands
        .spawn_bundle(PbrBundle {
            mesh: meshes.player_mesh.clone(),
            material: materials.player.clone(),
            ..Default::default()
        })
        .insert(Player {
            direction: Movement::LEFT,
        })
        .insert(Position {
            x: 0.,
            y: 0.,
            z: 0.,
        })
        .insert(Health("1".to_string()))
        .insert(Velocity { speed: 1.0 })
        .id();

    let player_camera = commands
        .spawn_bundle(PerspectiveCameraBundle {
            transform: Transform::from_xyz(0.0, 1.5, 5.0).looking_at(Vec3::ZERO, Vec3::Y),
            ..Default::default()
        })
        .insert(PlayerCamera)
        .insert(Position {
            x: -2.0,
            y: 2.5,
            z: 5.0,
        })
        .id();

    commands.entity(player).push_children(&[player_camera]);
    commands.entity(world).push_children(&[player]);
}
