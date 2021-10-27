use std;
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;
use std::error::Error;
use std::mem::swap;
use std::str::FromStr;
use std::path::Path;

mod tga_writer;
mod wavefront_parser;

use tga_writer::Image;
use tga_writer::Color;
use wavefront_parser::read;

fn draw_line(mut x0: f32, mut y0: f32, mut x1: f32, mut y1: f32, color: Color, image: &mut Image) {
    let dx = (x0 - x1).abs();
    let dy = (y0 - y1).abs();
    let is_steep = dx < dy;

    if is_steep {
        swap(&mut x0, &mut y0);
        swap(&mut x1, &mut y1);
    }

    if x0 > x1 {
        swap(&mut x0, &mut x1);
        swap(&mut y0, &mut y1);
    }

    for x in x0 as i32..x1 as i32 {
        let normalized_step: f32 = (x as f32 - x0) / (x1 - x0);
        let y = y0 * (1. - normalized_step) + y1 * normalized_step;
        if is_steep {
            image.set_pixel(y as i32, x as i32, color);
        } else {
            image.set_pixel(x as i32, y as i32, color);
        }
    }
}

fn draw_triangle(mut t0: (f32, f32), mut t1: (f32, f32), mut t2: (f32, f32), color: Color, image: &mut Image) {
    if t0.1 == t1.1 && t0.1 == t2.1 { return; }

    if t0.1 > t1.1 { swap(&mut t0, &mut t1) };
    if t0.1 > t2.1 { swap(&mut t0, &mut t2) };
    if t1.1 > t2.1 { swap(&mut t1, &mut t2) };

    let total_height = t2.1 - t0.1;
    for i in 0..total_height as i32 {
        let is_second_half = i > (t1.1 - t0.1) as i32 || t1.1 == t0.1;
        let segment_height = (if is_second_half { t2.1 - t1.1 } else { t1.1 - t0.1 }) as i32;
        let alpha = i as f32 / total_height;
        let beta = ( i as f32 - ( if is_second_half { t1.1 - t0.1 } else { 0.0 } )) / segment_height as f32;
        let mut a = (t0.0 + (t2.0 - t0.0) * alpha, t0.1 + (t2.1 - t0.1) * alpha);
        let mut b = if is_second_half { (t1.0 + (t2.0 - t1.0) * beta, t1.1 + (t2.1 - t1.1) * beta) } else { (t0.0 + (t1.0 - t0.0) * beta, t0.1 + (t1.1 - t0.1) * beta) };

        if a.0 > b.0 { swap(&mut a, &mut b) };
        for j in a.0 as i32..(b.0 as i32 + 1) as i32 {
            image.set_pixel(j, t0.1 as i32 + i, Color::new(255, 255, 255))
        }
    }
}

fn render_object() {
    let white: Color = Color::new(255, 255, 255);
    let height = 1000;
    let width = 1000;
    let scale = 1.0;

    let wavefront_object = wavefront_parser::read("E:\\project\\simple-renderer\\src\\head.obj".to_string()).unwrap();
    let vectors = wavefront_object.0;
    let faces = wavefront_object.1;

    let mut image = Image::new(width + 1, height + 1);

    for i in 0..faces.len() {
        let face = faces.get(i).unwrap();

        for j in 0..3 {
            let v0 = vectors.get((*face.get(j).unwrap() - 1) as usize).unwrap();
            let v1 = vectors.get((*face.get((j + 1) % 3).unwrap() - 1) as usize).unwrap();

            let x0 = ((v0[0] as f64 + 1.0) * scale * (width as f64 / 2.0)) as f32;
            let y0 = ((v0[1] as f64 + 1.0) * scale * (height as f64 / 2.0)) as f32;
            let x1 = ((v1[0] as f64 + 1.0) * scale * (width as f64 / 2.0)) as f32;
            let y1 = ((v1[1] as f64 + 1.0) * scale * (height as f64 / 2.0)) as f32;
            draw_line(x0, y0, x1, y1, white, &mut image);
        }
    }
    image.write_to_tga("output.tga").unwrap();
}

fn main() {
    let height = 1000;
    let width = 1000;
    let mut image = Image::new(width + 1, height + 1);

    let t0 = vec![(10.0, 70.0), (50.0, 160.0), (70.0, 80.0)];
    let t1 = vec![(180.0, 50.0), (150.0, 1.0), (70.0, 180.0)];
    let t2 = vec![(180.0, 150.0), (120.0, 160.0), (130.0, 180.0)];

    draw_triangle(t0[0], t0[1], t0[2], Color::new(0, 255, 0), &mut image);
    draw_triangle(t1[0], t1[1], t1[2], Color::new(0, 255, 0), &mut image);
    draw_triangle(t2[0], t2[1], t2[2], Color::new(0, 255, 0), &mut image);
    image.write_to_tga("triangles.tga").unwrap();
}
