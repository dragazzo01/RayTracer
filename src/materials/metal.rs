use crate::prelude::*;

/// Represents a metallic material.
/// This material reflects light in a specific direction, simulating the behavior of metals.
#[derive(Debug, Copy, Clone)]
pub(crate) struct Metal {
    /// The albedo (reflectivity) of the material, represented as a color.
    albedo: Color3,
    /// The fuzziness of the reflection. A value of 0.0 means perfect reflection, while higher values add randomness.
    fuzz: f64,
}

impl Metal {
    /// Creates a new `Metal` material with the specified albedo and fuzziness.
    /// 
    /// # Arguments
    /// 
    /// * `albedo` - The reflectivity of the material as a `Color3`.
    /// * `fuzz` - The fuzziness of the reflection. Should be in the range [0.0, 1.0].
    /// 
    /// # Returns
    /// 
    /// A new instance of `Metal`.
    pub(crate) fn new(albedo: Color3, fuzz: f64) -> Self {
        Self { albedo, fuzz }
    }

    /// Computes how a ray scatters when it hits the metallic material.
    /// 
    /// # Arguments
    /// 
    /// * `ray_in` - The incoming ray hitting the material.
    /// * `hit_record` - Information about the hit point, including the normal and hit location.
    /// * `rng` - A random number generator used for generating random scatter directions.
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
        let reflected = 
            ray_in.direction.reflect(&hit_record.normal).normalize() 
            + self.fuzz * Vec3::random_unit(rng);
        let scattered = Ray::new_time(hit_record.point, reflected, ray_in.time);

        if scattered.direction.dot(&hit_record.normal) > 0.0 {
            Some((self.albedo, scattered))
        } else {
            None
        }
    }
}