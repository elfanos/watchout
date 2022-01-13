use bevy::prelude::*;
use camera::PlayerCamera;
use components::{ComponentInteraction, Player, Velocity};
use materials::Materials;
use meshes::Meshes;
use world::*;

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
    App::build()
        .insert_resource(WindowDescriptor {
            title: "Test!".to_string(),
            width: 500.0,
            height: 500.0,
            ..Default::default()
        })
        .add_plugins(DefaultPlugins)
        .add_startup_system(setup.system())
        .add_startup_stage("spawn_game", SystemStage::single(spawn_game.system()))
        .add_system_set_to_stage(
            CoreStage::PostUpdate,
            SystemSet::new().with_system(set_world_position.system()),
        )
        .add_system_set(
            SystemSet::new().with_system(move_player.system().label(ComponentInteraction::MOVING)),
        )
        .run();
}

// Move the player to a new vector position in the system
fn move_player(
    keys: Res<Input<KeyCode>>,
    time: Res<Time>,
    mut qr: Query<(&mut Transform, &mut Position, &Velocity), With<Player>>,
) {
    for (mut transform, mut position, velocity) in qr.iter_mut() {
        if keys.pressed(KeyCode::Q) {
            let newpos = Vec3::new(
                position.x,
                position.y + (velocity.speed * time.delta_seconds() * 4.0),
                position.z,
            );
            let some_translation = Transform::from_translation(newpos);
            println!(" Going y direction up {}", some_translation.translation);
            position.y = newpos.y;
            transform.translation = some_translation.translation;
        } else if keys.pressed(KeyCode::E) {
            let newpos = Vec3::new(
                position.x,
                position.y - (velocity.speed * time.delta_seconds() * 4.0),
                position.z,
            );
            let some_translation = Transform::from_translation(newpos);
            println!(
                " Going left y-axis direction down {}",
                some_translation.translation
            );
            position.y = newpos.y;
            transform.translation = some_translation.translation;
        } else if keys.pressed(KeyCode::S) {
            let newpos = Vec3::new(
                position.x,
                position.y,
                position.z + (velocity.speed * time.delta_seconds() * 4.0),
            );
            let some_translation = Transform::from_translation(newpos);
            println!(" Going z direction back{}", some_translation.translation);

            position.z = newpos.z;
            transform.translation = some_translation.translation;
        } else if keys.pressed(KeyCode::W) {
            let newpos = Vec3::new(
                position.x,
                position.y,
                position.z - (velocity.speed * time.delta_seconds() * 4.0),
            );
            let some_translation = Transform::from_translation(newpos);
            println!(
                " Going z direction backward {}",
                some_translation.translation
            );
            position.z = newpos.z;
            transform.translation = some_translation.translation;
        } else if keys.pressed(KeyCode::D) {
            let newpos = Vec3::new(
                position.x + (velocity.speed * time.delta_seconds() * 4.0),
                position.y,
                position.z,
            );
            let some_translation = Transform::from_translation(newpos);
            println!(" Going x direction right {}", some_translation.translation);

            position.x = newpos.x;
            transform.translation = some_translation.translation;
        } else if keys.pressed(KeyCode::A) {
            let newpos = Vec3::new(
                position.x - (velocity.speed * time.delta_seconds() * 4.0),
                position.y,
                position.z,
            );
            let some_translation = Transform::from_translation(newpos);
            println!(" Going z direction left {}", some_translation.translation);
            position.x = newpos.x;
            transform.translation = some_translation.translation;
        }
    }
}

fn set_world_position(
    mut parents_query: Query<(Entity), With<Player>>,
    mut transtorm_query: Query<(&Position, &mut Transform)>,
) {
    for parent in parents_query.iter_mut() {
        if let Ok((position, mut transform)) = transtorm_query.get_mut(parent) {
            transform.translation = Vec3::new(position.x, position.y, position.z);
        }
    }
}

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    // commands.spawn_bundle(PbrBundle {
    //     mesh: meshes.add(Mesh::from(shape::Plane { size: 5.0 })),
    //     material: materials.add(Color::rgb(0.3, 0.5, 0.4).into()),
    //     ..Default::default()
    // });
    commands.insert_resource(Materials {
        player: materials.add(Color::rgb(0.5, 0.2, 0.5).into()),
        ground: materials.add(Color::rgb(0.3, 0.5, 0.4).into()),
    });
    commands.insert_resource(Meshes {
        player_mesh: meshes.add(Mesh::from(shape::Cube { size: 0.3 })),
        ground_mesh: meshes.add(Mesh::from(shape::Plane { size: 5.0 })),
    });
}
