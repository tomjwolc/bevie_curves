use nalgebra::{Vector4, Vector3, Matrix4x3, Matrix4};

pub struct Curve {
    cached_points_matrix: Matrix4x3<f64>
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
            cached_points_matrix: characteristic_matrix * Matrix4x3::from_rows(&[
                p1.transpose(), 
                p2.transpose(), 
                p3.transpose(), 
                p4.transpose()
            ])
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
            cached_points_matrix: characteristic_matrix * Matrix4x3::from_rows(&[
                p1.transpose(), 
                p2.transpose(), 
                p3.transpose(), 
                p4.transpose()
            ])
        }
    }

    pub fn get_point(&self, t: f64) -> Vector3<f64> {
        (Vector4::new(1.0, t, t.powi(2), t.powi(3)).transpose() * self.cached_points_matrix).transpose()
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