use super::*;

const HANDLE_LINE_WIDTH: f32 = 1.5;
const HANDLE_OUTER_CIRCLE_RADIUS: f32 = 10.0;
const HANDLE_INNER_CIRCLE_RADIUS: f32 = 7.0;

const CURSOR_HANDLE_COLOR: Color = Color::rgb(0.5, 0.3, 0.3);
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

#[derive(Component, Clone, Copy)]
pub struct CursorHandle;

#[derive(Component, Clone, Copy)]
pub struct GhostHandle;

pub fn make_handle(
    commands: &mut Commands,
    meshes: &mut ResMut<Assets<Mesh>>,
    materials: &mut ResMut<Assets<ColorMaterial>>,
    color: Color,
    base: Vec2,
    head: Vec2,
    z: f32,
    component: impl Component + Copy
) {
    // line
    commands.spawn((GeometryBuilder::build_as(
        &get_line_path(base, head), 
        DrawMode::Stroke(StrokeMode::new(color, HANDLE_LINE_WIDTH)), 
        Transform::from_xyz(0.0, 0.0, z)
    ), component));

    // outer circle
    commands.spawn((MaterialMesh2dBundle {
        mesh: meshes.add(shape::Circle::default().into()).into(),
        material: materials.add(ColorMaterial::from(color)),
        transform: Transform
            ::from_xyz(head.x, head.y, z)
            .with_scale(Vec3::new(HANDLE_OUTER_CIRCLE_RADIUS, HANDLE_OUTER_CIRCLE_RADIUS, HANDLE_OUTER_CIRCLE_RADIUS)),
        ..default()
    }, component));

    // inner circle
    commands.spawn((MaterialMesh2dBundle {
        mesh: meshes.add(shape::Circle::default().into()).into(),
        material: materials.add(ColorMaterial::from(BACKGROUND_COLOR)),
        transform: Transform
            ::from_xyz(head.x, head.y, z + 0.01)
            .with_scale(Vec3::new(HANDLE_INNER_CIRCLE_RADIUS, HANDLE_INNER_CIRCLE_RADIUS, HANDLE_INNER_CIRCLE_RADIUS)),
        ..default()
    }, component));
}

fn setup_handles(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>
) {
    // cursor handle
    make_handle(
        &mut commands,
        &mut meshes, 
        &mut materials, 
        CURSOR_HANDLE_COLOR, 
        Vec2::ZERO, 
        Vec2::ZERO, 
        2.0, 
        CursorHandle
    );

    // Ghost handles
    make_handle(
        &mut commands,
        &mut meshes, 
        &mut materials, 
        GHOST_HANDLE_COLOR, 
        Vec2::ZERO, 
        Vec2::ZERO, 
        1.9, 
        GhostHandle
    );
}   

fn place_handle_at_cursor(
    mut handle_path_query: Query<&mut Path, (With<CursorHandle>, Without<GhostHandle>)>,
    mut handle_transform_query: Query<&mut Transform, (With<CursorHandle>, Without<GhostHandle>, Without<Path>)>,
    mut ghost_handle_path_query: Query<&mut Path, With<GhostHandle>>,
    mut ghost_handle_transform_query: Query<&mut Transform, (With<GhostHandle>, Without<Path>)>,
    next_point_query: Res<NextPointPos>,
    mut control_points_query: ResMut<ControlPoints>,
    cursor_pos: Res<CursorPos>,
    keys: Res<Input<KeyCode>>
) {
    let next_point = Vec2::new(next_point_query.0.x as f32, next_point_query.0.y as f32);
    let cursor_x = cursor_pos.0.x;
    let cursor_y = cursor_pos.0.y;

    control_points_query.0 = Vector3::new(
        cursor_x as f64,
        cursor_y as f64,
        0.0
    );

    *handle_path_query.single_mut() = get_line_path(
        next_point, 
        Vec2::new(cursor_x, cursor_y)
    );

    for mut handle_transform in handle_transform_query.iter_mut() {
        handle_transform.translation.x = cursor_x;
        handle_transform.translation.y = cursor_y;
    }

    let ghost_x = if keys.pressed(KeyCode::Q) {
        cursor_x
    } else {
        2.0 * next_point.x - cursor_x
    };

    let ghost_y = if keys.pressed(KeyCode::W) {
        cursor_y
    } else {
        2.0 * next_point.y - cursor_y
    };

    control_points_query.1 = Vector3::new(
        ghost_x as f64,
        ghost_y as f64,
        0.0
    );

    *ghost_handle_path_query.single_mut() = get_line_path(
        next_point, 
        Vec2::new(ghost_x, ghost_y)
    );

    for mut ghost_handle_transform in ghost_handle_transform_query.iter_mut() {
        ghost_handle_transform.translation.x = ghost_x;
        ghost_handle_transform.translation.y = ghost_y;
    }
}

fn set_curve(
    control_points_query: Res<ControlPoints>,
    mut current_curve: ResMut<CurrentCurve>
) {
    current_curve.0.as_mut().unwrap().set_control_point(2, control_points_query.0);
}