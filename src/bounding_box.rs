extern crate nalgebra as na;
use na::Vector2;

#[derive(Debug)]
pub struct BoundingBox {
    pub top: f64,
    pub right: f64,
    pub bottom: f64,
    pub left: f64,
}
impl BoundingBox {
    pub fn intersection(&self, other: &Self) -> Option<Self> {
        if self.left > other.right
            || self.right < other.left
            || self.top < other.bottom
            || self.bottom > other.top
        {
            return None;
        }

        let top = self.top.min(other.top);
        let right = self.right.min(other.right);
        let bottom = self.bottom.max(other.bottom);
        let left = self.left.max(other.left);

        Some(BoundingBox {
            top,
            right,
            bottom,
            left,
        })
    }
}
