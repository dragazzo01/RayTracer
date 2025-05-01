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
pub use std::sync::Arc;
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
    //_ = picture1();
    _ = picture2();
    Ok(())
}

#[allow(dead_code)]
fn picture1() -> Result<(), Error>  {
    let mat_ground = Lambertian::new(Color3::new(0.8, 0.8, 0.0));
    let mat_center = Lambertian::new(Color3::new(0.1, 0.2, 0.5));
    let mat_left = Dielectric::new(1.50);
    let mat_bubble = Dielectric::new(1.00 / 1.50);
    let mat_right = Metal::new(Color3::new(0.8, 0.6, 0.2), 1.0);

    let mut world = HittableList::empty();
    world.add(Arc::new(Sphere::new(Vec3::new( 0.0, -100.5, -1.0), 100.0, mat_ground)));
    world.add(Arc::new(Sphere::new(Vec3::new( 0.0,    0.0, -1.2),   0.5, mat_center)));
    world.add(Arc::new(Sphere::new(Vec3::new(-1.0,    0.0, -1.0),   0.5, mat_left)));
    world.add(Arc::new(Sphere::new(Vec3::new(-1.0,    0.0, -1.0),   0.4, mat_bubble)));
    world.add(Arc::new(Sphere::new(Vec3::new( 1.0,    0.0, -1.0),   0.5, mat_right)));

    let args = CamArgs {
        aspect_ratio : 16. / 9.,
        image_width : 400,
        samples_per_pixel : 100,
        max_depth : 50,
        vfov : 90.,
        look_from : Point3::new(-2., 2., 1.),
        look_at : Point3::new(0., 0., -1.),
        v_up : Vec3::new(0., 1., 0.),
        defocus_angle : 0.,
        focus_dist : 10.,
        thread_num : 4,
    };
    let camera = Camera::initilize(args);
    let _ = camera.render(&world, "images/temp.ppm");
    Ok(())
}

#[allow(dead_code)]
fn picture2() -> Result<(), Error>  {
    let mut world = HittableList::empty();
    
    let mat_ground = Lambertian::new(Color3::new(0.5, 0.5, 0.5));
    let ground = Sphere::new(Point3::new(0.0, -1000.0, 0.0), 1000.0, mat_ground);
    world.add(Arc::new(ground));

    let mut rng = rand::thread_rng();
    for a in -11..11 {
        for b in  -11..11{
            let choose_mat = gen_01(&mut rng);
            let center = Point3::new(a as f64 + 0.9*gen_01(&mut rng), 0.2, b as f64 + 0.9*gen_01(&mut rng));

            if (center - Point3::new(4.0, 0.2, 0.0)).length() > 0.9 {
                if choose_mat < 0.8 {
                    //diffuse
                    let albedo = Color3::random() * Color3::random();
                    let mat = Lambertian::new(albedo);
                    world.add(Arc::new(Sphere::new(center, 0.2, mat)));

                } else if choose_mat < 0.95 {
                    //metal
                    let albedo = Color3::random_bound(0.5, 1.0);
                    let fuzz = gen_bound(0.0, 0.5, &mut rng);
                    let mat = Metal::new(albedo, fuzz);
                    world.add(Arc::new(Sphere::new(center, 0.2, mat)));
                } else {
                    //glass
                    let mat = Dielectric::new(1.5);
                    world.add(Arc::new(Sphere::new(center, 0.2, mat)));
                }
            }
        }
    }

    let mat1 = Dielectric::new(1.5);
    world.add(Arc::new(Sphere::new(Point3::new(0.0, 1.0, 0.0), 1.0, mat1)));
    
    let mat2 = Lambertian::new(Color3::new(0.4, 0.2, 0.1));
    world.add(Arc::new(Sphere::new(Point3::new(-4.0, 1.0, 0.0), 1.0, mat2)));

    let mat3 = Metal::new(Color3::new(0.7, 0.6, 0.5), 0.0);
    world.add(Arc::new(Sphere::new(Point3::new(4.0, 1.0, 0.0), 1.0, mat3)));


    let args = CamArgs {
        aspect_ratio : 16.0 / 9.0,
        image_width : 1200,
        samples_per_pixel : 500,
        max_depth : 50,
        vfov : 20.,
        look_from : Point3::new(13., 2., 3.),
        look_at : Point3::new(0., 0., 0.),
        v_up : Vec3::new(0., 1., 0.),
        defocus_angle : 0.6,
        focus_dist : 10.,
        thread_num : 4,
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
    print!("\r{}\nPixels Remaining: {}         ", bar, lines_remaining);
    print!("\x1b[F");
    std::io::stdout().flush().unwrap(); // Ensure the output is displayed immediately
}