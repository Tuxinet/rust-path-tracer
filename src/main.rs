mod ray;
mod primitives;
mod world;
use crate::ray::Ray;
use crate::primitives::*;
use crate::world::World;

use image::{GenericImage, GenericImageView, ImageBuffer, RgbImage, Pixel};
use glam::Vec3;

#[derive(Copy, Clone, Debug, Default)]
struct color {
    pub r: f32,
    pub g: f32,
    pub b: f32,
}

fn main() {

    let mut w: World = World::new();

    w.add_obj(Sphere::new(Vec3::new(0.0, 0.0, -2.0), 0.5));
    w.add_obj(Sphere::new(Vec3::new(-1.0, 0.0, -2.0), 0.5));
    w.add_obj(Sphere::new(Vec3::new(1.0, 0.0, -2.0), 0.5));
    w.add_obj(Sphere::new(Vec3::new(0.0, -100.5, -2.0), 100.0));

    let aspect_ratio: f32 = 16.0 / 9.0;
    let image_width: u32 = 2048;
    let image_height: u32 = (image_width as f32 / aspect_ratio) as u32;

    let bounds: (usize, usize) = (image_width as usize, image_height as usize);
    let mut img = vec![color{r: 1.0, g: 0.0, b: 0.0}; bounds.0 * bounds.1];


    println!("Starting path tracing with dimensions:\n\tWidth: {}\n\tHeight: {}", image_width, image_height);

    let origin: Vec3 = Vec3::new(0.0, 0.0, 0.0);
    let horizontal: Vec3 = Vec3::new(4.0, 0.0, 0.0);
    let vertical: Vec3 = Vec3::new(0.0, 2.25, 0.0);
    let lower_left: Vec3 = origin - horizontal/2.0 - vertical/2.0 - Vec3::new(0.0, 0.0, 1.0);

    for j in (0..(image_height)).rev() {
        for i in 0..image_width {
            let u: f32 = i as f32 / (image_width as f32 - 1.0);
            let v: f32 = j as f32 / (image_height as f32 - 1.0);
            let r: Ray = Ray::new(origin, lower_left + u*horizontal + v*vertical);

            let c: color = ray_color(&r, &w);

            img[((i + j * image_width)) as usize] = c;
        }
    }

    write_image("trace.png", &img, bounds);

    
}

fn ray_color(r: &Ray, w: &World) -> color {
    
    match w.intersection(r, 0.0, std::f32::MAX) {
        Some(hit) => {
            return color {
                r: 0.5 * (1.0 + hit.normal.x()),
                g: 0.5 * (1.0 + hit.normal.y()),
                b: 0.5 * (1.0 + hit.normal.z()),
            }
        }
        None => {
            let t = 0.5 * (r.direction.normalize().y() + 1.0);
    
            let c: color = color {
                r: 1.0 - t + (t * 0.5),
                g: 1.0 - t + (t * 0.7),
                b: 1.0 - t + (t * 1.0)
            };
            
            return c;
        }
    }
}

fn hit_sphere(center: Vec3, radius: f32, r: &Ray) -> f32 {
    let oc = r.origin - center;
    let a = r.direction.dot(r.direction);
    let b = 2.0 * oc.dot(r.direction);
    let c = oc.dot(oc) - radius * radius;
    let discriminant = b*b - 4.0*a*c;
    
    if discriminant < 0.0 {
        return -1.0;
    } else {
        return (-b -discriminant.sqrt()) / (2.0*a);
    }
}

fn write_image(filename: &str, pixels: &[color], bounds: (usize, usize))
    -> Result<(), std::io::Error> {
    //let output = File::create("filename.png")?;

    // Construct a new RGB ImageBuffer with the specified width and height.
    let mut img: RgbImage = ImageBuffer::new(bounds.0 as u32, bounds.1 as u32);

    let mut buffer = vec![0u8; bounds.0 * bounds.1 * 3];
    //img.put_pixel(20, 20, image::Rgb([255,0,0]));
    let mut ind: usize = 0;
    for y in (0..(bounds.1-1)).rev() {
        for x in 0..bounds.0 {
            buffer[x * 3 + y * 3 * bounds.0] = (pixels[x + (bounds.1 - y - 1) * bounds.0].r * 255.0) as u8;
            buffer[x * 3 + y * 3 * bounds.0 + 1] = (pixels[x + (bounds.1 - y - 1) * bounds.0].g * 255.0) as u8;
            buffer[x * 3 + y * 3 * bounds.0 + 2] = (pixels[x + (bounds.1 - y - 1) * bounds.0].b * 255.0) as u8;
        }
    }
    image::save_buffer(filename, &buffer, bounds.0 as u32, bounds.1 as u32, image::ColorType::Rgb8).unwrap();

    //let encoder = PNGEncoder::new(output);
    //encoder.encode(&pixels,
                    //bounds.0 as u32, bounds.1 as u32,
                    //ColorType::Gray(8))?;


    Ok(())
}
