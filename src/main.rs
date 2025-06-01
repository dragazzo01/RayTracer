mod camera;
mod constants;
mod hittables;
mod interval;
mod materials;
mod prelude;
mod random;
mod ray;
mod vec3;
mod perlin;
mod texture;

use crate::camera::{CamArgs, Camera};
use crate::hittables::hittables::HittableList;
use crate::prelude::*;

fn main() -> Result<(), Error> {
    match 9 {
        0 => temp1()?,
        1 => temp2()?,
        2 => final1()?,
        3 => earth()?,
        4 => quads()?,
        5 => simple_light()?,
        6 => perlin_spheres()?,
        7 => cornell_box()?,
        8 => cornell_smoke()?,
        9 => final_scene()?,
        _ => (),
    }

    Ok(())
}

#[allow(dead_code)]
fn final_scene() -> Result<(), Error> {
    println!("Rendering Final Scene");
    let mut world = HittableList::empty();
    let rng = &mut rand::thread_rng();

    let mut boxes1 = HittableList::empty();

    // Ground boxes
    let ground = Materials::lambertian_solid(Color3::new(0.48, 0.83, 0.53));
    let boxes_per_side = 20;
    for i in 0..boxes_per_side {
        for j in 0..boxes_per_side {
            let w = 100.0;
            let x0 = -1000.0 + i as f64 * w;
            let z0 = -1000.0 + j as f64 * w;
            let y0 = 0.0;
            let x1 = x0 + w;
            let y1 = gen_bound(1.0, 101.0, rng);
            let z1 = z0 + w;

            boxes1.append(&mut HittableList::create_box(
                Point3::new(x0, y0, z0),
                Point3::new(x1, y1, z1),
                ground.clone(),
            ));
        }
    }
    world.add(Rc::new(boxes1.create_bvh()));

    // Light
    let light = Materials::emmiter_solid(Color3::new(7.0, 7.0, 7.0));
    world.add_quad(
        Point3::new(123.0, 554.0, 147.0),
        Vec3::new(300.0, 0.0, 0.0),
        Vec3::new(0.0, 0.0, 265.0),
        light,
    );

    // Moving sphere
    let center1 = Point3::new(400.0, 400.0, 200.0);
    let center2 = center1 + Vec3::new(30.0, 0.0, 0.0);
    let sphere_material = Materials::lambertian_solid(Color3::new(0.7, 0.3, 0.1));
    world.add_moving_sphere(center1, center2, 50.0, sphere_material);

    // Other spheres
    world.add_sphere(
        Point3::new(260.0, 150.0, 45.0),
        50.0,
        Materials::dielectric(1.5),
    );
    world.add_sphere(
        Point3::new(0.0, 150.0, 145.0),
        50.0,
        Materials::metal(Color3::new(0.8, 0.8, 0.9), 1.0),
    );

    // Medium spheres
    let mut boundary = HittableList::empty();
    boundary.add_sphere(
        Point3::new(360.0, 150.0, 145.0),
        70.0,
        Materials::dielectric(1.5),
    );
    world.add_solid_medium(boundary.into_hittable(), 0.2, Color3::new(0.2, 0.4, 0.9));
    world.append(&mut boundary);
    

    let mut boundary2 = HittableList::empty();
    boundary2.add_sphere(
        Point3::new(0.0, 0.0, 0.0),
        5000.0,
        Materials::dielectric(1.5),
    );
    world.add_solid_medium(boundary2.into_hittable(), 0.0001, Color3::new(1.0, 1.0, 1.0));

    // Earth and noise spheres
    let earth_texture = Textures::image("assets/earthmap.jpg");
    let earth_material = Materials::lambertian(earth_texture);
    world.add_sphere(Point3::new(400.0, 200.0, 400.0), 100.0, earth_material);

    let pertext = Materials::lambertian(Textures::noise(0.2, rng));
    world.add_sphere(Point3::new(220.0, 280.0, 300.0), 80.0, pertext);

    // Box of spheres
    let mut boxes2 = HittableList::empty();
    let white = Materials::lambertian_solid(Color3::new(0.73, 0.73, 0.73));
    let ns = 1000;
    for _ in 0..ns {
        boxes2.add_sphere(
            Point3::random_bound(0.0, 165.0, rng),
            10.0,
            white.clone(),
        );
    }

    let mut rotated_boxes = boxes2;
    rotated_boxes.rotate_y(15.0);
    rotated_boxes.translate(Vec3::new(-100.0, 270.0, 395.0));
    world.append(&mut rotated_boxes);

    // Camera
    let args = CamArgs {
        aspect_ratio: 1.0,
        image_width: 800, // Default value, can be parameterized
        samples_per_pixel: 100, // Default value, can be parameterized
        max_depth: 50, // Default value, can be parameterized
        vfov: 40.0,
        look_from: Point3::new(478.0, 278.0, -600.0),
        look_at: Point3::new(278.0, 278.0, 0.0),
        v_up: Vec3::new(0.0, 1.0, 0.0),
        defocus_angle: 0.0,
        focus_dist: 10.0,
        background: Color3::zero(),
        // thread_num: 4,
    };

    let camera = Camera::initilize(args);
    let _ = camera.render(world.create_bvh(), "images/final_2.ppm");
    Ok(())
}

#[allow(dead_code)]
fn perlin_spheres() -> Result<(), Error> {
    println!("Rendering Perlin");
    let mut world = HittableList::empty();
    let rng = &mut rand::thread_rng();
    // Materials
    let pertext = Materials::lambertian(Textures::noise(4.0, rng));
    // Quads
    world.add_sphere(Point3::new(0.0, -1000.0, 0.0), 1000., pertext.clone());
    world.add_sphere(Point3::new(0.0, 2.0, 0.0), 2., pertext.clone());

    // Camera
    let args = CamArgs {
        aspect_ratio: 16.0 / 9.0,
        image_width: 400,
        samples_per_pixel: 100,
        max_depth: 50,
        vfov: 20.0,
        look_from: Point3::new(13.,2.,3.),
        look_at: Point3::new(0.,0.,0.),
        v_up: Vec3::new(0.,1.,0.),
        defocus_angle: 0.0,
        focus_dist: 10.0,
        background: Color3::new(0.7, 0.8, 1.),
        // thread_num: 3,
    };

    let camera = Camera::initilize(args);
    let _ = camera.render(world.create_bvh(), "images/perlin.ppm");
    Ok(())
}

#[allow(dead_code)]
fn cornell_smoke() -> Result<(), Error> {
    println!("Rendering Cornell Smoke");
    let mut world = HittableList::empty();

    // Materials
    let red   = Materials::lambertian_solid(Color3::new(0.65, 0.05, 0.05));
    let white = Materials::lambertian_solid(Color3::new(0.73, 0.73, 0.73));
    let green = Materials::lambertian_solid(Color3::new(0.12, 0.45, 0.15));
    let light = Materials::emmiter_solid(Color3::new(15.0, 15.0, 15.0));

    // Quads
    world.add_quad(Point3::new(555.0, 0.0, 0.0), Vec3::new(0.0, 555.0, 0.0), Vec3::new(0.0, 0.0, 555.0), green);
    world.add_quad(Point3::new(0.0, 0.0, 0.0), Vec3::new(0.0, 555.0, 0.0), Vec3::new(0.0, 0.0, 555.0), red);
    world.add_quad(Point3::new(343.0, 554.0, 332.0), Vec3::new(-130.0, 0.0, 0.0), Vec3::new(0.0, 0.0, -105.0), light);
    world.add_quad(Point3::new(0.0, 0.0, 0.0), Vec3::new(555.0, 0.0, 0.0), Vec3::new(0.0, 0.0, 555.0), white.clone());
    world.add_quad(Point3::new(555.0, 555.0, 555.0), Vec3::new(-555.0, 0.0, 0.0), Vec3::new(0.0, 0.0, -555.0), white.clone());
    world.add_quad(Point3::new(0.0, 0.0, 555.0), Vec3::new(555.0, 0.0, 0.0), Vec3::new(0.0, 555.0, 0.0), white.clone());

    let mut box1 = HittableList::create_box(Point3::new(0.,0.,0.), Point3::new(165.,330.,165.), white.clone());
    box1.rotate_y(15.);
    box1.translate(Vec3::new(265., 0., 295.));
    //world.append(&mut box1);

    let mut box2 = HittableList::create_box(Point3::new(0.,0.,0.), Point3::new(165.,165.,165.), white.clone());
    box2.rotate_y(-18.);
    box2.translate(Vec3::new(130., 0., 65.));
    //world.append(&mut box2);

    world.add_solid_medium(box1.into_hittable(), 0.01, Color3::zero());
    world.add_solid_medium(box2.into_hittable(), 0.01, Color3::new(1., 1., 1.));

    // Camera
    let args = CamArgs {
        aspect_ratio: 1.0,
        image_width: 600, //600
        samples_per_pixel: 200, //200
        max_depth: 50, //50
        vfov: 40.0,
        look_from: Point3::new(278.0, 278.0, -800.0),
        look_at: Point3::new(278.0, 278.0, 0.0),
        v_up: Vec3::new(0.0, 1.0, 0.0),
        defocus_angle: 0.0,
        focus_dist: 10.0,
        background: Color3::zero(),
        // thread_num: 1,
    };

    let camera = Camera::initilize(args);
    let _ = camera.render(world.create_bvh(), "images/cornell_smoke.ppm");
    Ok(())
}

#[allow(dead_code)]
fn cornell_box() -> Result<(), Error> {
    println!("Rendering Cornell Box");
    let mut world = HittableList::empty();

    // Materials
    let red   = Materials::lambertian_solid(Color3::new(0.65, 0.05, 0.05));
    let white = Materials::lambertian_solid(Color3::new(0.73, 0.73, 0.73));
    let green = Materials::lambertian_solid(Color3::new(0.12, 0.45, 0.15));
    let light = Materials::emmiter_solid(Color3::new(15.0, 15.0, 15.0));

    // Quads
    world.add_quad(Point3::new(555.0, 0.0, 0.0), Vec3::new(0.0, 555.0, 0.0), Vec3::new(0.0, 0.0, 555.0), green);
    world.add_quad(Point3::new(0.0, 0.0, 0.0), Vec3::new(0.0, 555.0, 0.0), Vec3::new(0.0, 0.0, 555.0), red);
    world.add_quad(Point3::new(343.0, 554.0, 332.0), Vec3::new(-130.0, 0.0, 0.0), Vec3::new(0.0, 0.0, -105.0), light);
    world.add_quad(Point3::new(0.0, 0.0, 0.0), Vec3::new(555.0, 0.0, 0.0), Vec3::new(0.0, 0.0, 555.0), white.clone());
    world.add_quad(Point3::new(555.0, 555.0, 555.0), Vec3::new(-555.0, 0.0, 0.0), Vec3::new(0.0, 0.0, -555.0), white.clone());
    world.add_quad(Point3::new(0.0, 0.0, 555.0), Vec3::new(555.0, 0.0, 0.0), Vec3::new(0.0, 555.0, 0.0), white.clone());

    let mut box1 = HittableList::create_box(Point3::new(0.,0.,0.), Point3::new(165.,330.,165.), white.clone());
    box1.rotate_y(15.);
    box1.translate(Vec3::new(265., 0., 295.));
    world.append(&mut box1);

    let mut box2 = HittableList::create_box(Point3::new(0.,0.,0.), Point3::new(165.,165.,165.), white.clone());
    box2.rotate_y(-18.);
    box2.translate(Vec3::new(130., 0., 65.));
    world.append(&mut box2);

    // Camera
    let args = CamArgs {
        aspect_ratio: 1.0,
        image_width: 400, //600
        samples_per_pixel: 150, //200
        max_depth: 40, //50
        vfov: 40.0,
        look_from: Point3::new(278.0, 278.0, -800.0),
        look_at: Point3::new(278.0, 278.0, 0.0),
        v_up: Vec3::new(0.0, 1.0, 0.0),
        defocus_angle: 0.0,
        focus_dist: 10.0,
        background: Color3::zero(),
        // thread_num: 1,
    };

    let camera = Camera::initilize(args);
    let _ = camera.render(world.create_bvh(), "images/cornell_box.ppm");
    Ok(())
}

#[allow(dead_code)]
fn simple_light() -> Result<(), Error> {
    println!("Rendering Simple Light");
    let mut world = HittableList::empty();
    let rng = &mut rand::thread_rng();

    // Materials
    let pertex   = Materials::lambertian(Textures::noise(4.0, rng));

    // Quads
    world.add_sphere(Vec3::new(0.0, -1000., 0.), 1000.0, pertex.clone());
    world.add_sphere(Vec3::new(0.0, 2., 0.), 2., pertex);

    let difflight = Materials::emmiter_solid(Color3::new(4., 4., 4.));
    world.add_sphere(Vec3::new(0.0, 7., 0.), 2.0, difflight.clone());
    world.add_quad(Point3::new(3., 1., -2.), Vec3::new(2., 0., 0.), Vec3::new(0., 2., 0.), difflight);


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

        background: Color3::new(0., 0., 0.),
        // thread_num: 2,
    };

    let camera = Camera::initilize(args);
    let _ = camera.render(world.create_bvh(), "images/simple_light.ppm");
    Ok(())
  
}

#[allow(dead_code)]
fn quads() -> Result<(), Error> {
    println!("Rendering Quads");
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
        // thread_num: 2,
    };

    let camera = Camera::initilize(args);
    let _ = camera.render(world.create_bvh(), "images/quads.ppm");
    Ok(())
  
}

#[allow(dead_code)]
fn earth() -> Result<(), Error> {
    println!("Rendering Earth");
    let earth_texture =Textures::image("assets/earthmap.jpg");
    let earth_surface = Materials::lambertian(earth_texture);
    let mut world = HittableList::empty();
    world.add_sphere(Point3::new(0.,0.,0.), 2., earth_surface);


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
        // thread_num: 2,
    };

    let camera = Camera::initilize(args);
    let _ = camera.render(world.create_bvh(), "images/world.ppm");
    Ok(())
  
}

#[allow(dead_code)]
fn temp1() -> Result<(), Error> {
    println!("Rendering Temp1");
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
        // thread_num: 4,
    };
    let camera = Camera::initilize(args);
    let _ = camera.render(world.create_bvh(), "images/temp1.ppm");
    Ok(())
}

#[allow(dead_code)]
fn final1() -> Result<(), Error> {
    println!("Rendering Final 1");
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
        // thread_num: 6,
    };

    let camera = Camera::initilize(args);
    let _ = camera.render(world.create_bvh(), "images/final1.ppm");
    Ok(())
}

#[allow(dead_code)]
fn temp2() -> Result<(), Error> {
    println!("Rendering Temp2");
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
        // thread_num: 2,
    };

    let camera = Camera::initilize(args);
    let _ = camera.render(world.create_bvh(), "images/temp2.ppm");
    Ok(())
}
