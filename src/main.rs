use bevy::{prelude::*,window::PresentMode};
use bevy_rapier2d::prelude::*;

mod camera;
mod player;
mod attack;
mod movement;

use camera::CameraPlugin;
use player::PlayerPlugin;
use attack::AttackPlugin;
use movement::MovementPlugin;
use movement::PlayerInputEvent;

fn main() {
    App::new()
        .add_event::<PlayerInputEvent>()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: String::from("Fight Game"),
                present_mode: PresentMode::Mailbox,
                ..default()
            }),
            ..default()
        }))
        .add_plugins(RapierPhysicsPlugin::<NoUserData>::pixels_per_meter(100.0))
        .add_plugins(RapierDebugRenderPlugin::default())
        .add_plugins(CameraPlugin)
        .add_plugins(MovementPlugin)
        .add_plugins(PlayerPlugin)
        .add_plugins(AttackPlugin)
        .run();
}
