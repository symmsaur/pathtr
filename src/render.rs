extern crate rand_xorshift;
extern crate threadpool;

use rand::prelude::*;
use render::rand_xorshift::XorShiftRng;

use std::io::{self, Write};
use std::sync::{mpsc, Arc};

use material;
use math::*;
use preview;
use scene;

const THREADS: i64 = 4;

fn start_render_job(
    pool: &threadpool::ThreadPool,
    scene: &Arc<scene::Scene>,
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
                    let ray = gen_ray_c(&my_camera, &mut rng, x, y, width, height);
                    let val = sample(&my_scene, ray, &mut rng);
                    buffer[width * y + x] += val;
                }
            }
        }
        // TODO: After shut down avoid failing here.
        my_tx.send(buffer).unwrap();
    });
}

pub fn render(
    preview_window: &preview::Preview,
    scene: Arc<scene::Scene>,
    camera: Arc<scene::Camera>,
    width: usize,
    height: usize,
    rays_per_pixel: i64,
) -> Vec<u8> {
    let num_jobs = rays_per_pixel / 10;
    let (tx, rx) = mpsc::channel();
    let pool = threadpool::ThreadPool::new(THREADS as usize);
    println!("Running on {} cores", THREADS);
    println!("Spawining {} jobs", num_jobs);
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
        print!("\r{:.2}%", 100. * finished_jobs as f64 / num_jobs as f64);
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

        let res = preview_window.submit_image(&img_buffer);
        match res {
            Err(_) => {
                println!();
                println!("Stopped, outputting image...");
                break;
            }
            _ => {}
        }
    }
    println!();
    img_buffer
}

fn compute_gain(buffer: &Vec<material::Color>) -> f64 {
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

fn sample(scene: &scene::Scene, initial_ray: Ray, rng: &mut XorShiftRng) -> material::Color {
    let mut ray = material::ElRay {
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
        match shoot_ray(&scene, &ray.ray) {
            Some((o, p, n, _)) => {
                ray = o.material.new_ray(ray, p, n, rng);
            }
            None => {
                return ray.light;
            }
        }
        if ray.done || ray.count > 100 {
            return ray.light;
        }
    }
}

fn shoot_ray<'a>(
    scene: &'a scene::Scene,
    ray: &Ray,
) -> Option<(&'a scene::Object, Point, Vector, f64)> {
    let mut closest_intersection: Option<(&'a scene::Object, Point, Vector, f64)> = None;
    for obj in scene.objs.iter() {
        let new_intersection = obj.shape.intersect(&ray);
        match new_intersection {
            Some((p, n, t)) => match closest_intersection {
                Some((_, _, _, t_old)) => {
                    if t < t_old {
                        closest_intersection = Some((obj, p, n, t));
                    }
                }
                None => {
                    closest_intersection = Some((obj, p, n, t));
                }
            },
            None => {}
        }
    }
    return closest_intersection;
}

fn gen_ray_c(
    cam: &scene::Camera,
    rng: &mut XorShiftRng,
    x: usize,
    y: usize,
    width: usize,
    height: usize,
) -> Ray {
    let origin = cam.look_from;
    let p_orig = translate(origin, cam.direction);
    let left = cross(cam.up, cam.direction);
    let lr_range = (cam.fov / 2.0).tan();
    let ud_range = lr_range / cam.aspect;

    let param_x = 2.0 * ((x as f64 / width as f64) + (1. / width as f64) * rng.gen::<f64>());
    let param_y = 2.0 * ((y as f64 / height as f64) + (1. / height as f64) * rng.gen::<f64>());

    let p_x = lr_range * (-1.0 + param_x);
    // Screen y goes from top to bottom
    let p_y = ud_range * (1.0 - param_y);

    let p_disp = p_y * cam.up + p_x * left;
    let through = translate(p_orig, p_disp);
    Ray::create(origin, through)
}
