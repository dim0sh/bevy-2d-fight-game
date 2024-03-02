use bevy::prelude::*;

mod camera;
mod player;
mod attack;

use camera::CameraPlugin;
use player::PlayerPlugin;
use attack::AttackPlugin;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(CameraPlugin)
        .add_plugins(PlayerPlugin)
        .add_plugins(AttackPlugin)
        .run();
}
