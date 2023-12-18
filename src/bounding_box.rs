extern crate nalgebra as na;
use na::Vector2;

#[derive(Debug)]
pub struct BoundingBox {
    pub top_left: Vector2<f64>,
    pub bottom_right: Vector2<f64>,
}
impl BoundingBox {
    pub fn intersection(&self, other: &Self) -> Option<Self> {
        if self.top_left.x > other.bottom_right.x
            || self.bottom_right.x < other.top_left.x
            || self.top_left.y < other.bottom_right.y
            || self.bottom_right.y > other.top_left.y
        {
            return None;
        }

        let top = self.top_left.y.min(other.top_left.y);
        let left = self.top_left.x.max(other.top_left.x);
        let right = self.bottom_right.x.min(other.bottom_right.x);
        let bottom = self.bottom_right.y.max(other.bottom_right.y);

        Some(BoundingBox {
            top_left: Vector2::new(left, top),
            bottom_right: Vector2::new(right, bottom),
        })
    }
}
