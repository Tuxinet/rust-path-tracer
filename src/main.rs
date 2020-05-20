extern crate rand;
mod ray;
mod primitives;
mod world;
mod camera;
mod vecutil;
mod material;
mod aabb;
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

extern crate nalgebra as na;
use na::{Vector3,RowVector3};

use glam::Vec3;

struct PixelData {
    x: u32,
    y: u32,
    c: Vec3,
}

fn main() {
    let mut w: World = World::new();
    let mut rng = rand::thread_rng();

    //w.add_obj(Arc::new(Sphere::new(Vec3::new(-4.0, 1.0, 0.0), 1.0, Arc::new(DiffuseLight::new(Vec3::new(1.0, 1.0, 1.0))))));
    //w.add_obj(Arc::new(Sphere::new(Vec3::new(0.0, 1.0, 0.0), 1.0, Arc::new(Dielectric::new(1.5)))));
    w.add_obj(Arc::new(Sphere::new(Vector3::new(0.0, -500.0, -3.0), 500.0, Arc::new(Lambertian::new(Vec3::new(0.5, 0.5, 0.5))))));
    w.add_obj(Arc::new(Sphere::new(Vector3::new(2.0, 1.0, -0.0), 1.0, Arc::new(Metal::new(Vec3::new(0.7, 0.6, 0.5), 0.5)))));
    w.add_obj(Arc::new(Sphere::new(Vector3::new(4.0, 1.0, -0.0), 1.0, Arc::new(Metal::new(Vec3::new(0.7, 0.6, 0.5), 0.5)))));
    //w.add_obj(Arc::new(Sphere::new(Vec3::new(0.0, -500.0, -3.0), 500.0, Arc::new(Dielectric::new(1.5)))));
    w.add_obj(Arc::new(Sphere::new(Vector3::new(0.0, 1.0, -0.0), 1.0, Arc::new(DiffuseLight::new(Vec3::new(100.0, 100.0, 100.0))))));
    //w.add_obj(Arc::new(Sphere::new(Vector3::new(0.0, 1.0, 3.0), 1.0, Arc::new(Metal::new(Vec3::new(0.7, 0.6, 0.5), 0.5)))));
    w.add_obj(Arc::new(Sphere::new(Vector3::new(-2.0, 1.0, 0.0), 1.0, Arc::new(Metal::new(Vec3::new(0.7, 0.6, 0.5), 0.5)))));
    
    //w.add_obj(Arc::new(Sphere::new(Vec3::new(4.0, 1.0, -1.0), 1.0, Arc::new(DiffuseLight::new(Vec3::new(1.0, 1.0, 1.0))))));
    //w.add_obj(Arc::new(Sphere::new(Vec3::new(0.0, 20.0, 0.0), 5.0, Arc::new(DiffuseLight::new(Vec3::new(1.0, 1.0, 1.0))))));

    let min = 0;
    let max = 0;

    for a in min..max { 
        for b in min..max {
            let choose_mat = rng.gen_range(0.0, 1.0);
            let mut center = Vector3::new(a as f64 + 0.9 * rng.gen_range(0.0, 1.0), 0.2, b as f64 + 0.9 * rng.gen_range(0.0, 1.0));
            center = VecUtil::random_in_unit_disk(&mut rng) * 5.0;

            if (center - Vector3::new(4.0, 0.2, 0.0)).norm() as f64 > 0.9 {
                if choose_mat < 0.8 {
                    //diffuse
                    let a = VecUtil::random(0.5, 1.0, &mut rng);
                    let albedo = Vec3::new(a.x as f32, a.y as f32, a.z as f32);
                    
                    let mat = Arc::new(Lambertian::new(albedo));
                    w.add_obj(Arc::new(Sphere::new(center, 0.2, mat)));
                } else if choose_mat < 0.95 {

                    //metal
                    let a = VecUtil::random(0.5, 1.0, &mut rng);
                    let albedo = Vec3::new(a.x as f32, a.y as f32, a.z as f32);
                    let fuzz = rng.gen_range(0.0, 0.5);
                    let mat = Arc::new(Metal::new(albedo, fuzz));
                    w.add_obj(Arc::new(Sphere::new(center, 0.2, mat)));
                } else {
                    //light
                    let mat = Arc::new(Dielectric::new(1.5));
                    //let mat = Arc::new(DiffuseLight::new(Vec3::new(1.0, 1.0, 1.0)));
                    w.add_obj(Arc::new(Sphere::new(center, 0.2, mat)));
                }
            }
        }
        //let p = VecUtil::random_in_unit_sphere(&mut rng).normalize() * 1.52 + Vec3::new(0.0, 0.0, -2.0);
        //w.add_obj(Arc::new(Sphere::new(p, 0.1, Arc::new(Dielectric::new(1.5)))));
        //w.add_obj(Arc::new(Sphere::new(p, -0.095, Arc::new(Dielectric::new(1.5)))));
    }

    let aspect_ratio: f32 = 32.0 / 9.0;
    let image_width: u32 = 1000;
    let image_height: u32 = (image_width as f32 / aspect_ratio) as u32;

    let bounds: (usize, usize) = (image_width as usize, image_height as usize);
    let mut img = vec![Vec3::new(0.0, 0.0, 0.0); bounds.0 * bounds.1];
    let samples_per_pixel: u32 = 200;
    let num_bounces = 20;


    println!("Starting path tracing with dimensions:\n\tWidth: {}\n\tHeight: {}", image_width, image_height);

    let mut pool = Pool::new( 16 );

    let num_rows_per_task: u32 = 5;

    let o = Vector3::<f64>::new(-0.0, 50.0, 25.);
    let at = Vector3::<f64>::new(0.0, 1.0, 0.0);
    let c: Camera = Camera::new(o, at, Vector3::<f64>::new(0.0, 1.0, 0.0), 10.0, image_width as f64 / image_height as f64, 0.0, (o-at).norm());

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
                let mut start = j;
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
                            let u: f64 = (i as f64 + rng.gen_range(0.0, 1.0)) / (image_width as f64 - 1.0);
                            let v: f64 = (j as f64 + rng.gen_range(0.0, 1.0)) / (image_height as f64 - 1.0);
                            let r: Ray = c.get_ray(u, v, &mut rng);
                
                            let c: Vec3 = ray_color(&r, Vec3::new(0.0, 0.0, 0.0), &w, num_bounces, &mut rng);
            
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

        for _ in 0..(((image_width * image_height)) as usize) {
            let p = rx.recv().unwrap();
            img[(p.x + p.y * image_width) as usize] = p.c;
            if last_update.elapsed().as_secs() > 30 {
                write_image("trace.png", &mut img, bounds).unwrap();
                last_update = Instant::now();
            }
        }
    });

    println!("Execution took {} ms", start.elapsed().as_millis());

    write_image("trace.png", &mut img, bounds).unwrap();

    
}

fn ray_color(r: &Ray, background: Vec3, w: &World, depth: u32, rng: &mut rand::prelude::ThreadRng) -> Vec3 {
    if depth <= 0 {
        return Vec3::new(0.0, 0.0, 0.0);
    }

    match w.intersection(r, 0.0001, std::f64::MAX) {
        Some(hit) => {

            match hit.material.scatter(r, &hit, rng) {
                Some(record) => {
                    let a: Vec3 = hit.material.emitted(0.0, 0.0, background) + (record.attenuation * (ray_color(&record.scattered, background, w, depth-1, rng)));
                    return a;
                }

                None => {

                    return hit.material.emitted(0.0, 0.0, background);
                
                    
                }
            }
        }
        None => {
            return background;
        }
    }
}

fn write_image(filename: &str, pixels: &mut [Vec3], bounds: (usize, usize))
    -> Result<(), std::io::Error> {
    //let output = File::create("filename.png")?;

    // Construct a new RGB ImageBuffer with the specified width and height.

    let mut buffer = vec![0u8; bounds.0 * bounds.1 * 3];
    //img.put_pixel(20, 20, image::Rgb([255,0,0]));

    // Finding maximum pixel value so that we can normalize the whole
    // pixel array to not blow out any details
    let mut max_value: f32 = 0.0;

    for y in (0..(bounds.1)).rev() {
        for x in 0..bounds.0 {
            // Making sure values doesn't overflow as we can have lights
            // brighter than 1
            if pixels[x + (bounds.1 - y - 1) * bounds.0].x() > max_value {
                max_value = pixels[x + (bounds.1 - y - 1) * bounds.0].x();
            }

            if pixels[x + (bounds.1 - y - 1) * bounds.0].y() > max_value {
                max_value = pixels[x + (bounds.1 - y - 1) * bounds.0].y();
            }

            if pixels[x + (bounds.1 - y - 1) * bounds.0].z() > max_value {
                max_value = pixels[x + (bounds.1 - y - 1) * bounds.0].z();
            }
        }
    }

    for y in (0..(bounds.1)).rev() {
        for x in 0..bounds.0 {
            // Making sure values doesn't overflow as we can have lights
            // brighter than 1

            buffer[x * 3 + y * 3 * bounds.0] = (pixels[x + (bounds.1 - y - 1) * bounds.0].x() / max_value * 255.0) as u8;
            buffer[x * 3 + y * 3 * bounds.0 + 1] = (pixels[x + (bounds.1 - y - 1) * bounds.0].y() / max_value * 255.0) as u8;
            buffer[x * 3 + y * 3 * bounds.0 + 2] = (pixels[x + (bounds.1 - y - 1) * bounds.0].z() / max_value * 255.0) as u8;
        }
    }
    image::save_buffer(filename, &buffer, bounds.0 as u32, bounds.1 as u32, image::ColorType::Rgb8).unwrap();

    //let encoder = PNGEncoder::new(output);
    //encoder.encode(&pixels,
                    //bounds.0 as u32, bounds.1 as u32,
                    //ColorType::Gray(8))?;


    Ok(())
}
