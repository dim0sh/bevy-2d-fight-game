use crate::movement::{PlayerInput, PlayerInputEvent, Velocity};
use bevy::prelude::*;
use bevy_rapier2d::prelude::*;
#[derive(Component, Copy, Clone, Debug)]
pub enum Direction {
    Left,
    Right,
}
#[derive(Component, Copy, Clone, Debug)]
pub enum AttackHeight {
    Low,
    Normal,
}

#[derive(Component, Clone, Debug)]
pub struct AttackCooldown(pub Timer);

#[derive(Component, Clone)]
pub struct Player;

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app //.add_systems(Startup, spawn_player)
            .add_systems(PreUpdate, input_player)
            .add_systems(Update, (player_jump, collision_vel_reset, gravity))
            .add_systems(PostUpdate, apply_velocity);
    }
}

fn input_player(
    mut query: Query<(&mut AttackHeight, &mut Velocity, &mut Direction), With<Player>>,
    mut ev_input: EventReader<PlayerInputEvent>,
) {
    for input in ev_input.read() {
        for (mut attack_height, mut velocity, mut direction) in query.iter_mut() {
            if input.0.contains(&PlayerInput::Left) {
                velocity.velocity.x = (-velocity.max_speed).max(velocity.velocity.x - 10.0);
                *direction = Direction::Left;
            } else if input.0.contains(&PlayerInput::Right) {
                velocity.velocity.x = velocity.max_speed.min(velocity.velocity.x + 10.0);
                *direction = Direction::Right;
            } else if velocity.velocity.x > 0.0 {
                velocity.velocity.x = (velocity.velocity.x - 10.0).max(0.0);
            } else if velocity.velocity.x < 0.0 {
                velocity.velocity.x = (velocity.velocity.x + 10.0).min(0.0);
            }

            // controller.translation = Some(velocity.velocity * time.delta_seconds());
            if input.0.contains(&PlayerInput::Down) {
                *attack_height = AttackHeight::Low;
            } else {
                *attack_height = AttackHeight::Normal;
            }
        }
    }
}

fn player_jump(
    mut query: Query<(&mut Velocity, &KinematicCharacterControllerOutput), With<Player>>,
    mut ev_input: EventReader<PlayerInputEvent>,
) {
    for input in ev_input.read() {
        for (mut velocity, controller) in query.iter_mut() {
            if input.0.contains(&PlayerInput::Up) && controller.grounded {
                velocity.velocity.y = 200.0;
            }
        }
    }
}

pub fn gravity(
    mut query: Query<(&mut Velocity, &KinematicCharacterControllerOutput), With<Player>>,
    time: Res<Time>,
) {
    let delta_y = -400.0 * time.delta_seconds();
    for (mut velocity, character_controller) in query.iter_mut() {
        if !character_controller.grounded {
            velocity.velocity.y += delta_y;
        }
    }
}

fn collision_vel_reset(
    mut query: Query<(&mut Velocity, &KinematicCharacterControllerOutput), With<Player>>,
) {
    for (mut velocity, character_controller) in query.iter_mut() {
        for contact in character_controller.collisions.iter() {
            match contact.toi.details {
                Some(c) => {
                    if c.normal1.y < -0.5 {
                        velocity.velocity.y = 0.0;
                    }
                    if c.normal1.y > 0.5 && velocity.velocity.y < 0.0 {
                        velocity.velocity.y = 0.0;
                    }
                    if c.normal1.x.abs() > 0.5 {
                        velocity.velocity.x = 0.0;
                    }
                }

                _ => {}
            }
        }
    }
}

fn apply_velocity(
    time: Res<Time>,
    mut query: Query<(&Velocity, &mut KinematicCharacterController), With<Player>>,
) {
    for (velocity, mut controller) in query.iter_mut() {
        controller.translation = Some(velocity.velocity * time.delta_seconds());
    }
}
