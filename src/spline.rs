use nalgebra::{Vector4, Vector3, Matrix4x3, Matrix4};

pub struct Curve {
    cached_points_matrix: Matrix4x3<f64>
}

impl Curve {
    fn new_bezier(p1: Vector3<f64>, p2: Vector3<f64>, p3: Vector3<f64>, p4: Vector3<f64>) -> Self {
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
    
    fn new_bspline(p1: Vector3<f64>, p2: Vector3<f64>, p3: Vector3<f64>, p4: Vector3<f64>) -> Self {
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


    fn get_point(&self, t: f64) -> Vector3<f64> {
        (Vector4::new(1.0, t, t.powi(2), t.powi(3)).transpose() * self.cached_points_matrix).transpose()
    }
}

struct Spline {
    curves: Vec<Curve>
}

impl Spline {
    fn get_point(&self, t: f64) -> Vector3<f64> {
        
    }
}