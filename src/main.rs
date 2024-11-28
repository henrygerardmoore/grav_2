#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
use bevy::{prelude::*, window::Cursor};

mod components;
mod helpers;
mod resources;
mod systems;

use resources::*;
use systems::*;

// TODO(henrygerardmoore): test on macOS
fn main() {
    App::new()
        .add_plugins(
            DefaultPlugins
                .set(ImagePlugin::default_nearest())
                .set(WindowPlugin {
                    primary_window: Some(Window {
                        title: "Henry's 3D N-body gravity sim!".into(),
                        name: Some("grav_2.app".into()),
                        mode: bevy::window::WindowMode::Fullscreen,
                        cursor: Cursor {
                            visible: false,
                            ..default()
                        },
                        ..default()
                    }),
                    ..default()
                }),
        )
        // make the background look like space
        .insert_resource(ClearColor(Color::BLACK))
        .insert_resource(TimePaused(false))
        .insert_resource(SphereInfo::default())
        .insert_resource(TimeRate(1.))
        .add_systems(Update, modify_time)
        .add_systems(Startup, (create_sphere_info, initial_spawn).chain())
        .add_systems(Startup, camera_spawn)
        .add_systems(Startup, create_osd)
        .add_systems(
            Update,
            (
                update_body_velocities,
                update_body_positions,
                resolve_body_collisions,
                update_body_meshes,
            )
                .chain(),
        )
        .insert_resource(BodySpawningOptions::default())
        .add_systems(Update, mouse_button_input)
        .add_systems(Update, capture_or_release_cursor)
        .add_systems(Update, exit_system)
        .add_systems(Update, rotate_camera)
        .add_systems(Update, move_camera)
        .add_systems(Update, reset_bodies)
        .add_systems(Update, reset_camera)
        .add_systems(Update, update_osd)
        .add_systems(Startup, spawn_help)
        .add_systems(Update, show_hide_help)
        .run();
}
