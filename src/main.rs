extern crate image;

mod math;
mod render;
mod scene;

use math::*;
use std::path::Path;

fn main() {
    let camera = scene::Camera {
        look_from: Point {x: -10.0, y: -1.0, z: 3.0},
        direction: (Vector {x: 10.0, y: 1.0, z: -2.0}).normalize(),
        up: Vector {x:0.0, y: 0.0, z: 1.0},
        fov: 3.14/6.0,
        aspect: 1.0,
    };

    let mut scene = scene::Scene::new();

    let p1 = Plane {
        point: Point {x: 0.0, y: 0.0, z: 0.5},
        normal: Vector {x: 0.0, y: 0.0, z: 1.0},
    };
    scene.objs.push(Box::new(p1));

    //let p1 = Sphere {
        //center: Point {x: 0.0, y: 0.0, z: -10000.0},
        //radius: 10000.0
    //};
    //scene.objs.push(Box::new(p1));

    let p2 = Sphere {
        center: Point {x: -1.0, y: -1.0, z: 1.0},
        radius: 1.0,
    };
    scene.objs.push(Box::new(p2));

    let p3 = Sphere {
        center: Point {x: 1.0, y: 1.0, z: 1.0},
        radius: 1.0,
    };
    scene.objs.push(Box::new(p3));

    let p4 = Sphere {
        center: Point {x: -1.0, y: 1.0, z: 1.0},
        radius: 1.0,
    };
    scene.objs.push(Box::new(p4));

    const WIDTH: usize = 300;
    const HEIGHT: usize = 300;

    let img_buffer = render::render(&scene, &camera, WIDTH, HEIGHT);

    image::save_buffer(&Path::new("image.png"), &img_buffer[..],
                       WIDTH as u32, HEIGHT as u32, image::RGBA(8))
        .unwrap();
}


