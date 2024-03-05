use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

mod camera;
mod player;
mod attack;

use camera::CameraPlugin;
use player::PlayerPlugin;
use attack::AttackPlugin;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(RapierPhysicsPlugin::<NoUserData>::pixels_per_meter(100.0))
        .add_plugins(RapierDebugRenderPlugin::default())
        .add_plugins(CameraPlugin)
        .add_plugins(PlayerPlugin)
        .add_plugins(AttackPlugin)
        .run();
}
