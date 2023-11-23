use std::io::{stderr, Write};

use rand::Rng;

use raytracing::camera::Camera;
use raytracing::config::{IMAGE_HEIGHT, IMAGE_WIDTH, SAMPLES_PER_PIXEL};
use raytracing::hit::Hittable;
use raytracing::sphere::Sphere;
use raytracing::white;
use raytracing::world::World;
use raytracing::Point3;
use raytracing::Ray;
use raytracing::{color, Color};
use raytracing::{lerp, remap};

fn main() {
    // World
    let world: Vec<Box<dyn Hittable>> = vec![
        Box::new(Sphere::new(Point3::new(0.0, 0.0, -1.0), 0.5)),
        Box::new(Sphere::new(Point3::new(0.0, -100.5, -1.0), 100.0)),
    ];

    // Camera
    let camera = Camera::new();

    // RNG
    let mut rng = rand::thread_rng();

    println!("P3");
    println!("{} {}", IMAGE_WIDTH, IMAGE_HEIGHT);
    println!("255");

    for j in (0..IMAGE_HEIGHT).rev() {
        eprintln!("Scanlines remaining: {}", j);
        stderr().flush().unwrap();

        for i in 0..IMAGE_WIDTH {
            // let u = (i as f64) / ((IMAGE_WIDTH - 1) as f64);
            // let v = (j as f64) / ((IMAGE_HEIGHT - 1) as f64);
            //
            // let r = Ray::new(
            //     ORIGIN.clone(),
            //     LOWER_LEFT_CORNER + u * HORIZONTAL + v * VERTICAL - ORIGIN,
            // );
            //
            // let pixel_color = ray_color(&r, &world);
            //
            // println!("{}", pixel_color.fmt_color());

            let mut pixel_color = color![0];
            for _ in 0..SAMPLES_PER_PIXEL {
                let random_u: f64 = rng.gen();
                let random_v: f64 = rng.gen();

                let u = ((i as f64) + random_u) / ((IMAGE_WIDTH - 1) as f64);
                let v = ((j as f64) + random_v) / ((IMAGE_HEIGHT - 1) as f64);

                let ray = camera.get_ray(u, v);
                pixel_color += ray_color(&ray, &world);
            }

            println!("{}", pixel_color.fmt_color());
        }
    }

    eprintln!("\x07Done");
}

pub fn hit_circle(center: &Point3, radius: f64, r: &Ray) -> Option<f64> {
    let oc = r.get_origin() - center.clone();
    let r_direction = r.get_direction_denormalized();

    let a = r_direction.length().powi(2);
    let half_b = oc.dot(&r_direction);
    let c = oc.length().powi(2) - radius.powi(2);

    let discriminant = half_b.powi(2) - a * c;

    if discriminant < 0.0 {
        None
    } else {
        Some((-half_b - discriminant.sqrt()) / a)
    }
}

pub fn ray_color(ray: &Ray, world: &World) -> Color {
    if let Some(record) = world.hit(ray, 0.0, f64::INFINITY) {
        remap!(value: record.normal.unwrap(), from: -1_f64, 1_f64, to: 0_f64, 1_f64)
    } else {
        let direction = ray.get_direction();
        let t = remap!(value: direction.y(), from: -1_f64, 1_f64, to: 0_f64, 1_f64);

        lerp!(white!(), t, color![0.5, 0.7, 1])
    }
}
