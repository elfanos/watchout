extern crate bevy_rapier3d;

use bevy::prelude::*;
use camera::PlayerCamera;
use components::{ComponentInteraction, Player, Velocity};
use materials::Materials;
use meshes::Meshes;
use world::*;
use bevy_rapier3d::prelude::*;

mod camera;
mod components;
mod materials;
mod meshes;
mod world;

// #[derive(Bundle)]
// struct Test {
//     #[bundle]
//     test: PerspectiveCameraBundle,
// }

fn main() {
    App::new()
        .insert_resource(WindowDescriptor {
            title: "Test!".to_string(),
            width: 500.0,
            height: 500.0,
            ..Default::default()
        })
        .add_plugins(DefaultPlugins)
        .add_plugin(RapierPhysicsPlugin::<NoUserData>::default())
        .add_plugin(RapierRenderPlugin)
        .add_startup_system(setup)
        .add_startup_stage("spawn_game", SystemStage::single(spawn_game))
        .add_system(update_camera_translation_from_player)
        // .add_system_set_to_stage(
        //     CoreStage::PostUpdate,
        //     SystemSet::new().with_system(set_world_position),
        // )
        .add_system_set(
            SystemSet::new().with_system(move_player.label(ComponentInteraction::MOVING)),
        )
        .run();
}

fn update_camera_translation_from_player (mut qr: Query<(&mut PlayerCamera, &mut Transform)>, 
    mut rb_query: Query<&mut RigidBodyPositionComponent>,
) {
    
    for (camera, mut camera_transform) in qr.iter_mut() {
        let player_position = rb_query.get_mut(camera.player).expect("Could not get player transform");
        let player_vector = player_position.position.translation;
        player_vector.x;

        camera_transform.look_at(Vec3::new(player_vector.x, player_vector.y, player_vector.z), Vec3::Y);
    }
}

// Move the player to a new vector position in the system
fn move_player(
    keys: Res<Input<KeyCode>>,
    time: Res<Time>,
    mut qr: Query<(&mut RigidBodyForcesComponent, &Velocity), With<Player>>,
) {
    for (mut rb_force, velocity) in qr.iter_mut() {
        // let mut position = rb_pos.position.translation.vector;
        // rb_pos.position.translation = rb_pos.position.translation + Vec3::new(0.0, 0.0, 0.0);
    
        if keys.pressed(KeyCode::Q) {
            rb_force.force = Vec3::new(0.0, 1.0, 0.0).into();
            // position.y = position.y + (velocity.speed * time.delta_seconds() * 4.0);
        } else if keys.pressed(KeyCode::E) {
            rb_force.force = Vec3::new(0.0, -1.0, 0.0).into();
            // position.y = position.y - (velocity.speed * time.delta_seconds() * 4.0);
        } else if keys.pressed(KeyCode::S) {
            rb_force.force = Vec3::new(0.0, 0.0, 1.0).into();
            // position.z = position.z + (velocity.speed * time.delta_seconds() * 4.0);
        } else if keys.pressed(KeyCode::W) {
            rb_force.force = Vec3::new(0.0, 0.0, -1.0).into();
            // position.z = position.z - (velocity.speed * time.delta_seconds() * 4.0);
        } else if keys.pressed(KeyCode::D) {
            rb_force.force = Vec3::new(1.0, 0.0, 0.0).into();
            // position.x = position.x + (velocity.speed * time.delta_seconds() * 4.0);
        } else if keys.pressed(KeyCode::A) {
            rb_force.force = Vec3::new(-1.0, 0.0, 0.0).into();
            // position.x = position.x - (velocity.speed * time.delta_seconds() * 4.0);
        }
    }
}

// fn set_world_position(
//     mut parents_query: Query<(Entity), With<Player>>,
//     mut transtorm_query: Query<(&Position, &mut Transform)>,
// ) {
//     for parent in parents_query.iter_mut() {
//         if let Ok((position, mut transform)) = transtorm_query.get_mut(parent) {
//             transform.translation = Vec3::new(position.x, position.y, position.z);
//         }
//     }
// }

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    commands.insert_resource(Materials {
        player: materials.add(Color::rgb(0.5, 0.2, 0.5).into()),
        ground: materials.add(Color::rgb(0.3, 0.5, 0.4).into()),
    });
    commands.insert_resource(Meshes {
        player_mesh: meshes.add(Mesh::from(bevy::prelude::shape::Cube { size: 0.3 })),
        ground_mesh: meshes.add(Mesh::from(bevy::prelude::shape::Plane { size: 5.0 })),
    });
}