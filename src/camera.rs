use crate::*;
use std::thread;
use std::sync::Mutex;

pub struct CamArgs {
    pub aspect_ratio : f64,
    pub image_width : i32,
    pub samples_per_pixel : i32,
    pub max_depth : i32,
    pub vfov : f64,
    pub look_from : Point3,
    pub look_at : Point3,
    pub v_up : Vec3,
    pub defocus_angle : f64,
    pub focus_dist : f64,
    pub thread_num : i32,
}

#[derive(Clone)]
pub struct Camera {
    samples_per_pixel : i32,
    //aspect_ratio : f64, 
    max_depth : i32,
    image_width : i32,
    image_height : i32,
    pixel_samples_scale : f64,
    center : Point3,
    pixel00_loc : Point3,
    pixel_delta_u : Vec3,
    pixel_delta_v : Vec3,
    defocus_angle : f64,
    defocus_disk_u : Vec3,
    defocus_disk_v : Vec3,
    thread_num : i32,
    //look_from : Point3,
    //look_at : Point3,
    //v_up : Vec3,
    //u : Vec3,
    //v : Vec3,
    //w : Vec3,
    //vfov : f64,
}

impl Camera {
    pub fn initilize(args : CamArgs) -> Self {
        let aspect_ratio = args.aspect_ratio;
        let image_width = args.image_width;
        let samples_per_pixel = args.samples_per_pixel;
        let max_depth = args.max_depth;
        let vfov = args.vfov;
        let look_from = args.look_from;
        let look_at = args.look_at;
        let v_up = args.v_up;
        let defocus_angle = args.defocus_angle;
        let focus_dist = args.focus_dist;
        let thread_num = args.thread_num;


        let image_height = (image_width as f64 / aspect_ratio) as i32;
        let image_height = if image_height < 1 {1} else {image_height};

        let pixel_samples_scale = 1.0 / samples_per_pixel as f64;

        let center = look_from;
        // Camera
        let theta = degrees_to_radians(vfov);
        let h = f64::tan(theta / 2.0);
        let viewport_height = 2.0 * h * focus_dist;
        let viewport_width = viewport_height * (image_width as f64 / image_height as f64);
        
        //Calculate the u, v, w basis
        let w = (look_from - look_at).normalize();
        let u = v_up.cross(&w).normalize();
        let v = u.cross(&w);

        
        // Calculate the vectors across the horizontal and down the vertical viewport edges.
        let viewport_u = viewport_width * u;
        let viewport_v = viewport_height * v;
        
        // Calculate the horizontal and vertical delta vectors from pixel to pixel.
        let pixel_delta_u = viewport_u / image_width as f64;
        let pixel_delta_v = viewport_v / image_height as f64;
        
        // Calculate the location of the upper left pixel.
        let viewport_upper_left = center
                                    - (focus_dist * w) - viewport_u/2.0 - viewport_v/2.0;
        let pixel00_loc = viewport_upper_left + 0.5 * (pixel_delta_u + pixel_delta_v);


        //Calc focus basis vectors
        let defocus_radius = focus_dist * f64::tan(degrees_to_radians(defocus_angle / 2.0));
        let defocus_disk_u = u * defocus_radius;
        let defocus_disk_v = v * defocus_radius;

        Self {
            samples_per_pixel,
            //aspect_ratio,
            max_depth,
            image_width,
            image_height,
            pixel_samples_scale,
            center,
            pixel00_loc,
            pixel_delta_u,
            pixel_delta_v,
            defocus_angle,
            defocus_disk_u,
            defocus_disk_v,
            thread_num,
            //look_from,
            //look_at,
            //v_up,
            //u,
            //v,
            //w,
            //vfov,
        }
    }

    fn ray_color(ray : &Ray, world : &HittableList, depth : i32) -> Color3 {
        if depth <= 0 {
            return Color3::zero();
        }

        match world.hit(ray, Interval::new(0.001, INF)) {
            None => (),
            Some(hr) => {
                match hr.mat.scatter(ray, &hr) {
                    None => return Color3::zero(),
                    Some((attenuation, scattered)) => 
                        return attenuation * Self::ray_color(&scattered, world, depth - 1),
                }
            },
        }

        let unit_direction = ray.direction.normalize();
        let a = 0.5*(unit_direction.y + 1.0);
        return (1.0-a)*Color3::new(1.0, 1.0, 1.0) + a*Color3::new(0.5, 0.7, 1.0);
    }

    fn sample_square(rng : &mut ThreadRng) -> Vec3 {
        Vec3::new(gen_01(rng) - 0.5, gen_01(rng) - 0.5, 0.0)
    }

    fn defocus_disk_sample(&self) -> Point3 {
        let p = Vec3::random_disk();
        self.center + (p.x * self.defocus_disk_u) + (p.y * self.defocus_disk_v)
    }

    fn get_ray(&self, i : i32, j : i32, rng : &mut ThreadRng) -> Ray {
        let offset = Self::sample_square(rng);
        let pixel_sample = self.pixel00_loc
                          + ((i as f64 + offset.x) * self.pixel_delta_u)
                          + ((j as f64 + offset.y) * self.pixel_delta_v);

        let ray_origin = if self.defocus_angle <= 0. {self.center} 
                         else {self.defocus_disk_sample()};
        let ray_direction = pixel_sample - ray_origin;

        Ray::new(ray_origin, ray_direction)
    }

    fn render_line(&self, world : &HittableList, j : i32) -> Vec<Color3> {
        let mut rng = rand::thread_rng();
        let mut scan_line = Vec::new();
        for i in 0..self.image_width {
            let mut pixel_color = Color3::new(0.0, 0.0, 0.0);
            for _ in 0..self.samples_per_pixel {
                let r = self.get_ray(i, j, &mut rng);
                pixel_color = pixel_color + Self::ray_color(&r, world, self.max_depth);
            }
            scan_line.push(pixel_color * self.pixel_samples_scale);
        }
        scan_line
    }
    
    pub fn render(&self, world : &HittableList, path : &str) -> Result<(), Error>  {
        let mut handles = vec![]; 
        let lines : Vec<Vec<Color3>> = (0..self.image_height).map(|_| Vec::new()).collect(); 
    
        let results = Arc::new(Mutex::new(lines));
        
        println!("Creating a {} x {} image", self.image_width, self.image_height);
        let total_lines = self.image_height;
        let lines_left = Arc::new(Mutex::new(self.image_height));
        for thread in 0..self.thread_num {
            let cam_clone = self.clone();
            let wor_clone = world.clone();
            let results_clone = Arc::clone(&results);
            let line_counter = Arc::clone(&lines_left);
            let lines_per_thread = self.image_height / self.thread_num;
            let lines_to_do = if thread == self.thread_num - 1 {self.image_height - (thread * lines_per_thread)} else {lines_per_thread};
            let handle = thread::spawn(move || {
                for j in 0..lines_to_do {
                    let line_idx = lines_per_thread * thread + j;
                    let scan_line = cam_clone.render_line(&wor_clone, line_idx);

                    let mut lines_left = line_counter.lock().unwrap();
                    *lines_left -= 1;
                    write_progress(*lines_left, total_lines);

                    let mut results = results_clone.lock().unwrap();
                    results[line_idx as usize] = scan_line;
                }             
            });
            handles.push(handle);
        }
        
       
        // Wait for all threads to complete
        for handle in handles {
            handle.join().unwrap();
        }
        let x = Arc::clone(&lines_left);
        assert!(*(x.lock().unwrap()) == 0);

        // Creates or overwrites the file
        let mut file = File::create(path)?; 
        // Write data as bytes
        file.write_all(format!("P3\n{} {}\n255\n", self.image_width, self.image_height).as_bytes())?; 

        let binding = Arc::clone(&results);
        let pixels = binding.lock().unwrap();
        for j in 0..self.image_height {
            for i in 0..self.image_width {
               let ju = j as usize;
                let iu = i as usize;
                pixels[ju][iu].writeln_color(&mut file)?;
            }
        } 
        println!("\nDone!                   ");
        Ok(())
    }

    /* pub fn render(&self, world : &HittableList, path : &str) -> Result<(), Error>  {
        let mut file = File::create(path)?; // Creates or overwrites the file
        // Write data as bytes
        file.write_all(format!("P3\n{} {}\n255\n", self.image_width, self.image_height).as_bytes())?;        

        let mut rng = rand::thread_rng();
        println!("Creating a {} x {} image", self.image_width, self.image_height);
        for j in 0..self.image_height {
            write_progress(self.image_height - j, self.image_height);
            for i in 0..self.image_width {
                let mut pixel_color = Color3::new(0.0, 0.0, 0.0);
                for _ in 0..self.samples_per_pixel {
                    let r = self.get_ray(i, j, &mut rng);
                    pixel_color = pixel_color + Self::ray_color(&r, &world, self.max_depth);
                }
                pixel_color = pixel_color * self.pixel_samples_scale;
                pixel_color.writeln_color(&mut file)?;
            }
        }
        println!("\nDone!                   ");
        Ok(())
    } */
}