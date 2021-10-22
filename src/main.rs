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

fn draw_line(x0: u16, y0: u16, x1: u16, y1: u16, color: Color, image: &mut Image) {
    for x in x0..x1 {
        let normalized_step: f32 = (x - x0) as f32 / (x1 - x0) as f32;
        let y = y0 as f32 * (1. - normalized_step) + y1 as f32 * normalized_step;
        image.set_pixel(x as i32, y as i32, color);
    }
}

fn main() {
    let white: Color = Color::new(0, 0, 255);

    let mut image = Image::new(100, 100);

    draw_line(13, 20, 80, 40, white, &mut image);
    draw_line(20, 13, 40, 80, white, &mut image);
    draw_line(80, 40, 13, 20, white, &mut image);
    image.write_to_tga("output.tga").unwrap();
}
