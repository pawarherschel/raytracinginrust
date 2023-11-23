use std::io::{stderr, Write};

use raytracing::lerp;
use raytracing::Ray;
use raytracing::{color, Color};
use raytracing::{point3, Point3};
use raytracing::{vec3, Vec3};

static ASPECT_RATIO: f64 = 16.0 / 9.0;
static IMAGE_WIDTH: u64 = 256;
static IMAGE_HEIGHT: u64 = (IMAGE_WIDTH as f64 / ASPECT_RATIO) as u64;
static VIEWPORT_HEIGHT: f64 = 2.0;
static VIEWPORT_WIDTH: f64 = VIEWPORT_HEIGHT * ASPECT_RATIO;
static FOCAL_LENGTH: f64 = 1.0;

fn main() {
    println!("P3");
    println!("{} {}", IMAGE_WIDTH, IMAGE_HEIGHT);
    println!("255");

    let origin = point3![0];
    let horizontal = vec3![VIEWPORT_WIDTH, 0, 0];
    let vertical = vec3![0, VIEWPORT_HEIGHT, 0];
    let lower_left_corner = origin.clone()
        - horizontal.clone() / 2.0
        - vertical.clone() / 2.0
        - vec3![0, 0, FOCAL_LENGTH];

    for j in (0..IMAGE_HEIGHT).rev() {
        eprintln!("Scanlines remaining: {}", j);
        stderr().flush().unwrap();
        for i in 0..IMAGE_WIDTH {
            let u = (i as f64) / ((IMAGE_WIDTH - 1) as f64);
            let v = (j as f64) / ((IMAGE_HEIGHT - 1) as f64);

            let r = Ray::new(
                origin.clone(),
                lower_left_corner.clone() + u * horizontal.clone() + v * vertical.clone()
                    - origin.clone(),
            );

            let pixel_color = ray_color(&r);

            println!("{}", pixel_color.fmt_color());
        }
    }

    eprintln!("\x07Done");
}

pub fn hit_circle(center: &Point3, radius: f64, r: &Ray) -> bool {
    let oc = r.get_origin() - center.clone();
    let r_direction = r.get_direction();

    let a = r_direction.dot(&r_direction);
    let b = 2.0 * oc.dot(&r_direction);
    let c = oc.dot(&oc) - radius * radius;

    let discriminant = b * b - 4.0 * a * c;

    discriminant > 0.0
}

pub fn ray_color(r: &Ray) -> Color {
    if hit_circle(&point3!(0, 0, -1), 0.5, r) {
        return color![1, 0, 0];
    }

    let direction = r.get_direction();
    let t = 0.5 * (direction.y() + 1.0);

    lerp!(color![1], t, color![0.5, 0.7, 1])
}
