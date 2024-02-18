use crate::bbox::Shape;
use crate::material::Material;
use crate::math::*;

#[derive(Debug)]
pub struct Object {
    pub shape: Box<dyn Shape>,
    pub material: Material,
}
pub struct Camera {
    pub look_from: Point,
    pub direction: Vector,
    pub up: Vector,
    pub fov: f32,
    pub aspect: f32,
    pub aperture: f32,
    pub focal_distance: f32,
}
