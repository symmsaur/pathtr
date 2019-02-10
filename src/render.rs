extern crate rand;

use render::rand::{Rng, XorShiftRng};

use std::io::{self, Write};
use std::sync::{mpsc, Arc};
use std::thread;

use math::*;
use scene;
use material;

const THREADS: i64 = 4;

fn start_render_thread(scene: &Arc<scene::Scene>, camera: &Arc<scene::Camera>, tx: &mpsc::Sender<Vec<f64>>, width: usize, height: usize, n: i64) {
    let my_scene = Arc::clone(&scene);
    let my_camera = Arc::clone(&camera);
    let my_tx = mpsc::Sender::clone(&tx);
    thread::spawn(move || {
        let mut buffer = vec![0.0; width * height];
        let mut rng = rand::XorShiftRng::new_unseeded();
        for i in 0..n {
            let (x, y, ray) = gen_ray_c(&my_camera, &mut rng);
            let val = sample(&my_scene, ray, &mut rng);
            let p_x = (width as f64 * x) as usize;
            let p_y = (height as f64 * y) as usize;
            buffer[width * p_y + p_x] += val;
            if i % 1_000_000 == 0 {
                print!(".");
                io::stdout().flush().unwrap();
            }
        }
        my_tx.send(buffer).unwrap();
    });
}

pub fn render(scene: Arc<scene::Scene>, camera: Arc<scene::Camera>, width: usize, height: usize, n: i64)
              -> Vec<u8> {
    let (tx, rx) = mpsc::channel();
    println!("Running on {} cores", THREADS);
    println!("Total {} rays", n);
    for _i in 0..THREADS {
        start_render_thread(&scene, &camera, &tx, width, height, n / THREADS);
    }

    drop(tx);

    let mut accumulator = vec![0.0; width * height];
    for buffer in rx {
        for (i, val) in buffer.iter().enumerate() {
            accumulator[i] += val;
        }
    }

    let factor = compute_gain(&accumulator);

    let mut img_buffer = vec![0; width * height * 4];
    let mut i = 0;
    for val in accumulator {
        //println!("Value: {}", val);
        let img_val = (val * factor) as u8;
        img_buffer[i] = img_val;
        img_buffer[i+1] = img_val;
        img_buffer[i+2] = img_val;
        img_buffer[i+3] = 255;
        i += 4;
    }

    println!("\r100%");
    return img_buffer;
}


fn compute_gain(buffer: &Vec<f64>) -> f64 {
    let mut max = 0.;
    for &val in buffer {
        if val > max {
            max = val
        }
    }
    return 255. * 1./max
}

fn sample(scene: &scene::Scene, initial_ray: Ray, rng: &mut XorShiftRng) -> f64 {
    let mut ray = material::ElRay {
        ray: initial_ray,
        light: 1.,
        ior: 1.,
        count: 0,
    };
    //println!("new sample");
    loop {
        match shoot_ray(&scene, &ray.ray) {
            Some((o, p, n, _)) => {
                ray = o.material.new_ray(ray, p, n, rng);
            }
            None => {
                return ray.light;
            }
        }
        if ray.count > 10 {
            return ray.light;
        }
    }
}

fn shoot_ray<'a>(scene: &'a scene::Scene, ray: &Ray) -> Option<(&'a scene::Object, Point, Vector, f64)> {
    let mut closest_intersection: Option<(&'a scene:: Object, Point, Vector, f64)> = None;
    for obj in scene.objs.iter() {
        let new_intersection = obj.shape.intersect(&ray);
        match new_intersection {
            Some((p, n, t)) => {
                match closest_intersection {
                    Some((_, _, _, t_old)) => {
                        if t < t_old {
                            closest_intersection = Some((obj, p, n, t));
                        }
                    }
                    None => {
                        closest_intersection = Some((obj, p, n, t));
                    }
                }
            }
            None => {}
        }
    }
    return closest_intersection;
}

fn gen_ray_c(cam: &scene::Camera, rng: &mut XorShiftRng) -> (f64, f64, Ray) {
    let origin = cam.look_from;
    let p_orig = translate(origin, cam.direction);
    let left = cross(cam.up, cam.direction);
    let lr_range = (cam.fov / 2.0).tan();
    let ud_range = lr_range / cam.aspect;
    let param_x = 2.0 * rng.gen::<f64>();
    let p_x = lr_range * (-1.0 + param_x);
    //println!{"p_x {}", p_x};
    let param_y = 2.0 * rng.gen::<f64>();
    // Screen y goes from top to bottom
    let p_y = ud_range * (1.0 - param_y);
    //println!{"p_y {}", p_y};
    let p_disp = p_y * cam.up + p_x * left;
    let through = translate(p_orig, p_disp);
    (param_x / 2.0, param_y / 2.0, Ray::create(origin, through))
}
