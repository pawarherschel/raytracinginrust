use std::fs;
use std::sync::{mpsc, Arc, RwLock};

use indicatif::ParallelProgressIterator;
use rand::Rng;
use rayon::prelude::*;

use raytracing::config::*;
use raytracing::util::*;
use raytracing::*;

fn main() {
    dbg!(HI_RES);

    // World
    let world: World = Arc::new(vec![
        Arc::new(RwLock::new(Sphere::new(
            point3!(0.0, 0.0, -1.0),
            0.5,
            CENTER_SPHERE_MATERIAL.clone(),
        ))),
        Arc::new(RwLock::new(Sphere::new(
            point3!(0.0, -100.5, -1.0),
            100.0,
            GROUND_MATERIAL.clone(),
        ))),
        Arc::new(RwLock::new(Sphere::new(
            point3!(-1, 0, -1),
            0.5,
            LEFT_SPHERE_MATERIAL.clone(),
        ))),
        Arc::new(RwLock::new(Sphere::new(
            point3!(1, 0, -1),
            0.5,
            RIGHT_SPHERE_MATERIAL.clone(),
        ))),
    ]);

    // Camera
    let camera = Camera::new();

    let mut output = vec![
        "P3".to_string(),
        format!("{} {}", IMAGE_WIDTH, IMAGE_HEIGHT),
        "255".to_string(),
    ];

    let scanlines = (0..IMAGE_HEIGHT).rev().collect::<Vec<_>>();
    let scanlines_len = scanlines.len() as u64;

    let (sender, receiver) = mpsc::channel();

    time_it!(at once | "rendering" => scanlines
        .into_par_iter()
        .progress_with(get_pb(scanlines_len, "rendering"))
        .for_each(|j| {
            (0..IMAGE_WIDTH).into_par_iter().for_each(|i| {
                let mut rng = rand::thread_rng();

                let mut pixel_color = color!(0);
                for _ in 0..SAMPLES_PER_PIXEL {
                    let random_u: f64 = rng.gen();
                    let random_v: f64 = rng.gen();

                    let u = ((i as f64) + random_u) / ((IMAGE_WIDTH - 1) as f64);
                    let v = ((j as f64) + random_v) / ((IMAGE_HEIGHT - 1) as f64);

                    let ray = camera.get_ray(u, v);

                    pixel_color += ray.color(&world, MAX_DEPTH);
                }

                sender.send((j * IMAGE_WIDTH + i, pixel_color)).unwrap();
            })
        })
    );

    let mut pixels = Vec::with_capacity((IMAGE_WIDTH * IMAGE_HEIGHT) as usize);

    time_it!(at once | "receiving pixels from various threads" => for _ in 0..IMAGE_WIDTH * IMAGE_HEIGHT {
        pixels.push(receiver.recv().unwrap());
    });

    time_it!("sorting pixels" => pixels.par_sort_by(|(idx1, _), (idx2, _)| idx2.cmp(idx1)));

    for (_, pixel) in pixels {
        output.push(pixel.fmt_color().to_string());
    }

    let output_file = if HI_RES { "hi_res.ppm" } else { "img.ppm" };

    println!("output: {}", output_file);

    if fs::metadata(output_file).is_ok() {
        fs::remove_file(output_file).unwrap();
    }

    fs::write(output_file, output.join("\n")).unwrap();

    println!("\x07Done");
}
