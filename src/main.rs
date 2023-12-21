mod bounding_box;
mod canvas_triangle;

extern crate minigw;
extern crate nalgebra as na;

use bounding_box::BoundingBox;
use canvas_triangle::{CanvasTriangle, Edge};

use std::time;

use minigw::RcCell;
use minigw::RenderTexture;
use na::{Vector2, Vector3};

fn main() {
    minigw::new::<u8, _>(
        "Example",
        400,
        400,
        move |_window, _input, render_texture, _imgui| {
            let frame_start = time::Instant::now();
            let mut triangle = CanvasTriangle::new(
                Vector3::new(0.0, 0.0, 1.0),
                Vector3::new(50.0, 100.0, 1.0),
                Vector3::new(100.0, 0.0, 1.0),
            );
            triangle.set_colour(Vector3::new(1.0, 0.0, 0.0));

            let texture_bb = BoundingBox {
                top_left: Vector2::new(0.0, render_texture.as_ref().get_height() as f64 - 1.0),
                bottom_right: Vector2::new(render_texture.as_ref().get_width() as f64 - 1.0, 0.0),
            };

            // draw_line(render_texture, (0, 0), (100, 100), 255, 255, 255);

            for _ in 0..10_000 {
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

fn draw_triangle(
    texture: &RcCell<RenderTexture<u8>>,
    texture_bb: &BoundingBox,
    triangle: &CanvasTriangle,
) {
    let mut texture = texture.as_mut();

    let triangle_bb = triangle.bounding_box();

    if let Some(intersection) = triangle_bb.intersection(&texture_bb) {
        // go over each pixel in this intersection and check if its in the triangle :D

        // Barycentric coordinates at minX/minY corner
        let point = Vector3::new(
            intersection.top_left.x + 0.5,
            intersection.bottom_right.y + 0.5,
            1.0,
        );
        let w0_edge = Edge::new(triangle.v1, triangle.v2);
        let w1_edge = Edge::new(triangle.v2, triangle.v0);
        let w2_edge = Edge::new(triangle.v0, triangle.v1);

        let mut w0_row = w0_edge.get_at(point);
        let mut w1_row = w1_edge.get_at(point);
        let mut w2_row = w2_edge.get_at(point);

        for y in ((intersection.bottom_right.y as u32)..(intersection.top_left.y as u32))
            .step_by(Edge::STEP_Y_SIZE.try_into().unwrap())
        {
            let mut w0 = w0_row;
            let mut w1 = w1_row;
            let mut w2 = w2_row;

            for x in ((intersection.top_left.x as u32)..(intersection.bottom_right.x as u32))
                .step_by(Edge::STEP_X_SIZE.try_into().unwrap())
            {
                for i in 0..4 {
                    let pw0 = w0[i];
                    let pw1 = w1[i];
                    let pw2 = w2[i];

                    if pw0 >= 0.0 && pw1 >= 0.0 && pw2 >= 0.0 {
                        texture.set_pixel(x + (i as u32), y, 255, 255, 255);
                    }
                }

                // One step to the right
                w0 += w0_edge.step_x;
                w1 += w1_edge.step_x;
                w2 += w2_edge.step_x;
            }

            // One row step
            w0_row += w0_edge.step_y;
            w1_row += w1_edge.step_y;
            w2_row += w2_edge.step_y;
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
