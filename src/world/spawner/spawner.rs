use bevy::prelude::*;

use crate::{
    components::{Health, Player},
    world::position::Position,
};

pub fn spawn_player(mut commands: Commands) {
    commands
        .spawn()
        .insert(Player)
        .insert(Position {
            x: 0.,
            y: 0.,
            z: 0.,
        })
        .insert(Health("1".to_string()));
}

/**
 * Spawns the world ground
 */
pub fn spawn_ground(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    commands.spawn_bundle(PbrBundle {
        mesh: meshes.add(Mesh::from(shape::Plane { size: 5.0 })),
        material: materials.add(Color::rgb(0.3, 0.5, 0.4).into()),
        ..Default::default()
    });
}

/**
 * Spawns world camera
 */
pub fn spawn_camera(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) -> PerspectiveCameraBundle {
    PerspectiveCameraBundle {
        transform: Transform::from_xyz(-2.0, 2.5, 5.0).looking_at(Vec3::ZERO, Vec3::Y),
        ..Default::default()
    }
}

/**
 * Spawn world lights
 */
pub fn spawn_lights(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    commands.spawn_bundle(LightBundle {
        light: Light {
            color: Color::rgb(0.2, 0.3, 0.2),
            intensity: 1500.0,
            ..Default::default()
        },
        transform: Transform::from_xyz(4.0, 8.0, 4.0),
        ..Default::default()
    });
}
