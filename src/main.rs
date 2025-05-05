mod prelude;
mod vec3;
mod ray;
mod hittable;
mod constants;
mod interval;
mod camera;
mod random;
mod material;

use crate::prelude::*;
//use pprof::ProfilerGuardBuilder;



fn main() -> Result<(), Error> {
    // let guard = pprof::ProfilerGuardBuilder::default()
    //             .frequency(100)
    //             .blocklist(&["libc", "libgcc", "pthread", "vdso", "perf"])
    //             .build()
    //             .unwrap();
                
    //_ = picture1();
    _ = picture2();
    // if let Ok(report) = guard.report().build() {
    //     let file = std::fs::File::create("flamegraph.svg").unwrap();
    //     report.flamegraph(file).unwrap();
    // }

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
    world.add(Sphere::new(Vec3::new( 0.0, -100.5, -1.0), 100.0, mat_ground));
    world.add(Sphere::new(Vec3::new( 0.0,    0.0, -1.2),   0.5, mat_center));
    world.add(Sphere::new(Vec3::new(-1.0,    0.0, -1.0),   0.5, mat_left));
    world.add(Sphere::new(Vec3::new(-1.0,    0.0, -1.0),   0.4, mat_bubble));
    world.add(Sphere::new(Vec3::new( 1.0,    0.0, -1.0),   0.5, mat_right));

    let args = CamArgs {
        aspect_ratio : 16. / 9.,
        image_width : 800,
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
    world.add(ground);

    let mut rng = rand::thread_rng();
    for a in -11..11 {
        for b in  -11..11{
            let choose_mat = gen_01(&mut rng);
            let center = Point3::new(a as f64 + 0.9*gen_01(&mut rng), 0.2, b as f64 + 0.9*gen_01(&mut rng));

            if (center - Point3::new(4.0, 0.2, 0.0)).length() > 0.9 {
                if choose_mat < 0.8 {
                    //diffuse
                    let albedo = Color3::random(&mut rng) * Color3::random(&mut rng);
                    let mat = Lambertian::new(albedo);
                    world.add(Sphere::new(center, 0.2, mat));

                } else if choose_mat < 0.95 {
                    //metal
                    let albedo = Color3::random_bound(0.5, 1.0, &mut rng);
                    let fuzz = gen_bound(0.0, 0.5, &mut rng);
                    let mat = Metal::new(albedo, fuzz);
                    world.add(Sphere::new(center, 0.2, mat));
                } else {
                    //glass
                    let mat = Dielectric::new(1.5);
                    world.add(Sphere::new(center, 0.2, mat));
                }
            }
        }
    }

    let mat1 = Dielectric::new(1.5);
    world.add(Sphere::new(Point3::new(0.0, 1.0, 0.0), 1.0, mat1));
    
    let mat2 = Lambertian::new(Color3::new(0.4, 0.2, 0.1));
    world.add(Sphere::new(Point3::new(-4.0, 1.0, 0.0), 1.0, mat2));

    let mat3 = Metal::new(Color3::new(0.7, 0.6, 0.5), 0.0);
    world.add(Sphere::new(Point3::new(4.0, 1.0, 0.0), 1.0, mat3));


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
        thread_num : 1,
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