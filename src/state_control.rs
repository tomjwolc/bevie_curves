use super::*;

const POST_GAME_SCREEN_COLOR: Color = Color::rgba(0.6, 0.6, 0.6, 0.8);

pub struct StateControlPlugin;

impl Plugin for StateControlPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_system_set(
                SystemSet::on_enter(AppState::PreGame)
                .with_system(reset_pregame)
            ).add_system_set(
                SystemSet::on_update(AppState::PreGame)
                .with_system(check_for_click)
            ).add_system_set(
                SystemSet::on_update(AppState::InGame)
                .with_system(check_out_of_bounds)
            ).add_system_set(
                SystemSet::on_enter(AppState::PostGame)
                .with_system(post_game_screen)
            ).add_system_set(
                SystemSet::on_update(AppState::PostGame)
                .with_system(check_for_restart)
            )
        ;
    }
}

#[derive(Component)]
struct EndScreenStuff;

#[allow(clippy::too_many_arguments)]
fn reset_pregame(
    mut commands: Commands,
    mut all_objects_transform_query: Query<&mut Transform, Without<CurvePath>>,
    curve_path_entity_query: Query<Entity, With<CurvePath>>,
    end_screen_entities_query: Query<Entity, With<EndScreenStuff>>,
    mut all_paths_query: Query<&mut Path, (Without<Transform>, Without<CurvePath>)>,
    mut next_point_pos: ResMut<NextPointPos>,
    mut current_curve: ResMut<CurrentCurve>,
    mut t: ResMut<T>,
    mut control_points: ResMut<ControlPoints>,
    mut start_game_time: ResMut<StartGameTime>,
    time: Res<Time>
) {
    if let Ok(curve_path_entity) = curve_path_entity_query.get_single() {
        commands.entity(curve_path_entity).despawn();
    }

    for end_screen_entity in end_screen_entities_query.iter() {
        commands.entity(end_screen_entity).despawn();
    }

    for mut object_transform in all_objects_transform_query.iter_mut() {
        object_transform.translation.x = 0.0;
        object_transform.translation.y = 0.0;
    }

    for mut path in all_paths_query.iter_mut() {
        *path = get_line_path(Vec2::ZERO, Vec2::ZERO);
    }

    t.0 = 0.0;
    next_point_pos.0 = Vector3::zeros();
    control_points.0 = Vector3::zeros();
    control_points.1 = Vector3::zeros();
    current_curve.0 = None;
    start_game_time.0 = time.elapsed_seconds();
}

fn check_for_click(
    buttons: Res<Input<MouseButton>>,
    mut app_state: ResMut<State<AppState>>
) {
    if buttons.just_pressed(MouseButton::Left) {
        app_state.set(AppState::InGame).unwrap();
    }
}

fn check_out_of_bounds(
    windows: Res<Windows>,
    player_transform_query: Query<&Transform, With<Player>>,
    mut app_state: ResMut<State<AppState>>
) {
    let window = windows.get_primary().unwrap();
    let player_pos = player_transform_query.single().translation;

    if 
        player_pos.x > window.width()  /  2.0 ||
        player_pos.x < window.width()  / -2.0 ||
        player_pos.y > window.height() /  2.0 ||
        player_pos.y < window.height() / -2.0
    {
        app_state.set(AppState::PostGame).unwrap();
    }
}

fn post_game_screen(
    mut commands: Commands,
    windows: Res<Windows>,
    asset_server: Res<AssetServer>,
    camera_transform_query: Query<&Transform, With<Camera>>
) {
    let window = windows.get_primary().unwrap();
    let camera_pos = camera_transform_query.single().translation;

    commands.spawn((SpriteBundle {
        sprite: Sprite {
            color: POST_GAME_SCREEN_COLOR,
            custom_size: Some(Vec2::new(window.width(), window.height())),
            ..default()
        },
        transform: Transform
            ::from_xyz(camera_pos.x, camera_pos.y, 5.0),
        ..default()
    }, EndScreenStuff));

    let text_style = TextStyle { 
        font: asset_server.load("fonts/HankenGrotesk-VariableFont_wght.ttf"), 
        font_size: 100.0, 
        color: Color::BLACK 
    };

    commands.spawn((TextBundle::from_section(
        "// Game Over", 
        text_style
    )
        .with_text_alignment(TextAlignment::TOP_CENTER)
        .with_style(Style {
            position_type: PositionType::Absolute,
            position: UiRect {
                bottom: Val::Px(20.0),
                left: Val::Px(40.0),
                ..default()
            },
            justify_content: JustifyContent::Center,
            ..default()
        }), EndScreenStuff
    ));
}

fn check_for_restart(
    mut buttons: ResMut<Input<MouseButton>>,
    mut app_state: ResMut<State<AppState>>
) {
    if buttons.just_pressed(MouseButton::Left) {
        buttons.reset(MouseButton::Left);
        app_state.set(AppState::PreGame).unwrap();
    }
}