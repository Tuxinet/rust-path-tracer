extern crate rand;
mod ray;
mod primitives;
mod world;
mod camera;
mod vecutil;
mod material;
use crate::ray::Ray;
use crate::primitives::*;
use crate::world::World;
use crate::camera::Camera;
use rand::*;
use std::ops::Mul;
use std::ops::Div;
use std::sync::Arc;
use crate::vecutil::VecUtil;
use crate::material::*;

use image::{GenericImage, GenericImageView, ImageBuffer, RgbImage, Pixel};
use glam::Vec3;

fn main() {

    let mut w: World = World::new();
    let mut rng = rand::thread_rng();

    let l: Lambertian = Lambertian::new(Vec3::new(0.5, 0.1, 0.3));
    let m: Metal = Metal::new(Vec3::new(1.0, 1.0, 1.0));

    w.add_obj(Sphere::new(Vec3::new(0.0, -2.0, -2.0), 1.0, Arc::new(Metal::new(Vec3::new(0.8, 0.8, 0.8)))));
    w.add_obj(Sphere::new(Vec3::new(0.0, 2.0, -2.0), 1.0, Arc::new(Metal::new(Vec3::new(0.8, 0.8, 0.8)))));
    w.add_obj(Sphere::new(Vec3::new(-2.0, 0.0, -2.0), 1.0, Arc::new(Metal::new(Vec3::new(0.8, 0.8, 0.8)))));
    w.add_obj(Sphere::new(Vec3::new(2.0, 0.0, -2.0), 1.0, Arc::new(Metal::new(Vec3::new(0.8, 0.8, 0.8)))));
    w.add_obj(Sphere::new(Vec3::new(0.0, 0.0, -4.0), 1.0, Arc::new(Metal::new(Vec3::new(0.8, 0.8, 0.8)))));
    //w.add_obj(Sphere::new(Vec3::new(-1.0, 0.0, -2.0), 0.5, Arc::new(Metal::new(Vec3::new(1.0, 0.0, 1.0)))));
    //w.add_obj(Sphere::new(Vec3::new(1.0, 0.0, -2.0), 0.5));
    //w.add_obj(Sphere::new(Vec3::new(0.0, -100.5, -2.0), 100.0, Arc::new(Metal::new(Vec3::new(0.9, 0.9, 0.9)))));

    for i in 0..0 { 
        w.add_obj(Sphere::new((VecUtil::random_in_unit_sphere(&mut rng).normalize() * 1.52) + Vec3::new(0.0, 0.0, -2.0), 0.04, Arc::new(Metal::new(VecUtil::random_in_unit_sphere(&mut rng).normalize()))));
    }

    let aspect_ratio: f32 = 16.0 / 9.0;
    let image_width: u32 = 3840;
    let image_height: u32 = (image_width as f32 / aspect_ratio) as u32;

    let bounds: (usize, usize) = (image_width as usize, image_height as usize);
    let mut img = vec![Vec3::new(0.0, 0.0, 0.0); bounds.0 * bounds.1];
    let samples_per_pixel: u32 = 200;


    println!("Starting path tracing with dimensions:\n\tWidth: {}\n\tHeight: {}", image_width, image_height);

    let c: Camera = Camera::new();

    for j in (0..(image_height)).rev() {
        for i in 0..image_width {
            let mut ac: Vec3 = Vec3::new(0.0, 0.0, 0.0);
            for s in 0..samples_per_pixel {
                let u: f32 = (i as f32 + rng.gen_range(0.0, 1.0)) / (image_width as f32 - 1.0);
                let v: f32 = (j as f32 + rng.gen_range(0.0, 1.0)) / (image_height as f32 - 1.0);
                let r: Ray = c.get_ray(u, v);
    
                let c: Vec3 = ray_color(&r, &w, 100, &mut rng);

                ac += c;
            }

            ac = ac / samples_per_pixel as f32;
            ac.set_x(ac.x().sqrt());
            ac.set_y(ac.y().sqrt());
            ac.set_z(ac.z().sqrt());
            img[((i + j * image_width)) as usize] = ac;
        }
    }

    write_image("trace.png", &img, bounds).unwrap();

    
}

fn ray_color(r: &Ray, w: &World, depth: u32, rng: &mut rand::prelude::ThreadRng) -> Vec3 {
    if depth <= 0 {
        return Vec3::new(0.0, 0.0, 0.0);
    }

    match w.intersection(r, 0.0001, std::f32::MAX) {
        Some(hit) => {

            match hit.material.scatter(r, &hit, rng) {
                Some(record) => {
                    return record.attenuation * ray_color(&record.scattered, w, depth-1, rng);
                }

                None => {}
            }
        }
        None => {
            let t = 0.5 * (r.direction.normalize().y() + 1.0);

            let c = (1.0 - t) * Vec3::new(1.0, 1.0, 1.0) + t * Vec3::new(0.5, 0.7, 1.0);
            
            return c;
        }
    }

    return Vec3::new(0.0, 0.0, 0.0);
}

fn write_image(filename: &str, pixels: &[Vec3], bounds: (usize, usize))
    -> Result<(), std::io::Error> {
    //let output = File::create("filename.png")?;

    // Construct a new RGB ImageBuffer with the specified width and height.
    let mut img: RgbImage = ImageBuffer::new(bounds.0 as u32, bounds.1 as u32);

    let mut buffer = vec![0u8; bounds.0 * bounds.1 * 3];
    //img.put_pixel(20, 20, image::Rgb([255,0,0]));
    let mut ind: usize = 0;
    for y in (0..(bounds.1-1)).rev() {
        for x in 0..bounds.0 {
            buffer[x * 3 + y * 3 * bounds.0] = (pixels[x + (bounds.1 - y - 1) * bounds.0].x() * 255.0) as u8;
            buffer[x * 3 + y * 3 * bounds.0 + 1] = (pixels[x + (bounds.1 - y - 1) * bounds.0].y() * 255.0) as u8;
            buffer[x * 3 + y * 3 * bounds.0 + 2] = (pixels[x + (bounds.1 - y - 1) * bounds.0].z() * 255.0) as u8;
        }
    }
    image::save_buffer(filename, &buffer, bounds.0 as u32, bounds.1 as u32, image::ColorType::Rgb8).unwrap();

    //let encoder = PNGEncoder::new(output);
    //encoder.encode(&pixels,
                    //bounds.0 as u32, bounds.1 as u32,
                    //ColorType::Gray(8))?;


    Ok(())
}
