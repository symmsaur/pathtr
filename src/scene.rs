use math::*;

pub struct Scene {
    pub objs: Vec<Box<Intersectable>>,
}

impl Scene {
    pub fn new() -> Scene {
        Scene {objs: Vec::new()}
    }
}

pub struct Camera {
    pub look_from: Point,
    pub direction: Vector,
    pub up: Vector,
    pub fov: f64,
    pub aspect: f64,
}
