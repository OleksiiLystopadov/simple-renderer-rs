use std;
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;
use std::error::Error;
use std::mem::swap;
use std::str::FromStr;
use std::path::Path;

mod tga_writer;
use tga_writer::Image;
use tga_writer::Color;

fn draw_line(x0: f32, y0: f32, x1: f32, y1: f32, color: Color, image: &mut Image) {
    for step in 1..100 {
        let normalized_step: f32 = step as f32 * 0.01;
        let x = x0 * (1. - normalized_step) + x1 * normalized_step;
        let y = y0 * (1. - normalized_step) + y1 * normalized_step;
        image.set_pixel(x as i32, y as i32, color);
    }
}

fn main() {
    let red: Color = Color::new(255, 0, 0);

    let mut image = Image::new(100, 100);

    draw_line(10.0, 20.0, 80.0, 40.0, red, &mut image);
    image.write_to_tga("output.tga").unwrap();
}
