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

fn main() {
    let white: Color = Color::new(255, 255, 255);
    let height = 1000;
    let width = 1000;

    let wavefront_object = wavefront_parser::read("C:\\Users\\oleksii.lystopadov\\Projects\\simple-renderer-rs\\src\\head.obj".to_string()).unwrap();
    let vectors = wavefront_object.0;
    let faces = wavefront_object.1;

    let mut image = Image::new(width + 1, height + 1);

    for i in 0..faces.len() {
        let face = faces.get(i).unwrap();

        for j in 0..3 {
            let v0 = vectors.get((*face.get(j).unwrap() - 1) as usize).unwrap();
            let v1 = vectors.get((*face.get((j + 1) % 3).unwrap() - 1) as usize).unwrap();

            let x0 = ((v0[0] as f64 + 1.0) * (width as f64 / 2.0)) as f32;
            let y0 = ((v0[1] as f64 + 1.0) * (height as f64 / 2.0)) as f32;
            let x1 = ((v1[0] as f64 + 1.0) * (width as f64 / 2.0)) as f32;
            let y1 = ((v1[1] as f64 + 1.0) * (height as f64 / 2.0)) as f32;
            draw_line(x0, y0, x1, y1, white, &mut image);
        }
    }
    image.write_to_tga("output.tga").unwrap();
}
