use bevy::{asset::transformer, prelude::*, sprite::MaterialMesh2dBundle};
use crate::player::{AttackHeight, Direction, Player};
use bevy_rapier2d::prelude::*;
#[derive(Component)]
pub struct Attack {
    damage: f32,
    range: f32,
}

#[derive(Bundle)]
pub struct AttackBundle {
    model: MaterialMesh2dBundle<ColorMaterial>,
    timer: AttackTimer,
    collider: Collider,
    sensor: Sensor,
}

#[derive(Component)]
pub struct AttackTimer {
    pub timer: Timer,
}

pub struct AttackPlugin;

impl Plugin for AttackPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, spawn_attack)
            .add_systems(PostUpdate, despawn_attack);
    }
}

fn spawn_attack(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>, 
    mut materials: ResMut<Assets<ColorMaterial>>,
    query: Query<(&AttackHeight, &Direction, &Transform),With<Player>>,
    keyboard_input: Res<ButtonInput<KeyCode>>,
) {
    if !keyboard_input.just_pressed(KeyCode::Space) {
        return;
    }

    let width = 60.0;
    let height = 40.0;

    for (attack_height, player_direction, player_transform) in query.iter() {
        let mut x_attack_direction = 0.0;
        let mut y_attack_direction = 0.0;
        match player_direction {
            Direction::Left => {
                x_attack_direction -= 60.0;
            }
            Direction::Right => {
                x_attack_direction += 60.0;
            }
        }
        match attack_height {
            AttackHeight::Low => {
                y_attack_direction -= 30.0;
            }
            AttackHeight::Normal => {
                y_attack_direction += 0.0;
            }
            AttackHeight::High => {
                y_attack_direction += 30.0;
            }
        }


        commands.spawn((
            AttackBundle {
                model: MaterialMesh2dBundle {
                    mesh: meshes.add(Rectangle::new(width, height)).into(),
                    material: materials.add(Color::rgb(1.0, 0.0, 0.0)).into(),
                    transform: Transform::from_translation(Vec3::new(
                        player_transform.translation.x + x_attack_direction,
                        player_transform.translation.y + y_attack_direction,
                        0.0,
                    )) ,
                    ..Default::default()
                },
                timer: AttackTimer {
                    timer: Timer::from_seconds(0.2, TimerMode::Once)
                },
                collider: Collider::cuboid(width-30.0, height-20.0),
                sensor: Sensor,
            },
            Attack {
                damage: 10.0,
                range: 100.0,
            },
        ));
    }
    
}

fn despawn_attack(
    time: Res<Time>,
    mut commands: Commands,
    mut query: Query<(Entity, &mut AttackTimer), With<Attack>>,
) {
    for (entity, mut attack_timer) in query.iter_mut() {
        attack_timer.timer.tick(time.delta());
        if attack_timer.timer.finished() {
            commands.entity(entity).despawn();
        }
    }
}