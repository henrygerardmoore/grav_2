use crate::helpers;
use bevy::prelude::*;

#[derive(Clone, Copy, PartialEq)]
pub enum SpawnSelectionMode {
    None,
    Size,
    Speed,
    Fire,
}

impl Default for SpawnSelectionMode {
    fn default() -> Self {
        Self::None
    }
}

#[derive(Resource, Clone, Copy)]
pub struct BodySpawningOptions {
    pub mode: SpawnSelectionMode,
    pub radius: f32,
    pub speed: f32,
}

// override default values for size and speed (f32 default is 0)
impl Default for BodySpawningOptions {
    fn default() -> Self {
        Self {
            mode: Default::default(),
            radius: helpers::get_default_sphere_radius(),
            speed: 1.,
        }
    }
}

#[derive(Resource, Clone, Copy)]
pub struct TimePaused(pub bool);

#[derive(Resource, Clone, Default)]
pub struct SphereInfo(pub Handle<Mesh>, pub Handle<StandardMaterial>);

#[derive(Resource, Clone, Copy)]
pub struct TimeRate(pub f32);
