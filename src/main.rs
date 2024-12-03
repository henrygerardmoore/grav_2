#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
use std::{
    fs::File,
    io::Read,
};

use bevy::{prelude::*, window::Cursor};

mod components;
mod config;
mod helpers;
mod resources;
mod systems;

use config::Configuration;
use resources::*;
use systems::*;

// TODO(henrygerardmoore): test on macOS
fn main() {
    let config = if let Ok(mut f) = File::open("config.json") {
        let mut data = String::new();
        if f.read_to_string(&mut data).is_ok() {
            match serde_json::from_str(&data) {
                Ok(j) => j,
                Err(_) => {
                    println!("Could not read your config.json");
                    Configuration::default()
                }
            }
        } else {
            println!("Couldn't read config.json as a string");
            Configuration::default()
        }
    } else {
        Configuration::default()
    };

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
        // start unpaused
        .insert_resource(TimePaused(false))
        // insert the common sphere that all bodies use
        .insert_resource(SphereInfo::default())
        // start at 1x time
        .insert_resource(TimeRate(1.))
        // start the spawn selection at default
        .insert_resource(BodySpawningOptions::default())
        // add configuration resource for use by systems
        .insert_resource(config)
        // initialize cursor lock tracking
        .insert_resource(LastFrameUnlocked::default())

        // add startup systems
        .add_systems(Startup, (create_sphere_info, initial_spawn).chain())
        .add_systems(Startup, camera_spawn)
        .add_systems(Startup, create_osd)
        .add_systems(Startup, spawn_help)

        // integration (must be performed in order)
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
        // resetting the world
        .add_systems(Update, reset_bodies)
        .add_systems(Update, reset_camera)
        // changing time rate
        .add_systems(Update, modify_time)
        // spawning bodies
        .add_systems(Update, mouse_button_input)
        // camera
        .add_systems(Update, rotate_camera)
        .add_systems(Update, move_camera)
        // UI
        .add_systems(Update, update_osd)
        .add_systems(Update, show_hide_help)
        // general
        .add_systems(Update, capture_or_release_cursor)
        .add_systems(Update, exit_system)
        .run();
}
