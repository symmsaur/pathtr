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

const WIDTH: usize = 1000;
const HEIGHT: usize = 1000;
const N_RAYS: i64 = (WIDTH * HEIGHT * 100) as i64;

fn main() {
    let camera = Arc::new(scene::Camera {
        look_from: Point {
            x: -12.0,
            y: -8.0,
            z: 3.0,
        },
        direction: (Vector {
            x: 10.0,
            y: 6.75,
            z: -2.0,
        })
        .normalize(),
        up: Vector {
            x: 0.0,
            y: 0.0,
            z: 1.0,
        },
        fov: 3.14 / 8.0,
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
    println!(
        "Rays per second: {}",
        1000 * N_RAYS / total.num_milliseconds()
    );
    image::save_buffer(
        &Path::new("image.png"),
        &img_buffer[..],
        WIDTH as u32,
        HEIGHT as u32,
        image::RGBA(8),
    )
    .unwrap();
}

fn prep_scene() -> scene::Scene {
    let mut scene = scene::Scene::new();

    let p1 = Plane {
        point: Point {
            x: 0.0,
            y: 0.0,
            z: 0.0,
        },
        normal: Vector {
            x: 0.0,
            y: 0.0,
            z: 1.0,
        },
    };
    let m1 = material::Material::create(0.5, 1.0, 0.0);
    let obj1 = scene::Object {
        shape: Box::new(p1),
        material: m1,
    };
    scene.objs.push(obj1);

    let p2 = Plane {
        point: Point {
            x: 0.0,
            y: 2.0,
            z: 0.0,
        },
        normal: Vector {
            x: 0.0,
            y: -1.0,
            z: 0.0,
        },
    };
    let m2 = material::Material::create(0.2, 1.0, 0.0);
    let obj2 = scene::Object {
        shape: Box::new(p2),
        material: m2,
    };
    scene.objs.push(obj2);

    let p3 = Sphere {
        center: Point {
            x: 1.0,
            y: -1.0,
            z: 1.0,
        },
        radius: 1.0,
    };
    let m3 = material::Material::create(0.5, 1.0, 0.0);
    let obj3 = scene::Object {
        shape: Box::new(p3),
        material: m3,
    };
    scene.objs.push(obj3);

    let p4 = Sphere {
        center: Point {
            x: -1.0,
            y: -1.0,
            z: 0.7,
        },
        radius: 0.7,
    };
    let m4 = material::Material::create(0.1, 1.3, 0.9);
    let obj4 = scene::Object {
        shape: Box::new(p4),
        material: m4,
    };
    scene.objs.push(obj4);
    // let p3 =     // scene.objs.push(Box::new(p3));

    let p5 = Sphere {
        center: Point {
            x: 1.0,
            y: 1.0,
            z: 1.0,
        },
        radius: 1.0,
    };
    let m5 = material::Material::create(0.5, 1.5, 0.0);
    let obj5 = scene::Object {
        shape: Box::new(p5),
        material: m5,
    };
    scene.objs.push(obj5);
    return scene;
}
