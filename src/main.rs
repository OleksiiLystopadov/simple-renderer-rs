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

fn draw_line(mut x0: f32, mut y0: f32, mut x1: f32, mut y1: f32, color: Color, image: &mut Image) {
    let dx = (x0 - x1).abs();
    let dy = (y0 - y1).abs();
    let is_steep = dx < dy;

    if is_steep {
        swap(&mut x0, &mut y0);
        swap(&mut x1, &mut y1);
    }

    if x0 > x1 {
        swap(&mut x0,&mut x1);
        swap(&mut y0,&mut y1);
    }

    for x in x0 as i32 .. x1 as i32  {
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

    let mut image = Image::new(100, 100);

    draw_line(13.0, 20.0, 80.0, 40.0, white, &mut image);
    draw_line(20.0, 13.0, 40.0, 80.0, white, &mut image);
    draw_line(80.0, 40.0, 13.0, 20.0, white, &mut image);
    image.write_to_tga("output.tga").unwrap();
}
