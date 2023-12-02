use std::fs;
use std::sync::{mpsc, Arc};
use std::time::Instant;

use indicatif::ParallelProgressIterator;
use rayon::prelude::*;

use raytracing::config::*;
use raytracing::prelude::*;
use raytracing::time_it;
use raytracing::{color, point3, remap};

fn main() {
    let start = Instant::now();

    dbg!(cfg!(debug_assertions));
    dbg!(HI_RES);

    // World
    let world: World = Arc::new(vec![
        Arc::new(Sphere::new(
            point3!(0.0, 0.0, -1.0),
            0.5,
            CENTER_SPHERE_MATERIAL.clone(),
        )),
        Arc::new(Sphere::new(
            point3!(0.0, -100.5, -1.0),
            100.0,
            GROUND_MATERIAL.clone(),
        )),
        Arc::new(Sphere::new(
            point3!(-1, 0, -1),
            0.5,
            LEFT_SPHERE_MATERIAL.clone(),
        )),
        Arc::new(Sphere::new(
            point3!(1, 0, -1),
            0.5,
            RIGHT_SPHERE_MATERIAL.clone(),
        )),
    ]);

    // Camera
    let camera = Camera::new();

    let output = vec![
        "P3".to_string(),
        format!("{} {}", IMAGE_WIDTH, IMAGE_HEIGHT),
        "255".to_string(),
    ];

    let scanlines = (0..IMAGE_HEIGHT).rev().collect::<Vec<_>>();
    let scanlines_len = scanlines.len() as u64;

    let (sender, receiver) = mpsc::channel();

    let square_side_f64 = (SAMPLES_PER_PIXEL as f64).sqrt();
    let square_side_u64 = square_side_f64 as u64;
    if square_side_f64.floor() as u64 != square_side_u64 {
        eprintln!("SAMPLES_PER_PIXEL must be perfect square");
        std::process::exit(-1);
    }

    time_it!(at once | "rendering" => scanlines
        .into_par_iter()
        .progress_with(get_pb(scanlines_len, "rendering"))
        .for_each(|j| {
            (0..IMAGE_WIDTH).into_par_iter().for_each(|i| {
                let mut pixel_color = color!(0);
                for idx in 0..SAMPLES_PER_PIXEL {
                    let idx = idx + 1;
                    let random_u: f64 = remap!(
                        value: (idx % square_side_u64) as f64,
                        from: 0.0, square_side_f64,
                        to: 0.0, 1.0
                    );
                    let random_v: f64 = square_side_f64 / (idx as f64);

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

    let output_pixels = time_it!("collecting pixels" =>
        pixels.clone().into_par_iter().map(|(_, pixel)| pixel.fmt_color().to_string())
    );

    time_it!("saving to png" => {
        use image::*;
        let rgb_pixels = pixels
            .clone()
            .iter()
            .map(|(_, it)| {
                let it = it.fmt_u8();
                Rgb::from(it)
            })
            .collect::<Vec<_>>();
        let img = ImageBuffer::from_fn(IMAGE_WIDTH as u32, IMAGE_HEIGHT as u32, |w, h| {
            rgb_pixels[(h * IMAGE_WIDTH as u32 + w) as usize]
        });
        img.save(RENDER_FILE).unwrap();
    });

    let output = time_it!("collecting output" =>
        output.into_par_iter().chain(output_pixels).collect::<Vec<String>>()
    );

    println!("writing to: {}", OUTPUT_FILE);

    if fs::metadata(OUTPUT_FILE).is_ok() {
        fs::remove_file(OUTPUT_FILE).unwrap();
    }

    fs::write(OUTPUT_FILE, output.join("\n")).unwrap();

    println!("\x07Done, whole program took {:?}", start.elapsed());
}
