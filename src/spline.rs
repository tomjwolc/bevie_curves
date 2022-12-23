use nalgebra::{Vector4, Vector3, Matrix4x3, Matrix4};
use bevy_prototype_lyon::prelude::{Path, PathBuilder};
use bevy::prelude::Vec2;

pub fn get_line_path(p1: Vec2, p2: Vec2) -> Path {
    let mut path_builder = PathBuilder::new();
    path_builder.move_to(p1);
    path_builder.line_to(p2);

    path_builder.build()
}

#[derive(Debug)]
pub struct Curve {
    characteristic_matrix: Matrix4<f64>,
    cached_points_matrix: Matrix4x3<f64>,
    pub points: [Vector3<f64>; 4]
}

impl Curve {
    pub fn new_bezier(p1: Vector3<f64>, p2: Vector3<f64>, p3: Vector3<f64>, p4: Vector3<f64>) -> Self {
        let characteristic_matrix = Matrix4::new(
             1.0,  0.0,  0.0, 0.0,
            -3.0,  3.0,  0.0, 0.0,
             3.0, -6.0,  3.0, 0.0,
            -1.0,  3.0, -3.0, 1.0
        );

        Curve { 
            characteristic_matrix,
            cached_points_matrix: characteristic_matrix * Matrix4x3::from_rows(&[
                p1.transpose(), 
                p2.transpose(), 
                p3.transpose(), 
                p4.transpose()
            ]),
            points: [p1, p2, p3, p4]
        }
    }

    pub fn new_bspline(p1: Vector3<f64>, p2: Vector3<f64>, p3: Vector3<f64>, p4: Vector3<f64>) -> Self {
        let characteristic_matrix = 1.0/6.0 * Matrix4::new(
             1.0,  0.0,  0.0, 0.0,
            -3.0,  0.0,  3.0, 0.0,
             3.0, -6.0,  3.0, 0.0,
            -1.0,  3.0, -3.0, 1.0
        );

        Curve { 
            characteristic_matrix,
            cached_points_matrix: characteristic_matrix * Matrix4x3::from_rows(&[
                p1.transpose(), 
                p2.transpose(), 
                p3.transpose(), 
                p4.transpose()
            ]),
            points: [p1, p2, p3, p4]
        }
    }

    pub fn set_control_point(&mut self, index: usize, p: Vector3<f64>) {
        if index >= 4 { panic!("Tried to set point on a curve with index: {}, but curves only have 4 points", index) }

        self.points[index] = p;

        self.cached_points_matrix = self.characteristic_matrix * Matrix4x3::from_rows(&[
            self.points[0].transpose(), 
            self.points[1].transpose(), 
            self.points[2].transpose(), 
            self.points[3].transpose()
        ]);
    }

    pub fn get_point(&self, t: f64) -> Vector3<f64> {
        (Vector4::new(1.0, t, t.powi(2), t.powi(3)).transpose() * self.cached_points_matrix).transpose()
    }

    pub fn to_bezier_path(&self) -> Path {
        let mut path_builder = PathBuilder::new();
        path_builder.move_to(Vec2::new(self.points[0].x as f32, self.points[0].y as f32));
        path_builder.cubic_bezier_to(
            Vec2::new(self.points[1].x as f32, self.points[1].y as f32), 
            Vec2::new(self.points[2].x as f32, self.points[2].y as f32), 
            Vec2::new(self.points[3].x as f32, self.points[3].y as f32)
        );

        path_builder.build()
    }
}

pub struct Spline {
    curves: Vec<Curve>
}

impl Spline {
    pub fn new_bezier(points: Vec<Vector3<f64>>) -> Result<Self, String> {
        match points.len() {
            0 => Err("Cannot create a spline without points".to_string()),
            n if n < 4 => Err("Cannot create a bezier spline without at least four points".to_string()),
            n if n % 3 != 1 => Err(format!("Cannot create a bezier spline with {} points", n)),
            n => {
                let mut curves = Vec::new();
                let num_curves = (n - 4) / 3 + 1;

                for i in 0..num_curves {
                    curves.push(Curve::new_bezier(
                        points[3 * i], 
                        points[3 * i + 1], 
                        points[3 * i + 2], 
                        points[3 * i + 3]
                    ))
                };

                Ok(Self { curves })
            }
        }
    }

    pub fn size(&self) -> usize {
        self.curves.len()
    }

    pub fn get_point(&self, t: f64) -> Vector3<f64> {
        let mut index = t.floor() as isize;

        index = if index < 0 { 
            0 
        } else if index >= self.curves.len() as isize { 
            self.curves.len() as isize - 1 
        } else { 
            index 
        };

        self.curves[index as usize].get_point(t - (index as f64))
    }
}