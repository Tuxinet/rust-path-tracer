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
use std::sync::Arc;
use crate::vecutil::VecUtil;
use crate::material::*;
use scoped_threadpool::Pool;
use std::sync::mpsc::*;
use std::time::Instant;

use glam::Vec3;

struct PixelData {
    x: u32,
    y: u32,
    c: Vec3,
}

fn main() {
    let mut w: World = World::new();
    let mut rng = rand::thread_rng();

    w.add_obj(Sphere::new(Vec3::new(0.0, -2.0, -2.0), 1.0, Arc::new(Metal::new(Vec3::new(0.7, 0.7, 0.7)))));
    w.add_obj(Sphere::new(Vec3::new(0.0, 2.0, -2.0), 1.0, Arc::new(Metal::new(Vec3::new(0.7, 0.7, 0.7)))));
    w.add_obj(Sphere::new(Vec3::new(-2.0, 0.0, -2.0), 1.0, Arc::new(Metal::new(Vec3::new(0.7, 0.7, 0.7)))));
    w.add_obj(Sphere::new(Vec3::new(2.0, 0.0, -2.0), 1.0, Arc::new(Metal::new(Vec3::new(0.7, 0.7, 0.7)))));
    w.add_obj(Sphere::new(Vec3::new(0.0, 0.0, -4.0), 1.0, Arc::new(Metal::new(Vec3::new(0.7, 0.7, 0.7)))));
    w.add_obj(Sphere::new(Vec3::new(0.0, 0.0, 4.0), 3.5, Arc::new(Metal::new(Vec3::new(0.7, 0.7, 0.7)))));
    //w.add_obj(Sphere::new(Vec3::new(1.0, 0.0, -2.0), 0.5));
    w.add_obj(Sphere::new(Vec3::new(0.0, -2.0, 0.0), 1.0, Arc::new(Lambertian::new(Vec3::new(1.0, 1.0, 0.5)))));

    for _ in 0..0 { 
        w.add_obj(Sphere::new((VecUtil::random_in_unit_sphere(&mut rng).normalize() * 1.52) + Vec3::new(0.0, 0.0, -2.0), 0.04, Arc::new(Metal::new(VecUtil::random_in_unit_sphere(&mut rng).normalize()))));
    }

    let aspect_ratio: f32 = 16.0 / 9.0;
    let image_width: u32 = 16000;
    let image_height: u32 = (image_width as f32 / aspect_ratio) as u32;

    let bounds: (usize, usize) = (image_width as usize, image_height as usize);
    let mut img = vec![Vec3::new(0.0, 0.0, 0.0); bounds.0 * bounds.1];
    let samples_per_pixel: u32 = 100;


    println!("Starting path tracing with dimensions:\n\tWidth: {}\n\tHeight: {}", image_width, image_height);

    let mut pool = Pool::new( 16 );

    let num_rows_per_task: u32 = 60;

    let c: Camera = Camera::new();

    let start = Instant::now();
    pool.scoped(|scoped| {
        let (tx, rx): (Sender<PixelData>, Receiver<PixelData>) = channel();

        for j in (0..=image_height).rev().step_by(num_rows_per_task as usize) {
            // Cloning variables before moving to thread
            let c = c.clone();
            let w = w.clone();
            let tx = tx.clone();

            scoped.execute(move || {
                let mut rng = rand::thread_rng();
                let start = j;
                let mut _end = 0;
                if (start as i32 - num_rows_per_task as i32) > 0 {
                    _end = start - num_rows_per_task as u32;
                }
                else {
                    _end = 0;
                }

                for j in _end..start {

                    for i in 0..image_width {
                        let mut ac: Vec3 = Vec3::new(0.0, 0.0, 0.0);
                        for _ in 0..samples_per_pixel {
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
    
                        let p = PixelData {
                            x: i,
                            y: j,
                            c: ac,
                        };
    
                        tx.send(p).unwrap();
                    }
                }
            });
        }

        let mut last_update = Instant::now();

        for _ in 0..((image_width * image_height) as usize) {
            let p = rx.recv().unwrap();
            img[(p.x + p.y * image_width) as usize] = p.c;
            if last_update.elapsed().as_secs() > 30 {
                write_image("trace.png", &img, bounds).unwrap();
                last_update = Instant::now();
            }
        }
    });

    println!("Execution took {} ms", start.elapsed().as_millis());

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

    let mut buffer = vec![0u8; bounds.0 * bounds.1 * 3];
    //img.put_pixel(20, 20, image::Rgb([255,0,0]));
    for y in (0..(bounds.1)).rev() {
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
