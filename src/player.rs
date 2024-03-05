use bevy::{prelude::*, sprite::MaterialMesh2dBundle};
use bevy_rapier2d::prelude::*;

#[derive(Copy,Clone,Debug)]
pub enum Direction {
    Left,
    Right,
}

#[derive(Component)]
pub struct Player {
    pub direction: Direction,
    pub low: bool,
    pub speed: f32,
}

#[derive(Bundle)]
pub struct PlayerBundle {
    model: MaterialMesh2dBundle<ColorMaterial>,
    controller: KinematicCharacterController,
    collider: Collider,
}   

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_player)
            .add_systems(Update, move_player);
    }
}

fn spawn_player(
    mut commands: Commands, 
    mut meshes: ResMut<Assets<Mesh>>, 
    mut materials: ResMut<Assets<ColorMaterial>>
) {
    commands.spawn((
        PlayerBundle {
            model: MaterialMesh2dBundle {
                mesh: meshes.add(Rectangle::new( 40.0, 100.0 )).into(),
                material: materials.add(Color::rgb(0.5, 0.5, 1.0)).into(),
                ..Default::default()
            },
            controller: KinematicCharacterController::default(),
            collider: Collider::cuboid(40.0-20.0, 100.0-50.0)
            
        },
        Player {
                direction: Direction::Left,
                low: false,
                speed: 100.0,
        },
    ));
}

fn move_player(
    time: Res<Time>,
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut query: Query<(&mut Player, &mut KinematicCharacterController)>
) {
    for (mut player, mut controller) in query.iter_mut() {
        let mut vec = Vec2::new(0.0, 0.0);
        if keyboard_input.pressed(KeyCode::KeyA) {
            vec.x -= time.delta_seconds() * player.speed;
            player.direction = Direction::Left;
        }
        if keyboard_input.pressed(KeyCode::KeyD) {
            vec.x += time.delta_seconds() * player.speed;
            player.direction = Direction::Right;
        }
        controller.translation = Some(vec);
        player.low = keyboard_input.pressed(KeyCode::KeyS) 
    }
}