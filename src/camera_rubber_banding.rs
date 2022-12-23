use super::*;

const TIGHTNESS: f32 = 0.1;
const MAX_DISTANCE: f32 = 100000.0;

pub struct CameraRubberBandingPlugin;

impl Plugin for CameraRubberBandingPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_system_set(
                SystemSet::on_update(AppState::InGame)
                .with_system(rubber_band_camera)
            )
        ;
    }
}

fn rubber_band_camera(
    player_transform_query: Query<&Transform, (With<Player>, Without<Camera>)>,
    mut camera_transform_query: Query<&mut Transform, With<Camera>>
) {
    let camera_pos = &mut camera_transform_query.single_mut().translation;
    let player_pos = &player_transform_query.single().translation;
    let dist = camera_pos.distance(*player_pos);

    if dist < MAX_DISTANCE {
        camera_pos.x = camera_pos.x + TIGHTNESS * (player_pos.x - camera_pos.x);
        camera_pos.y = camera_pos.y + TIGHTNESS * (player_pos.y - camera_pos.y);
    } else {
        camera_pos.x = player_pos.x + (MAX_DISTANCE / dist) * (camera_pos.x - player_pos.x);
        camera_pos.y = player_pos.y + (MAX_DISTANCE / dist) * (camera_pos.y - player_pos.y);
    }
}