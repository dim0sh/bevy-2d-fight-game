use bevy::prelude::*;

#[derive(Component,Copy,Clone,Debug)]
pub struct Velocity {
    pub velocity: Vec2,
    pub max_speed: f32,
}