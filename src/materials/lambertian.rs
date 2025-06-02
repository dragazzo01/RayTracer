use crate::prelude::*;

/// Represents a Lambertian (diffuse) material.
/// This material scatters light uniformly in all directions.
#[derive(Debug, Clone)]
pub(crate) struct Lambertian {
    /// The albedo (reflectivity) of the material, represented as a color.
    texture: Arc<Textures>,
}

impl Lambertian {
    /// Creates a new `Lambertian` material with the specified albedo.
    ///
    /// # Arguments
    ///
    /// * `albedo` - The reflectivity of the material as a `Color3`.
    ///
    /// # Returns
    ///
    /// A new instance of `Lambertian`.
    pub(crate) fn solid(albedo: Color3) -> Self {
        Self { texture : Textures::solid_color(albedo) }
    }

    pub(crate) fn new(texture: Arc<Textures>) -> Self {
        Self { texture }
    }

    /// Computes how a ray scatters when it hits the Lambertian material.
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
        let scatter_direction = {
            let res = hit_record.normal + Vec3::random_unit(rng);
            if res.near_zero() {
                hit_record.normal
            } else {
                res
            }
        };

        let attenuation = self.texture.value(hit_record.u, hit_record.v, &hit_record.point);
        let scattered = Ray::new_time(hit_record.point, scatter_direction, ray_in.time);
        Some((attenuation, scattered))
    }
}
