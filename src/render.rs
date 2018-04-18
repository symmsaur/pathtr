extern crate rand;

use render::rand::{Rng, XorShiftRng};

use std::io::{self, Write};

use math::*;
use scene;

pub fn render(scene: &scene::Scene, camera: &scene::Camera, width: usize, height: usize) -> Vec<u8> {
    let mut buffer = vec![0.0; width * height];

    let n_rays = width*height*200;
    println!("n_rays: {}", n_rays);
    let mut rng = rand::XorShiftRng::new_unseeded();
    for i in 0..n_rays {
        let (x, y, ray) = gen_ray_c(&camera, &mut rng);
        let val = sample(&scene, ray, &mut rng);
        //println!("{}",val);
        let p_x = (width as f64 * x) as usize;
        let p_y = (height as f64 * y) as usize;
        //println!("{}, {}", p_x, p_y);
        // How should we add upp the samples...
        buffer[width * p_y + p_x] += val;
        if i % 1_000_000 == 0 {
            print!("\r{:.0}%", i as f64 / n_rays as f64 * 100.0);
            io::stdout().flush().unwrap();
        }
    }

    // Automatic exposure control
    //let factor = compute_gain(&buffer);

    let mut img_buffer = vec![0; width * height * 4];
    let mut i = 0;
    for val in buffer {
        //println!("Value: {}", val);
        let img_val = val as u8;
        img_buffer[i] = img_val;
        img_buffer[i+1] = img_val;
        img_buffer[i+2] = img_val;
        img_buffer[i+3] = 255;
        i += 4;
    }

    println!("\r100%");
    return img_buffer;
}

//fn compute_gain(buffer: &Vec<f64>) -> f64 {
//}

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

    let mut intersections = Vec::new();
    for obj in scene.objs.iter() {
        let ix = obj.intersect(&ray);
        match ix {
            Some(ix) => {intersections.push(ix);}
            None => {}
        }
    }
    if intersections.len() == 0 {
        return None;
    }
    let mut min_ix = intersections[0];
    for ix in intersections {
        let (_, _, d) = ix;
        let (_, _, min_d) = min_ix;
        if d < min_d {
            min_ix = ix;
        }
    }
    return Some(min_ix);
}

fn gen_ray_n(start: Point, normal: Vector, rng: &mut XorShiftRng) -> Ray
{
    // uniform sample over half sphere
    loop {
        let x = 2.0 * rng.gen::<f64>() - 1.0;
        let y = 2.0 * rng.gen::<f64>() - 1.0;
        let z = 2.0 * rng.gen::<f64>() - 1.0;
        let v = Vector {x, y, z};
        if v.square_length() < 1.0 && dot(v, normal) > 0.0 {
            return Ray{origin: start, direction: v}
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
