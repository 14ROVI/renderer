extern crate nalgebra as na;
use na::{Vector2, Vector3};

use crate::bounding_box::BoundingBox;

pub struct CanvasTriangle {
    pub v0: Vector3<f64>,
    pub v1: Vector3<f64>,
    pub v2: Vector3<f64>,
    pub colour: Vector3<f64>,
}
impl CanvasTriangle {
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
