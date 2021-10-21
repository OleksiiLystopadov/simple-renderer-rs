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

fn main() {
    let red: Color = Color::new(255, 0, 0);

    let mut image = Image::new(100, 100);
    image.set_pixel(52, 41, red);
    image.write_to_tga("output.tga").unwrap();
}
