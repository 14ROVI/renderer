extern crate nalgebra as na;
use na::{Vector2, Vector3, Vector4};

use crate::bounding_box::BoundingBox;

pub struct CanvasTriangle {
    pub v0: Vector3<f64>,
    pub v1: Vector3<f64>,
    pub v2: Vector3<f64>,
    pub colour: Vector3<f64>,
}
impl CanvasTriangle {
    pub fn new(v0: Vector3<f64>, v1: Vector3<f64>, v2: Vector3<f64>) -> Self {
        let area = (v2.x - v0.x) * (v1.y - v0.y) - (v2.y - v0.y) * (v1.x - v0.x);

        if area < 0.0 {
            CanvasTriangle {
                v0,
                v1,
                v2,
                colour: Vector3::new(1.0, 1.0, 1.0),
            }
        } else {
            CanvasTriangle {
                v0,
                v1: v2,
                v2: v1,
                colour: Vector3::new(1.0, 1.0, 1.0),
            }
        }
    }

    pub fn set_colour(&mut self, colour: Vector3<f64>) {
        self.colour = colour;
    }

    pub fn bounding_box(&self) -> BoundingBox {
        // returns top left, top right
        let top = self.v0.y.max(self.v1.y).max(self.v2.y);
        let left = self.v0.x.min(self.v1.x).min(self.v2.x);
        let right = self.v0.x.max(self.v1.x).max(self.v2.x);
        let bottom = self.v0.y.min(self.v1.y).min(self.v2.y);

        BoundingBox {
            top_left: Vector2::new(left, top),
            bottom_right: Vector2::new(right, bottom),
        }
    }
}

pub struct Edge {
    a: f64,
    b: f64,
    c: f64,
    pub step_x: Vector4<f64>,
    pub step_y: Vector4<f64>,
}
impl Edge {
    pub const STEP_X_SIZE: usize = 4;
    pub const STEP_Y_SIZE: usize = 1;

    pub fn edge_function(v0: &Vector3<f64>, v1: &Vector3<f64>, point: &Vector3<f64>) -> f64 {
        (v1.x - v0.x) * (point.y - v0.y) - (v1.y - v0.y) * (point.x - v0.x)
    }

    pub fn new(v0: &Vector3<f64>, v1: &Vector3<f64>) -> Self {
        let a = v0.y - v1.y;
        let b = v1.x - v0.x;
        let c = (v0.x * v1.y) - (v0.y * v1.x);

        let step_x = Vector4::repeat(a * 4.0); // 4 = x step size
        let step_y = Vector4::repeat(b * 1.0); // 1 = y step size

        Edge {
            a,
            b,
            c,
            step_x,
            step_y,
        }
    }

    pub fn get_at(&self, point: &Vector3<f64>) -> Vector4<f64> {
        let x = Vector4::repeat(point.x) + Vector4::new(0.0, 1.0, 2.0, 3.0);
        let y = Vector4::repeat(point.y);

        return self.a * x + self.b * y + Vector4::repeat(self.c);
    }
}
