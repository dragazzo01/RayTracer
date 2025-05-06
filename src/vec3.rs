use crate::prelude::*;

use std::ops::{
    Add,
    Sub,
    Mul,
    Div,
    Index
};

/// A 3D vector with `x`, `y`, and `z` components.
/// 
/// This struct is used to represent points, colors, and directions in 3D space.
#[derive(Debug, Copy, Clone)]
pub struct Vec3 {
    /// The x-component of the vector.
    pub x : f64,
    /// The y-component of the vector.
    pub y : f64,
    /// The z-component of the vector.
    pub z : f64,
}

/// Type alias for a 3D point.
pub type Point3 = Vec3;
/// Type alias for a 3D color.
pub type Color3 = Vec3;

impl Add for Vec3 {
    type Output = Vec3;

    /// Adds two vectors component-wise.
    fn add(self, other: Vec3) -> Vec3 {
        Vec3 {
            x: self.x + other.x,
            y: self.y + other.y,
            z: self.z + other.z,
        }
    }
}

impl Sub for Vec3 {
    type Output = Vec3;

    /// Subtracts two vectors component-wise.
    fn sub(self, other: Vec3) -> Vec3 {
        Vec3 {
            x: self.x - other.x,
            y: self.y - other.y,
            z: self.z - other.z,
        }
    }
}

impl Mul<Vec3> for Vec3 {
    type Output = Vec3;

    /// Multiplies two vectors component-wise.
    fn mul(self, other: Vec3) -> Vec3 {
        Vec3 {
            x: self.x * other.x,
            y: self.y * other.y,
            z: self.z * other.z,
        }
    }
}

// Implement the `Mul` trait for `Vec3` to scale by a scalar
impl Mul<f64> for Vec3 {
    type Output = Vec3;

    /// Scales the vector by a scalar.
    fn mul(self, scalar: f64) -> Vec3 {
        Vec3 {
            x: self.x * scalar,
            y: self.y * scalar,
            z: self.z * scalar,
        }
    }
}

impl Mul<Vec3> for f64 {
    type Output = Vec3;

    /// Scales the vector by a scalar (commutative).
    fn mul(self, vec: Vec3) -> Vec3 {
        vec * self
    } 
}

impl Div<f64> for Vec3 {
    type Output = Vec3;

    /// Divides the vector by a scalar.
    fn div(self, scalar: f64) -> Vec3 {
        Vec3 {
            x: self.x / scalar,
            y: self.y / scalar,
            z: self.z / scalar,
        }
    }
}

impl Div<Vec3> for f64 {
    type Output = Vec3;

    /// Divides a scalar by the vector component-wise.
    fn div(self, vec: Vec3) -> Vec3 {
        vec / self
    } 
}

// Implement the `Index` trait for indexing
impl Index<usize> for Vec3 {
    type Output = f64;

    /// Indexes the vector components by position (0 for x, 1 for y, 2 for z).
    fn index(&self, index: usize) -> &f64 {
        match index {
            0 => &self.x,
            1 => &self.y,
            2 => &self.z,
            _ => panic!("Index out of bounds for Vec3"),
        }
    }
}

impl Vec3 {
    /// Creates a vector with all components set to zero.
    pub fn zero() -> Self {
        Self {x : 0.0, y: 0.0, z: 0.0}
    }

    /// Creates a new vector with the specified components.
    pub fn new(x : f64, y : f64, z : f64) -> Self {
        Self {x, y, z}
    }

    /// Generates a random vector with components in the range [0, 1).
    pub fn random(rng : &mut ThreadRng) -> Self {
        Self {
            x : gen_01(rng),
            y : gen_01(rng),
            z: gen_01(rng),
        }
    }

    /// Generates a random vector with components in the specified range.
    pub fn random_bound(min : f64, max : f64, rng: &mut ThreadRng) -> Self {
        Self {
            x : crate::random::gen_bound(min, max, rng),
            y : crate::random::gen_bound(min, max,rng),
            z: crate::random::gen_bound(min, max, rng),
        }
    }

    /// Generates a random unit vector.
    pub fn random_unit(rng: &mut ThreadRng) -> Self {
        loop {
            let canidate = Self::random_bound(-1.0, 1.0, rng);
            let norm = canidate.norm();
            if 1e-60 < norm && norm <= 1.0 {
                return canidate.normalize();
            }
        }
    }

    /// Generates a random vector in the hemisphere around a given normal.
    pub fn random_hemisphere(normal : &Self, rng: &mut ThreadRng) -> Self {
        let on_sphere = Self::random_unit(rng);
        if on_sphere.dot(normal) > 0.0 {
            on_sphere
        } else {
            -1.0 * on_sphere
        }
    }

    /// Generates a random vector in a disk on the XY plane.
    pub fn random_disk(rng: &mut ThreadRng) -> Self {
        loop {
            let p = Self::new(
                        crate::random::gen_bound(-1.0, 1.0, rng), 
                        crate::random::gen_bound(-1.0, 1.0, rng), 
                        0.0
                    );
            if p.norm() < 1.0 {
                return p;
            }
        }
    }

    /// Checks if the vector is near zero in all components.
    pub fn near_zero(&self) -> bool {
        let s = 1e-8;
        self.x < s && self.y < s && self.z < s
    }

    /// Reflects the vector around a given normal.
    pub fn reflect(&self, n : &Self) -> Self {
        *self - 2.0 * self.dot(n) * *n
    }

    /// Refracts the vector through a surface with a given refraction ratio.
    pub fn refract(&self, n : &Self, etai_over_etat : f64) -> Self {
        let cos_theta = f64::min((-1.0 * *self).dot(n), 1.0);
        let r_out_perp = etai_over_etat * (*self + cos_theta * *n);
        let r_out_parallel = -(1.0 - r_out_perp.norm()).abs().sqrt() * *n;
        r_out_perp + r_out_parallel
    }

    /// Computes the squared length (norm) of the vector.
    pub fn norm(self) -> f64 {
        self.x*self.x + self.y*self.y + self.z*self.z
    }

    /// Computes the length (magnitude) of the vector.
    pub fn length(self) -> f64 {
        self.norm().sqrt()
    }

    /// Computes the dot product of two vectors.
    pub fn dot(self, other : &Self) -> f64 {
        self.x * other.x + self.y * other.y + self.z * other.z
    }

    /// Computes the cross product of two vectors.
    pub fn cross(self, other: &Self) -> Self {
        Self {
            x: self.y * other.z - self.z * other.y,
            y: self.z * other.x - self.x * other.z,
            z: self.x * other.y - self.y * other.x,
        }
    }

    /// Normalizes the vector to have a length of 1.
    pub fn normalize(self) -> Self {
        self / self.length()
    }

    /// Writes the vector components to a file without a newline.
    pub fn write_vec<W: Write>(&self, file: &mut W) -> Result<(), Error> {
        file.write_all(format!("{} {} {}", self.x, self.y, self.z).as_bytes())?;
        Ok(())
    }

    /// Writes the vector components to a file with a newline.
    pub fn writeln_vec<W: Write>(&self, file: &mut W) -> Result<(), Error> {
        self.write_vec(file)?;
        file.write_all(b"\n")?;
        Ok(())
    }

    /// Converts a linear color component to gamma-corrected space.
    fn linear_to_gamma(linear_component : f64) -> f64 {
        if linear_component > 0.0 {
            linear_component.sqrt()
        } else {
            0.0
        }
    }

    /// Writes the vector as a color to a file, applying gamma correction.
    pub fn write_color<W: Write>(&self, file: &mut W) -> Result<(), Error> {
        let r = Self::linear_to_gamma(self.x);
        let g = Self::linear_to_gamma(self.y);
        let b = Self::linear_to_gamma(self.z);
        
        let intenstity = Interval::new(0.0, 0.999);
        let ir = (256.0 * intenstity.clamp(r)) as i32;
        let ig = (256.0 * intenstity.clamp(g)) as i32;
        let ib = (256.0 * intenstity.clamp(b)) as i32;

        file.write_all(format!("{} {} {}", ir, ig, ib).as_bytes())?;
        Ok(())
    }

    /// Writes the vector as a color to a file with a newline, applying gamma correction.
    pub fn writeln_color<W: Write>(&self, file: &mut W) -> Result<(), Error> {
        self.write_color(file)?;
        file.write_all(b"\n")?;
        Ok(())
    }
}