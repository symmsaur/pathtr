mod bbox;
mod material;
mod math;
mod preview;
mod render;
mod scene;
mod trace;

use math::*;
use rand::prelude::*;
use rand_xorshift::XorShiftRng;
use std::cell::Cell;
use std::path::Path;
use std::sync::Arc;
use time::PreciseTime;

const WIDTH: usize = 1920 / 4;
const HEIGHT: usize = 1200 / 4;
const RAYS_PER_PIXEL: i64 = 10000;

fn main() {
    let camera = Arc::new(scene::Camera {
        look_from: Point {
            x: -0.1,
            y: -15.0,
            z: 4.8,
        },
        direction: (Vector {
            x: 0.05,
            y: 1.0,
            z: -0.25,
        })
        .normalize(),
        up: Vector {
            x: 0.0,
            y: 0.0,
            z: 1.0,
        },
        fov: 3.14 / 4.0,
        aspect: 1.6,
        aperture: 0.1,
        focal_distance: 16.0,
    });
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

fn create_spheres(
    num: u32,
    min_radius: f64,
    max_radius: f64,
    material: material::Material,
) -> Vec<scene::Object> {
    let mut rng = XorShiftRng::from_entropy();
    (0..num)
        .map(|_| {
            let radius = (max_radius - min_radius) * rng.gen::<f64>() + min_radius;
            let center = Point {
                x: 20.0 * rng.gen::<f64>() - 10.0,
                y: 20.0 * rng.gen::<f64>() - 10.0,
                z: radius,
            };
            scene::Object {
                shape: Box::new(Sphere { center, radius }),
                material,
            }
        })
        .collect()
}

fn prep_scene() -> bbox::BoundingBoxTree {
    // Let's go for 1000 objects!
    let tree = Cell::new(bbox::BoundingBoxTree::create_empty());
    let white = material::Color {
        red: 1.0,
        green: 1.0,
        blue: 1.0,
    };

    // TODO: Make planes possible to use again
    // let p1 = Plane {
    //     point: Point {
    //         x: 0.0,
    //         y: 0.0,
    //         z: 0.0,
    //     },
    //     normal: Vector {
    //         x: 0.0,
    //         y: 0.0,
    //         z: 1.0,
    //     },
    // };
    // let m1 = material::Material::create(white * 0.8, 1.0, 0.0);
    // let obj1 = scene::Object {
    //     shape: Box::new(p1),
    //     material: m1,
    // };
    // tree.set(tree.take().add(obj1));
    let mut spheres = create_spheres(2500, 0.1, 0.2, material::Material::create_colored_1());
    spheres.append(&mut create_spheres(
        2500,
        0.1,
        0.2,
        material::Material::create_colored_2(),
    ));
    spheres.append(&mut create_spheres(
        2500,
        0.1,
        0.2,
        material::Material::create_colored_3(),
    ));
    spheres.append(&mut create_spheres(
        2500,
        0.1,
        0.2,
        material::Material::create_glass(),
    ));
    for sphere in spheres {
        tree.set(tree.take().add(sphere));
    }
    // // Lights
    tree.set(tree.take().add(scene::Object {
        shape: Box::new(Sphere {
            center: Point {
                x: 20.3,
                y: -20.0,
                z: 20.35,
            },
            radius: 5.0,
        }),
        material: material::Material::create_emissive(white * 5.0),
    }));
    tree.set(tree.take().add(scene::Object {
        shape: Box::new(Sphere {
            center: Point {
                x: -20.0,
                y: -5.0,
                z: 10.35,
            },
            radius: 4.0,
        }),
        material: material::Material::create_emissive(white * 2.0),
    }));
    tree.into_inner()

    // scene.objs.append(&mut );
    // scene.objs.append(&mut create_spheres(
    //     250,
    //     0.2,
    //     0.3,
    //     material::Material::create_colored_2(),
    // ));
    // scene.objs.append(&mut create_spheres(
    //     250,
    //     0.05,
    //     0.15,
    //     material::Material::create_colored_3(),
    // ));
    // scene.objs.append(&mut create_spheres(
    //     250,
    //     0.1,
    //     0.4,
    //     material::Material::create_glass(),
    // ));
}
