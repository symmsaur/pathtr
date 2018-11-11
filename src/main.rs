extern crate image;
extern crate time;

mod material;
mod math;
mod render;
mod scene;

use math::*;
use std::path::Path;
use std::sync::Arc;
use time::PreciseTime;

const WIDTH: usize = 200;
const HEIGHT: usize = 200;
const N_RAYS: i64 = (WIDTH * HEIGHT * 20000) as i64;

fn main() {
    let camera = Arc::new(scene::Camera {
        look_from: Point {x: -12.0, y: -8.0, z: 3.0},
        direction: (Vector {x: 10.0, y: 6.75, z: -2.0}).normalize(),
        up: Vector {x:0.0, y: 0.0, z: 1.0},
        fov: 3.14/8.0,
        aspect: 1.0,
    });

    let scene = Arc::new(prep_scene());

    let width = WIDTH;
    let height = HEIGHT;
    let n_rays = N_RAYS;

    let start = PreciseTime::now();
    let img_buffer = render::render(scene, camera, width, height, n_rays);
    let end = PreciseTime::now();

    let total = start.to(end);
    println!("Time: {} ms", total.num_milliseconds());
    println!("Rays per second: {}", 1000 * N_RAYS / total.num_milliseconds());
    image::save_buffer(&Path::new("image.png"), &img_buffer[..],
                       WIDTH as u32, HEIGHT as u32, image::RGBA(8))
        .unwrap();
}

fn prep_scene() -> scene::Scene {
    let mut scene = scene::Scene::new();

    let p1 = Plane {
        point: Point {x: 0.0, y: 0.0, z: 0.0},
        normal: Vector {x: 0.0, y: 0.0, z: 1.0},
    };
    scene.objs.push(Box::new(p1));

    let p1 = Plane {
        point: Point {x: 0.0, y: 2.0, z: 0.0},
        normal: Vector {x: 0.0, y: -1.0, z: 0.0},
    };
    scene.objs.push(Box::new(p1));

    let p2 = Sphere {
        center: Point {x: 1.0, y: -1.0, z: 1.0},
        radius: 1.0,
    };
    scene.objs.push(Box::new(p2));

    let p3 = Sphere {
        center: Point {x: -1.0, y: 1.0, z: 1.0},
        radius: 1.0,
    };
    scene.objs.push(Box::new(p3));

    let p4 = Sphere {
        center: Point {x: 1.0, y: 1.0, z: 1.0},
        radius: 1.0,
    };
    scene.objs.push(Box::new(p4));
    return scene;
}

