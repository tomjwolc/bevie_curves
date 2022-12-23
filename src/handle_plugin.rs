use super::*;

const HANDLE_COLOR: Color = Color::rgb(0.5, 0.3, 0.3);
const GHOST_HANDLE_COLOR: Color = Color::rgb(0.7, 0.7, 0.7);

pub struct HandlePlugin;

impl Plugin for HandlePlugin {
    fn build(&self, app: &mut App) {
        app
            .add_startup_system(setup_handles)
            .add_system_set(
                SystemSet::on_update(AppState::PreGame)
                .with_system(place_handle_at_cursor)
            )
            .add_system_set(
                SystemSet::on_update(AppState::InGame)
                .with_system(place_handle_at_cursor)
                .with_system(set_curve)
            )
        ;
    }
}

#[derive(Component)]
pub enum HandlePart {
    Circle(Transform),
    Line(Path)
}

#[derive(Component)]
pub struct CurveHandle;

#[derive(Component)]
pub struct GhostHandle;

fn setup_handles(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>
) {
    commands.spawn((HandleBundle::new(
        Vec2::ZERO,
        Vec2::ZERO,
        HANDLE_COLOR,
        2.0,
        &mut meshes,
        &mut materials
    ), CurveHandle));

    // Ghost handles
    commands.spawn((HandleBundle::new(
        Vec2::ZERO,
        Vec2::ZERO,
        GHOST_HANDLE_COLOR,
        1.9,
        &mut meshes,
        &mut materials
    ), GhostHandle));
}   

#[allow(clippy::complexity)]
fn place_handle_at_cursor(
    mut handle_query: Query<(&mut Path, &mut Transform), (With<CurveHandle>, Without<GhostHandle>)>,
    mut ghost_handle_path_query: Query<(&mut Path, &mut Transform), With<GhostHandle>>,
    next_point_query: Res<NextPointPos>,
    mut control_points_query: ResMut<ControlPoints>,
    cursor_pos: Res<CursorPos>,
    
) {
    let next_point = Vec2::new(next_point_query.0.x as f32, next_point_query.0.y as f32);
    let cursor_x = cursor_pos.0.x;
    let cursor_y = cursor_pos.0.y;

    control_points_query.0 = Vector3::new(
        cursor_x as f64,
        cursor_y as f64,
        0.0
    );

    // handle_query.single_mut().new_points(
    //     next_point, 
    //     Vec2::new(cursor_x, cursor_y)
    // );

    control_points_query.1 = Vector3::new(
        (2.0 * next_point.x - cursor_x) as f64,
        (2.0 * next_point.y - cursor_y) as f64,
        0.0
    );

    // handle_query.single_mut().new_points(
    //     next_point, 
    //     Vec2::new(2.0 * next_point.x - cursor_x, 2.0 * next_point.y - cursor_y)
    // );
}

fn set_curve(
    control_points_query: Res<ControlPoints>,
    mut current_curve: ResMut<CurrentCurve>
) {
    current_curve.0.as_mut().unwrap().set_control_point(2, control_points_query.0);
}