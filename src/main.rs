use bevy::prelude::*;
use components::{Health, Player};
use world::*;

mod components;
mod materials;
mod world;

fn main() {
    println!("Hello, world!");
    App::build()
        .insert_resource(Msaa { samples: 4 })
        .add_plugins(DefaultPlugins)
        .add_startup_system(setup.system())
        .add_startup_system(spawn_player.system())
        // .add_system(world.system())
        // .add_system(player_controller.system())
        // .add_system(get_player_position.system())
        .run();
}

fn world() {
    println!("hellop");
}

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    // commands.spawn_batch(bundles_iter)
    // Plane
    commands.spawn_bundle(PbrBundle {
        mesh: meshes.add(Mesh::from(shape::Plane { size: 5.0 })),
        material: materials.add(Color::rgb(0.3, 0.5, 0.4).into()),
        ..Default::default()
    });

    // cube
    commands.spawn_bundle(PbrBundle {
        mesh: meshes.add(Mesh::from(shape::Cube { size: 1.0 })),
        material: materials.add(Color::rgb(0.8, 0.7, 0.6).into()),
        ..Default::default()
    });

    commands.spawn_bundle(LightBundle {
        light: Light {
            color: Color::rgb(0.2, 0.3, 0.2),
            intensity: 1500.0,
            ..Default::default()
        },
        transform: Transform::from_xyz(4.0, 8.0, 4.0),
        ..Default::default()
    });

    commands.spawn_bundle(PerspectiveCameraBundle {
        transform: Transform::from_xyz(-2.0, 2.5, 5.0).looking_at(Vec3::ZERO, Vec3::Y),
        ..Default::default()
    });
}

fn player_controller(query: Query<&Health, With<Player>>) {
    for name in query.iter() {
        println!("Some cole healthj {}", name.0);
    }
}
fn get_player_position(query: Query<&Position, With<Player>>) {
    for pos in query.iter() {
        println!("Some position x = {} y = {}, z = {}", pos.x, pos.y, pos.z);
    }
}
