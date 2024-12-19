mod vec3;
use vec3::*;
mod ray;
use ray::Ray;
use std::fs::File;
use std::io::{Error, Write};

fn main() -> Result<(), Error> {
    // Image
    let aspect_ratio = 16.0 / 9.0;
    let image_width = 200;

    // Calculate the image height, and ensure that it's at least 1.
    let image_height = (image_width as f64 / aspect_ratio) as i32;
    let image_height = if image_height < 1 {1} else {image_height};

    // Camera
    let focal_length = 1.0;
    let viewport_height = 2.0;
    let viewport_width = viewport_height * (image_width as f64 / image_height as f64);
    let camera_center = Vec3::zero();
    
    // Calculate the vectors across the horizontal and down the vertical viewport edges.
    let viewport_u = Vec3::new(viewport_width, 0.0, 0.0);
    let viewport_v = Vec3::new(0.0, -viewport_height, 0.0);
    
    // Calculate the horizontal and vertical delta vectors from pixel to pixel.
    let pixel_delta_u = viewport_u / image_width as f64;
    let pixel_delta_v = viewport_v / image_height as f64;
    
    // Calculate the location of the upper left pixel.
    let viewport_upper_left = camera_center
                                 - Vec3::new(0.0, 0.0, focal_length) - viewport_u/2.0 - viewport_v/2.0;
    let pixel00_loc = viewport_upper_left + 0.5 * (pixel_delta_u + pixel_delta_v);
    
    
    //Render
    let mut file = File::create("images/temp.ppm")?; // Creates or overwrites the file
    file.write_all(format!("P3\n{} {}\n255\n", image_width, image_height).as_bytes())?;        // Write data as bytes

    println!("Creating a {} x {} image", image_width, image_height);
    for j in 0..image_height {
        //println!("\rScanlines remaining: {}", (image_height - j));
        for i in 0..image_width {
            let pixel_center = pixel00_loc + (i as f64 * pixel_delta_u) + (j as f64 * pixel_delta_v);
            let ray_direction = pixel_center - camera_center;
            let r = Ray::new(camera_center, ray_direction);

            r.ray_color().writeln_color(&mut file)?;
        }
    }
    //println!("Done                      \n");
    Ok(())
}