use bevy::prelude::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, startup)
        .add_systems(Update, (sprite_movement, camera_movement))
        .run();
}

#[derive(Component)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

fn startup(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn(Camera2dBundle::default());
    commands.spawn((
        SpriteBundle {
            texture: asset_server.load("player.png"),
            transform: Transform::from_xyz(100., 0., 0.),
            ..default()
        },
        Direction::Down,
    ));
}

fn sprite_movement(
    keyboard_input: Res<Input<KeyCode>>,
    mut sprite_position: Query<(&mut Direction, &mut Transform)>,
) {
    static SPEED: f32 = 4.;
    for (mut direction, mut transform) in &mut sprite_position {
        if keyboard_input.pressed(KeyCode::W) {
            *direction = Direction::Up;
            transform.translation.y += SPEED;
        }
        if keyboard_input.pressed(KeyCode::A) {
            *direction = Direction::Left;
            transform.translation.x -= SPEED;
        }
        if keyboard_input.pressed(KeyCode::S) {
            *direction = Direction::Down;
            transform.translation.y -= SPEED;
        }
        if keyboard_input.pressed(KeyCode::D) {
            *direction = Direction::Right;
            transform.translation.x += SPEED;
        }
    }
}
