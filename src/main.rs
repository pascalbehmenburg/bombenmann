use bevy::prelude::*;
use bevy_rapier2d::prelude::*;
use bevy::window::{WindowMode, PresentMode};
use bevy::input::InputSystem;

fn main() {
    App::new()
        .add_plugins((DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                present_mode: PresentMode::Fifo,
                mode: WindowMode::Windowed,
                fit_canvas_to_parent: true,
                ..default()
            }),
            ..default()
        }).set(ImagePlugin::default_nearest())
        .build(), RapierPhysicsPlugin::<NoUserData>::default(), RapierDebugRenderPlugin{enabled: true, ..default()}, InputPlugin))
        .add_systems(Startup, startup)
        .add_systems(Update, movement)
        .run();
}

fn startup(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn(Camera2dBundle::default());
    commands.spawn((
            RigidBody::Dynamic,
            Velocity::zero(),
        SpriteBundle {
            texture: asset_server.load("player.png"),
            transform: Transform::from_xyz(100., 0., 0.),
            ..default()
        }
    ));
}

#[derive(Resource, Default)]
struct PlayerInput {
    pub move_direction: Vec2
}

pub struct InputPlugin;

impl Plugin for InputPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            PreUpdate,
            (
               movement_input 
            )
                .after(InputSystem),
        )
        .init_resource::<PlayerInput>();
    }
}

fn movement_input(
    keys: Res<Input<KeyCode>>,
    mut player_input: ResMut<PlayerInput>
) {
    let mut direction = Vec2::default();

    if keys.pressed(KeyCode::S) {
        direction += Vec2::new(0.0, -1.0);
    }
    if keys.pressed(KeyCode::W) {
        direction += Vec2::new(0.0, 1.0);
    }
    if keys.pressed(KeyCode::D) {
        direction += Vec2::new(1.0, 0.0);
    }
    if keys.pressed(KeyCode::A) {
        direction += Vec2::new(-1.0, 0.0);
    }

    player_input.move_direction = direction.normalize_or_zero();
}

fn movement(
    mut q_velocity: Query<&mut Velocity>,
    player_input: Res<PlayerInput>
    ) {
        for mut velocity in &mut q_velocity { 
            if player_input.move_direction == Vec2::default() {
                velocity.linvel = Vec2::ZERO;
                return;
            }

            velocity.linvel = player_input.move_direction * 200.0;
        }
}
