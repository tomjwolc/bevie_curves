pub use bevy::{prelude::*, sprite::MaterialMesh2dBundle, ecs::system::EntityCommands};
pub use bevy_prototype_lyon::prelude::*;
pub use nalgebra::Vector3;

pub mod spline;
pub use spline::*;

mod handle_plugin;
use handle_plugin::*;

mod curve_movement;
use curve_movement::*;

mod state_control;
use state_control::*;

mod camera_rubber_banding;
use camera_rubber_banding::*;

mod lifetime_plugin;
use lifetime_plugin::*;

const BACKGROUND_COLOR: Color = Color::rgb(0.8, 0.8, 0.8);

pub const POINT_COLOR: Color = Color::rgb(0.5, 0.3, 0.3);
pub const POINT_RADIUS: f32 = 7.0;

const PLAYER_COLOR: Color = Color::RED;
const PLAYER_RADIUS: f32 = 10.0;

#[derive(Resource)]
struct T(f32);

#[derive(Resource)]
struct StartGameTime(f32);

#[derive(Resource)]
struct NextPointPos(Vector3<f64>);

#[derive(Resource)]
struct ControlPoints(Vector3<f64>, Vector3<f64>);

#[derive(Resource)]
struct CurrentCurve(Option<Curve>);

#[derive(Resource)]
struct CursorPos(Vec2);

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
enum AppState {
    PreGame,
    InGame,
    PostGame
}

fn main() {
    App::new()
        .insert_resource(T(0.0))
        .insert_resource(StartGameTime(0.0))
        .insert_resource(NextPointPos(Vector3::new(0.0, 0.0, 0.0)))
        .insert_resource(ControlPoints(Vector3::zeros(), Vector3::zeros()))
        .insert_resource(CurrentCurve(None))
        .insert_resource(CursorPos(Vec2::ZERO))
        
        .insert_resource(Msaa { samples: 4 })
        .insert_resource(ClearColor(BACKGROUND_COLOR))
        .add_state(AppState::PreGame)

        .add_plugins(DefaultPlugins.set(WindowPlugin {
            window: WindowDescriptor {
                title: "Bevy-ier curves".to_string(),
                ..Default::default()
            },
            ..Default::default()
        }))
        .add_plugin(ShapePlugin)

        .add_plugin(HandlePlugin)
        .add_plugin(CurveMovementPlugin)
        .add_plugin(StateControlPlugin)
        .add_plugin(CameraRubberBandingPlugin)
        .add_plugin(LifetimePlugin)
        
        .add_startup_system(setup)
        .add_system(bevy::window::close_on_esc)
        .add_system(set_cursor_pos)
        .run();
}

#[derive(Component)]
struct NextPoint;

#[derive(Component)]
struct Player;

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    mut windows: ResMut<Windows>
) {
    let window = windows.get_primary_mut().unwrap();

    window.set_cursor_visibility(false);

    commands.spawn(Camera2dBundle::default());

    // commands.spawn(SpriteBundle {
    //     sprite: Sprite {
    //         color: Color::rgba(1.0, 1.0, 1.0, 0.1),
    //         custom_size: Some(Vec2::new(window.width(), window.height())),
    //         ..default()
    //     },
    //     transform: Transform
    //         ::from_xyz(0.0, 0.0, 0.0),
    //     ..default()
    // });

    commands.spawn((MaterialMesh2dBundle {
        mesh: meshes.add(shape::Circle::default().into()).into(),
        material: materials.add(ColorMaterial::from(POINT_COLOR)),
        transform: Transform
            ::from_xyz(0.0, 0.0, 2.9)
            .with_scale(Vec3::new(POINT_RADIUS, POINT_RADIUS, POINT_RADIUS)),
        ..default()
    }, NextPoint));

    commands.spawn((MaterialMesh2dBundle {
        mesh: meshes.add(shape::Circle::default().into()).into(),
        material: materials.add(ColorMaterial::from(PLAYER_COLOR)),
        transform: Transform
            ::from_xyz(0.0, 0.0, 3.0)
            .with_scale(Vec3::new(PLAYER_RADIUS, PLAYER_RADIUS, PLAYER_RADIUS)),
        ..default()
    }, Player));
}

fn set_cursor_pos(
    mut cursor_pos: ResMut<CursorPos>,
    camera_transform_query: Query<&Transform, With<Camera>>,
    windows: Res<Windows>
) {
    let window = windows.get_primary().unwrap();
    let camera_pos = camera_transform_query.single().translation;

    if let Some(cursor_position) = window.cursor_position() {
        cursor_pos.0.x = cursor_position.x - window.width() / 2.0 + camera_pos.x;
        cursor_pos.0.y = cursor_position.y - window.height() / 2.0 + camera_pos.y;
    }
}