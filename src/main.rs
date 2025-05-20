mod camera;
mod constants;
mod hittables;
mod interval;
mod materials;
mod prelude;
mod random;
mod ray;
mod vec3;
mod texture;

use crate::camera::{CamArgs, Camera};
use crate::hittables::hittables::HittableList;
use crate::prelude::*;

fn main() -> Result<(), Error> {
    match 6 {
        0 => temp1()?,
        1 => temp2()?,
        2 => final1()?,
        3 => earth()?,
        4 => quads()?,
        5 => simple_light()?,
        6 => cornell_box()?,
        _ => (),
    }

    Ok(())
}

#[allow(dead_code)]
fn cornell_box() -> Result<(), Error> {
    let mut world = HittableList::empty();

    // Materials
    let red   = Materials::lambertian_solid(Color3::new(0.65, 0.05, 0.05));
    let white = Materials::lambertian_solid(Color3::new(0.73, 0.73, 0.73));
    let green = Materials::lambertian_solid(Color3::new(0.12, 0.45, 0.15));
    let light_color = Color3::new(15.0, 15.0, 15.0);
    let light = Materials::emmiter(Textures::solid_color(light_color));

    // Quads
    world.add_quad(Point3::new(555.0, 0.0, 0.0), Vec3::new(0.0, 555.0, 0.0), Vec3::new(0.0, 0.0, 555.0), green);
    world.add_quad(Point3::new(0.0, 0.0, 0.0), Vec3::new(0.0, 555.0, 0.0), Vec3::new(0.0, 0.0, 555.0), red);
    world.add_quad(Point3::new(343.0, 554.0, 332.0), Vec3::new(-130.0, 0.0, 0.0), Vec3::new(0.0, 0.0, -105.0), light);
    world.add_quad(Point3::new(0.0, 0.0, 0.0), Vec3::new(555.0, 0.0, 0.0), Vec3::new(0.0, 0.0, 555.0), Arc::clone(&white));
    //world.add_quad(Point3::new(555.0, 555.0, 555.0), Vec3::new(-555.0, 0.0, 0.0), Vec3::new(0.0, 0.0, -555.0), Arc::clone(&white));
    world.add_quad(Point3::new(0.0, 0.0, 555.0), Vec3::new(555.0, 0.0, 0.0), Vec3::new(0.0, 555.0, 0.0), white.clone());

    let mut box1 = HittableList::create_box(Point3::new(130., 0., 65.), Point3::new(295., 165., 230.), white.clone());
    //let mut box1 = box1.rotate_y(15.);
    //let mut box1 = box1.translate(Vec3::new(265., 0., 295.));
    world.append(&mut box1);

    //let box2 = HittableList::create_box(Point3::zero(), Point3::new(165.,165.,165.), white.clone());
    //let box2 = box2.rotate_y(-18.);
    //let mut box2 = box2.translate(Vec3::new(130., 0., 65.));
    //world.append(&mut box2);

    // let mut box1 = HittableList::create_box(Point3::new(130., 0., 65.), Point3::new(295., 165., 230.), white.clone());
    // world.append(&mut box1);

    // let mut box2 = HittableList::create_box(Point3::new(265., 0., 295.), Point3::new(430., 330., 460.), white.clone());
    // world.append(&mut box2);

    let world = world.create_bvh();

    // Camera
    let args = CamArgs {
        aspect_ratio: 1.0,
        image_width: 400,
        samples_per_pixel: 100,
        max_depth: 50,
        vfov: 40.0,
        look_from: Point3::new(278.0, 278.0, -800.0),
        look_at: Point3::new(278.0, 278.0, 0.0),
        v_up: Vec3::new(0.0, 1.0, 0.0),
        defocus_angle: 0.0,
        focus_dist: 10.0,
        background: Color3::new(0.0, 0.0, 0.0),
        thread_num: 2,
    };

    let camera = Camera::initilize(args);
    let _ = camera.render(world, "images/cornell_box.ppm");
    Ok(())
}

#[allow(dead_code)]
fn simple_light() -> Result<(), Error> {
    let mut world = HittableList::empty();

    // Materials
    let teal   = Materials::lambertian_solid(Color3::new(0.2, 0.8, 0.8));

    // Quads
    world.add_sphere(Vec3::new(0.0, -1000., 0.), 1000.0, teal.clone());
    world.add_sphere(Vec3::new(0.0, 2., 0.), 2., teal);

    let difflight = Materials::emmiter(Textures::solid_color(Color3::new(4., 4., 4.)));
    world.add_sphere(Vec3::new(0.0, 7., 0.), 2.0, difflight.clone());
    world.add_quad(Point3::new(3., 1., -2.), Vec3::new(2., 0., 0.), Vec3::new(0., 2., 0.), difflight);

    let world = world.create_bvh();

    let args = CamArgs {
        aspect_ratio: 16. / 9.,
        image_width: 400,
        samples_per_pixel: 100,
        max_depth: 50,

        vfov: 20.,
        look_from: Point3::new(26.,3.,6.),
        look_at: Point3::new(0.,2.,0.),
        v_up: Vec3::new(0.,1.,0.),
        defocus_angle: 0.,
        focus_dist: 10.,

        background: Color3::new(0.2, 0.3, 0.3),
        thread_num: 2,
    };

    let camera = Camera::initilize(args);
    let _ = camera.render(world, "images/simple_light.ppm");
    Ok(())
  
}

#[allow(dead_code)]
fn quads() -> Result<(), Error> {
    let mut world = HittableList::empty();

    // Materials
    let left_red     = Materials::lambertian_solid(Color3::new(1.0, 0.2, 0.2));
    let back_green   = Materials::lambertian_solid(Color3::new(0.2, 1.0, 0.2));
    let right_blue   = Materials::lambertian_solid(Color3::new(0.2, 0.2, 1.0));
    let upper_orange = Materials::lambertian_solid(Color3::new(1.0, 0.5, 0.0));
    let lower_teal   = Materials::lambertian_solid(Color3::new(0.2, 0.8, 0.8));

    // Quads
    world.add_quad(Point3::new(-3.,-2., 5.), Vec3::new(0., 0.,-4.), Vec3::new(0., 4., 0.), left_red);
    world.add_quad(Point3::new(-2.,-2., 0.), Vec3::new(4., 0., 0.), Vec3::new(0., 4., 0.), back_green);
    world.add_quad(Point3::new( 3.,-2., 1.), Vec3::new(0., 0., 4.), Vec3::new(0., 4., 0.), right_blue);
    world.add_quad(Point3::new(-2., 3., 1.), Vec3::new(4., 0., 0.), Vec3::new(0., 0., 4.), upper_orange);
    world.add_quad(Point3::new(-2.,-3., 5.), Vec3::new(4., 0., 0.), Vec3::new(0., 0.,-4.), lower_teal);

    let world = world.create_bvh();

    let args = CamArgs {
        aspect_ratio: 1.,
        image_width: 400,
        samples_per_pixel: 100,
        max_depth: 50,
        vfov: 80.,
        look_from: Point3::new(0.,0.,9.),
        look_at: Point3::new(0.,0.,0.),
        v_up: Vec3::new(0.,1.,0.),
        defocus_angle: 0.,
        focus_dist: 10.,
        background: Color3::new(0.7, 0.8, 1.),
        thread_num: 2,
    };

    let camera = Camera::initilize(args);
    let _ = camera.render(world, "images/quads.ppm");
    Ok(())
  
}

#[allow(dead_code)]
fn earth() -> Result<(), Error> {
    let earth_texture =Textures::image("assets/earthmap.jpg");
    let earth_surface = Materials::lambertian(earth_texture);
    let mut world = HittableList::empty();
    world.add_sphere(Point3::new(0.,0.,0.), 2., earth_surface);

    let world = world.create_bvh();

    let args = CamArgs {
        aspect_ratio: 16. / 9.,
        image_width: 400,
        samples_per_pixel: 100,
        max_depth: 50,
        vfov: 20.,
        look_from: Point3::new(0.,0.,12.),
        look_at: Point3::new(0.,0.,0.),
        v_up: Vec3::new(0.,1.,0.),
        defocus_angle: 0.,
        focus_dist: 10.,
        background: Color3::new(0.7, 0.8, 1.),
        thread_num: 2,
    };

    let camera = Camera::initilize(args);
    let _ = camera.render(world, "images/world.ppm");
    Ok(())
  
}

#[allow(dead_code)]
fn temp1() -> Result<(), Error> {
    let mat_ground = Materials::lambertian_solid(Color3::new(0.8, 0.8, 0.0));
    let mat_center = Materials::lambertian_solid(Color3::new(0.1, 0.2, 0.5));
    let mat_left = Materials::dielectric(1.50);
    let mat_bubble = Materials::dielectric(1.00 / 1.50);
    let mat_right = Materials::metal(Color3::new(0.8, 0.6, 0.2), 1.0);

    let mut world = HittableList::empty();
    world.add_sphere(Vec3::new(0.0, -100.5, -1.0), 100.0, mat_ground);
    world.add_sphere(Vec3::new(0.0, 0.0, -1.2), 0.5, mat_center);
    world.add_sphere(Vec3::new(-1.0, 0.0, -1.0), 0.5, mat_left);
    world.add_sphere(Vec3::new(-1.0, 0.0, -1.0), 0.4, mat_bubble);
    world.add_sphere(Vec3::new(1.0, 0.0, -1.0), 0.5, mat_right);

    let world = world.create_bvh();

    let args = CamArgs {
        aspect_ratio: 16. / 9.,
        image_width: 800,
        samples_per_pixel: 100,
        max_depth: 50,
        vfov: 90.,
        look_from: Point3::new(-2., 2., 1.),
        look_at: Point3::new(0., 0., -1.),
        v_up: Vec3::new(0., 1., 0.),
        defocus_angle: 0.,
        focus_dist: 10.,
        background: Color3::new(0.7, 0.8, 1.),
        thread_num: 4,
    };
    let camera = Camera::initilize(args);
    let _ = camera.render(world, "images/temp1.ppm");
    Ok(())
}

#[allow(dead_code)]
fn final1() -> Result<(), Error> {
    let mut world = HittableList::empty();

    let mat_ground = Materials::lambertian_solid(Color3::new(0.5, 0.5, 0.5));
    world.add_sphere(Point3::new(0.0, -1000.0, 0.0), 1000.0, mat_ground);

    let mut rng = rand::thread_rng();
    for a in -11..11 {
        for b in -11..11 {
            let choose_mat = gen_01(&mut rng);
            let center = Point3::new(
                a as f64 + 0.9 * gen_01(&mut rng),
                0.2,
                b as f64 + 0.9 * gen_01(&mut rng),
            );

            if (center - Point3::new(4.0, 0.2, 0.0)).length() > 0.9 {
                if choose_mat < 0.7 {
                    //diffuse
                    let albedo = Color3::random(&mut rng) * Color3::random(&mut rng);
                    let mat = Materials::lambertian_solid(albedo);
                    world.add_sphere(center, 0.2, mat);
                } else if choose_mat < 0.85 {
                    //metal
                    let albedo = Color3::random_bound(0.5, 1.0, &mut rng);
                    let fuzz = gen_bound(0.0, 0.5, &mut rng);
                    let mat = Materials::metal(albedo, fuzz);
                    world.add_sphere(center, 0.2, mat);
                } else {
                    //glass
                    let mat = Materials::dielectric(1.5);
                    world.add_sphere(center, 0.2, mat);
                }
            }
        }
    }

    let mat1 = Materials::dielectric(1.5);
    world.add_sphere(Point3::new(0.0, 1.0, 0.0), 1.0, mat1);

    let mat2 = Materials::lambertian_solid(Color3::new(0.4, 0.2, 0.1));
    world.add_sphere(Point3::new(-4.0, 1.0, 0.0), 1.0, mat2);

    let mat3 = Materials::metal(Color3::new(0.7, 0.6, 0.5), 0.0);
    world.add_sphere(Point3::new(4.0, 1.0, 0.0), 1.0, mat3);

    let world = world.create_bvh();

    let args = CamArgs {
        aspect_ratio: 16.0 / 9.0,
        image_width: 1200,
        samples_per_pixel: 500,
        max_depth: 50,
        vfov: 20.,
        look_from: Point3::new(13., 2., 3.),
        look_at: Point3::new(0., 0., 0.),
        v_up: Vec3::new(0., 1., 0.),
        defocus_angle: 0.6,
        focus_dist: 10.,
        background: Color3::new(0.7, 0.8, 1.),
        thread_num: 6,
    };

    let camera = Camera::initilize(args);
    let _ = camera.render(world, "images/final1.ppm");
    Ok(())
}

#[allow(dead_code)]
fn temp2() -> Result<(), Error> {
    let mut world = HittableList::empty();

    let tex_even = Textures::rgb(0.2, 0.3, 0.1);
    let tex_odd = Textures::rgb(0.9, 0.9, 0.9);
    let tex_ground = Textures::checker(0.32, tex_even, tex_odd);
    let mat_ground = Materials::lambertian(tex_ground);
    world.add_sphere(Point3::new(0.0, -1000.0, 0.0), 1000.0, mat_ground);

    let mut rng = rand::thread_rng();
    
    for a in -11..11 {
        for b in -11..11 {
            let choose_mat = gen_01(&mut rng);
            let center = Point3::new(
                a as f64 + 0.9 * gen_01(&mut rng),
                0.2,
                b as f64 + 0.9 * gen_01(&mut rng),
            );

            if (center - Point3::new(4.0, 0.2, 0.0)).length() > 0.9 {
                if choose_mat < 0.8 {
                    //diffuse
                    let albedo = Color3::random(&mut rng) * Color3::random(&mut rng);
                    let diffuse_mat = Materials::lambertian_solid(albedo);
        
                    let center2 = center + Vec3::new(0., gen_bound(0., 0.5, &mut rng), 0.);
                    world.add_moving_sphere(center, center2, 0.2, diffuse_mat);
                    
                    //world.add_sphere(center, 0.2, diffuse_mat);
                } else if choose_mat < 0.95 {
                    //metal
                    let albedo = Color3::random_bound(0.5, 1.0, &mut rng);
                    let fuzz = gen_bound(0.0, 0.5, &mut rng);
                    let mat = Materials::metal(albedo, fuzz);
                    
                    world.add_sphere(center, 0.2, mat);
                } else {
                    //glass
                    let mat = Materials::dielectric(1.5);
                    world.add_sphere(center, 0.2, mat);
                }
            }
        }
    }

    let mat1 = Materials::dielectric(1.5);
    world.add_sphere(Point3::new(0.0, 1.0, 0.0), 1.0, mat1);

    let mat2 = Materials::lambertian_solid(Color3::new(0.4, 0.2, 0.1));
    world.add_sphere(Point3::new(-4.0, 1.0, 0.0), 1.0, mat2);

    let mat3 = Materials::metal(Color3::new(0.7, 0.6, 0.5), 0.0);
    world.add_sphere(Point3::new(4.0, 1.0, 0.0), 1.0, mat3);

    let world = world.create_bvh();

    let args = CamArgs {
        aspect_ratio: 16.0 / 9.0,
        image_width: 400,
        samples_per_pixel: 100,
        max_depth: 50,
        vfov: 20.,
        look_from: Point3::new(13., 2., 3.),
        look_at: Point3::new(0., 0., 0.),
        v_up: Vec3::new(0., 1., 0.),
        defocus_angle: 0.6,
        focus_dist: 10.,
        background: Color3::new(0.7, 0.8, 1.),
        thread_num: 2,
    };

    let camera = Camera::initilize(args);
    let _ = camera.render(world, "images/temp2.ppm");
    Ok(())
}
