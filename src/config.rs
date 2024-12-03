use bevy::prelude::Resource;
use serde::{Deserialize, Serialize};

#[derive(Clone, Resource, Serialize, Deserialize)]
pub struct Configuration {
    pub gravity_constant: f32,
    pub mouse_sensitivity: f32,
    pub camera_speed: f32,
    pub spawn_size_mousewheel_sensitivity: f32,
    pub spawn_speed_mousewheel_sensitivity: f32,
    pub spawn_speed_max: f32,
    pub spawn_size_max: f32,
    pub time_rate_sensitivity: f32,
    pub speed_mod_factor: f32,
}

impl Default for Configuration {
    fn default() -> Self {
        Self {
            gravity_constant: 8.,
            mouse_sensitivity: 0.002,
            camera_speed: 5.,
            spawn_size_mousewheel_sensitivity: 0.05,
            spawn_speed_mousewheel_sensitivity: 0.05,
            spawn_speed_max: 20.,
            spawn_size_max: 5.,
            time_rate_sensitivity: 0.1,
            speed_mod_factor: 5.,
        }
    }
}