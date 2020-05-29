use criterion::{black_box, criterion_group, criterion_main, Criterion};
use pathtrace::primitives::*;
use pathtrace::material::*;
use pathtrace::ray::Ray;
use glam::Vec3;
use std::sync::Arc;

extern crate nalgebra as na;
use na::{Vector3,RowVector3};

pub fn criterion_benchmark(c: &mut Criterion) {
    c.bench_function("fib 20", |b| b.iter(|| 1 + 1));
}

pub fn sphere_intersect(c: &mut Criterion) {
    let s: Sphere = Sphere::new(Vector3::new(2.0, 0.0, 0.0), 1.0, Arc::new(Metal::new(Vec3::new(0.7, 0.6, 0.5), 0.5)));
    let r: Ray = Ray::new(Vector3::new(0.0, 0.0, 0.0), Vector3::new(1.0, 0.0, 0.0));
    c.bench_function("Sphere interesect", |b| b.iter(|| s.intersection(&r, 0.0, std::f64::MAX)));
}

criterion_group!(benches, sphere_intersect);
criterion_main!(benches);