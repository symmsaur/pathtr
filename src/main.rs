extern crate rand;

use std::ops::Mul;
use std::ops::Add;
use std::ops::Sub;

fn main() {
    let cam = Camera {
        look_from: Point {x: 0.0, y: 1.0, z: -10.0},
        direction: Vector {x: 0.0, y: 0.0, z: 1.0},
        up: Vector {x:0.0, y: 1.0, z: 1.0},
        fov: 1.0,
        aspect: 1.0,
    };
    let r = generate_ray(cam);
    println!("{}", r.origin.x);
    println!("{}", r.origin.y);
    println!("{}", r.origin.z);
    println!("{}", r.direction.x);
    println!("{}", r.direction.y);
    println!("{}", r.direction.z);
}

struct Ray {
    origin: Point,
    direction: Vector,
}

impl Ray {
    fn create(origin: Point, through: Point) -> Ray {
        let direction = displacement(origin, through).normalize();
        Ray {origin: origin, direction: direction}
    }
}

#[derive(Copy, Clone)]
struct Point {
    x: f64,
    y: f64,
    z: f64,
}

#[derive(Copy, Clone)]
struct Vector {
    x: f64,
    y: f64,
    z: f64,
}

impl Vector {
    fn normalize(self) -> Vector
    {
        let l = self.length();
        Vector{x: self.x / l, y: self.y, z: self.z}
    }
    fn length(self) -> f64
    {
        self.square_length().sqrt()
    }
    fn square_length(self) -> f64
    {
        self.x * self.x + self.y * self.y + self.z * self.z
    }
}

impl Add for Vector {
    type Output = Vector;

    fn add(self, rhs: Vector) -> Vector {
        Vector {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
            z: self.z + rhs.z,
        }
    }
}

impl Sub for Vector {
    type Output = Vector;

    fn sub(self, rhs: Vector) -> Vector {
        Vector {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
            z: self.z - rhs.z,
        }
    }
}

impl Sub for Point {
    type Output = Vector;

    fn sub(self, rhs: Point) -> Vector {
        Vector {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
            z: self.z - rhs.z,
        }
    }
}

impl Mul<Vector> for f64 {
    type Output = Vector;

    fn mul(self, rhs: Vector) -> Vector {
        Vector {
            x: self * rhs.x,
            y: self * rhs.y,
            z: self * rhs.z,
        }
    }
}

struct Sphere {
    center: Point,
    radius: f64,
}

struct Plane {
    point: Point,
    normal: Vector,
}

struct Camera {
    look_from: Point,
    direction: Vector,
    up: Vector,
    fov: f64,
    aspect: f64,
}

fn displacement(p1: Point, p2: Point) -> Vector {
    Vector {x: p2.x - p1.x, y: p2.y - p1.y, z: p2.z - p1.z}
}

fn translate(p: Point, v: Vector) -> Point {
    Point {x: p.x + v.x, y: p.y + v.y, z: p.z + v.z }
}

fn dot(v1: Vector, v2: Vector) -> f64 {
    v1.x * v2.x + v1.y * v2.y + v1.z * v2.z
}

fn cross(v1: Vector, v2: Vector) -> Vector {
    Vector {
        x: v1.y * v2.z - v1.z * v2.y,
        y: v1.z * v2.x - v1.x * v2.z,
        z: v1.x * v2.y - v1.y * v1.x
    }
}

fn generate_ray(cam: Camera) -> Ray {
    let origin = cam.look_from;
    let p_orig = translate(origin, cam.direction);
    let up = cam.up;
    let left = cross(up, cam.direction);
    let lr_range = (cam.fov / 2.0).tan();
    let ud_range = lr_range / cam.aspect;
    let p_x = lr_range * (-1.0 + 2.0 * rand::random::<f64>());
    let p_y = ud_range * (-1.0 + 2.0 * rand::random::<f64>());
    let p_disp = p_y * up + p_x * left;
    let through = translate(p_orig, p_disp);
    Ray::create(origin, through)
}

// Maybe we need to return more than one point ...
fn intersect(ray: Ray, sphere: Sphere) -> Option<Point> {
    let b = 2.0 * dot(ray.direction, ray.origin - sphere.center);
    println!("b: {}", b);
    let c = (ray.origin - sphere.center).square_length() - sphere.radius
        * sphere.radius;
    println!("c: {}", c);
    let delta = b * b - 4.0 * c;
    println!("delta: {}", delta);
    if delta < 0.0
    {
        return None;
    }
    let t = (-b - delta.sqrt())/2.0;
    println!("t: {}", t);
    Some(translate(ray.origin, t * ray.direction))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn displacement_test() {
        let p1 = Point {x: 1.0, y: 2.0, z: 3.0};
        let p2 = Point {x: 2.0, y: 4.0, z: 6.0};
        let v = displacement(p1, p2);
        assert_eq!(1.0, v.x);
        assert_eq!(2.0, v.y);
        assert_eq!(3.0, v.z);
    }

    #[test]
    fn length_test() {
        let v = Vector{x: 1.0, y: 2.0, z: 3.0};
        let res = v.length();
        // Reasonable eps?
        assert!((1.0_f64 + 2.0_f64 + 3.0_f64).sqrt() - res < 1e-10);
    }

    #[test]
    fn translate_test() {
        let p = Point {x: 1.0, y: 2.0, z: 3.0};
        let v = Vector {x: 4.0, y: 5.0, z: 6.0};
        let res = translate(p, v);
        assert_eq!(5.0, res.x);
        assert_eq!(7.0, res.y);
        assert_eq!(9.0, res.z);
    }

    #[test]
    fn cross_test_1() {
        let v1 = Vector{x: 1.0, y: 0.0, z: 0.0};
        let v2 = Vector{x: 0.0, y: 1.0, z: 0.0};
        let res = cross(v1,v2);
        assert_eq!(0.0, res.x);
        assert_eq!(0.0, res.y);
        assert_eq!(1.0, res.z);
    }

    #[test]
    fn cross_test_2() {
        let v1 = Vector{x: 0.0, y: 0.0, z: 1.0};
        let v2 = Vector{x: 0.0, y: 0.0, z: 1.0};
        let res = cross(v1,v2);
        assert_eq!(0.0, res.x);
        assert_eq!(0.0, res.y);
        assert_eq!(0.0, res.z);
    }

    #[test]
    fn dot_test() {
        let v1 = Vector{x: 1.0, y: 2.0, z: 3.0};
        let v2 = Vector{x: 5.0, y: 7.0, z: 11.0};
        let res = dot(v1, v2);
        assert_eq!(5.0 + 14.0 + 33.0, res);
    }

    #[test]
    fn sphere_ray_intersect_test() {
        let ray = Ray{
            origin: Point {x: -2.0, y: 0.0, z: 0.0},
            direction: Vector {x: 1.0, y: 0.0, z: 0.0}
        };
        let sphere = Sphere{
            center: Point{x: 0.0, y: 0.0, z: 0.0},
            radius: 1.0
        };
        let res = intersect(ray, sphere).unwrap();
        assert_eq!(-1.0, res.x);
        assert_eq!(0.0, res.y);
        assert_eq!(0.0, res.z);
    }

    #[test]
    fn sphere_ray_intersect_test_2() {
        let ray = Ray{
            origin: Point {x: -2.0, y: 0.0, z: 0.0},
            direction: Vector {x: 1.0, y: 0.0, z: 0.0}
        };
        let sphere = Sphere{
            center: Point{x: 0.0, y: 0.0, z: 0.0},
            radius: 0.5
        };
        let res = intersect(ray, sphere).unwrap();
        assert_eq!(-0.5, res.x);
        assert_eq!(0.0, res.y);
        assert_eq!(0.0, res.z);
    }
}
