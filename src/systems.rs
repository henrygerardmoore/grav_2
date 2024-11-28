use bevy::{
    core::FrameCount,
    input::mouse::{MouseMotion, MouseWheel},
    math::NormedVectorSpace,
    prelude::*,
    render::{
        render_asset::RenderAssetUsages,
        render_resource::{Extent3d, TextureDimension, TextureFormat},
    },
    window::PrimaryWindow,
};

use crate::components::{Body, HelpText, HelpUI, Position, SpawnText, SpawnUI, Velocity};
use crate::helpers::{get_mass, get_radius};
use crate::resources::{BodySpawningOptions, SpawnSelectionMode, SphereInfo, TimePaused, TimeRate};

// TODO(henrygerardmoore): add loadable config file that controls below consts (as well as full screen/resolution, etc.) through a `Resource`
const G: f32 = 8.;
const MOUSE_SENSITIVITY: f32 = 0.002; // ?
const CAMERA_SPEED: f32 = 5.; // m/s
const SPAWN_SIZE_MOUSEWHEEL_SENSITIVITY: f32 = 0.05;
const SPAWN_SPEED_MOUSEWHEEL_SENSITIVITY: f32 = 0.05;
const SPAWN_SPEED_MAX: f32 = 20.;
const SPAWN_SIZE_MAX: f32 = 5.;
const TIME_RATE_SENSITIVITY: f32 = 0.1;
const SPEED_MOD_FACTOR: f32 = 5.;

pub fn text_section(color: Srgba, value: &str) -> TextSection {
    TextSection::new(
        value,
        TextStyle {
            font_size: 40.0,
            color: color.into(),
            ..default()
        },
    )
}

pub fn spawn_help(mut commands: Commands) {
    commands
        .spawn((
            HelpUI,
            NodeBundle {
                background_color: Color::BLACK.into(),
                visibility: Visibility::Visible, // start visible so user sees the help screen
                z_index: ZIndex::Global(i32::MAX),
                style: Style {
                    position_type: PositionType::Absolute,
                    right: Val::Px(10.),
                    padding: UiRect::all(Val::Px(5.0)),
                    ..default()
                },
                ..default()
            },
        ))
        .with_children(|c| {
            c.spawn((
                TextBundle::from_sections([
                    TextSection::new(
                        "Controls",
                        TextStyle {
                            font_size: 72.0,
                            color: Color::WHITE.into(),
                            ..default()
                        },
                    ),
                    TextSection::new(
                        "\n\nGeneral",
                        TextStyle {
                            font_size: 60.,
                            color: Color::WHITE.into(),
                            ..default()
                        },
                    ),
                    text_section(Color::WHITE.into(), "\nH to show or hide this help display"),
                    text_section(Color::WHITE.into(), "\nR to reset the simulation"),
                    text_section(Color::WHITE.into(), "\nEsc to quit"),
                    text_section(
                        Color::WHITE.into(),
                        "\nShift to increase speed (of any other control)",
                    ),
                    text_section(
                        Color::WHITE.into(),
                        "\nAlt to decrease speed (of any other control)",
                    ),
                    TextSection::new(
                        "\n\nMovement",
                        TextStyle {
                            font_size: 60.,
                            color: Color::WHITE.into(),
                            ..default()
                        },
                    ),
                    text_section(Color::WHITE.into(), "\nWASD to move laterally"),
                    text_section(Color::WHITE.into(), "\nMouse to look"),
                    TextSection::new(
                        "\n\nSpawning",
                        TextStyle {
                            font_size: 60.,
                            color: Color::WHITE.into(),
                            ..default()
                        },
                    ),
                    text_section(
                        Color::WHITE.into(),
                        "\nF or click middle mouse to spawn a body",
                    ),
                    text_section(
                        Color::WHITE.into(),
                        "\nScroll mouse wheel to modify selected spawn option",
                    ),
                    text_section(Color::WHITE.into(), "\nLeft click to select spawn speed"),
                    text_section(Color::WHITE.into(), "\nRight click to select spawn size"),
                    TextSection::new(
                        "\n\nTime",
                        TextStyle {
                            font_size: 60.,
                            color: Color::WHITE.into(),
                            ..default()
                        },
                    ),
                    text_section(Color::WHITE.into(), "\nP to pause time"),
                    text_section(
                        Color::WHITE.into(),
                        "\nEquals key to increase simulation rate",
                    ),
                    text_section(
                        Color::WHITE.into(),
                        "\nHyphen key to decrease simulation rate",
                    ),
                ])
                .with_text_justify(JustifyText::Center),
                HelpText,
            ));
        });
}

pub fn show_hide_help(
    mut query: Query<&mut Visibility, With<HelpUI>>,
    keys: Res<ButtonInput<KeyCode>>,
) {
    if keys.just_pressed(KeyCode::KeyH) {
        // show help
        *query.single_mut() = match *query.single_mut() {
            Visibility::Hidden => Visibility::Visible,
            _ => Visibility::Hidden,
        };
    }
}

// see bevymark.rs
pub fn create_osd(mut commands: Commands) {
    commands
        .spawn((
            NodeBundle {
                style: Style {
                    position_type: PositionType::Absolute,
                    padding: UiRect::all(Val::Px(5.0)),
                    ..default()
                },
                z_index: ZIndex::Global(i32::MAX - 1),
                background_color: Color::WHITE.with_alpha(0.5).into(),
                ..default()
            },
            SpawnUI,
        ))
        .with_children(|c| {
            c.spawn((
                TextBundle::from_sections([
                    text_section(Color::BLACK.into(), "Body spawn speed: "),
                    text_section(Color::BLACK.into(), ""),
                    text_section(Color::BLACK.into(), "\nBody spawn size: "),
                    text_section(Color::BLACK.into(), ""),
                    text_section(Color::BLACK.into(), "\nTime speed: "),
                    text_section(Color::BLACK.into(), ""),
                ]),
                SpawnText,
            ));
        });
}

pub fn update_osd(
    mut query: Query<&mut Text, With<SpawnText>>,
    spawn_options: Res<BodySpawningOptions>,
    time_rate: Res<TimeRate>,
    time_paused: Res<TimePaused>,
) {
    let mut text = query.single_mut();
    text.sections[1].value = format!("{0:.2}", spawn_options.speed);
    text.sections[3].value = format!("{0:.2}", spawn_options.radius);
    match spawn_options.mode {
        SpawnSelectionMode::SIZE => {
            text.sections[3].style.color = Color::srgb(1., 0., 0.);
            text.sections[1].style.color = Color::BLACK;
        }
        SpawnSelectionMode::SPEED => {
            text.sections[1].style.color = Color::srgb(1., 0., 0.);
            text.sections[3].style.color = Color::BLACK;
        }
        _ => {
            text.sections[1].style.color = Color::BLACK;
            text.sections[3].style.color = Color::BLACK;
        }
    };
    if time_paused.0 {
        text.sections[5].value = "paused".into();
    } else {
        text.sections[5].value = format!("{0:.2}x", time_rate.0);
    }
}

pub fn reset_bodies(
    keys: Res<ButtonInput<KeyCode>>,
    query: Query<Entity, With<Body>>,
    mut commands: Commands,
    sphere_info: Res<SphereInfo>,
) {
    if keys.just_pressed(KeyCode::KeyR) {
        let mut entity_iter = query.iter();
        while let Some(entity) = entity_iter.next() {
            commands.entity(entity).despawn();
        }
        initial_spawn(commands, sphere_info);
    }
}

pub fn reset_camera(
    keys: Res<ButtonInput<KeyCode>>,
    query: Query<Entity, With<Camera>>,
    mut commands: Commands,
) {
    if keys.just_pressed(KeyCode::KeyR) {
        let mut entity_iter = query.iter();
        while let Some(entity) = entity_iter.next() {
            commands.entity(entity).despawn();
        }
        camera_spawn(commands);
    }
}

// TODO(henrygerardmoore): fix on windows
pub fn capture_or_release_cursor(
    mut window: Query<&mut Window, With<PrimaryWindow>>,
    frames: Res<FrameCount>,
    mut paused: ResMut<TimePaused>,
) {
    // https://github.com/bevyengine/bevy/issues/16238
    // wait for a bit before capturing the cursor
    if frames.0 >= 6 {
        let mut primary_window = window.single_mut();
        if primary_window.focused {
            primary_window.cursor.visible = false;
        } else {
            primary_window.cursor.visible = true;
            paused.0 = true;
        }
    }
}

// from examples/camera/first_person_view_model.rs
pub fn rotate_camera(
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

pub fn mouse_button_input(
    buttons: Res<ButtonInput<MouseButton>>,
    camera: Query<&Transform, With<Camera>>,
    mut spawn_options: ResMut<BodySpawningOptions>,
    mut evr_scroll: EventReader<MouseWheel>,
    mut commands: Commands,
    sphere_info: Res<SphereInfo>,
    keys: Res<ButtonInput<KeyCode>>,
) {
    // shift lets you control more coarsely
    let mut sens_mod = if keys.pressed(KeyCode::ShiftLeft) {
        SPEED_MOD_FACTOR
    } else {
        1.
    };
    // alt lets you control more finely
    if keys.pressed(KeyCode::AltLeft) {
        sens_mod /= SPEED_MOD_FACTOR;
    }
    for ev in evr_scroll.read() {
        match spawn_options.mode {
            SpawnSelectionMode::NONE => continue,
            SpawnSelectionMode::SIZE => {
                spawn_options.radius += ev.y * SPAWN_SIZE_MOUSEWHEEL_SENSITIVITY * sens_mod
            }
            SpawnSelectionMode::SPEED => {
                spawn_options.speed += ev.y * SPAWN_SPEED_MOUSEWHEEL_SENSITIVITY * sens_mod
            }
            SpawnSelectionMode::FIRE => continue,
        }
    }
    spawn_options.radius = spawn_options.radius.clamp(
        SPAWN_SIZE_MOUSEWHEEL_SENSITIVITY / SPEED_MOD_FACTOR,
        SPAWN_SIZE_MAX,
    );
    spawn_options.speed = spawn_options.speed.clamp(0., SPAWN_SPEED_MAX);
    if buttons.just_pressed(MouseButton::Left) {
        spawn_options.mode = SpawnSelectionMode::SPEED;
    }
    if buttons.just_pressed(MouseButton::Right) {
        spawn_options.mode = SpawnSelectionMode::SIZE;
    }
    if buttons.just_pressed(MouseButton::Middle) || keys.just_pressed(KeyCode::KeyF) {
        spawn_options.mode = SpawnSelectionMode::FIRE;
    }

    // check if we need to spawn
    if spawn_options.mode == SpawnSelectionMode::FIRE {
        spawn_options.mode = SpawnSelectionMode::NONE;
        let tf = camera.single();
        if spawn_options.radius <= 0. {
            return;
        }
        let mass = get_mass(spawn_options.radius);
        commands.spawn(body_bundle(
            mass,
            Vec3 {
                x: tf.translation.x,
                y: tf.translation.y,
                z: tf.translation.z,
            } + Vec3::from(tf.forward()) * (spawn_options.radius + 1.01), // move it in front of the camera
            tf.forward() * spawn_options.speed,
            &sphere_info,
        ));
    }
}

pub fn move_camera(
    keys: Res<ButtonInput<KeyCode>>,
    mut camera: Query<&mut Transform, With<Camera>>,
    time: Res<Time>,
) {
    // move faster when shift is held
    let mut speed_mod = if keys.pressed(KeyCode::ShiftLeft) {
        SPEED_MOD_FACTOR
    } else {
        1.
    };
    if keys.pressed(KeyCode::AltLeft) {
        speed_mod /= SPEED_MOD_FACTOR;
    }
    let motion_distance = time.delta_seconds() * CAMERA_SPEED * speed_mod;
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

pub fn modify_time(
    keys: Res<ButtonInput<KeyCode>>,
    mut paused: ResMut<TimePaused>,
    mut rate: ResMut<TimeRate>,
) {
    // shift lets you control more coarsely
    let mut sens_mod = if keys.pressed(KeyCode::ShiftLeft) {
        SPEED_MOD_FACTOR
    } else {
        1.
    };

    // alt lets you control more finely
    if keys.pressed(KeyCode::AltLeft) {
        sens_mod /= SPEED_MOD_FACTOR;
    }
    if keys.just_pressed(KeyCode::KeyP) {
        paused.0 = !paused.0;
    }
    if keys.just_pressed(KeyCode::Equal) {
        rate.0 += TIME_RATE_SENSITIVITY * sens_mod;
    }
    if keys.just_pressed(KeyCode::Minus) {
        rate.0 -= TIME_RATE_SENSITIVITY * sens_mod;
    }
    rate.0 = rate.0.clamp(TIME_RATE_SENSITIVITY / SPEED_MOD_FACTOR, 10.);
}

pub fn exit_system(keys: Res<ButtonInput<KeyCode>>, mut exit: EventWriter<AppExit>) {
    if keys.just_pressed(KeyCode::Escape) {
        exit.send(AppExit::Success);
    }
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

pub fn create_sphere_info(
    mut sphere_info: ResMut<SphereInfo>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut images: ResMut<Assets<Image>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    let mesh_handle = meshes.add(Sphere::default().mesh().uv(32, 18));
    let material_handle = materials.add(StandardMaterial {
        base_color_texture: Some(images.add(uv_debug_texture())),
        ..default()
    });
    sphere_info.0 = mesh_handle;
    sphere_info.1 = material_handle;
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

pub fn initial_spawn(mut commands: Commands, sphere_info: Res<SphereInfo>) {
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
        &sphere_info,
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
        &sphere_info,
    ));
}

pub fn camera_spawn(mut commands: Commands) {
    // camera
    commands.spawn(Camera3dBundle {
        transform: Transform::from_xyz(-2.5, 4.5, 9.0).looking_at(Vec3::ZERO, Vec3::Y),
        ..default()
    });
}

// sum gravitational forces on bodies to arrive at their acceleration, euler integrate acceleration to modify velocity
pub fn update_body_velocities(
    mut query: Query<(&Body, &Position, &mut Velocity)>,
    time: Res<Time>,
    paused: Res<TimePaused>,
    time_rate: Res<TimeRate>,
) {
    let dt = if paused.0 {
        0.
    } else {
        time.delta_seconds() * time_rate.0
    };
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
pub fn update_body_positions(
    mut query: Query<(&mut Position, &Velocity)>,
    time: Res<Time>,
    paused: Res<TimePaused>,
    time_rate: Res<TimeRate>,
) {
    let dt = if paused.0 {
        0.
    } else {
        time.delta_seconds() * time_rate.0
    };
    query.iter_mut().for_each(|(mut position, velocity)| {
        position.0.x += velocity.0.x * dt;
        position.0.y += velocity.0.y * dt;
        position.0.z += velocity.0.z * dt;
    });
}

// combine colliding bodies into one
pub fn resolve_body_collisions(
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
            body2.mass = 0.;
        }
    }
}

pub fn update_body_meshes(mut query: Query<(&mut Transform, &Position, &Body)>) {
    for (mut transform, position, body) in &mut query {
        transform.translation = position.0;
        transform.scale = Vec3::ONE * get_radius(*body);
    }
}