use std::ops::*;
use std::io::{Error, Write};

#[derive(Debug, Copy, Clone)]
pub struct Vec3 {
    pub x : f64,
    pub y : f64,
    pub z : f64,
}
pub type Point3 = Vec3;
pub type Color3 = Vec3;

impl Add for Vec3 {
    type Output = Vec3;

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

    fn mul(self, vec: Vec3) -> Vec3 {
        vec * self
    } 
}

impl Div<f64> for Vec3 {
    type Output = Vec3;

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

    fn div(self, vec: Vec3) -> Vec3 {
        vec / self
    } 
}

// Implement the `Index` trait for indexing
impl Index<usize> for Vec3 {
    type Output = f64;

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
    pub fn zero() -> Self {
        Self {x : 0.0, y: 0.0, z: 0.0}
    }

    pub fn new(x : f64, y : f64, z : f64) -> Self {
        Self {x, y, z}
    }

    pub fn norm(self) -> f64 {
        self.x*self.x + self.y*self.y + self.z*self.z
    }

    pub fn length(self) -> f64 {
        self.norm().sqrt()
    }

    pub fn dot(self, other : Vec3) -> f64 {
        self.x * other.x + self.y * other.y + self.z * other.z
    }

    pub fn cross(self, other: Vec3) -> Vec3 {
        Vec3 {
            x: self.y * other.z - self.z * other.y,
            y: self.z * other.x - self.x * other.z,
            z: self.x * other.y - self.y * other.x,
        }
    }

    pub fn normalize(self) -> Vec3 {
        self / self.length()
    }

    pub fn write_vec<W: Write>(&self, file: &mut W) -> Result<(), Error> {
        file.write_all(format!("{} {} {}", self.x, self.y, self.z).as_bytes())?;
        Ok(())
    }

    pub fn writeln_vec<W: Write>(&self, file: &mut W) -> Result<(), Error> {
        self.write_vec(file)?;
        file.write_all(b"\n")?;
        Ok(())
    }

    pub fn write_color<W: Write>(&self, file: &mut W) -> Result<(), Error> {
        let ir = (255.999 * self.x) as i32;
        let ig = (255.999 * self.y) as i32;
        let ib = (255.999 * self.z) as i32;

        file.write_all(format!("{} {} {}", ir, ig, ib).as_bytes())?;
        Ok(())
    }

    pub fn writeln_color<W: Write>(&self, file: &mut W) -> Result<(), Error> {
        self.write_color(file)?;
        file.write_all(b"\n")?;
        Ok(())
    }
}