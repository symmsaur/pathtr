extern crate rand_xorshift;
use rand::prelude::*;
use rand_xorshift::XorShiftRng;

use std::io::{self, Write};
use std::sync::{mpsc, Arc};

use crate::bbox;
use crate::material;
use crate::math::*;
use crate::preview;
use crate::scene;
use crate::trace;

const THREADS: i64 = 4;

fn start_render_job(
    pool: &threadpool::ThreadPool,
    scene: &Arc<bbox::BoundingBoxTree>,
    camera: &Arc<scene::Camera>,
    tx: &mpsc::Sender<Vec<material::Color>>,
    width: usize,
    height: usize,
    rays_per_pixel: i64,
) {
    let my_scene = Arc::clone(&scene);
    let my_camera = Arc::clone(&camera);
    let my_tx = mpsc::Sender::clone(&tx);
    pool.execute(move || {
        let mut buffer = vec![
            material::Color {
                red: 0.0,
                green: 0.0,
                blue: 0.0
            };
            width * height
        ];
        let mut rng = rand_xorshift::XorShiftRng::from_entropy();
        for y in 0..height {
            for x in 0..width {
                for _ in 0..rays_per_pixel {
                    let ray = generate_camera_ray(&my_camera, &mut rng, x, y, width, height);
                    let val = sample(&my_scene, ray, &mut rng);
                    buffer[width * y + x] += val;
                }
            }
        }
        // The receiver may have shut down and then we send the data into the void.
        let _ = my_tx.send(buffer);
    });
}

pub fn render(
    preview_window: &Option<preview::Preview>,
    scene: Arc<bbox::BoundingBoxTree>,
    camera: Arc<scene::Camera>,
    width: usize,
    height: usize,
    rays_per_pixel: i64,
) -> Vec<u8> {
    let num_jobs = rays_per_pixel;
    let (tx, rx) = mpsc::channel();
    let pool = threadpool::ThreadPool::new(THREADS as usize);
    println!("Running on {} cores", THREADS);
    println!("Spawning {} jobs", num_jobs);
    for _i in 0..num_jobs {
        start_render_job(
            &pool,
            &scene,
            &camera,
            &tx,
            width,
            height,
            rays_per_pixel / num_jobs,
        );
    }
    drop(tx);

    let mut accumulator = vec![
        material::Color {
            red: 0.0,
            green: 0.0,
            blue: 0.0
        };
        width * height
    ];
    let mut img_buffer = vec![0; width * height * 4];
    let mut finished_jobs = 0;
    for buffer in rx {
        finished_jobs += 1;
        print!("\r{:.2}%", 100. * finished_jobs as f32 / num_jobs as f32);
        io::stdout().flush().unwrap();
        for (i, val) in buffer.iter().enumerate() {
            accumulator[i] += *val;
        }
        let factor = compute_gain(&accumulator);

        let mut i = 0;
        for val in accumulator.iter() {
            img_buffer[i + 0] = (val.red * factor) as u8;
            img_buffer[i + 1] = (val.green * factor) as u8;
            img_buffer[i + 2] = (val.blue * factor) as u8;
            img_buffer[i + 3] = 255;
            i += 4;
        }

        if let Some(p) = preview_window {
            let res = p.submit_image(&img_buffer);
            match res {
                Err(_) => {
                    println!();
                    println!("Stopped, outputting image...");
                    break;
                }
                _ => {}
            }
        }
    }
    println!();
    img_buffer
}

fn compute_gain(buffer: &Vec<material::Color>) -> f32 {
    let mut max = 0.;
    for &val in buffer {
        if val.red > max {
            max = val.red;
        }
        if val.green > max {
            max = val.green;
        }
        if val.blue > max {
            max = val.blue;
        }
    }
    return 255. / max;
}

fn sample(
    tree: &bbox::BoundingBoxTree,
    initial_ray: Ray,
    rng: &mut XorShiftRng,
) -> material::Color {
    let mut ray = material::LightRay {
        ray: initial_ray,
        light: material::Color {
            red: 1.0,
            green: 1.0,
            blue: 1.0,
        },
        ior: 1.,
        count: 0,
        done: false,
    };
    loop {
        match trace::shoot_ray(&tree, &ray.ray) {
            Some((o, p, n, _, i)) => {
                ray = o.material.new_ray(ray, p, n, i, rng);
            }
            None => {
                return material::Color {
                    red: 0.0,
                    green: 0.0,
                    blue: 0.0,
                };
            }
        }
        if ray.done || ray.count > 100 {
            return ray.light;
        }
    }
}

fn generate_camera_ray(
    cam: &scene::Camera,
    rng: &mut XorShiftRng,
    x: usize,
    y: usize,
    width: usize,
    height: usize,
) -> Ray {
    let origin = cam.look_from;
    let right = cross(cam.direction, cam.up).normalize();
    let down = cross(cam.direction, right).normalize();

    let x_range = (cam.fov / 2.0).tan();
    let y_range = x_range / cam.aspect;
    // Goes from -1 to 1
    let param_x = 2.0 * ((x as f32 / width as f32) + (1. / width as f32) * rng.gen::<f32>()) - 1.0;
    let param_y =
        2.0 * ((y as f32 / height as f32) + (1. / height as f32) * rng.gen::<f32>()) - 1.0;

    let p_x = x_range * param_x;
    let p_y = y_range * param_y;

    let p_disp = p_y * down + p_x * right;
    let p_orig = translate(origin, cam.direction);
    let through_screen = translate(p_orig, p_disp);
    let displacement = through_screen - origin;
    let through = translate(origin, cam.focal_distance * displacement);

    // perturb ray
    let mut perturbation_param_x = 2.0;
    let mut perturbation_param_y = 2.0;
    while perturbation_param_x * perturbation_param_x + perturbation_param_y * perturbation_param_y
        > 1.0
    {
        perturbation_param_x = 2.0 * rng.gen::<f32>() - 1.0;
        perturbation_param_y = 2.0 * rng.gen::<f32>() - 1.0;
    }
    let perturbation_x = perturbation_param_x * cam.aperture;
    let perturbation_y = perturbation_param_y * cam.aperture;

    let perturbed_origin = translate(origin, (perturbation_x * right) + (perturbation_y * down));

    Ray::create(perturbed_origin, through)
}
