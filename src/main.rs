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

const WIDTH: usize = 800;
const HEIGHT: usize = 800;
const N_RAYS: i64 = (WIDTH * HEIGHT * 200) as i64;

fn main() {
    let camera = Arc::new(scene::Camera {
        look_from: Point {
            x: -1.9,
            y: -1.9,
            z: 1.9,
        },
        direction: (Vector {
            x: 1.0,
            y: 1.0,
            z: -1.0,
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
    println!("Rays per ms: {}", N_RAYS / total.num_milliseconds());
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
    let white = material::Color {
        red: 1.0,
        green: 1.0,
        blue: 1.0,
    };

    let p1 = Plane {
        point: Point {
            x: 0.0,
            y: 0.0,
            z: -2.0,
        },
        normal: Vector {
            x: 0.0,
            y: 0.0,
            z: 1.0,
        },
    };
    let m1 = material::Material::create(white * 0.8, 1.0, 0.0);
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
    let m2 = material::Material::create(white * 0.8, 1.0, 0.0);
    let obj2 = scene::Object {
        shape: Box::new(p2),
        material: m2,
    };
    scene.objs.push(obj2);

    let p3 = Sphere {
        center: Point {
            x: 0.2,
            y: -0.2,
            z: 0.2,
        },
        radius: 0.2,
    };
    let m3 = material::Material::create(
        material::Color {
            red: 0.2,
            green: 0.2,
            blue: 0.8,
        },
        1.2,
        0.0,
    );
    let obj3 = scene::Object {
        shape: Box::new(p3),
        material: m3,
    };
    scene.objs.push(obj3);

    let p4 = Sphere {
        center: Point {
            x: 0.3,
            y: 0.3,
            z: -0.3,
        },
        radius: 0.3,
    };
    let m4 = material::Material::create(white * 0.0, 1.5, 1.0);
    let obj4 = scene::Object {
        shape: Box::new(p4),
        material: m4,
    };
    scene.objs.push(obj4);

    scene.objs.push(scene::Object {
        shape: Box::new(Sphere {
            center: Point {
                x: 0.2,
                y: 0.2,
                z: 0.2,
            },
            radius: 0.2,
        }),
        material: material::Material::create(
            material::Color {
                red: 0.7,
                green: 0.3,
                blue: 0.3,
            },
            1.3,
            0.0,
        ),
    });

    scene.objs.push(scene::Object {
        shape: Box::new(Sphere {
            center: Point {
                x: -0.3,
                y: 0.3,
                z: 0.3,
            },
            radius: 0.3,
        }),
        material: material::Material::create(
            material::Color {
                red: 0.7,
                green: 0.7,
                blue: 0.7,
            },
            1.3,
            0.0,
        ),
    });

    let p6 = Plane {
        point: Point {
            x: -2.0,
            y: 0.0,
            z: 2.0,
        },
        normal: Vector {
            x: 1.0,
            y: 0.0,
            z: 0.0,
        },
    };
    let m6 = material::Material::create(white * 0.8, 1.0, 0.0);
    let obj6 = scene::Object {
        shape: Box::new(p6),
        material: m6,
    };
    scene.objs.push(obj6);

    let p7 = Plane {
        point: Point {
            x: 0.0,
            y: 0.0,
            z: 2.0,
        },
        normal: Vector {
            x: 0.0,
            y: 0.0,
            z: -1.0,
        },
    };
    let m7 = material::Material::create(white * 0.8, 1.0, 0.0);
    let obj7 = scene::Object {
        shape: Box::new(p7),
        material: m7,
    };
    scene.objs.push(obj7);

    scene.objs.push(scene::Object {
        shape: Box::new(Plane {
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
        }),
        material: material::Material::create(white * 0.8, 1.0, 0.0),
    });

    scene.objs.push(scene::Object {
        shape: Box::new(Plane {
            point: Point {
                x: 0.0,
                y: -2.0,
                z: 0.0,
            },
            normal: Vector {
                x: 0.0,
                y: 1.0,
                z: 0.0,
            },
        }),
        material: material::Material::create(white * 0.8, 1.0, 0.0),
    });

    scene.objs.push(scene::Object {
        shape: Box::new(Sphere {
            center: Point {
                x: -1.0,
                y: -1.0,
                z: -1.0,
            },
            radius: 0.4,
        }),
        material: material::Material::create_emissive(material::Color {
            red: 1.0,
            green: 1.0,
            blue: 1.0,
        }),
    });

    return scene;
}
