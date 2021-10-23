use criterion::{black_box, criterion_group, criterion_main, Criterion};
use pathtrace::primitives::*;
use pathtrace::material::*;
use pathtrace::ray::Ray;
use pathtrace::ray::Hit;
use pathtrace::lehmer::Lehmer;
use glam::Vec3;
use std::sync::Arc;
use rand::*;

extern crate nalgebra as na;
use na::{Vector3,RowVector3};

pub fn criterion_benchmark(c: &mut Criterion) {
    c.bench_function("fib 20", |b| b.iter(|| 1 + 1));
}

pub fn sphere(c: &mut Criterion) {
    let s: Sphere = Sphere::new(Vector3::new(2.0, 0.0, 0.0), 1.0, Arc::new(Metal::new(Vec3::new(0.7, 0.6, 0.5), 0.5)));
    let r: Ray = Ray::new(Vector3::new(0.0, 0.0, 0.0), Vector3::new(1.0, 0.0, 0.0));
    c.bench_function("Sphere interesect", |b| b.iter(|| s.intersection(&r, 0.0, std::f64::MAX)));
}

pub fn material(c: &mut Criterion) {
    let metal = Metal::new(Vec3::new(0.7, 0.6, 0.5), 0.5);
    let dielectric = Dielectric::new(1.5);
    let lambertian = Lambertian::new(Vec3::new(1.0, 1.0, 1.0));
    let diffuse_light = DiffuseLight::new(Vec3::new(1.0, 1.0, 1.0));
    let s: Sphere = Sphere::new(Vector3::new(2.0, 0.0, 0.0), 1.0, Arc::new(Metal::new(Vec3::new(0.7, 0.6, 0.5), 0.5)));
    let r: Ray = Ray::new(Vector3::new(0.0, 0.0, 0.0), Vector3::new(1.0, 0.0, 0.0));
    let mut rng = Lehmer::new(1);
    
    match s.intersection(&r, 0.0001, std::f64::MAX) {
        Some(hit) => {
            
            c.bench_function("Metal->scatter", |b| b.iter(|| 
                metal.scatter(&r, &hit, &mut rng)));

            c.bench_function("Dielectric->scatter", |b| b.iter(|| 
                dielectric.scatter(&r, &hit, &mut rng)));

            c.bench_function("Lambertian->scatter", |b| b.iter(|| 
                lambertian.scatter(&r, &hit, &mut rng)));

            c.bench_function("DiffuseLight->scatter", |b| b.iter(|| 
                diffuse_light.scatter(&r, &hit, &mut rng)));
            
        }
        None => {
        }
    }
}

pub fn rng(c: &mut Criterion) {
    let mut r = Lehmer::new(1);
    c.bench_function("lehmerf64", |b| b.iter(|| r.random_float(0.0, 1.0)));

    let mut rng = rand::rngs::SmallRng::from_entropy();
    c.bench_function("smallrng", |b| b.iter(|| rng.gen_range(0.0, 1.0)));
}

criterion_group!(benches, rng, sphere, material);
criterion_main!(benches);