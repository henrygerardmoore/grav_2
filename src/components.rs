use bevy::prelude::*;

#[derive(Component, Clone, Copy)]
pub struct Body {
    pub mass: f32,
}

#[derive(Component, Clone, Copy)]
pub struct Position(pub Vec3);

#[derive(Component, Clone, Copy)]
pub struct Velocity(pub Vec3);

// marks spawn display
#[derive(Component, Clone, Copy)]
pub struct SpawnText;

// marks spawn UI
#[derive(Component, Clone, Copy)]
pub struct SpawnUI;

#[derive(Component, Clone, Copy)]
pub struct HelpText;

#[derive(Component, Clone, Copy)]
pub struct HelpUI;
