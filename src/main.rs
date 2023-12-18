mod bounding_box;
mod canvas_triangle;

extern crate minigw;
extern crate nalgebra as na;

use bounding_box::BoundingBox;
use canvas_triangle::CanvasTriangle;

use std::time;

use minigw::RcCell;
use minigw::RenderTexture;
use na::{Vector2, Vector3};

fn main() {
    minigw::new::<u8, _>(
        "Example",
        720,
        720,
        move |_window, _input, render_texture, _imgui| {
            let frame_start = time::Instant::now();
            let triangle = CanvasTriangle {
                v0: Vector3::new(0.0, 0.0, 1.0),
                v1: Vector3::new(5.0, 5.0, 1.0),
                v2: Vector3::new(10.0, 0.0, 1.0),
                colour: Vector3::new(1.0, 1.0, 1.0),
            };

            let texture_bb = BoundingBox {
                top_left: Vector2::new(0.0, render_texture.as_ref().get_height() as f64 - 1.0),
                bottom_right: Vector2::new(render_texture.as_ref().get_width() as f64 - 1.0, 0.0),
            };

            // draw_line(render_texture, (0, 0), (100, 100), 255, 255, 255);

            for _ in 0..100000 {
                draw_triangle(&render_texture, &texture_bb, &triangle);
            }

            let frame_end = time::Instant::now();
            println!(
                "frame took: {:}ms",
                frame_end.duration_since(frame_start).as_millis()
            )
        },
    );
}

fn edge_function(v0: Vector3<f64>, v1: Vector3<f64>, point: Vector3<f64>) -> f64 {
    (point.x - v0.x) * (v1.y - v0.y) - (point.y - v0.y) * (v1.x - v0.x)
}

fn draw_triangle(
    texture: &RcCell<RenderTexture<u8>>,
    texture_bb: &BoundingBox,
    triangle: &CanvasTriangle,
) {
    let mut texture = texture.as_mut();

    let triangle_bb = triangle.bounding_box();

    if let Some(intersection) = triangle_bb.intersection(&texture_bb) {
        // go over each pixel in this intersection and check if its in the triangle :D
        for y in (intersection.bottom_right.y as u32)..(intersection.top_left.y as u32) {
            for x in (intersection.top_left.x as u32)..(intersection.bottom_right.x as u32) {
                let point = Vector3::new(x as f64 + 0.5, y as f64 + 0.5, 1.0);
                let w0 = edge_function(triangle.v1, triangle.v2, point);
                let w1 = edge_function(triangle.v2, triangle.v0, point);
                let w2 = edge_function(triangle.v0, triangle.v1, point);

                if w0 >= 0.0 && w1 >= 0.0 && w2 >= 0.0 {
                    texture.set_pixel(point.x as u32, point.y as u32, 255, 255, 255);
                }
            }
        }
    }
}

fn draw_line(t: RcCell<RenderTexture<u8>>, s: (i64, i64), e: (i64, i64), r: u8, g: u8, b: u8) {
    let mut t = t.as_mut();

    let dx = (e.0 - s.0) as f64;
    let dy = (e.1 - s.1) as f64;

    let step_x = dx / dx.abs();
    let step_y = dy / dy.abs();

    for i in s.0..(e.0 + 1) {
        let x = (s.0 + i) as f64 * step_x;
        let y = (s.1 + i) as f64 * step_y;
        println!("{}, {}", x, y);
        t.set_pixel(x as u32, y as u32, r, g, b);
    }
}
