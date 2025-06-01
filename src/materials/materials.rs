use crate::materials::dielectric::Dielectric;
use crate::materials::lambertian::Lambertian;
use crate::materials::metal::Metal;
use crate::materials::emmiter::Diffuse;
use crate::materials::isotropic::Isotropic;
use crate::prelude::*;

/// Represents the different types of materials that can be used in the ray tracer.
/// Each material has its own scattering behavior.
#[derive(Debug, Clone)]
pub enum Materials {
    /// A Lambertian (diffuse) material.
    Lambertian(Lambertian),
    /// A Dielectric (transparent) material.
    Dielectric(Dielectric),
    /// A Metallic material.
    Metal(Metal),
    /// Diffuse emitter
    Diffuse(Diffuse),

    Isotropic(Isotropic)
}

impl Materials {
    /// Creates a new Lambertian material.
    ///
    /// # Arguments
    ///
    /// * `albedo` - The reflectivity of the material as a `Color3`.
    ///
    /// # Returns
    ///
    /// A `Materials` enum variant containing a Lambertian material.
    pub fn lambertian_solid(albedo: Color3) -> Rc<Self> {
        Rc::new(Self::Lambertian(Lambertian::solid(albedo)))
    }

    pub fn lambertian(texture: Rc<Textures>) -> Rc<Self> {
        Rc::new(Self::Lambertian(Lambertian::new(texture)))
    }

    /// Creates a new Metallic material.
    ///
    /// # Arguments
    ///
    /// * `albedo` - The reflectivity of the material as a `Color3`.
    /// * `fuzz` - The fuzziness of the reflection. Should be in the range [0.0, 1.0].
    ///
    /// # Returns
    ///
    /// A `Materials` enum variant containing a Metallic material.
    pub fn metal(albedo: Color3, fuzz: f64) -> Rc<Self> {
        let fuzz = if fuzz < 1.0 { fuzz } else { 1.0 };
        Rc::new(Self::Metal(Metal::new(albedo, fuzz)))
    }

    /// Creates a new Dielectric material.
    ///
    /// # Arguments
    ///
    /// * `refraction_index` - The index of refraction of the material.
    ///
    /// # Returns
    ///
    /// A `Materials` enum variant containing a Dielectric material.
    pub fn dielectric(refraction_index: f64) -> Rc<Self> {
        Rc::new(Self::Dielectric(Dielectric::new(refraction_index)))
    }

    // pub fn emmiter(texture : Rc<Textures>) -> Rc<Self> {
    //     Rc::new(Self::Diffuse(Diffuse::new(texture)))
    // }

    pub fn emmiter_solid(color: Color3) -> Rc<Self> {
        Rc::new(Self::Diffuse(Diffuse::solid(color)))
    }

    // pub fn isotropic(tex: Rc<Textures>) -> Rc<Self> {
    //     Rc::new(Self::Isotropic(Isotropic::new(tex)))
    // }

    pub fn isotropic_solid(albedo: Color3) -> Rc<Self> {
        Rc::new(Self::Isotropic(Isotropic::solid(albedo)))
    }

    pub fn emitted(&self, u : f64, v : f64, p : &Point3) -> Color3 {
        match self {
            Self::Diffuse(d) => d.emitted(u, v, p),
            _ => Color3::zero(),
        }
    }

    /// Computes how a ray scatters when it hits the material.
    ///
    /// # Arguments
    ///
    /// * `ray` - The incoming ray hitting the material.
    /// * `hit_record` - Information about the hit point, including the normal and hit location.
    /// * `rng` - A random number generator used for probabilistic decisions.
    ///
    /// # Returns
    ///
    /// An optional tuple containing the attenuation color and the scattered ray.
    /// If `None` is returned, the ray is absorbed.
    pub fn scatter(
        &self,
        ray: &Ray,
        hit_record: &HitRecord,
        rng: &mut ThreadRng,
    ) -> Option<(Color3, Ray)> {
        match self {
            Self::Lambertian(l) => l.scatter(ray, hit_record, rng),
            Self::Dielectric(d) => d.scatter(ray, hit_record, rng),
            Self::Metal(m) => m.scatter(ray, hit_record, rng),
            Self::Isotropic(mat) => mat.scatter(ray, hit_record, rng),
            _ => None,
        }
    }
}
