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
mod geometry;
use tga_writer::Image;
use tga_writer::Color;
use wavefront_parser::read;
use std::time::Instant;
use crate::geometry::point_3d::Point3D;

const HEIGHT: f32 = 1000.0;
const WIDTH: f32 = 1000.0;
const DEPTH: f32  = 255.0;

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

fn draw_triangle(mut t0: Point3D<i32>, mut t1: Point3D<i32>, mut t2: Point3D<i32>, z_buffer: &mut Vec<i32>,color: Color, image: &mut Image) {
    if t0.y == t1.y && t0.y == t2.y { return; }

    if t0.y > t1.y { swap(&mut t0, &mut t1) };
    if t0.y > t2.y { swap(&mut t0, &mut t2) };
    if t1.y > t2.y { swap(&mut t1, &mut t2) };

    let total_height = t2.y - t0.y;
    for i in 0..total_height as i32 {
        let is_second_half = i > (t1.y - t0.y) || t1.y == t0.y;
        let segment_height = (if is_second_half { t2.y - t1.y } else { t1.y - t0.y }) ;
        let alpha = i as f32 / total_height as f32;
        let beta = (i - (if is_second_half { t1.y - t0.y } else { 0 })) as f32 / segment_height as f32;
        //ToDo: Replace with Point struct
        let mut a = ((t0.x as f32 + (t2.x - t0.x) as f32 * alpha) as i32, (t0.y as f32 + (t2.y - t0.y) as f32 * alpha) as i32, (t0.z as f32 + (t2.z - t0.z) as f32 * alpha) as i32);
        let mut b = if is_second_half {
            ((t1.x as f32 + (t2.x - t1.x) as f32 * beta) as i32, (t1.y as f32 + (t2.y - t1.y) as f32 * beta) as i32, (t1.z as f32 + (t2.z - t1.z) as f32 * beta) as i32)
        } else {
            ((t0.x as f32 + (t1.x - t0.x) as f32 * beta) as i32, (t0.y as f32 + (t1.y - t0.y) as f32 * beta) as i32, (t0.z as f32 + (t1.z - t0.z) as f32 * beta) as i32)
        };

        if a.0 > b.0 { swap(&mut a, &mut b) };
        for j in a.0 as i32..(b.0 as i32 + 1) as i32 {
            let phi: f32 = if b.0 == a.0 { 1. } else { (j - a.0) as f32 / (b.0 - a.0) as f32 };
            let p = (a.0 as f32 + (b.0 - a.0) as f32 * phi, a.1 as f32 + (b.1 - a.1) as f32 * phi, a.2 as f32 + (b.2 - a.2) as f32 * phi);
            let idx = (p.0 + p.1 * WIDTH) as usize;

            if z_buffer[idx] < p.2 as i32 {
                z_buffer[idx] = p.2 as i32;
                image.set_pixel(p.0 as i32, p.1 as i32, color);
            }
        }
    }
}

fn normalize_vector(vector: Vec<f32>) -> Vec<f32> {
    let length = vector.clone().into_iter().map(|value| value.powi(2)).reduce(|sum, value| sum + value).unwrap().sqrt();

    vector.into_iter().map(|coordinate| coordinate / length).collect()
}

fn render_object() {
    let light_dir: Vec<f32> = vec![0.0, 0.0, 1.0];

    let mut z_buffer = [i32::MIN; (WIDTH * HEIGHT) as usize].to_vec();

    let wavefront_object = wavefront_parser::read("E:\\project\\simple-renderer\\src\\head.obj".to_string()).unwrap();
    let vectors = wavefront_object.0;
    let faces = wavefront_object.1;

    let start_time = Instant::now();

    let mut image = Image::new(WIDTH as i32, HEIGHT as i32);

    for i in 0..faces.len() {
        let face = faces.get(i).unwrap();
        let mut screen_coordinates: Vec<Point3D<f32>> = vec![];
        let mut world_coordinates: Vec<Point3D<f32>> = vec![];
        for j in 0..3 {
            let v = vectors.get((*face.get(j).unwrap() - 1) as usize).unwrap();
            screen_coordinates.push(Point3D::<f32>::new(
                ((v[0] + 1.0) * WIDTH / 2.0) as f32,
                ((v[1] + 1.0) * HEIGHT / 2.0) as f32,
                ((v[2] + 1.0) * DEPTH / 2.0) as f32
            ));
            world_coordinates.push(Point3D::<f32>::new(v[0], v[1], v[2]));
        }

        let v1 = world_coordinates[0] - world_coordinates[1];
        let v2 = world_coordinates[1] - world_coordinates[2];


        let n: Vec<f32> = normalize_vector(
            vec![
                (v1.y as f32 * v2.z as f32) - (v1.z as f32 * v2.y as f32),
                (v1.z as f32 * v2.x as f32) - (v1.x as f32 * v2.z as f32),
                (v1.x as f32 * v2.y as f32) - (v1.y as f32 * v2.x as f32)
            ]
        );

        let intensity = n[0] * light_dir[0] + n[1] * light_dir[1] + n[2] * light_dir[2];
        if intensity > 0.0 {
            draw_triangle(
                Point3D::<i32>::from(screen_coordinates[0]),
                Point3D::<i32>::from(screen_coordinates[1]),
                Point3D::<i32>::from(screen_coordinates[2]),
                &mut z_buffer,
                Color::new((255.0 * intensity) as u8, (255.0 * intensity) as u8, (255.0 * intensity) as u8),
                &mut image,
            );
        }
    }
    image.write_to_tga("output.tga").unwrap();
    let elapsed_time = start_time.elapsed();
    println!("Done in: {} ms", elapsed_time.as_millis());
}

fn main() {
    render_object();
}
