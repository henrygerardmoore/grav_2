use crate::components::{Body, Position, Velocity};
use crate::resources::SphereInfo;
use bevy::{
    prelude::*,
    render::{
        render_asset::RenderAssetUsages,
        render_resource::{Extent3d, TextureDimension, TextureFormat},
    },
};

pub fn get_radius(body: Body) -> f32 {
    body.mass.cbrt() * get_default_sphere_radius()
}

pub fn get_mass(radius: f32) -> f32 {
    (radius / get_default_sphere_radius()).powf(3.)
}

pub fn get_default_sphere_radius() -> f32 {
    Sphere::default().radius
}

pub fn body_bundle(
    mass: f32,
    position: Vec3,
    velocity: Vec3,
    sphere_info: &Res<SphereInfo>,
) -> impl Bundle {
    // get or add the mesh handle
    let mesh_handle = sphere_info.0.clone();

    // get or add the material handle
    let material_handle = sphere_info.1.clone();
    (
        Body { mass },
        Position(position),
        Velocity(velocity),
        PbrBundle {
            mesh: mesh_handle,
            material: material_handle,
            ..default()
        },
    )
}

// from 3d_shapes.rs bevy example
pub fn uv_debug_texture() -> Image {
    const TEXTURE_SIZE: usize = 8;

    let mut palette: [u8; 32] = [
        255, 102, 159, 255, 255, 159, 102, 255, 236, 255, 102, 255, 121, 255, 102, 255, 102, 255,
        198, 255, 102, 198, 255, 255, 121, 102, 255, 255, 236, 102, 255, 255,
    ];

    let mut texture_data = [0; TEXTURE_SIZE * TEXTURE_SIZE * 4];
    for y in 0..TEXTURE_SIZE {
        let offset = TEXTURE_SIZE * y * 4;
        texture_data[offset..(offset + TEXTURE_SIZE * 4)].copy_from_slice(&palette);
        palette.rotate_right(4);
    }

    Image::new_fill(
        Extent3d {
            width: TEXTURE_SIZE as u32,
            height: TEXTURE_SIZE as u32,
            depth_or_array_layers: 1,
        },
        TextureDimension::D2,
        &texture_data,
        TextureFormat::Rgba8UnormSrgb,
        RenderAssetUsages::RENDER_WORLD,
    )
}
