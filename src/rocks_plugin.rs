use bevy_prototype_lyon::{prelude::*, entity::ShapeBundle};
use bevy::prelude::*;

use std::{f32::{INFINITY, NEG_INFINITY, consts::PI}, ops::Range};
use rand::prelude::{thread_rng, Rng};

const ROCK_FILL_COLOR: Color = Color::rgba(0.0, 0.5, 0.4, 0.2);
const ROCK_OUTLINE_COLOR: Color = Color::rgb(0.0, 0.5, 0.4);
const ROCK_OUTLINE_WIDTH: f32 = 2.0;

pub struct RocksPlugin;

impl Plugin for RocksPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_startup_system(test_rocks)
        ;
    }
}

#[derive(Component)]
pub struct PolygonPoints(pub Vec<Vec2>);

#[derive(Component, Debug)]
pub struct PolygonBoundingBox(pub f32, pub f32, pub f32, pub f32);

#[derive(Bundle)]
pub struct RockBundle {
    #[bundle]
    shape: ShapeBundle,
    polygon: PolygonPoints,
    bounding_box: PolygonBoundingBox
}

impl RockBundle {
    pub fn new(mut points: Vec<Vec2>) -> Self {
        let mut bbox = PolygonBoundingBox(
            INFINITY,     // left
            NEG_INFINITY, // top
            NEG_INFINITY, // right
            INFINITY      // bottom
        );

        let mut path_builder = PathBuilder::new();
        path_builder.move_to(points[0]);
        points.push(points[0]);

        for i in 1..points.len() {
            bbox.0 = bbox.0.min(points[i].x);
            bbox.1 = bbox.1.max(points[i].y);
            bbox.2 = bbox.2.max(points[i].x);
            bbox.3 = bbox.3.min(points[i].y);

            path_builder.line_to(points[i]);
        }

        path_builder.line_to(points[0]);

        let shape = path_builder.build();

        Self {
            shape: GeometryBuilder::build_as(
                &shape, 
                DrawMode::Outlined { 
                    fill_mode: FillMode::color(ROCK_FILL_COLOR), 
                    outline_mode: StrokeMode::new(ROCK_OUTLINE_COLOR, ROCK_OUTLINE_WIDTH) 
                }, 
                Transform::from_xyz(0.0, 0.0, 8.0)
            ),
            polygon: PolygonPoints(points),
            bounding_box: bbox
        }
    }

    pub fn rand(
        sides: usize,
        size: f32,
        center_range_x: Range<f32>, 
        center_range_y: Range<f32>, 
        corner_deviation_range: Range<f32>
    ) -> Self {
        let mut rng = thread_rng();
        let angle_offset = rng.gen_range(0.0..(2.0 * PI));
        let center = Vec2::new(
            rng.gen_range(center_range_x.clone()),
            rng.gen_range(center_range_y.clone())
        );

        let points = (0..sides).map(|i| {
            let a = 2.0 * PI * (i as f32) / (sides as f32) + angle_offset;
            let deviation_angle = rng.gen_range(0.0..(2.0 * PI));
            let deviation_radius = rng.gen_range(corner_deviation_range.clone());

            center + Vec2::new(
                size * a.cos(), 
                size * a.sin()
            ) + Vec2::new(
                deviation_radius * deviation_angle.cos(),
                deviation_radius * deviation_angle.sin()
            )
        }).collect();

        Self::new(points)
    }
}

fn test_rocks(
    mut commands: Commands
) {
    for i in 0..100 {
        let x = 200.0 * (i / 2) as f32;
        let y = 500.0 * (i % 2) as f32 - 250.0;

        commands.spawn(RockBundle::rand(
            4,
            100.0,
            x..(x + 0.01),
            y..(y + 0.01),
            0.0..40.0
        ));
    }
}

#[allow(dead_code)]
pub fn is_intersecting(polygon_points: &Vec<Vec2>, player_pos: &Vec2) -> bool {
    if player_pos.x == 0.0 && player_pos.y == 0.0 { return false };

    let mut num_oultine_intersections = 0;
    let slope = player_pos.y / (player_pos.x + 0.0001);

    for i in 0..polygon_points.len() {
        let p = [polygon_points[i], polygon_points[(i + 1) % polygon_points.len()]];

        // If both points are on the same side of the line formed by the player and (0, 0)
        if (p[0].x * slope > p[0].y) == (p[1].x * slope > p[1].y) { continue; }

        let poly_slope = (p[1].y - p[0].y) / (p[1].x - p[0].x + 0.0001);
        let poly_y_int = p[0].y - poly_slope * p[0].x;
        let intersection_x = poly_y_int / (slope - poly_slope);

        // Or if the intersection point of the lines is outside of the segment
        if 
            (intersection_x < 0.0) != (player_pos.x < 0.0) ||
            player_pos.x.abs() < intersection_x.abs()
        { continue; }

        num_oultine_intersections += 1;
    }

    num_oultine_intersections % 2 == 1
}