use std::ops::Add;
use std::ops::AddAssign;
use std::ops::Mul;

use rand::Rng;

use crate::math::*;

#[derive(Copy, Clone)]
pub struct Color {
    pub red: f32,
    pub green: f32,
    pub blue: f32,
}

impl Mul for Color {
    type Output = Color;
    fn mul(self, rhs: Color) -> Color {
        Color {
            red: self.red * rhs.red,
            green: self.green * rhs.green,
            blue: self.blue * rhs.blue,
        }
    }
}

impl Mul<f32> for Color {
    type Output = Color;
    fn mul(self, rhs: f32) -> Color {
        Color {
            red: self.red * rhs,
            green: self.green * rhs,
            blue: self.blue * rhs,
        }
    }
}

impl Add for Color {
    type Output = Color;
    fn add(self, rhs: Color) -> Color {
        Color {
            red: self.red + rhs.red,
            green: self.green + rhs.green,
            blue: self.blue + rhs.blue,
        }
    }
}

impl AddAssign for Color {
    fn add_assign(&mut self, rhs: Color) {
        *self = Color {
            red: self.red + rhs.red,
            green: self.green + rhs.green,
            blue: self.blue + rhs.blue,
        }
    }
}

pub struct Material {
    diffuse: Color,
    // Emissivity takes priority
    emissive: Color,
    ior: f32,
    transparency: f32,
}

pub struct LightRay {
    pub ray: Ray,
    pub light: Color,
    pub ior: f32,
    pub count: i32,
    pub done: bool,
}

impl Material {
    pub fn create(diffuse: Color, ior: f32, transparency: f32) -> Material {
        Material {
            diffuse,
            ior,
            transparency,
            emissive: Color {
                red: 0.0,
                green: 0.0,
                blue: 0.0,
            },
        }
    }

    pub fn create_emissive(emissive: Color) -> Material {
        Material {
            diffuse: Color {
                red: 0.0,
                green: 0.0,
                blue: 0.0,
            },
            ior: 1.0,
            transparency: 0.0,
            emissive,
        }
    }

    pub fn create_glass() -> Material {
        Material {
            diffuse: Color {
                red: 0.0,
                green: 0.0,
                blue: 0.0,
            },
            ior: 1.5,
            transparency: 1.0,
            emissive: Color {
                red: 0.0,
                green: 0.0,
                blue: 0.0,
            },
        }
    }

    pub fn create_colored_1() -> Material {
        Material {
            diffuse: Color {
                red: 0.2,
                green: 1.0,
                blue: 1.0,
            },
            ior: 1.5,
            transparency: 0.0,
            emissive: Color {
                red: 0.0,
                green: 0.0,
                blue: 0.0,
            },
        }
    }

    pub fn create_colored_2() -> Material {
        Material {
            diffuse: Color {
                red: 1.0,
                green: 0.8,
                blue: 0.2,
            },
            ior: 1.5,
            transparency: 0.0,
            emissive: Color {
                red: 0.0,
                green: 0.0,
                blue: 0.0,
            },
        }
    }

    pub fn create_colored_3() -> Material {
        Material {
            diffuse: Color {
                red: 0.9,
                green: 0.6,
                blue: 1.0,
            },
            ior: 1.5,
            transparency: 0.0,
            emissive: Color {
                red: 0.0,
                green: 0.0,
                blue: 0.0,
            },
        }
    }

    pub fn new_ray<R: Rng + ?Sized>(
        &self,
        ray: LightRay,
        point: Point,
        normal: Vector,
        inside: bool,
        mut rng: &mut R,
    ) -> LightRay {
        let incoming_direction = ray.ray.direction;
        let cos_theta = -dot(incoming_direction, normal);
        if self.emissive.red != 0.0 || self.emissive.blue != 0.0 || self.emissive.green != 0.0 {
            LightRay {
                ray: Ray {
                    origin: point,
                    direction: normal,
                },
                light: ray.light * self.emissive,
                ior: ray.ior,
                count: ray.count + 1,
                done: true,
            }
        } else if rng.gen::<f32>() < reflection_coefficient(ray.ior, self.ior, cos_theta) {
            LightRay {
                ray: Ray {
                    origin: point,
                    direction: reflection(incoming_direction, normal),
                },
                light: ray.light,
                ior: ray.ior,
                count: ray.count + 1,
                done: false,
            }
        } else if self.transparency > 0. && rng.gen::<f32>() < self.transparency {
            let mut new_ray = LightRay {
                ray: Ray {
                    origin: point,
                    direction: refraction(ray.ior, self.ior, incoming_direction, normal),
                },
                light: ray.light,
                ior: if inside { 1.0 } else { self.ior },
                count: ray.count + 1,
                done: false,
            };
            new_ray.ray.origin = translate(new_ray.ray.origin, 1e-8 * new_ray.ray.direction);
            new_ray
        } else {
            LightRay {
                ray: generate_half_sphere_ray(point, normal, &mut rng),
                light: self.diffuse * ray.light,
                ior: ray.ior,
                count: ray.count + 1,
                done: false,
            }
        }
    }
}

fn refraction(in_ior: f32, out_ior: f32, in_direction: Vector, normal: Vector) -> Vector {
    let r = in_ior / out_ior;
    let cos_theta = -dot(normal, in_direction);
    let sin2_theta = r * r * (1.0 - cos_theta * cos_theta);
    if sin2_theta > 1.0 {
        // Total reflection
        reflection(in_direction, normal)
    } else {
        // Refraction
        r * in_direction + (r * cos_theta - f32::sqrt(1.0 - sin2_theta)) * normal
    }
}

fn reflection(direction: Vector, normal: Vector) -> Vector {
    let cos_theta = -dot(direction, normal);
    direction + 2. * cos_theta * normal
}

fn reflection_coefficient(in_ior: f32, out_ior: f32, cos_theta: f32) -> f32 {
    // Using Schlick's approximation
    let r0 = f32::powi((in_ior - out_ior) / (in_ior + out_ior), 2);
    return r0 + (1. - r0) * f32::powi(1. - cos_theta, 5);
}

fn generate_half_sphere_ray<R: Rng + ?Sized>(start: Point, normal: Vector, rng: &mut R) -> Ray {
    // uniform sample over half sphere
    loop {
        let x = 2.0 * rng.gen::<f32>() - 1.0;
        let y = 2.0 * rng.gen::<f32>() - 1.0;
        let z = 2.0 * rng.gen::<f32>() - 1.0;
        let v = Vector { x, y, z };
        if v.square_length() < 1.0 {
            if dot(v, normal) > 0.0 {
                return Ray {
                    origin: start,
                    direction: v.normalize(),
                };
            } else {
                return Ray {
                    origin: start,
                    direction: -v.normalize(),
                };
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn refraction_straight_test() {
        let in_direction = Vector {
            x: 1.,
            y: 0.,
            z: 0.,
        };
        let normal = Vector {
            x: -1.,
            y: 0.,
            z: 0.,
        };
        let res = refraction(1., 1.5, in_direction, normal);
        assert_eq!(1., res.x);
        assert_eq!(0., res.y);
        assert_eq!(0., res.z);
    }

    #[test]
    fn refraction_straight_test_2() {
        let in_direction = Vector {
            x: 1.,
            y: 0.,
            z: 0.,
        };
        let normal = Vector {
            x: -1.,
            y: 0.,
            z: 0.,
        };
        let res = refraction(1.5, 1.0, in_direction, normal);
        assert_eq!(1., res.x);
        assert_eq!(0., res.y);
        assert_eq!(0., res.z);
    }

    #[test]
    fn refraction_angled() {
        let in_direction = (Vector {
            x: 1.,
            y: 1.,
            z: 0.,
        })
        .normalize();
        let normal = Vector {
            x: -1.,
            y: 0.,
            z: 0.,
        };
        let res = refraction(1.0, 1.5, in_direction, normal);
        assert!(res.x > 0.);
        assert!(res.y < in_direction.y);
    }

    #[test]
    fn refaction_total_reflection() {
        let in_direction = (Vector {
            x: 1.,
            y: 1.,
            z: 0.,
        })
        .normalize();
        let normal = Vector {
            x: -1.,
            y: 0.,
            z: 0.,
        };
        let res = refraction(1.5, 1.0, in_direction, normal);
        assert!(-in_direction.x - res.x < 1e-9);
        //assert!(res.y < in_direction.y);
    }
}
