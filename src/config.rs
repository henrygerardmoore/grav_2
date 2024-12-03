#[derive(Default, Clone)]
struct Configuration {
    gravity_constant: f32,
    mouse_sensitivity: f32,
    camera_speed: f32,
}

impl Default for Configuration {
    fn default() -> Self {
        Self {
            gravity_constant: Default::default(),
            mouse_sensitivity: Default::default(),
        }
    }
}
const G: f32 = 8.;
const MOUSE_SENSITIVITY: f32 = 0.002; // ?
const CAMERA_SPEED: f32 = 5.; // m/s
const SPAWN_SIZE_MOUSEWHEEL_SENSITIVITY: f32 = 0.05;
const SPAWN_SPEED_MOUSEWHEEL_SENSITIVITY: f32 = 0.05;
const SPAWN_SPEED_MAX: f32 = 20.;
const SPAWN_SIZE_MAX: f32 = 5.;
const TIME_RATE_SENSITIVITY: f32 = 0.1;
const SPEED_MOD_FACTOR: f32 = 5.;
