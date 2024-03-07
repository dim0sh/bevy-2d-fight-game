use bevy::{prelude::*, sprite::MaterialMesh2dBundle};
use crate::player::{AttackCooldown, AttackHeight, Direction, Player};
use crate::movement::{PlayerInputEvent, PlayerInput};
use bevy_rapier2d::prelude::*;
#[derive(Component)]
pub struct Attack;
#[derive(Component)]
pub struct AttackProperties {
    pub damage: f32,
    pub range: f32,
}

#[derive(Bundle)]
pub struct AttackBundle {
    model: MaterialMesh2dBundle<ColorMaterial>,
    timer: AttackDespawnTimer,
    collider: Collider,
    sensor: Sensor,
    attack_properties: AttackProperties,
}

#[derive(Component)]
pub struct AttackDespawnTimer {
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
    time: Res<Time>,
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>, 
    mut materials: ResMut<Assets<ColorMaterial>>,
    mut query: Query<(&mut AttackCooldown,&AttackHeight, &Direction, &Transform),With<Player>>,
    mut ev_input: EventReader<PlayerInputEvent>,
) {
    
    for (mut attack_cooldown,attack_height, player_direction, player_transform) in query.iter_mut() {
        attack_cooldown.0.tick(time.delta());
        for input in ev_input.read() {
            if !(input.0.contains(&PlayerInput::Attack)) {return}
            if !attack_cooldown.0.finished() {return}
            let width = 11.0;
            let height = 11.0;
            let mut x_attack_direction = 0.0;
            let mut y_attack_direction = 0.0;
            match player_direction {
                Direction::Left => {
                    x_attack_direction -= 20.0;
                }
                Direction::Right => {
                    x_attack_direction += 20.0;
                }
            }
            match attack_height {
                AttackHeight::Low => {
                    y_attack_direction -= 10.0;
                }
                AttackHeight::Normal => {
                    y_attack_direction += 0.0;
                }
            }
            attack_cooldown.0.reset();

            let offset_vec = Vec2::new(704.0, 530.0);

            commands.spawn((
                AttackBundle {
                    model: MaterialMesh2dBundle {
                        mesh: meshes.add(Mesh::from(shape::Quad::new(Vec2::new(width, height)))).into(),
                        material: materials.add(ColorMaterial::from(Color::rgb(1.0, 0.0, 0.0))).into(),
                        transform: Transform::from_translation(Vec3::new(
                            (player_transform.translation.x + x_attack_direction) - offset_vec.x,
                            (player_transform.translation.y + y_attack_direction) - offset_vec.y,
                            0.0,
                        )) ,
                        ..Default::default()
                    },
                    timer: AttackDespawnTimer {
                        timer: Timer::from_seconds(0.2, TimerMode::Once)
                    },
                    collider: Collider::cuboid(width-30.0, height-20.0),
                    sensor: Sensor,
                    attack_properties: AttackProperties {
                        damage: 10.0,
                        range: 60.0,
                    },
                },
                Attack,
            ));
        }
    }
}
    

fn despawn_attack(
    time: Res<Time>,
    mut commands: Commands,
    mut query: Query<(Entity, &mut AttackDespawnTimer), With<Attack>>,
) {
    for (entity, mut attack_timer) in query.iter_mut() {
        attack_timer.timer.tick(time.delta());
        if attack_timer.timer.finished() {
            commands.entity(entity).despawn();
        }
    }
}