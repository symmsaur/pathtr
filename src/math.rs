use std::ops::Add;
use std::ops::Mul;
use std::ops::Neg;
use std::ops::Sub;

pub trait Intersectable: Sync + Send {
    fn intersect(&self, ray: &Ray) -> Option<(Point, Vector, f64, bool)>;
}

pub struct Ray {
    pub origin: Point,
    pub direction: Vector,
}

impl Ray {
    pub fn create(origin: Point, through: Point) -> Ray {
        let direction = (through - origin).normalize();
        Ray { origin, direction }
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
        Vector {
            x: self.x / l,
            y: self.y / l,
            z: self.z / l,
        }
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

impl Neg for Vector {
    type Output = Vector;

    fn neg(self) -> Vector {
        Vector {
            x: -self.x,
            y: -self.y,
            z: -self.z,
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

impl Plane {
    fn signed_distance(&self, point: Point) -> f64 {
        let pp = point - self.point;
        dot(pp, self.normal)
    }
}

pub fn translate(p: Point, v: Vector) -> Point {
    Point {
        x: p.x + v.x,
        y: p.y + v.y,
        z: p.z + v.z,
    }
}

pub fn dot(v1: Vector, v2: Vector) -> f64 {
    v1.x * v2.x + v1.y * v2.y + v1.z * v2.z
}

pub fn cross(v1: Vector, v2: Vector) -> Vector {
    Vector {
        x: v1.y * v2.z - v1.z * v2.y,
        y: v1.z * v2.x - v1.x * v2.z,
        z: v1.x * v2.y - v1.y * v2.x,
    }
}

impl Intersectable for Plane {
    fn intersect(&self, ray: &Ray) -> Option<(Point, Vector, f64, bool)> {
        let v = -dot(ray.direction, self.normal);
        if v <= 0.0 {
            // The ray is moving away from the plane.
            return None;
        }
        let d = self.signed_distance(ray.origin);
        if d <= 0.0 {
            // The ray started inside the plane.
            return None;
        }
        let t = d / v;
        Some((
            translate(ray.origin, t * ray.direction),
            self.normal,
            t,
            false,
        ))
    }
}

impl Intersectable for Sphere {
    fn intersect(&self, ray: &Ray) -> Option<(Point, Vector, f64, bool)> {
        let b = 2.0 * dot(ray.direction, ray.origin - self.center);
        //println!("b: {}", b);
        let c = (ray.origin - self.center).square_length() - self.radius * self.radius;
        //println!("c: {}", c);
        let delta = b * b - 4.0 * c;
        //println!("delta: {}", delta);
        if delta < 0.0 {
            return None;
        }
        // Let's assume this is a normal second degree equation solution.
        let t1 = (-b - delta.sqrt()) / 2.0;
        if t1 > 0.0 {
            let intersection = translate(ray.origin, t1 * ray.direction);
            let normal = (intersection - self.center).normalize();
            return Some((intersection, normal, t1, false));
        }
        let t2 = (-b + delta.sqrt()) / 2.0;
        if t2 > 0.0 {
            let intersection = translate(ray.origin, t2 * ray.direction);
            // We are inside the sphere
            let normal = (self.center - intersection).normalize();
            return Some((intersection, normal, t2, true));
        }
        return None;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn length_test() {
        let v = Vector {
            x: 1.0,
            y: 2.0,
            z: 3.0,
        };
        let res = v.length();
        // Reasonable eps?
        assert!((1.0_f64 + 2.0_f64 + 3.0_f64).sqrt() - res < 1e-10);
    }

    #[test]
    fn normalize_test_x() {
        let v = Vector {
            x: 2.0,
            y: 0.0,
            z: 0.0,
        };
        let res = v.normalize();
        assert!(res.x - 1.0 < 1e-10);
    }

    #[test]
    fn normalize_test_y() {
        let v = Vector {
            x: 0.0,
            y: 4.0,
            z: 0.0,
        };
        let res = v.normalize();
        assert!(res.y - 1.0 < 1e-10);
    }

    #[test]
    fn normalize_test_z() {
        let v = Vector {
            x: 0.0,
            y: 0.0,
            z: 5.0,
        };
        let res = v.normalize();
        assert!(res.z - 1.0 < 1e-10);
    }

    #[test]
    fn normalize_test() {
        let v = Vector {
            x: 76.0,
            y: 14.0,
            z: 5.0,
        };
        let res = v.normalize();
        assert!(res.length() - 1.0 < 1e-10);
    }

    #[test]
    fn translate_test() {
        let p = Point {
            x: 1.0,
            y: 2.0,
            z: 3.0,
        };
        let v = Vector {
            x: 4.0,
            y: 5.0,
            z: 6.0,
        };
        let res = translate(p, v);
        assert_eq!(5.0, res.x);
        assert_eq!(7.0, res.y);
        assert_eq!(9.0, res.z);
    }

    #[test]
    fn cross_test_1() {
        let v1 = Vector {
            x: 1.0,
            y: 0.0,
            z: 0.0,
        };
        let v2 = Vector {
            x: 0.0,
            y: 1.0,
            z: 0.0,
        };
        let res = cross(v1, v2);
        assert_eq!(0.0, res.x);
        assert_eq!(0.0, res.y);
        assert_eq!(1.0, res.z);
    }

    #[test]
    fn cross_test_2() {
        let v1 = Vector {
            x: 0.0,
            y: 0.0,
            z: 1.0,
        };
        let v2 = Vector {
            x: 0.0,
            y: 0.0,
            z: 1.0,
        };
        let res = cross(v1, v2);
        assert_eq!(0.0, res.x);
        assert_eq!(0.0, res.y);
        assert_eq!(0.0, res.z);
    }

    #[test]
    fn dot_test() {
        let v1 = Vector {
            x: 1.0,
            y: 2.0,
            z: 3.0,
        };
        let v2 = Vector {
            x: 5.0,
            y: 7.0,
            z: 11.0,
        };
        let res = dot(v1, v2);
        assert_eq!(5.0 + 14.0 + 33.0, res);
    }

    #[test]
    fn vec_scalar_mul_test() {
        let v1 = Vector {
            x: 1.0,
            y: 2.0,
            z: 3.0,
        };
        let s = 5.0;
        let res = s * v1;
        assert_eq!(5.0, res.x);
        assert_eq!(10.0, res.y);
        assert_eq!(15.0, res.z);
    }

    #[test]
    fn point_sub_test() {
        let p1 = Point {
            x: 4.0,
            y: 4.0,
            z: 4.0,
        };
        let p2 = Point {
            x: 1.0,
            y: 2.0,
            z: 3.0,
        };
        let res = p1 - p2;
        assert_eq!(3.0, res.x);
        assert_eq!(2.0, res.y);
        assert_eq!(1.0, res.z);
    }

    #[test]
    fn sphere_ray_intersect_test() {
        let ray = Ray {
            origin: Point {
                x: -2.0,
                y: 0.0,
                z: 0.0,
            },
            direction: Vector {
                x: 1.0,
                y: 0.0,
                z: 0.0,
            },
        };
        let sphere = Sphere {
            center: Point {
                x: 0.0,
                y: 0.0,
                z: 0.0,
            },
            radius: 1.0,
        };
        let (res, _, _, _) = sphere.intersect(&ray).unwrap();
        assert_eq!(-1.0, res.x);
        assert_eq!(0.0, res.y);
        assert_eq!(0.0, res.z);
    }

    #[test]
    fn sphere_ray_intersect_test_2() {
        let ray = Ray {
            origin: Point {
                x: 0.0,
                y: -1.0,
                z: 0.0,
            },
            direction: Vector {
                x: 0.0,
                y: 1.0,
                z: 0.0,
            },
        };
        let sphere = Sphere {
            center: Point {
                x: 0.0,
                y: 0.0,
                z: 0.0,
            },
            radius: 0.5,
        };
        let (res, _, _, _) = sphere.intersect(&ray).unwrap();
        assert_eq!(0.0, res.x);
        assert_eq!(-0.5, res.y);
        assert_eq!(0.0, res.z);
    }

    #[test]
    fn sphere_ray_intersect_translated() {
        let ray = Ray {
            origin: Point {
                x: 1.0,
                y: 1.0,
                z: 3.0,
            },
            direction: Vector {
                x: 0.0,
                y: 1.0,
                z: 0.0,
            },
        };
        let sphere = Sphere {
            center: Point {
                x: 1.0,
                y: 2.0,
                z: 3.0,
            },
            radius: 0.5,
        };
        let (res, _, t, _) = sphere.intersect(&ray).unwrap();
        assert_eq!(1.0, res.x);
        assert_eq!(1.5, res.y);
        assert_eq!(3.0, res.z);
        assert_eq!(0.5, t);
    }

    #[test]
    fn sphere_ray_intersect_test_miss() {
        let ray = Ray {
            origin: Point {
                x: 1.5,
                y: 1.5,
                z: -10.0,
            },
            direction: Vector {
                x: 0.0,
                y: 0.0,
                z: 1.0,
            },
        };
        let sphere = Sphere {
            center: Point {
                x: 0.0,
                y: 0.0,
                z: 0.0,
            },
            radius: 2.0,
        };
        let res = sphere.intersect(&ray);
        assert!(res.is_none());
    }

    #[test]
    fn sphere_ray_intersect_test_normal() {
        let ray = Ray {
            origin: Point {
                x: 0.0,
                y: -1.0,
                z: 0.0,
            },
            direction: Vector {
                x: 0.0,
                y: 1.0,
                z: 0.0,
            },
        };
        let sphere = Sphere {
            center: Point {
                x: 0.0,
                y: 0.0,
                z: 0.0,
            },
            radius: 0.5,
        };
        let (_, normal, _, _) = sphere.intersect(&ray).unwrap();
        assert_eq!(0.0, normal.x);
        assert_eq!(-1.0, normal.y);
        assert_eq!(0.0, normal.z);
    }

    fn almost_eq(v1: f64, v2: f64) -> bool {
        (v1 - v2).abs() < 1e-10
    }

    #[test]
    fn sphere_ray_intersect_test_normal_inside() {
        let direction = (Vector {
            x: 4.0,
            y: 5.0,
            z: 6.0,
        })
        .normalize();
        let ray = Ray {
            origin: Point {
                x: 1.0,
                y: 2.0,
                z: 3.0,
            },
            direction,
        };
        let sphere = Sphere {
            center: Point {
                x: 1.0,
                y: 2.0,
                z: 3.0,
            },
            radius: 0.5,
        };
        let (_, normal, _, _) = sphere.intersect(&ray).unwrap();
        assert!(almost_eq(-direction.x, normal.x));
        assert!(almost_eq(-direction.y, normal.y));
        assert!(almost_eq(-direction.z, normal.z));
    }

    #[test]
    fn plane_signed_distance_positive() {
        let plane = Plane {
            point: Point {
                x: -1.0,
                y: 100.0,
                z: 101.0,
            },
            normal: Vector {
                x: 1.0,
                y: 0.0,
                z: 0.0,
            },
        };
        let point = Point {
            x: 1.0,
            y: 0.0,
            z: 0.0,
        };

        let d = plane.signed_distance(point);

        assert_eq!(d, 2.0);
    }

    #[test]
    fn plane_signed_distance_negative() {
        let plane = Plane {
            point: Point {
                x: 3.0,
                y: 0.0,
                z: 0.0,
            },
            normal: Vector {
                x: 1.0,
                y: 0.0,
                z: 0.0,
            },
        };
        let point = Point {
            x: 1.0,
            y: 0.0,
            z: 0.0,
        };

        let d = plane.signed_distance(point);

        assert_eq!(d, -2.0);
    }

    #[test]
    fn plane_ray_intersection_hit() {
        let ray = Ray {
            origin: Point {
                x: 1.0,
                y: 2.0,
                z: 3.0,
            },
            direction: Vector {
                x: -1.0,
                y: 0.0,
                z: 0.0,
            },
        };
        let plane = Plane {
            point: Point {
                x: -1.0,
                y: 0.0,
                z: 0.0,
            },
            normal: Vector {
                x: 1.0,
                y: 0.0,
                z: 0.0,
            },
        };

        let (p, _, _, _) = plane.intersect(&ray).unwrap();

        assert_eq!(2.0, p.y);
        assert_eq!(3.0, p.z);
        assert_eq!(-1.0, p.x);
    }

    #[test]
    fn plane_ray_intersection_hit_angled() {
        let ray = Ray {
            origin: Point {
                x: 1.0,
                y: 0.0,
                z: 0.0,
            },
            direction: (Vector {
                x: -1.0,
                y: -2.0,
                z: 0.0,
            })
            .normalize(),
        };
        let plane = Plane {
            point: Point {
                x: -1.0,
                y: 0.0,
                z: 0.0,
            },
            normal: Vector {
                x: 1.0,
                y: 0.0,
                z: 0.0,
            },
        };

        let (p, _, t, _) = plane.intersect(&ray).unwrap();

        assert_eq!(-1.0, p.x);
        assert_eq!(-4.0, p.y);
        assert_eq!(0.0, p.z);
        assert_eq!(
            (Vector {
                x: -2.0,
                y: -4.0,
                z: 0.0
            })
            .length(),
            t
        );
    }

    #[test]
    fn plane_ray_intersection_miss() {
        let ray = Ray {
            origin: Point {
                x: 1.0,
                y: 0.0,
                z: 0.0,
            },
            direction: Vector {
                x: 1.0,
                y: 0.0,
                z: 0.0,
            },
        };
        let plane = Plane {
            point: Point {
                x: -1.0,
                y: 0.0,
                z: 0.0,
            },
            normal: Vector {
                x: 1.0,
                y: 0.0,
                z: 0.0,
            },
        };

        let res = plane.intersect(&ray);

        assert!(res.is_none());
    }
}
