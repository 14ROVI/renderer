mod bounding_box;
mod canvas;
mod canvas_triangle;

extern crate nalgebra as na;

use minifb::{Key, ScaleMode, Window, WindowOptions};

use bounding_box::BoundingBox;
use canvas::Canvas;
use canvas_triangle::{CanvasTriangle, Edge};
use na::SimdPartialOrd;
use na::{Vector2, Vector3};
use std::time;

fn main() {
    let mut window = Window::new(
        "Trongles",
        400,
        400,
        WindowOptions {
            resize: true,
            scale_mode: ScaleMode::UpperLeft,
            ..WindowOptions::default()
        },
    )
    .expect("Unable to create the window");

    while window.is_open() && !window.is_key_down(Key::Escape) {
        let mut triangle = CanvasTriangle::new(
            Vector3::new(0.0, 0.0, 1.0),
            Vector3::new(50.0, 100.0, 1.0),
            Vector3::new(100.0, 0.0, 1.0),
        );
        triangle.set_colour(Vector3::new(1.0, 0.0, 0.0));

        let mut canvas = Canvas::new(window.get_size().0 as u32, window.get_size().1 as u32);

        let texture_bb = BoundingBox {
            top: canvas.get_height() as f64 - 1.0,
            right: canvas.get_width() as f64 - 1.0,
            bottom: 0.0,
            left: 0.0,
        };

        let frame_start = time::Instant::now();

        for _ in 0..10_000 {
            draw_triangle(&mut canvas, &texture_bb, &triangle);
        }

        let frame_end = time::Instant::now();
        println!(
            "frame took: {:}ms",
            frame_end.duration_since(frame_start).as_millis()
        );

        window
            .update_with_buffer(
                &canvas.get_canvas_buffer(),
                canvas.get_width() as usize,
                canvas.get_height() as usize,
            )
            .unwrap();
    }
}

fn draw_triangle(canvas: &mut Canvas, texture_bb: &BoundingBox, triangle: &CanvasTriangle) {
    let triangle_bb = triangle.bounding_box();

    if let Some(intersection) = triangle_bb.intersection(&texture_bb) {
        // go over each pixel in this intersection and check if its in the triangle :D

        let start_point = Vector3::new(intersection.left + 0.5, intersection.bottom + 0.5, 1.0);

        let w0_edge = Edge::new(&triangle.v1, &triangle.v2);
        let w1_edge = Edge::new(&triangle.v2, &triangle.v0);
        let w2_edge = Edge::new(&triangle.v0, &triangle.v1);

        let mut w0_row = w0_edge.get_at(&start_point);
        let mut w1_row = w1_edge.get_at(&start_point);
        let mut w2_row = w2_edge.get_at(&start_point);

        let min_y = intersection.bottom as usize;
        let max_y = intersection.top as usize;
        let min_x = intersection.left as usize;
        let max_x = intersection.right as usize;

        for y in (min_y..max_y).step_by(Edge::STEP_Y_SIZE) {
            let mut w0 = w0_row;
            let mut w1 = w1_row;
            let mut w2 = w2_row;

            for x in (min_x..max_x).step_by(Edge::STEP_X_SIZE) {
                for i in 0..Edge::STEP_X_SIZE {
                    if w0[i] >= 0.0 && w1[i] >= 0.0 && w2[i] >= 0.0 {
                        canvas.set_pixel((x + i) as u32, y as u32, 255, 255, 255, 1.0);
                    }
                }

                w0 += w0_edge.step_x;
                w1 += w1_edge.step_x;
                w2 += w2_edge.step_x;
            }

            w0_row += w0_edge.step_y;
            w1_row += w1_edge.step_y;
            w2_row += w2_edge.step_y;
        }
    }
}

// fn draw_line(t: RcCell<RenderTexture<u8>>, s: (i64, i64), e: (i64, i64), r: u8, g: u8, b: u8) {
//     let mut t = t.as_mut();

//     let dx = (e.0 - s.0) as f64;
//     let dy = (e.1 - s.1) as f64;

//     let step_x = dx / dx.abs();
//     let step_y = dy / dy.abs();

//     for i in s.0..(e.0 + 1) {
//         let x = (s.0 + i) as f64 * step_x;
//         let y = (s.1 + i) as f64 * step_y;
//         println!("{}, {}", x, y);
//         t.set_pixel(x as u32, y as u32, r, g, b);
//     }
// }
