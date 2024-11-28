use bevy::{
    core::FrameCount,
    input::mouse::{MouseMotion, MouseWheel},
    math::NormedVectorSpace,
    prelude::*,
    render::{
        render_asset::RenderAssetUsages,
        render_resource::{Extent3d, TextureDimension, TextureFormat},
    },
    window::{Cursor, CursorGrabMode, PrimaryWindow},
};
pub struct GravPlugin;
const G: f32 = 8.;
const MOUSE_SENSITIVITY: f32 = 0.002; // ?
const CAMERA_SPEED: f32 = 5.; // m/s
const SPAWN_SIZE_MOUSEWHEEL_SENSITIVITY: f32 = 1.;
const SPAWN_SPEED_MOUSEWHEEL_SENSITIVITY: f32 = 1.;

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
        .add_systems(Update, pause_time)
        .add_systems(Startup, (create_sphere_info, initial_spawn).chain())
        .add_systems(Startup, camera_spawn)
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
        .add_systems(Update, reset_sim)
        .run();
}

fn reset_sim(
    keys: Res<ButtonInput<KeyCode>>,
    query: Query<Entity, With<Body>>,
    mut commands: Commands,
    sphere_info : Res<SphereInfo>
) {
    if keys.just_pressed(KeyCode::KeyR) {
        let mut entity_iter = query.iter();
        while let Some(entity) = entity_iter.next() {
            commands.entity(entity).despawn();
        }
        initial_spawn(commands, sphere_info);
    }
}

fn capture_or_release_cursor(
    mut window: Query<&mut Window, With<PrimaryWindow>>,
    frames: Res<FrameCount>,
    mut paused: ResMut<TimePaused>,
) {
    // https://github.com/bevyengine/bevy/issues/16238
    // wait for a bit before capturing the cursor
    if frames.0 >= 6 {
        let mut primary_window = window.single_mut();
        if primary_window.focused {
            primary_window.cursor.grab_mode = CursorGrabMode::Locked;
            primary_window.cursor.visible = false;
        } else {
            primary_window.cursor.grab_mode = CursorGrabMode::None;
            primary_window.cursor.visible = true;
            paused.0 = true;
        }
    }
}

// from examples/camera/first_person_view_model.rs
fn rotate_camera(
    mut mouse_motion: EventReader<MouseMotion>,
    mut camera: Query<&mut Transform, With<Camera>>,
) {
    let mut transform = camera.single_mut();
    for motion in mouse_motion.read() {
        let yaw = -motion.delta.x * MOUSE_SENSITIVITY;
        let pitch = -motion.delta.y * MOUSE_SENSITIVITY;
        transform.rotate_y(yaw);
        transform.rotate_local_x(pitch);
    }
}

#[derive(Clone, Copy, PartialEq)]
enum SpawnSelectionMode {
    NONE,
    SIZE,
    SPEED,
    FIRE,
}

impl Default for SpawnSelectionMode {
    fn default() -> Self {
        Self::NONE
    }
}

#[derive(Resource, Clone, Copy)]
struct BodySpawningOptions {
    mode: SpawnSelectionMode,
    size: f32,
    speed: f32,
}

// override default values for size and speed (f32 default is 0)
impl Default for BodySpawningOptions {
    fn default() -> Self {
        Self {
            mode: Default::default(),
            size: 1.,
            speed: 1.,
        }
    }
}

fn mouse_button_input(
    buttons: Res<ButtonInput<MouseButton>>,
    camera: Query<&Transform, With<Camera>>,
    mut spawn_options: ResMut<BodySpawningOptions>,
    mut evr_scroll: EventReader<MouseWheel>,
    mut commands: Commands,
    sphere_info : Res<SphereInfo>,
    keys: Res<ButtonInput<KeyCode>>,
) {
    for ev in evr_scroll.read() {
        match spawn_options.mode {
            SpawnSelectionMode::NONE => continue,
            SpawnSelectionMode::SIZE => {
                spawn_options.size += ev.y * SPAWN_SIZE_MOUSEWHEEL_SENSITIVITY
            }
            SpawnSelectionMode::SPEED => {
                spawn_options.speed += ev.y * SPAWN_SPEED_MOUSEWHEEL_SENSITIVITY
            }
            SpawnSelectionMode::FIRE => continue,
        }
    }
    spawn_options.size = spawn_options.size.clamp(0., 5.);
    spawn_options.speed = spawn_options.speed.clamp(0., 5.);
    if buttons.just_pressed(MouseButton::Left) {
        spawn_options.mode = SpawnSelectionMode::SPEED;
    }
    if buttons.just_pressed(MouseButton::Right) {
        spawn_options.mode = SpawnSelectionMode::SIZE;
    }
    if buttons.just_pressed(MouseButton::Middle) || keys.just_pressed(KeyCode::KeyF) {
        spawn_options.mode = SpawnSelectionMode::FIRE;
    }

    //TODO(henrygerardmoore): display current options on screen

    // check if we need to spawn
    if spawn_options.mode == SpawnSelectionMode::FIRE {
        spawn_options.mode = SpawnSelectionMode::NONE;
        let tf = camera.single();
        if spawn_options.size <= 0. {
            return;
        }
        commands.spawn(body_bundle(
            spawn_options.size,
            Vec3 {
                x: tf.translation.x,
                y: tf.translation.y,
                z: tf.translation.z,
            },
            tf.forward() * spawn_options.speed,
            &sphere_info
        ));
    }
}

fn move_camera(
    keys: Res<ButtonInput<KeyCode>>,
    mut camera: Query<&mut Transform, With<Camera>>,
    time: Res<Time>,
) {
    let motion_distance = time.delta_seconds() * CAMERA_SPEED;
    let mut transform = camera.single_mut();
    let mut net_translation = Vec3::ZERO;
    if keys.pressed(KeyCode::KeyW) {
        net_translation += *transform.forward();
    }
    if keys.pressed(KeyCode::KeyS) {
        net_translation += *transform.back();
    }
    if keys.pressed(KeyCode::KeyA) {
        net_translation += *transform.left();
    }
    if keys.pressed(KeyCode::KeyD) {
        net_translation += *transform.right();
    }
    if keys.pressed(KeyCode::Space) {
        net_translation += *transform.up();
    }
    if keys.pressed(KeyCode::ControlLeft) || keys.pressed(KeyCode::ControlRight) {
        net_translation += *transform.down();
    }
    transform.translation += net_translation.normalize_or_zero() * motion_distance;
}

fn pause_time(keys: Res<ButtonInput<KeyCode>>, mut paused: ResMut<TimePaused>) {
    if keys.just_pressed(KeyCode::KeyP) {
        paused.0 = !paused.0;
    }
}

fn exit_system(keys: Res<ButtonInput<KeyCode>>, mut exit: EventWriter<AppExit>) {
    if keys.just_pressed(KeyCode::Escape) {
        exit.send(AppExit::Success);
    }
}

// from 3d_shapes.rs bevy example
fn uv_debug_texture() -> Image {
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

#[derive(Resource, Clone, Copy)]
struct TimePaused(bool);

#[derive(Component, Clone, Copy)]
struct Body {
    mass: f32,
}

#[derive(Component, Clone, Copy)]
struct Position(Vec3);

#[derive(Component, Clone, Copy)]
struct Velocity(Vec3);

#[derive(Resource, Clone, Default)]
struct SphereInfo(Handle<Mesh>, Handle<StandardMaterial>);

fn create_sphere_info(
    mut sphere_info : ResMut<SphereInfo>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut images: ResMut<Assets<Image>>,
    mut materials: ResMut<Assets<StandardMaterial>>,) {
    let mesh_handle = meshes.add(Sphere { radius: 1. }.mesh().uv(32, 18));
    let material_handle = materials.add(StandardMaterial {
        base_color_texture: Some(images.add(uv_debug_texture())),
        ..default()
    });
    sphere_info.0 = mesh_handle;
    sphere_info.1 = material_handle;
}

// TODO(henrygerardmoore): fix debug texture and extract mesh addition to other function
fn body_bundle(
    mass: f32,
    position: Vec3,
    velocity: Vec3,
    sphere_info : &Res<SphereInfo>,
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

fn initial_spawn(
    mut commands: Commands, sphere_info : Res<SphereInfo>,
) {
    commands.spawn(body_bundle(
        1.,
        Vec3 {
            x: 0.,
            y: 0.,
            z: 2.,
        },
        Vec3 {
            x: 0.,
            y: 1.,
            z: 0.,
        },
        &sphere_info
    ));
    commands.spawn(body_bundle(
        1.,
        Vec3 {
            x: 0.,
            y: 0.,
            z: -2.,
        },
        Vec3 {
            x: 0.,
            y: -1.,
            z: 0.,
        },
        &sphere_info
    ));
}

fn camera_spawn(mut commands: Commands) {
    // camera
    commands.spawn(Camera3dBundle {
        transform: Transform::from_xyz(-2.5, 4.5, 9.0).looking_at(Vec3::ZERO, Vec3::Y),
        ..default()
    });
}

fn get_radius(body: Body) -> f32 {
    body.mass.cbrt()
}

// sum gravitational forces on bodies to arrive at their acceleration, euler integrate acceleration to modify velocity
fn update_body_velocities(
    mut query: Query<(&Body, &Position, &mut Velocity)>,
    time: Res<Time>,
    paused: Res<TimePaused>,
) {
    let dt = if paused.0 { 0. } else { time.delta_seconds() };
    let mut query_next = query.iter_combinations_mut();
    while let Some([(body1, &p1, mut v1), (body2, &p2, mut v2)]) = query_next.fetch_next() {
        let m1 = body1.mass;
        let m2 = body2.mass;
        let r2 = p2.0 - p1.0;
        let r1 = -r2;
        let dist = r1.norm();
        let a1 = G * m2 * r2 / dist.powf(3.);
        let a2 = G * m1 * r1 / dist.powf(3.);
        v1.0 += a1 * dt;
        v2.0 += a2 * dt;
    }
}

// euler integrate body velocities to update body positions
fn update_body_positions(
    mut query: Query<(&mut Position, &Velocity)>,
    time: Res<Time>,
    paused: Res<TimePaused>,
) {
    let dt = if paused.0 { 0. } else { time.delta_seconds() };
    query.iter_mut().for_each(|(mut position, velocity)| {
        position.0.x += velocity.0.x * dt;
        position.0.y += velocity.0.y * dt;
        position.0.z += velocity.0.z * dt;
    });
}

// combine colliding bodies into one
fn resolve_body_collisions(
    mut query: Query<(Entity, &mut Body, &mut Position, &mut Velocity)>,
    mut commands: Commands,
) {
    let mut query_next = query.iter_combinations_mut();
    while let Some([(_entity1, mut body1, mut p1, mut v1), (entity2, mut body2, p2, v2)]) =
        query_next.fetch_next()
    {
        let dist_collision = get_radius(*body1) + get_radius(*body2);
        let dist_actual = (p2.0 - p1.0).norm();

        // are the bodies colliding?
        if dist_actual <= dist_collision {
            // calculate quantities we'll use
            let m1 = body1.mass;
            let m2 = body2.mass;
            // if either entity's mass is 0, skip (this collision doesn't matter)
            if m1 == 0. || m2 == 0. {
                continue;
            }

            let net_mass = m1 + m2;

            // set entity1's position to the center of mass of the two
            p1.0 = (m1 * p1.0 + m2 * p2.0) / net_mass;

            // add entity2's momentum to entity1
            v1.0 = (m1 * v1.0 + m2 * v2.0) / net_mass;
            body1.mass = net_mass;

            // enqueue the removal of entity2 and set its mass to 0 (so it won't collide with anything else)
            commands.entity(entity2).despawn();
            println!("Deleting entity {}", entity2);
            body2.mass = 0.;
        }
    }
}

fn update_body_meshes(mut query: Query<(&mut Transform, &Position, &Body)>) {
    for (mut transform, position, body) in &mut query {
        transform.translation = position.0;
        transform.scale = Vec3::ONE * get_radius(*body);
    }
}
