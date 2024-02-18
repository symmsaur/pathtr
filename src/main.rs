mod args;
mod bbox;
mod bbox_tree_stats;
mod material;
mod math;
mod preview;
mod render;
mod scene;
mod trace;

use clap::Parser;
use image::ColorType::Rgba8;
use math::*;
use rand::prelude::*;
use rand_xorshift::XorShiftRng;
use std::cell::Cell;
use std::path::Path;
use std::sync::Arc;
use std::time::Instant;

const WIDTH: usize = 800;
const HEIGHT: usize = 500;
const RAYS_PER_PIXEL: i64 = 1000;

#[derive(Parser)]
pub struct Args {
    #[arg(short, long)]
    pub preview: bool,
}

pub fn parse() -> Args {
    Args::parse()
}

fn main() {
    let args = Args::parse();
    let camera = Arc::new(scene::Camera {
        look_from: Point {
            x: -20.0,
            y: -20.0,
            z: 20.0,
        },
        direction: (Vector {
            x: 20.0,
            y: 20.0,
            z: -20.0,
        })
        .normalize(),
        up: Vector {
            x: 1.0,
            y: 0.0,
            z: 0.0,
        },
        fov: 3.1415 / 4.0,
        aspect: 1.6,
        aperture: 0.1,
        focal_distance: 20.0,
    });
    let scene = Arc::new(prep_scene());

    bbox_tree_stats::print_bounding_box_tree_stats(&scene);

    let width = WIDTH;
    let height = HEIGHT;

    let preview_window = if args.preview {
        Some(preview::open_window(WIDTH, HEIGHT).unwrap())
    } else {
        None
    };

    let start = Instant::now();
    let img_buffer = render::render(
        &preview_window,
        scene,
        camera,
        width,
        height,
        RAYS_PER_PIXEL,
    );
    let total = start.elapsed().as_millis();

    if let Some(p) = preview_window {
        p.wait();
    }

    println!("Time: {} ms", total);
    println!(
        "Rays per ms: {}",
        RAYS_PER_PIXEL as usize * width * height / total as usize
    );
    image::save_buffer(
        &Path::new("image.png"),
        &img_buffer[..],
        WIDTH as u32,
        HEIGHT as u32,
        Rgba8,
    )
    .unwrap();
    println!("Wrote image.png");
}

fn create_spheres(
    num: u32,
    min_radius: f32,
    max_radius: f32,
    material: material::Material,
    rng: &mut XorShiftRng,
) -> Vec<scene::Object> {
    (0..num)
        .map(|_| {
            let radius = (max_radius - min_radius) * rng.gen::<f32>() + min_radius;
            let center = Point {
                x: 20.0 * rng.gen::<f32>() - 10.0,
                y: 20.0 * rng.gen::<f32>() - 10.0,
                z: 20.0 * rng.gen::<f32>() - 10.0,
            };
            scene::Object {
                shape: Box::new(Sphere { center, radius }),
                material,
            }
        })
        .collect()
}

fn prep_scene() -> bbox::BoundingBoxTree {
    let num_objects = 100;
    let tree = Cell::new(bbox::BoundingBoxTree::create_empty());
    let white = material::Color {
        red: 1.0,
        green: 1.0,
        blue: 1.0,
    };
    let min_radius = 0.2 * f32::cbrt(300.0 / (num_objects as f32));
    let max_radius = 0.7 * f32::cbrt(300.0 / (num_objects as f32));

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
    let mut rng = XorShiftRng::seed_from_u64(0);
    let mut spheres = create_spheres(
        num_objects / 4,
        min_radius,
        max_radius,
        material::Material::create_colored_1(),
        &mut rng,
    );
    spheres.append(&mut create_spheres(
        num_objects / 4,
        min_radius,
        max_radius,
        material::Material::create_colored_2(),
        &mut rng,
    ));
    spheres.append(&mut create_spheres(
        num_objects / 4,
        min_radius,
        max_radius,
        material::Material::create_colored_3(),
        &mut rng,
    ));
    spheres.append(&mut create_spheres(
        num_objects / 4,
        min_radius,
        max_radius,
        material::Material::create_glass(),
        &mut rng,
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
                z: 200.0,
            },
            radius: 100.0,
        }),
        material: material::Material::create_emissive(white * 2.0),
    }));
    tree.into_inner()
}
