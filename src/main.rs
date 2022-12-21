use bevy::{prelude::*, sprite::MaterialMesh2dBundle};
use bevy_prototype_debug_lines::*;
use lazy_static::lazy_static;
use nalgebra::Vector3;
use rand::prelude::*;
use std::ops::Range;

mod spline;
use spline::*;

lazy_static! {
    static ref TEST_POINTS: Vec<Vector3<f64>> = rand_points(13, -400.0..400.0, -400.0..400.0);
    static ref TEST_SPLINE: Spline = Spline::new_bezier(
        TEST_POINTS.to_vec()
    ).unwrap();
}

fn rand_points(num: usize, x_range: Range<f64>, y_range: Range<f64>) -> Vec<Vector3<f64>> {
    let mut points = Vec::new();
    let mut rng = thread_rng();

    for _ in 0..num {
        points.push(Vector3::new(rng.gen_range(x_range.clone()), rng.gen_range(y_range.clone()), 0.0));
    }

    points
}

fn main() {
    App::new()
        .insert_resource(ClearColor(Color::WHITE))
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            window: WindowDescriptor {
                title: "Bevy-ier curves".to_string(),
                ..Default::default()
            },
            ..Default::default()
        }))
        .add_plugin(DebugLinesPlugin::default())
        .add_system(bevy::window::close_on_esc)
        .add_system(setup)
        .run();
}

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    mut lines: ResMut<DebugLines>
) {
    commands.spawn(Camera2dBundle::default());

    let samples = 100;

    for i in 0..samples {
        let p1 = TEST_SPLINE.get_point((TEST_SPLINE.size() as f64) * (i as f64) / (samples as f64));
        let p2 = TEST_SPLINE.get_point((TEST_SPLINE.size() as f64) * ((i + 1) as f64) / (samples as f64));

        lines.line_colored(
            Vec3::new(p1.x as f32, p1.y as f32, 0.0),
            Vec3::new(p2.x as f32, p2.y as f32, 0.0),
            1.0,
            Color::BLACK
        );
    }

    for point in TEST_POINTS.iter() {
        commands.spawn(MaterialMesh2dBundle {
            mesh: meshes.add(shape::Circle::default().into()).into(),
            material: materials.add(ColorMaterial::from(Color::BLACK)),
            transform: Transform
                ::from_translation(Vec3::new(point.x as f32, point.y as f32, 0.5))
                .with_scale(Vec3::new(10.0, 10.0, 0.0)),
            ..default()
        });

        commands.spawn(MaterialMesh2dBundle {
            mesh: meshes.add(shape::Circle::default().into()).into(),
            material: materials.add(ColorMaterial::from(Color::WHITE)),
            transform: Transform
                ::from_translation(Vec3::new(point.x as f32, point.y as f32, 1.0))
                .with_scale(Vec3::new(6.0, 6.0, 0.0)),
            ..default()
        });
    }
}