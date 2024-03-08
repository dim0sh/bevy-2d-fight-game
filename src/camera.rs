use crate::player::Player;
use bevy::prelude::*;

pub struct CameraPlugin;

impl Plugin for CameraPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_camera)
            .add_systems(PostUpdate, camera_movement);
    }
}

fn spawn_camera(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());
}

fn camera_movement(
    mut query: Query<&mut Transform, With<Camera>>,
    player_query: Query<&Transform, (With<Player>, Without<Camera>)>,
) {
    let offset_vec = Vec2::new(704.0, 530.0);
    for mut transform in query.iter_mut() {
        for player_transform in player_query.iter() {
            transform.translation.x = player_transform.translation.x - offset_vec.x;
            // transform.translation.y = player_transform.translation.y;
        }
    }
}
