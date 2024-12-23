//use std::fs::File;
pub mod vec3;
pub mod ray;
pub mod hittable;
//mod constants;
pub mod interval;
pub mod camera;
pub mod random;
pub mod constants;

pub use std::io::{Write, Error};
pub use std::rc::Rc;
pub use std::fs::File;

pub use rand::rngs::ThreadRng;
pub use rand::Rng;

pub use vec3::*;
pub use ray::Ray;
pub use hittable::{Sphere, Hittable, HittableList};
pub use interval::Interval;
pub use camera::Camera;
pub use random::*;
pub use constants::*;



fn main() -> Result<(), Error> {
    // Construct World
    let mut world = HittableList::empty();
    world.add(Rc::new(Sphere::new(Vec3::new(0.0, 0.0,-1.0), 0.5)));
    world.add(Rc::new(Sphere::new(Vec3::new(0.0,-100.5,-1.0), 100.0)));
    
    let camera = Camera::initilize(100, 16.0/9.0, 50, 400);
    let _ = camera.render(&world, "images/temp.ppm");
    Ok(())
}