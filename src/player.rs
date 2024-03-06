use bevy::{prelude::*, sprite::MaterialMesh2dBundle};
use bevy_rapier2d::prelude::*;
use crate::movement;
#[derive(Component,Copy,Clone,Debug)]
pub enum Direction {
    Left,
    Right,
}
#[derive(Component,Copy,Clone,Debug)]
pub enum AttackHeight {
    Low,
    Normal,
    High,
}

#[derive(Component)]
pub struct Player;

#[derive(Bundle)]
pub struct PlayerBundle {
    model: MaterialMesh2dBundle<ColorMaterial>,
    controller: KinematicCharacterController,
    collider: Collider,
    direction: Direction,
    attack_height: AttackHeight,
    velocity: movement::Velocity,
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
            collider: Collider::cuboid(40.0-20.0, 100.0-50.0),
            direction: Direction::Left,
            velocity: movement::Velocity { velocity: Vec2::new(0.0, 0.0), max_speed: 100.0},
            attack_height: AttackHeight::Normal,
        },
        Player,
    ));
}

fn move_player(
    time: Res<Time>,
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut query: Query<(&mut AttackHeight,&mut movement::Velocity,&mut Direction, &mut KinematicCharacterController),With<Player>>
) {
    for (mut attack_height,mut velocity, mut direction, mut controller) in query.iter_mut() {
        if keyboard_input.pressed(KeyCode::KeyA) {
            velocity.velocity.x = (-velocity.max_speed).max(velocity.velocity.x - 10.0);
            *direction = Direction::Left;
        } else if keyboard_input.pressed(KeyCode::KeyD) {
            velocity.velocity.x = velocity.max_speed.min(velocity.velocity.x + 10.0);
            *direction = Direction::Right;
        } else if velocity.velocity.x > 0.0 {
            velocity.velocity.x = (velocity.velocity.x - 10.0).max(0.0);
        } else if velocity.velocity.x < 0.0 {
            velocity.velocity.x = (velocity.velocity.x + 10.0).min(0.0);
        }
        	
        controller.translation = Some(velocity.velocity * time.delta_seconds());
        if keyboard_input.pressed(KeyCode::KeyS) {
            *attack_height = AttackHeight::Low;
        } else if keyboard_input.pressed(KeyCode::KeyW) {
            *attack_height = AttackHeight::High; 
        } else {
            *attack_height = AttackHeight::Normal;
        }
    }
}