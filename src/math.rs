
use std::ops::Mul;
use std::ops::Add;
use std::ops::Sub;

pub trait Intersectable {
    fn intersect(&self, ray: &Ray) -> Option<Point>;
}

pub struct Ray {
    origin: Point,
    direction: Vector,
}

impl Ray {
    pub fn create(origin: Point, through: Point) -> Ray {
        let direction = (through - origin).normalize();
        Ray {origin: origin, direction: direction}
    }
}

#[derive(Copy, Clone)]
pub struct Point {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

#[derive(Copy, Clone)]
pub struct Vector {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

impl Vector {
    pub fn normalize(self) -> Vector {
        let l = self.length();
        Vector{x: self.x / l, y: self.y / l, z: self.z / l}
    }
    pub fn length(self) -> f64 {
        self.square_length().sqrt()
    }
    pub fn square_length(self) -> f64 {
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

pub struct Sphere {
    pub center: Point,
    pub radius: f64,
}

pub struct Plane {
    pub point: Point,
    pub normal: Vector,
}

pub fn translate(p: Point, v: Vector) -> Point {
    Point {x: p.x + v.x, y: p.y + v.y, z: p.z + v.z }
}

pub fn dot(v1: Vector, v2: Vector) -> f64 {
    v1.x * v2.x + v1.y * v2.y + v1.z * v2.z
}

pub fn cross(v1: Vector, v2: Vector) -> Vector {
    Vector {
        x: v1.y * v2.z - v1.z * v2.y,
        y: v1.z * v2.x - v1.x * v2.z,
        z: v1.x * v2.y - v1.y * v1.x
    }
}

impl Intersectable for Plane {
    fn intersect(&self, _ray: &Ray) -> Option<Point> {
        //Some(Point {x:0.0, y:0.0, z:0.0})
        None
    }
}

impl Intersectable for Sphere {
    // Maybe we need to return more than one point ...
    fn intersect(&self, ray: &Ray) -> Option<Point> {
        let b = 2.0 * dot(ray.direction, ray.origin - self.center);
        //println!("b: {}", b);
        let c = (ray.origin - self.center).square_length() - self.radius
            * self.radius;
        //println!("c: {}", c);
        let delta = b * b - 4.0 * c;
        //println!("delta: {}", delta);
        if delta < 0.0
        {
            return None;
        }
        let t = (-b - delta.sqrt())/2.0;
        //println!("t: {}", t);
        Some(translate(ray.origin, t * ray.direction))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn length_test() {
        let v = Vector{x: 1.0, y: 2.0, z: 3.0};
        let res = v.length();
        // Reasonable eps?
        assert!((1.0_f64 + 2.0_f64 + 3.0_f64).sqrt() - res < 1e-10);
    }

    #[test]
    fn normalize_test_x() {
        let v = Vector{x: 2.0, y: 0.0, z: 0.0};
        let res = v.normalize();
        assert!(res.x - 1.0 < 1e-10);
    }

    #[test]
    fn normalize_test_y() {
        let v = Vector{x: 0.0, y: 4.0, z: 0.0};
        let res = v.normalize();
        assert!(res.y - 1.0 < 1e-10);
    }

    #[test]
    fn normalize_test_z() {
        let v = Vector{x: 0.0, y: 0.0, z: 5.0};
        let res = v.normalize();
        assert!(res.z - 1.0 < 1e-10);
    }

    #[test]
    fn normalize_test() {
        let v = Vector{x: 76.0, y: 14.0, z: 5.0};
        let res = v.normalize();
        assert!(res.length() - 1.0 < 1e-10);
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
    fn vec_scalar_mul_test() {
        let v1 = Vector{x: 1.0, y: 2.0, z: 3.0};
        let s = 5.0;
        let res = s * v1;
        assert_eq!(5.0, res.x);
        assert_eq!(10.0, res.y);
        assert_eq!(15.0, res.z);
    }

    #[test]
    fn point_sub_test() {
        let p1 = Point{x: 4.0, y: 4.0, z: 4.0};
        let p2 = Point{x: 1.0, y: 2.0, z: 3.0};
        let res = p1 - p2;
        assert_eq!(3.0, res.x);
        assert_eq!(2.0, res.y);
        assert_eq!(1.0, res.z);
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
        let res = sphere.intersect(&ray).unwrap();
        assert_eq!(-1.0, res.x);
        assert_eq!(0.0, res.y);
        assert_eq!(0.0, res.z);
    }

    #[test]
    fn sphere_ray_intersect_test_2() {
        let ray = Ray{
            origin: Point {x: 0.0, y: -1.0, z: 0.0},
            direction: Vector {x: 0.0, y: 1.0, z: 0.0}
        };
        let sphere = Sphere{
            center: Point{x: 0.0, y: 0.0, z: 0.0},
            radius: 0.5
        };
        let res = sphere.intersect(&ray).unwrap();
        assert_eq!(0.0, res.x);
        assert_eq!(-0.5, res.y);
        assert_eq!(0.0, res.z);
    }

    #[test]
    fn sphere_ray_intersect_test_miss() {
        let ray = Ray{
            origin: Point {x: 1.5, y: 1.5, z: -10.0},
            direction: Vector {x: 0.0, y: 0.0, z: 1.0}
        };
        let sphere = Sphere{
            center: Point{x: 0.0, y: 0.0, z: 0.0},
            radius: 2.0
        };
        let res = sphere.intersect(&ray);
        assert!(res.is_none());
    }
}
