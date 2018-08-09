extern crate rand;

use render::rand::{Rng, XorShiftRng};

use std::io::{self, Write};
use std::sync::{mpsc, Arc};
use std::thread;

use math::*;
use scene;

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

fn sample(scene: &scene::Scene, initial_ray: Ray, mut rng: &mut XorShiftRng) -> f64 {
    let mut ray = initial_ray;
    let mut res = 1.0;
    loop {
        match shoot_ray(&scene, &ray) {
            Some((p, n, _)) => {
                // decay light by 50% on each bounce
                res *= 0.5;
                ray = gen_ray_n(p, n, &mut rng);
            }
            None => {
                return res;
            }
        }
    }
}

fn shoot_ray(scene: &scene::Scene, ray: &Ray) -> Option<(Point, Vector, f64)> {
    let mut intersection: Option<(Point, Vector, f64)> = None;
    for obj in scene.objs.iter() {
        let new_intersection = obj.intersect(&ray);
        match new_intersection {
            Some(new_ix) => {
                match intersection {
                    Some(ix) => {
                        if new_ix.2 < ix.2 {
                            intersection = new_intersection;
                        }
                    }
                    None => {
                        intersection = new_intersection;
                    }
                }
            }
            None => {}
        }
    }
    return intersection;
}

fn gen_ray_n(start: Point, normal: Vector, rng: &mut XorShiftRng) -> Ray
{
    // uniform sample over half sphere
    loop {
        let x = 2.0 * rng.gen::<f64>() - 1.0;
        let y = 2.0 * rng.gen::<f64>() - 1.0;
        let z = 2.0 * rng.gen::<f64>() - 1.0;
        let v = Vector {x, y, z};
        if v.square_length() < 1.0 {
            if dot(v, normal) > 0.0 {
                return Ray{origin: start, direction: v}
            }
            else {
                return Ray{origin: start, direction: -v}
            }
        }
    }
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
