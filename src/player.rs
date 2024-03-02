use bevy::{prelude::*, sprite::MaterialMesh2dBundle};

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
        },
        Player {
                direction: Direction::Left,
                low: false,
                speed: 100.0,
        }   
    ));
}

fn move_player(
    time: Res<Time>,
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut query: Query<(&mut Player, &mut Transform)>
) {
    for (mut player, mut transform) in query.iter_mut() {
        if keyboard_input.pressed(KeyCode::KeyA) {
            transform.translation.x -= time.delta_seconds() * player.speed;
            player.direction = Direction::Left;
        }
        if keyboard_input.pressed(KeyCode::KeyD) {
            transform.translation.x += time.delta_seconds() * player.speed;
            player.direction = Direction::Right;
        }
        if keyboard_input.pressed(KeyCode::KeyS) {
            player.low = true;
        } else {
            player.low = false;
        }
        
    }
}