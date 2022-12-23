use super::*;
use rand::prelude::*;
use std::f64::consts::PI;

const NEW_POINT_GEN_RADIUS: f64 = 300.0;
const ANGLE_SPREAD: f64 = PI / 5.0;
const DISTRIBUTION: f64 = 5.0;

const PATH_COLOR: Color = Color::RED;
const PATH_WIDTH: f32 = 1.5;

const T_INCREMENT: fn(f32) -> f32 = |time| 0.1 + 0.01 * time;

pub struct CurveMovementPlugin;

impl Plugin for CurveMovementPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_system_set(
                SystemSet::on_enter(AppState::InGame)
                .with_system(setup_curve)
            )
            .add_system_set(
                SystemSet::on_update(AppState::InGame)
                .with_system(increment_t)
                .with_system(update_movement)
            );
    }
}

#[derive(Component)]
pub struct CurvePath;

fn setup_curve(
    mut commands: Commands,
    mut current_curve: ResMut<CurrentCurve>,
    control_points: Res<ControlPoints>,
    next_point_pos: ResMut<NextPointPos>,
    cursor_pos: Res<CursorPos>,
    next_point_transform_query: Query<&mut Transform, With<NextPoint>>
) {
    reset_current_curve(&mut current_curve, control_points, next_point_pos, cursor_pos, next_point_transform_query);

    commands.spawn((GeometryBuilder::build_as(
        &current_curve.0.as_ref().unwrap().to_bezier_path(), 
        DrawMode::Stroke(StrokeMode::new(PATH_COLOR, PATH_WIDTH)), 
        Transform::from_xyz(0.0, 0.0, 2.8)
    ), CurvePath));
}

fn increment_t(
    time: Res<Time>,
    mut t: ResMut<T>,
    start_game_time: Res<StartGameTime>,
    mut current_curve: ResMut<CurrentCurve>,
    control_points: Res<ControlPoints>,
    next_point_pos: ResMut<NextPointPos>,
    cursor_pos: Res<CursorPos>,
    next_point_transform_query: Query<&mut Transform, With<NextPoint>>
) {
    t.0 += T_INCREMENT(time.elapsed_seconds() - start_game_time.0) / 60.0;

    if t.0 >= 1.0 {
        t.0 = 0.0;

        reset_current_curve(&mut current_curve, control_points, next_point_pos, cursor_pos, next_point_transform_query);
    }
}

fn update_movement(
    t: Res<T>,
    current_curve: Res<CurrentCurve>,
    mut player_transform_query: Query<&mut Transform, With<Player>>,
    mut curve_path_query: Query<&mut Path, With<CurvePath>>
) {
    let new_player_pos = current_curve.0.as_ref().unwrap().get_point(t.0 as f64);
    let player_pos = &mut player_transform_query.single_mut().translation;

    player_pos.x = new_player_pos.x as f32;
    player_pos.y = new_player_pos.y as f32;

    *curve_path_query.single_mut() = current_curve.0.as_ref().unwrap().to_bezier_path();
}

fn reset_current_curve(
    current_curve: &mut ResMut<CurrentCurve>,
    control_points: Res<ControlPoints>,
    mut next_point_pos: ResMut<NextPointPos>,
    cursor_pos: Res<CursorPos>,
    mut next_point_transform_query: Query<&mut Transform, With<NextPoint>>
) {
    let last_point = next_point_pos.0.clone();

    let mut facing_dir = ((last_point.y - control_points.0.y) / (last_point.x - control_points.0.x)).atan();

    facing_dir += if last_point.x < control_points.0.x { PI } else { 0.0 };

    let mut rng = thread_rng();
    let angle = rng.gen_range((facing_dir - ANGLE_SPREAD)..(facing_dir + ANGLE_SPREAD));
    let dist = rng.gen_range(0.0..NEW_POINT_GEN_RADIUS.powf(DISTRIBUTION)).powf(1.0 / DISTRIBUTION);
    
    next_point_pos.0 = next_point_pos.0 + Vector3::new(dist * angle.cos(), dist * angle.sin(), 0.0);

    let next_handle = 
        2.0 * next_point_pos.0 - Vector3::new(cursor_pos.0.x as f64, cursor_pos.0.y as f64, 0.0);

    current_curve.0 = Some(Curve::new_bezier(
        last_point, 
        control_points.1, 
        next_handle, 
        next_point_pos.0
    ));

    next_point_transform_query.single_mut().translation = Vec3::new(next_point_pos.0.x as f32, next_point_pos.0.y as f32, 0.0);
}