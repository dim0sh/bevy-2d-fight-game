use bevy::prelude::*;
use std::collections::HashSet;
#[derive(Component, Copy, Clone, Debug)]
pub struct Velocity {
    pub velocity: Vec2,
    pub max_speed: f32,
}
impl Default for Velocity {
    fn default() -> Self {
        Self {
            velocity: Vec2::new(0.0, 0.0),
            max_speed: 100.0,
        }
    }
}
#[derive(Copy, Clone, Debug, PartialEq, Hash, Eq)]
pub enum PlayerInput {
    Left,
    Right,
    Up,
    Down,
    Attack,
    ResetLevel,
}
#[derive(Event, Clone, Debug, PartialEq)]
pub struct PlayerInputEvent(pub HashSet<PlayerInput>);

pub struct MovementPlugin;

impl Plugin for MovementPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(PreUpdate, handle_keyboard_input);
    }
}

fn handle_keyboard_input(
    keyboard_input: Res<Input<KeyCode>>,
    mut ev_input: EventWriter<PlayerInputEvent>,
) {
    let mut input = HashSet::new();
    if keyboard_input.pressed(KeyCode::A) {
        input.insert(PlayerInput::Left);
    }
    if keyboard_input.pressed(KeyCode::D) {
        input.insert(PlayerInput::Right);
    }
    if keyboard_input.pressed(KeyCode::W) {
        input.insert(PlayerInput::Up);
    }
    if keyboard_input.pressed(KeyCode::S) {
        input.insert(PlayerInput::Down);
    }
    if keyboard_input.pressed(KeyCode::Space) {
        input.insert(PlayerInput::Attack);
    }
    if keyboard_input.pressed(KeyCode::R) {
        input.insert(PlayerInput::ResetLevel);
    }

    ev_input.send(PlayerInputEvent(input));
}
