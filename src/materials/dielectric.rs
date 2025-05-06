use crate::prelude::*;

/// Represents a dielectric material with a given refraction index.
/// This material simulates the behavior of transparent materials like glass or water.
#[derive(Debug, Copy, Clone)]
pub(crate) struct Dielectric {
    /// The index of refraction of the material.
    refraction_index: f64,
}

/// Computes the reflectance using Schlick's approximation.
///
/// # Arguments
///
/// * `cosine` - The cosine of the angle between the incident ray and the surface normal.
/// * `refraction_index` - The index of refraction of the material.
///
/// # Returns
///
/// The reflectance value, which determines the probability of reflection.
fn reflectance(cosine: f64, refraction_index: f64) -> f64 {
    let mut r0 = (1.0 - refraction_index) / (1.0 + refraction_index);
    r0 = r0 * r0;
    r0 + (1.0 - r0) * f64::powf(1.0 - cosine, 5.0)
}

impl Dielectric {
    /// Creates a new `Dielectric` material with the specified refraction index.
    ///
    /// # Arguments
    ///
    /// * `refraction_index` - The index of refraction of the material.
    ///
    /// # Returns
    ///
    /// A new instance of `Dielectric`.
    pub(crate) fn new(refraction_index: f64) -> Self {
        Self { refraction_index }
    }

    /// Computes how a ray scatters when it hits the dielectric material.
    ///
    /// # Arguments
    ///
    /// * `ray_in` - The incoming ray hitting the material.
    /// * `hit_record` - Information about the hit point, including the normal and whether the hit was on the front face.
    /// * `rng` - A random number generator used for probabilistic decisions.
    ///
    /// # Returns
    ///
    /// An optional tuple containing the attenuation color and the scattered ray.
    /// If `None` is returned, the ray is absorbed.
    pub(crate) fn scatter(
        &self,
        ray_in: &Ray,
        hit_record: &HitRecord,
        rng: &mut ThreadRng,
    ) -> Option<(Color3, Ray)> {
        let ri = if hit_record.front_face {
            1.0 / self.refraction_index
        } else {
            self.refraction_index
        };
        let unit_direction = ray_in.direction.normalize();

        let cos_theta = f64::min(-1.0 * unit_direction.dot(&hit_record.normal), 1.0);
        let sin_theta = (1.0 - cos_theta * cos_theta).sqrt();

        let direction = if ri * sin_theta > 1.0 || reflectance(cos_theta, ri) > gen_01(rng) {
            unit_direction.reflect(&hit_record.normal)
        } else {
            unit_direction.refract(&hit_record.normal, ri)
        };

        let scattered = Ray::new_time(hit_record.point, direction, ray_in.time);
        Some((Color3::new(1.0, 1.0, 1.0), scattered))
    }
}
