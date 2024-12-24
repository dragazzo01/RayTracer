//use std::fs::File;
mod vec3;
mod ray;
mod hittable;
mod constants;
mod interval;
mod camera;
mod random;
mod material;

//std inserts
pub use std::io::{Write, Error};
pub use std::rc::Rc;
pub use std::fs::File;

//rand
pub use rand::rngs::ThreadRng;
pub use rand::Rng;

//internal
pub use vec3::*;
pub use ray::Ray;
pub use hittable::*;
pub use interval::Interval;
pub use camera::*;
pub use random::*;
pub use constants::*;
pub use material::*;



fn main() -> Result<(), Error> {
    // Construct World
    let mat_ground = Lambertian::new(Color3::new(0.8, 0.8, 0.0));
    let mat_center = Lambertian::new(Color3::new(0.1, 0.2, 0.5));
    let mat_left = Dielectric::new(1.50);
    let mat_bubble = Dielectric::new(1.00 / 1.50);
    let mat_right = Metal::new(Color3::new(0.8, 0.6, 0.2), 1.0);

    let mut world = HittableList::empty();
    world.add(Rc::new(Sphere::new(Vec3::new( 0.0, -100.5, -1.0), 100.0, mat_ground)));
    world.add(Rc::new(Sphere::new(Vec3::new( 0.0,    0.0, -1.2),   0.5, mat_center)));
    world.add(Rc::new(Sphere::new(Vec3::new(-1.0,    0.0, -1.0),   0.5, mat_left)));
    world.add(Rc::new(Sphere::new(Vec3::new(-1.0,    0.0, -1.0),   0.4, mat_bubble)));
    world.add(Rc::new(Sphere::new(Vec3::new( 1.0,    0.0, -1.0),   0.5, mat_right)));
    /* let radius = f64::cos(PI / 4.0);
    let mat_left = Lambertian::new(Color3::new(0.0, 0.0, 1.0));
    let mat_right = Lambertian::new(Color3::new(1.0, 0.0, 0.0));
    
    let mut world = HittableList::empty();
    world.add(Rc::new(Sphere::new(Point3::new(-radius, 0.0, -1.0), radius, mat_left)));
    world.add(Rc::new(Sphere::new(Point3::new(radius, 0.0, -1.0), radius, mat_right))); */
    

    let args = CamArgs {
        aspect_ratio : 16.0 / 9.0,
        image_width : 400,
        samples_per_pixel : 100,
        max_depth : 50,
        vfov : 20.0,
        look_from : Point3::new(-2.0, 2.0, 1.0),
        look_at : Point3::new(0.0, 0.0, -1.0),
        v_up : Vec3::new(0.0, 1.0, 0.0),
    };

    let camera = Camera::initilize(args);
    let _ = camera.render(&world, "images/temp.ppm");
    Ok(())
}

pub fn write_progress(lines_remaining : i32, total_lines : i32) {
    // Create a simple progress bar
    let bar_length = 50; // Length of the progress bar
    let filled_length = ((total_lines - lines_remaining) * bar_length) / total_lines;
    let bar = format!(
                "[{}>{}]",
                "=".repeat(filled_length as usize),  // Filled portion
                " ".repeat((bar_length - filled_length) as usize) // Empty portion
                );

    // Print the progress bar
    print!("\r{}\nScan Lines Remaining: {}         ", bar, lines_remaining);
    print!("\x1b[F");
    std::io::stdout().flush().unwrap(); // Ensure the output is displayed immediately
}