extern crate image;

mod math;
mod render;
mod scene;

use math::*;
use std::path::Path;

fn main() {
    let camera = scene::Camera {
        look_from: Point {x: -7.0, y: -1.0, z: 3.0},
        direction: (Vector {x: 7.0, y: 1.0, z: -3.0}).normalize(),
        up: Vector {x:0.0, y: 0.0, z: 1.0},
        fov: 3.14/4.0,
        aspect: 1.0,
    };
    let mut scene = scene::Scene::new();
    let p1 = Plane {
        point: Point {x: 0.0, y: 0.0, z: 0.0},
        normal: Vector {x: 0.0, y: 0.0, z: 1.0},
    };
    scene.objs.push(Box::new(p1));
    let p2 = math::Sphere {
        center: Point {x: -1.0, y: -1.0, z: 1.0},
        radius: 1.0,
    };
    scene.objs.push(Box::new(p2));

    let p3 = math::Sphere {
        center: Point {x: 1.0, y: 1.0, z: 1.0},
        radius: 1.0,
    };
    scene.objs.push(Box::new(p3));

    let p4 = math::Sphere {
        center: Point {x: -1.0, y: 1.0, z: 1.0},
        radius: 1.0,
    };
    scene.objs.push(Box::new(p4));

    const WIDTH: usize = 500;
    const HEIGHT: usize = 500;

    let img_buffer = render::render(&scene, &camera, WIDTH, HEIGHT);
    image::save_buffer(&Path::new("image.png"), &img_buffer[..],
                       WIDTH as u32, HEIGHT as u32, image::RGBA(8))
        .unwrap();
}


