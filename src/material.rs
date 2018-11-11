pub fn reflection_coefficient(refractive_index: f64, cos_theta: f64) -> f64 {
    // Using Schlick's approximation
    let r0 = (1. - refractive_index) / (1. + refractive_index)
        * (1. - refractive_index) / (1. + refractive_index);
    return r0 + (1. - r0) * f64::powi(1. - cos_theta, 5);
}
