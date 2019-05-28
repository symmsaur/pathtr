extern crate image;
extern crate rand;
extern crate sdl2;
extern crate time;

mod material;
mod math;
mod preview;
mod render;
mod scene;

use math::*;
use std::path::Path;
use std::sync::Arc;
use time::PreciseTime;

const WIDTH: usize = 640;
const HEIGHT: usize = 400;
const RAYS_PER_PIXEL: i64 = 1000;

fn main() {
    let camera = Arc::new(scene::Camera {
        look_from: Point {
            x: -12.0,
            y: -10.0,
            z: 4.8,
        },
        direction: (Vector {
            x: 10.0,
            y: 9.15,
            z: -1.8,
        })
        .normalize(),
        up: Vector {
            x: 0.0,
            y: 0.0,
            z: 1.0,
        },
        fov: 3.14 / 3.0,
        aspect: 1.6,
    });
    // let camera = Arc::new(scene::Camera {
    //     look_from: Point {
    //         x: 0.0,
    //         y: 0.0,
    //         z: 100.0,
    //     },
    //     direction: (Vector {
    //         x: 0.0,
    //         y: 0.0,
    //         z: -1.0,
    //     })
    //     .normalize(),
    //     up: Vector {
    //         x: 0.0,
    //         y: 1.0,
    //         z: 0.0,
    //     },
    //     fov: 3.14 / 8.0,
    //     aspect: 1.6,
    // });
    let scene = Arc::new(prep_scene());

    let width = WIDTH;
    let height = HEIGHT;

    let preview_window = preview::open_window(WIDTH, HEIGHT).unwrap();

    let start = PreciseTime::now();
    let img_buffer = render::render(
        &preview_window,
        scene,
        camera,
        width,
        height,
        RAYS_PER_PIXEL,
    );
    let end = PreciseTime::now();

    preview_window.wait();

    let total = start.to(end);
    println!("Time: {} ms", total.num_milliseconds());
    println!(
        "Rays per ms: {}",
        RAYS_PER_PIXEL as usize * width * height / total.num_milliseconds() as usize
    );
    image::save_buffer(
        &Path::new("image.png"),
        &img_buffer[..],
        WIDTH as u32,
        HEIGHT as u32,
        image::RGBA(8),
    )
    .unwrap();
    println!("Wrote image.png");
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
            z: 0.0,
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

    // let p2 = Plane {
    //     point: Point {
    //         x: 0.0,
    //         y: 2.0,
    //         z: 0.0,
    //     },
    //     normal: Vector {
    //         x: 0.0,
    //         y: -1.0,
    //         z: 0.0,
    //     },
    // };
    // let m2 = material::Material::create(white * 0.8, 1.0, 0.0);
    // let obj2 = scene::Object {
    //     shape: Box::new(p2),
    //     material: m2,
    // };
    // scene.objs.push(obj2);

    let p3 = Sphere {
        center: Point {
            x: 1.0,
            y: -1.0,
            z: 0.9,
        },
        radius: 0.9,
    };
    let m3 = material::Material::create(
        material::Color {
            red: 0.3,
            green: 0.3,
            blue: 0.7,
        },
        1.2,
        0.0,
    );
    let obj3 = scene::Object {
        shape: Box::new(p3),
        material: m3,
    };
    scene.objs.push(obj3);
    scene.objs.push(scene::Object {
        shape: Box::new(Sphere {
            center: Point {
                x: 1.0,
                y: -1.0,
                z: 2.25,
            },
            radius: 0.45,
        }),
        material: material::Material::create(white * 0.0, 1.5, 1.0),
    });

    let p4 = Sphere {
        center: Point {
            x: -1.0,
            y: -1.0,
            z: 0.7,
        },
        radius: 0.7,
    };
    let m4 = material::Material::create(white * 0.0, 1.5, 1.0);
    let obj4 = scene::Object {
        shape: Box::new(p4),
        material: m4,
    };
    scene.objs.push(obj4);

    let p5 = Sphere {
        center: Point {
            x: 1.0,
            y: 1.0,
            z: 1.0,
        },
        radius: 1.0,
    };
    let m5 = material::Material::create(
        material::Color {
            red: 0.7,
            green: 0.3,
            blue: 0.2,
        },
        1.3,
        0.0,
    );
    let obj5 = scene::Object {
        shape: Box::new(p5),
        material: m5,
    };
    scene.objs.push(obj5);
    scene.objs.push(scene::Object {
        shape: Box::new(Sphere {
            center: Point {
                x: 1.0,
                y: 1.0,
                z: 2.5,
            },
            radius: 0.5,
        }),
        material: material::Material::create(white * 0.0, 1.5, 1.0),
    });

    scene.objs.push(scene::Object {
        shape: Box::new(Sphere {
            center: Point {
                x: 10.0,
                y: 5.0,
                z: 5.0,
            },
            radius: 5.0,
        }),
        material: material::Material::create(
            material::Color {
                red: 0.05,
                green: 0.05,
                blue: 0.05,
            },
            1.7,
            0.0,
        ),
    });

    scene.objs.push(scene::Object {
        shape: Box::new(Sphere {
            center: Point {
                x: 5.0,
                y: 20.0,
                z: 10.0,
            },
            radius: 10.0,
        }),
        material: material::Material::create(
            material::Color {
                red: 0.05,
                green: 0.15,
                blue: 0.05,
            },
            1.7,
            0.0,
        ),
    });

    // Lens ball
    // let look_from = Point {
    //     x: -12.0,
    //     y: -12.0,
    //     z: 4.8,
    // };
    // let direction = (Vector {
    //     x: 10.0,
    //     y: 9.15,
    //     z: -2.8,
    // })
    // .normalize();
    // scene.objs.push(scene::Object {
    //     shape: Box::new(Sphere {
    //         center: translate(look_from, 8.0 * direction),
    //         radius: 0.8,
    //     }),
    //     material: material::Material::create(white * 0.0, 1.5, 1.0),
    // });

    // Lights
    scene.objs.push(scene::Object {
        shape: Box::new(Sphere {
            center: Point {
                x: 20.3,
                y: -20.0,
                z: 20.35,
            },
            radius: 5.0,
        }),
        material: material::Material::create_emissive(white * 2.0),
    });
    scene.objs.push(scene::Object {
        shape: Box::new(Sphere {
            center: Point {
                x: -20.0,
                y: -5.0,
                z: 10.35,
            },
            radius: 4.0,
        }),
        material: material::Material::create_emissive(white * 1.0),
    });

    return scene;
}
