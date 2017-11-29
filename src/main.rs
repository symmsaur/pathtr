extern crate rand;
extern crate image;

mod math;
mod trace;

use math::*;
use std::path::Path;

fn main() {
    let cam = Camera {
        look_from: math::Point {x: 0.0, y: 1.0, z: -10.0},
        direction: math::Vector {x: 0.0, y: 0.0, z: 1.0},
        up: math::Vector {x:0.0, y: 1.0, z: 1.0},
        fov: 1.0,
        aspect: 1.0,
    };
    let mut scene = Scene::new();
    let p1 = math::Plane {
        point: Point {x: 0.0, y: 0.0, z: 0.0},
        normal: Vector {x: 0.0, y: 0.0, z: 1.0},
    };
    scene.objs.push(Box::new(p1));
    let p2 = math::Sphere {
        center: Point {x: 10.0, y: 0.0, z: 5.0},
        radius: 1.0,
    };
    scene.objs.push(Box::new(p2));
    const width: usize = 200;
    const height: usize = 200;
    let mut buffer = vec![0.0; width * height];

    let max = width*height*100;
    for i in 0..max {
        let (x, y, ray) = generate_ray(&cam);
        let val = shoot_ray(&scene, ray);
        //println!("{}",val);
        let p_x = (width as f64 * x) as usize;
        let p_y = (height as f64 * y) as usize;
        //println!("{}, {}", p_x, p_y);
        // How should we add upp the samples...
        buffer[width * p_y + p_x] += val;
        if i % 100_000 == 0 {
            println!("{}%", i as f64 / max as f64 * 100.0);
        }
    }

    let mut img_buffer = [0; width * height * 4];
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
    image::save_buffer(&Path::new("image.png"), &img_buffer,
                       width as u32, height as u32, image::RGBA(8));

}

struct Scene {
    objs: Vec<Box<Intersectable>>,
}

impl Scene {
    fn new() -> Scene {
        Scene {objs: Vec::new()}
    }
}

struct Camera {
    look_from: math::Point,
    direction: math::Vector,
    up: math::Vector,
    fov: f64,
    aspect: f64,
}

fn shoot_ray(scene: &Scene, ray: Ray) -> f64 {
    let mut ret = 0.0;
    for obj in scene.objs.iter() {
        match obj.intersect(&ray) {
            Some(_) => ret += 1.0,
            None => ret += 0.0,
        }
    }
    ret
}

fn generate_ray(cam: &Camera) -> (f64, f64,math::Ray) {
    let origin = cam.look_from;
    let p_orig = math::translate(origin, cam.direction);
    let up = cam.up;
    let left = math::cross(up, cam.direction);
    let lr_range = (cam.fov / 2.0).tan();
    let ud_range = lr_range / cam.aspect;
    let param_x = 2.0 * rand::random::<f64>();
    let p_x = lr_range * (-1.0 + param_x);
    //println!{"p_x {}", p_x};
    let param_y = 2.0 * rand::random::<f64>();
    let p_y = ud_range * (-1.0 + param_y);
    //println!{"p_y {}", p_y};
    let p_disp = p_y * up + p_x * left;
    let through = math::translate(p_orig, p_disp);
    (param_x / 2.0, param_y / 2.0, math::Ray::create(origin, through))
}
