use bevy::{core_pipeline::core_2d::graph::input, prelude::*, sprite::MaterialMesh2dBundle};
use bevy_rapier2d::prelude::*;
use crate::movement::{Velocity, PlayerInputEvent, PlayerInput};
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
pub struct AttackCooldown(pub Timer);

#[derive(Component)]
pub struct Player;

#[derive(Bundle)]
pub struct PlayerBundle {
    model: MaterialMesh2dBundle<ColorMaterial>,
    controller: KinematicCharacterController,
    collider: Collider,
    direction: Direction,
    attack_height: AttackHeight,
    velocity: Velocity,
    attack_cooldown: AttackCooldown,
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
    let width = 40.0;
    let height = 100.0;
    commands.spawn((
        PlayerBundle {
            model: MaterialMesh2dBundle {
                mesh: meshes.add(Mesh::from(shape::Quad::new(Vec2::new(width, height)))).into(),
                material: materials.add(ColorMaterial::from(Color::rgb(0.5, 0.5, 1.0))).into(),
                ..Default::default()
            },
            controller: KinematicCharacterController::default(),
            collider: Collider::cuboid(40.0-20.0, 100.0-50.0),
            direction: Direction::Left,
            velocity: Velocity { velocity: Vec2::new(0.0, 0.0), max_speed: 100.0},
            attack_height: AttackHeight::Normal,
            attack_cooldown: AttackCooldown(Timer::from_seconds(0.5, TimerMode::Once)),
        },
        Player,
    ));
}

fn move_player(
    time: Res<Time>,
    mut query: Query<(&mut AttackHeight,&mut Velocity,&mut Direction, &mut KinematicCharacterController),With<Player>>,
    mut ev_input: EventReader<PlayerInputEvent>,
) {
    for input in ev_input.read() {
        for (mut attack_height,mut velocity, mut direction, mut controller) in query.iter_mut() {
            if input.0.contains(&PlayerInput::Left){
                velocity.velocity.x = (-velocity.max_speed).max(velocity.velocity.x - 10.0);
                *direction = Direction::Left;
            } else if input.0.contains(&PlayerInput::Right){
                velocity.velocity.x = velocity.max_speed.min(velocity.velocity.x + 10.0);
                *direction = Direction::Right;
            } else if velocity.velocity.x > 0.0 {
                velocity.velocity.x = (velocity.velocity.x - 10.0).max(0.0);
            } else if velocity.velocity.x < 0.0 {
                velocity.velocity.x = (velocity.velocity.x + 10.0).min(0.0);
            }
                
            controller.translation = Some(velocity.velocity * time.delta_seconds());
            if input.0.contains(&PlayerInput::Down){
                *attack_height = AttackHeight::Low;
            } else if input.0.contains(&PlayerInput::Up){
                *attack_height = AttackHeight::High; 
            } else {
                *attack_height = AttackHeight::Normal;
            }
        }
    }
    
}