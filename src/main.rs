use std::io::{stderr, Write};

use raytracing::{color, Color};
use raytracing::{lerp, remap};
use raytracing::{point3, Point3};
use raytracing::{x, y, z};
use raytracing::hit::Hittable;
use raytracing::Ray;
use raytracing::sphere::Sphere;
use raytracing::white;
use raytracing::world::World;

// IMAGE
static ASPECT_RATIO: f64 = 16.0 / 9.0;
static IMAGE_WIDTH: u64 = 256;
static IMAGE_HEIGHT: u64 = (IMAGE_WIDTH as f64 / ASPECT_RATIO) as u64;

// CAMERA
static VIEWPORT_HEIGHT: f64 = 2.0;
static VIEWPORT_WIDTH: f64 = VIEWPORT_HEIGHT * ASPECT_RATIO;
static FOCAL_LENGTH: f64 = 1.0;
static ORIGIN: &Point3 = &point3![0];
static HORIZONTAL: &Point3 = &x!(VIEWPORT_WIDTH);
static VERTICAL: &Point3 = &y!(VIEWPORT_HEIGHT);
static LOWER_LEFT_CORNER: &Point3 = &point3![
    ORIGIN.0[0] - HORIZONTAL.0[0] / 2.0 - VERTICAL.0[0] / 2.0 + z!(FOCAL_LENGTH).0[0],
    ORIGIN.0[1] - HORIZONTAL.0[1] / 2.0 - VERTICAL.0[1] / 2.0 + z!(FOCAL_LENGTH).0[1],
    ORIGIN.0[2] - HORIZONTAL.0[2] / 2.0 - VERTICAL.0[2] / 2.0 + z!(FOCAL_LENGTH).0[2]
];
// &(ORIGIN - HORIZONTAL / 2.0 - VERTICAL / 2.0 + z!(FOCAL_LENGTH));

fn main() {
    // World
    let world: Vec<Box<dyn Hittable>> = vec![
        Box::new(Sphere::new(Point3::new(0.0, 0.0, -1.0), 0.5)),
        Box::new(Sphere::new(Point3::new(0.0, -100.5, -1.0), 100.0)),
    ];

    println!("P3");
    println!("{} {}", IMAGE_WIDTH, IMAGE_HEIGHT);
    println!("255");

    for j in (0..IMAGE_HEIGHT).rev() {
        eprintln!("Scanlines remaining: {}", j);
        stderr().flush().unwrap();
        for i in 0..IMAGE_WIDTH {
            let u = (i as f64) / ((IMAGE_WIDTH - 1) as f64);
            let v = (j as f64) / ((IMAGE_HEIGHT - 1) as f64);

            let r = Ray::new(
                ORIGIN.clone(),
                LOWER_LEFT_CORNER + u * HORIZONTAL + v * VERTICAL - ORIGIN,
            );

            let pixel_color = ray_color(&r, &world);

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
