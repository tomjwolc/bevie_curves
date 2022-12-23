use bevy_prototype_lyon::entity::ShapeBundle;

use super::*;

const HANDLE_LINE_WIDTH: f32 = 1.5;
const HANDLE_OUTER_CIRCLE_RADIUS: f32 = 10.0;
const HANDLE_INNER_CIRCLE_RADIUS: f32 = 7.0;

#[derive(Bundle)]
pub struct HandleBundle {
    #[bundle]
    line: ShapeBundle,

    #[bundle]
    outer_circle: MaterialMesh2dBundle<ColorMaterial>,

    #[bundle]
    inner_circle: MaterialMesh2dBundle<ColorMaterial>
}

impl HandleBundle {
    pub fn new(
        from: Vec2, 
        to: Vec2, 
        color: Color,
        z: f32,
        meshes: &mut ResMut<Assets<Mesh>>,
        materials: &mut ResMut<Assets<ColorMaterial>>
    ) -> Self {
        HandleBundle { 
            line: GeometryBuilder::build_as(
                &get_line_path(from, to), 
                DrawMode::Stroke(StrokeMode::new(color, HANDLE_LINE_WIDTH)), 
                Transform::from_xyz(0.0, 0.0, z - 0.01)
            ), 
            outer_circle: MaterialMesh2dBundle {
                mesh: meshes.add(shape::Circle::default().into()).into(),
                material: materials.add(ColorMaterial::from(color)),
                transform: Transform
                    ::from_xyz(to.x, to.y, z)
                    .with_scale(Vec3::new(HANDLE_OUTER_CIRCLE_RADIUS, HANDLE_OUTER_CIRCLE_RADIUS, HANDLE_OUTER_CIRCLE_RADIUS)),
                ..default()
            }, 
            inner_circle: MaterialMesh2dBundle {
                mesh: meshes.add(shape::Circle::default().into()).into(),
                material: materials.add(ColorMaterial::from(BACKGROUND_COLOR)),
                transform: Transform
                    ::from_xyz(to.x, to.y, z + 0.01)
                    .with_scale(Vec3::new(HANDLE_INNER_CIRCLE_RADIUS, HANDLE_INNER_CIRCLE_RADIUS, HANDLE_INNER_CIRCLE_RADIUS)),
                ..default()
            }
        }
    }

    pub fn new_points(&mut self, new_base: Vec2, new_head: Vec2) {
        self.line.path = get_line_path(new_base, new_head);

        let outer_pos = &mut self.outer_circle.transform.translation;
        let inner_pos = &mut self.inner_circle.transform.translation;

        outer_pos.x = new_head.x;
        outer_pos.y = new_head.y;
        inner_pos.x = new_head.x;
        inner_pos.y = new_head.y;
    }
}