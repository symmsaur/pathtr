extern crate rand;

use material::rand::{Rng, XorShiftRng};

use math::*;

pub struct Material {
    diffuse: f64,
    ior: f64,
    transparency: f64,
}

pub struct ElRay {
    pub ray: Ray,
    pub light: f64,
    pub ior: f64,
    pub count: i32,
}

impl Material {
    pub fn create(diffuse: f64, ior: f64, transparency: f64) -> Material {
        Material {
            diffuse: diffuse,
            ior: ior,
            transparency: transparency,
        }
    }

    pub fn new_ray(&self, ray: ElRay, point: Point, normal: Vector, mut rng: &mut XorShiftRng) -> ElRay{
        let incoming_direction = ray.ray.direction;
        assert!(!incoming_direction.x.is_nan());
        assert!(!incoming_direction.y.is_nan());
        assert!(!incoming_direction.z.is_nan());
        //println!("new_ray");
        let cos_theta = -dot(incoming_direction, normal);
        if rng.gen::<f64>() < reflection_coefficient(ray.ior, self.ior, cos_theta) {
            //println!("reflection");
            ElRay {
                ray: Ray {
                    origin: point,
                    direction: incoming_direction + 2. * cos_theta * normal,
                },
                light: ray.light,
                ior: ray.ior,
                count: ray.count + 1,
            }
        }
        else if self.transparency > 0. && rng.gen::<f64>() < self.transparency {
            //println!("refraction");
            ElRay {
                ray: Ray {
                    origin: point,
                    direction: refraction(ray.ior, self.ior, incoming_direction, normal),
                },
                light: ray.light,
                ior: self.ior,
                count: ray.count + 1,
            }
        }
        else
        {
            //println!("diffuse");
            ElRay {
                ray: gen_ray_n(point, normal, &mut rng),
                light: self.diffuse * ray.light,
                ior: ray.ior,
                count: ray.count + 1,
            }
        }
    }
}

fn refraction(in_ior: f64, out_ior: f64, in_direction: Vector, normal: Vector) -> Vector {
    let r = in_ior / out_ior;
    let c = - dot(normal, in_direction);
    let a = 1. - r * r * (1. - c * c);
    let refracted = if a < 0. {
        // Total reflection perhaps?
        //println!("Total reflection1");
        -in_direction
    }
    else {
        r * in_direction + (r * c - f64::sqrt(a)) * normal
    };

    if dot(refracted, normal) < 0. {
        //println!("Refarcted");
        refracted
    }
    else {
        // Total reflection
        println!("Total reflection");
        -refracted
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn refraction_straight_test() {
        let in_direction = Vector {x: 1., y: 0., z: 0.};
        let normal = Vector {x: -1., y: 0., z: 0.};
        let res = refraction(1., 1.5, in_direction, normal);
        assert_eq!(1., res.x);
        assert_eq!(0., res.y);
        assert_eq!(0., res.z);
    }

    #[test]
    fn refraction_straight_test_2() {
        let in_direction = Vector {x: 1., y: 0., z: 0.};
        let normal = Vector {x: -1., y: 0., z: 0.};
        let res = refraction(1.5, 1.0, in_direction, normal);
        assert_eq!(1., res.x);
        assert_eq!(0., res.y);
        assert_eq!(0., res.z);
    }

    #[test]
    fn refraction_angled() {
        let in_direction = (Vector {x: 1., y: 1., z: 0.}).normalize();
        let normal = Vector {x: -1., y: 0., z: 0.};
        let res = refraction(1.0, 1.5, in_direction, normal);
        assert!(res.x > 0.);
        assert!(res.y < in_direction.y);
    }

    #[test]
    fn refaction_total_reflection() {
        let in_direction = (Vector {x: 1., y: 1., z: 0.}).normalize();
        let normal = Vector {x: -1., y: 0., z: 0.};
        let res = refraction(1.5, 1.0, in_direction, normal);
        assert_eq!(-in_direction.x, res.x);
        //assert!(res.y < in_direction.y);
    }
}

fn reflection_coefficient(in_ior: f64, out_ior: f64, cos_theta: f64) -> f64 {
    // Using Schlick's approximation
    let r0 = f64::powi((in_ior - out_ior) / (in_ior + out_ior), 2);
    return r0 + (1. - r0) * f64::powi(1. - cos_theta, 5);
}

fn gen_ray_n(start: Point, normal: Vector, rng: &mut XorShiftRng) -> Ray
{
    // uniform sample over half sphere
    loop {
        let x = 2.0 * rng.gen::<f64>() - 1.0;
        let y = 2.0 * rng.gen::<f64>() - 1.0;
        let z = 2.0 * rng.gen::<f64>() - 1.0;
        let v = Vector {x, y, z};
        if v.square_length() < 1.0 {
            if dot(v, normal) > 0.0 {
                return Ray{origin: start, direction: v}
            }
            else {
                return Ray{origin: start, direction: -v}
            }
        }
    }
}
