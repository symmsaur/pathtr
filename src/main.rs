mod material;
mod math;
mod preview;
mod render;
mod scene;

use math::*;

use clap::Parser;
use std::path::Path;
use std::sync::Arc;
use time::PreciseTime;

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
        aperture: 0.3,
        focal_distance: 16.0,
    });
    let scene = Arc::new(prep_scene());

    let width = WIDTH;
    let height = HEIGHT;

    let preview_window = if args.preview {
        Some(preview::open_window(WIDTH, HEIGHT).unwrap())
    } else {
        None
    };

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

    if let Some(p) = preview_window {
        p.wait();
    }

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

    add_sphere(
        &mut scene,
        0.1,
        -0.03,
        1.0,
        material::Material::create_colored_1(),
    );
    add_sphere(
        &mut scene,
        -1.7,
        -1.0,
        0.7,
        material::Material::create_colored_2(),
    );
    add_sphere(
        &mut scene,
        2.0,
        0.3,
        1.0,
        material::Material::create_glass(),
    );

    add_sphere(
        &mut scene,
        -1.0,
        -3.3,
        0.4,
        material::Material::create_colored_3(),
    );
    add_sphere(
        &mut scene,
        1.2,
        -3.3,
        0.4,
        material::Material::create_colored_2(),
    );

    add_sphere(
        &mut scene,
        2.2,
        5.3,
        0.8,
        material::Material::create_colored_2(),
    );
    add_sphere(
        &mut scene,
        -2.0,
        4.3,
        1.1,
        material::Material::create_colored_1(),
    );

    add_sphere(
        &mut scene,
        5.2,
        15.3,
        1.0,
        material::Material::create_colored_1(),
    );
    add_sphere(
        &mut scene,
        -0.2,
        10.3,
        1.0,
        material::Material::create_colored_3(),
    );

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
        material: material::Material::create_emissive(white * 5.0),
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
        material: material::Material::create_emissive(white * 2.0),
    });

    return scene;
}

fn add_sphere(scene: &mut scene::Scene, x: f64, y: f64, radius: f64, material: material::Material) {
    scene.objs.push(scene::Object {
        shape: Box::new(Sphere {
            center: Point {
                x: 1.5 * x,
                y: 1.5 * y,
                z: radius,
            },
            radius,
        }),
        material,
    });
}
