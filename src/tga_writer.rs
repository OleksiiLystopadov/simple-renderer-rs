//Author:
//https://github.com/jonvaldes

//Code snippet:
//https://gist.github.com/jonvaldes/607fbc380f816d205afb


use std::mem;
use std::slice;
use std;
use std::fs::File;
use std::io::Write;
use std::io;


#[derive(Clone, Copy)]
pub struct Color(u8, u8, u8);

pub struct Image {
    width: i32,
    height: i32,
    data: Vec<Color>,
}

pub unsafe fn struct_to_u8_slice<T>(s: &T) -> &[u8] {
    let data_ptr: *const u8 = mem::transmute(s);
    slice::from_raw_parts(data_ptr, mem::size_of::<T>())
}

pub unsafe fn slice_to_u8_slice<T>(s: &[T]) -> &[u8] {
    let data_ptr: *const u8 = mem::transmute(&s[0]);
    slice::from_raw_parts(data_ptr, mem::size_of::<T>() * s.len())
}
impl Color{
    pub fn new(r : u8, g : u8, b : u8) -> Color{
        let result = Color(r,g,b);
        result
    }
}
impl Image {
    pub fn new(width: i32, height: i32) -> Image {
        let v = vec![Color(0,0,0);(width*height) as usize];
        let result = Image {
            width: width,
            height: height,
            data: v,
        };

        result
    }

    pub fn apply_gamma(self: &mut Image, gamma: f32) {
        for c in self.data.iter_mut() {
            let Color(r, g, b) = *c;
            let fr = ((r as f32) / 255.0).powf(gamma);
            let fg = ((g as f32) / 255.0).powf(gamma);
            let fb = ((b as f32) / 255.0).powf(gamma);
            c.0 = (fr * 255.0) as u8;
            c.1 = (fg * 255.0) as u8;
            c.2 = (fb * 255.0) as u8;
        }
    }

    pub fn set_pixel(self: &mut Image, x: i32, y: i32, c: Color) {
        self.data[(x + y * self.width) as usize] = c;
    }

    pub fn write_to_tga(self: &Image, filename: &str) -> io::Result<()> {
        #[repr(C, packed)]
        #[derive(Default)]
        struct Header {
            id_length: u8,
            color_map_type: u8,
            image_type: u8,
            c_map_start: u16,
            c_map_length: u16,
            c_map_depth: u8,
            x_offset: u16,
            y_offset: u16,
            width: u16,
            height: u16,
            pixel_depth: u8,
            image_descriptor: u8,
        }
        let h = Header {
            image_type: 2,
            width: self.width as u16,
            height: self.height as u16,
            pixel_depth: 24,
            ..Header::default()
        };

        let mut f = r#try!(File::create(filename));
        unsafe {
            r#try!(f.write_all(struct_to_u8_slice(&h)));
            r#try!(f.write_all(slice_to_u8_slice(&self.data[..])));
        }
        Ok(())
    }
}