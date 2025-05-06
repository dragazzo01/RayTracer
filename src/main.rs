mod prelude;
mod vec3;
mod ray;
mod hittables;
mod constants;
mod interval;
mod camera;
mod random;
mod materials;

use crate::prelude::*;
use crate::camera::{CamArgs, Camera};



fn main() -> Result<(), Error> {              
    //_ = temp1();
    //_ = final1();
    _ = temp2();
    Ok(())
}

#[allow(dead_code)]
fn temp1() -> Result<(), Error>  {

    let mat_ground = Materials::lambertian(Color3::new(0.8, 0.8, 0.0));
    let mat_center = Materials::lambertian(Color3::new(0.1, 0.2, 0.5));
    let mat_left = Materials::dielectric(1.50);
    let mat_bubble = Materials::dielectric(1.00 / 1.50);
    let mat_right = Materials::metal(Color3::new(0.8, 0.6, 0.2), 1.0);

    let mut world = HittableList::empty();
    world.add_static_sphere(Vec3::new( 0.0, -100.5, -1.0), 100.0, mat_ground);
    world.add_static_sphere(Vec3::new( 0.0,    0.0, -1.2),   0.5, mat_center);
    world.add_static_sphere(Vec3::new(-1.0,    0.0, -1.0),   0.5, mat_left);
    world.add_static_sphere(Vec3::new(-1.0,    0.0, -1.0),   0.4, mat_bubble);
    world.add_static_sphere(Vec3::new( 1.0,    0.0, -1.0),   0.5, mat_right);

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
    let _ = camera.render(&world, "images/temp1.ppm");
    Ok(())
}

#[allow(dead_code)]
fn final1() -> Result<(), Error>  {
    let mut world = HittableList::empty();
    
    let mat_ground = Materials::lambertian(Color3::new(0.5, 0.5, 0.5));
    world.add_static_sphere(Point3::new(0.0, -1000.0, 0.0), 1000.0, mat_ground);

    let mut rng = rand::thread_rng();
    for a in -11..11 {
        for b in  -11..11{
            let choose_mat = gen_01(&mut rng);
            let center = Point3::new(a as f64 + 0.9*gen_01(&mut rng), 0.2, b as f64 + 0.9*gen_01(&mut rng));

            if (center - Point3::new(4.0, 0.2, 0.0)).length() > 0.9 {
                if choose_mat < 0.8 {
                    //diffuse
                    let albedo = Color3::random(&mut rng) * Color3::random(&mut rng);
                    let mat = Materials::lambertian(albedo);
                    world.add_static_sphere(center, 0.2, mat);

                } else if choose_mat < 0.95 {
                    //metal
                    let albedo = Color3::random_bound(0.5, 1.0, &mut rng);
                    let fuzz = gen_bound(0.0, 0.5, &mut rng);
                    let mat = Materials::metal(albedo, fuzz);
                    world.add_static_sphere(center, 0.2, mat);
                } else {
                    //glass
                    let mat = Materials::dielectric(1.5);
                    world.add_static_sphere(center, 0.2, mat);
                }
            }
        }
    }

    let mat1 = Materials::dielectric(1.5);
    world.add_static_sphere(Point3::new(0.0, 1.0, 0.0), 1.0, mat1);
    
    let mat2 = Materials::lambertian(Color3::new(0.4, 0.2, 0.1));
    world.add_static_sphere(Point3::new(-4.0, 1.0, 0.0), 1.0, mat2);

    let mat3 = Materials::metal(Color3::new(0.7, 0.6, 0.5), 0.0);
    world.add_static_sphere(Point3::new(4.0, 1.0, 0.0), 1.0, mat3);


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
        thread_num : 8,
    };

    let camera = Camera::initilize(args);
    let _ = camera.render(&world, "images/final1.ppm");
    Ok(())
}

#[allow(dead_code)]
fn temp2() -> Result<(), Error>  {
    let mut world = HittableList::empty();
    
    let mat_ground = Materials::lambertian(Color3::new(0.5, 0.5, 0.5));
    world.add_static_sphere(Point3::new(0.0, -1000.0, 0.0), 1000.0, mat_ground);

    let mut rng = rand::thread_rng();
    for a in -11..11 {
        for b in  -11..11{
            let choose_mat = gen_01(&mut rng);
            let center = Point3::new(a as f64 + 0.9*gen_01(&mut rng), 0.2, b as f64 + 0.9*gen_01(&mut rng));

            if (center - Point3::new(4.0, 0.2, 0.0)).length() > 0.9 {
                if choose_mat < 0.8 {
                    //diffuse
                    let albedo = Color3::random(&mut rng) * Color3::random(&mut rng);
                    let mat = Materials::lambertian(albedo);
                    //let center2 = center + Vec3::new(0., gen_bound(0., 0.5, &mut rng), 0.);
                    //world.add_moving_sphere(center, center2, 0.2, mat);
                    world.add_static_sphere(center, 0.2, mat);

                } else if choose_mat < 0.95 {
                    //metal
                    let albedo = Color3::random_bound(0.5, 1.0, &mut rng);
                    let fuzz = gen_bound(0.0, 0.5, &mut rng);
                    let mat = Materials::metal(albedo, fuzz);
                    world.add_static_sphere(center, 0.2, mat);
                } else {
                    //glass
                    let mat = Materials::dielectric(1.5);
                    world.add_static_sphere(center, 0.2, mat);
                }
            }
        }
    }

    let mat1 = Materials::dielectric(1.5);
    world.add_static_sphere(Point3::new(0.0, 1.0, 0.0), 1.0, mat1);
    
    let mat2 = Materials::lambertian(Color3::new(0.4, 0.2, 0.1));
    world.add_static_sphere(Point3::new(-4.0, 1.0, 0.0), 1.0, mat2);

    let mat3 = Materials::metal(Color3::new(0.7, 0.6, 0.5), 0.0);
    world.add_static_sphere(Point3::new(4.0, 1.0, 0.0), 1.0, mat3);


    let args = CamArgs {
        aspect_ratio : 16.0 / 9.0,
        image_width : 400,
        samples_per_pixel : 100,
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
    let _ = camera.render(&world, "images/temp2.ppm");
    Ok(())
}
