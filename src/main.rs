#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
use std::{fs::File, io::Read, path::PathBuf};

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
// TODO(henrygerardmoore): allow bodies to be spawned from a config file
fn main() {
    let config_path: Option<PathBuf> = if let Ok(path) = std::env::current_exe() {
        match path.parent() {
            Some(parent_path) => Some(parent_path.to_path_buf().join("config.json")),
            None => {
                println!("Could not open the executable's parent directory, using default configuration.");
                None
            }
        }
    } else {
        println!("Could not get the executable's directory, using default configuration.");
        None
    };
    let config = if config_path.is_some() {
        if let Ok(mut f) = File::open(config_path.unwrap()) {
            let mut data = String::new();
            if f.read_to_string(&mut data).is_ok() {
                match serde_json::from_str(&data) {
                    Ok(j) => j,
                    Err(_) => {
                        println!("Could not read your config.json into json, using default configuration");
                        Configuration::default()
                    }
                }
            } else {
                println!(
                    "Couldn't read your config.json into a string, using default configuration"
                );
                Configuration::default()
            }
        } else {
            // file doesn't exist. That's fine, just use default config
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
                        mode: bevy::window::WindowMode::BorderlessFullscreen,
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
        // insert the common sphere that all bodies use
        .insert_resource(SphereInfo::default())
        // start the spawn selection at default
        .insert_resource(BodySpawningOptions::default())
        // add configuration resource for use by systems
        .insert_resource(config)
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
        .add_systems(Update, spawn_mode_selection)
        .add_systems(Update, spawn_scrolling)
        .add_systems(Update, spawn)
        // camera
        .add_systems(Update, rotate_camera)
        .add_systems(Update, move_camera)
        // UI
        .add_systems(Update, update_osd)
        .add_systems(Update, show_hide_help)
        // general
        .add_systems(Update, capture_or_release_cursor)
        .add_systems(Update, exit_system)
        .add_systems(Update, scale_ui)
        .run();
}
