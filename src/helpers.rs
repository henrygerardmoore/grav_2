use crate::components::Body;
use bevy::prelude::*;

pub fn get_radius(body: Body) -> f32 {
    body.mass.cbrt() * get_default_sphere_radius()
}

pub fn get_mass(radius: f32) -> f32 {
    (radius / get_default_sphere_radius()).powf(3.)
}

pub fn get_default_sphere_radius() -> f32 {
    Sphere::default().radius
}
