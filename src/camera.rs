use crate::prelude::*;
use indicatif::{ProgressBar, ProgressStyle};
use std::sync::Arc;

/// Contains the arguments required to initialize a `Camera`.
pub struct CamArgs {
    /// The aspect ratio of the image.
    pub aspect_ratio: f64,
    /// The width of the image in pixels.
    pub image_width: usize,
    /// The number of samples per pixel for anti-aliasing.
    pub samples_per_pixel: i32,
    /// The maximum depth for ray recursion.
    pub max_depth: i32,
    /// The vertical field of view in degrees.
    pub vfov: f64,
    /// The position of the camera.
    pub look_from: Point3,
    /// The point the camera is looking at.
    pub look_at: Point3,
    /// The "up" direction vector for the camera.
    pub v_up: Vec3,
    /// The defocus angle for depth of field effects.
    pub defocus_angle: f64,
    /// The focus distance for depth of field effects.
    pub focus_dist: f64,

    pub background : Color3,
    /// The number of threads to use for rendering.
    pub thread_num: usize,
}

/// Represents a camera in the ray tracer.
#[derive(Clone)]
pub struct Camera {
    /// The number of samples per pixel for anti-aliasing.
    samples_per_pixel: i32,
    /// The maximum depth for ray recursion.
    max_depth: i32,
    /// The width of the image in pixels.
    image_width: usize,
    /// The height of the image in pixels.
    image_height: usize,
    /// Scale factor for pixel samples.
    pixel_samples_scale: f64,
    /// The position of the camera.
    center: Point3,
    /// The location of the top-left pixel in the viewport.
    pixel00_loc: Point3,
    /// The horizontal delta vector between adjacent pixels.
    pixel_delta_u: Vec3,
    /// The vertical delta vector between adjacent pixels.
    pixel_delta_v: Vec3,
    /// The defocus angle for depth of field effects.
    defocus_angle: f64,
    /// The horizontal basis vector for the defocus disk.
    defocus_disk_u: Vec3,
    /// The vertical basis vector for the defocus disk.
    defocus_disk_v: Vec3,

    pub background_color : Color3,
    /// The number of threads to use for rendering.
    thread_num: usize,
}

impl Camera {
    /// Initializes a new `Camera` instance using the provided arguments.
    ///
    /// # Arguments
    ///
    /// * `args` - A `CamArgs` struct containing the initialization parameters.
    ///
    /// # Returns
    ///
    /// A new `Camera` instance.
    pub fn initilize(args: CamArgs) -> Self {
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
        let background_color = args.background;
        let thread_num = args.thread_num;

        let image_height = (image_width as f64 / aspect_ratio) as usize;
        let image_height = if image_height < 1 { 1 } else { image_height };

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
        let viewport_upper_left = center - (focus_dist * w) - viewport_u / 2.0 - viewport_v / 2.0;
        let pixel00_loc = viewport_upper_left + 0.5 * (pixel_delta_u + pixel_delta_v);

        //Calc focus basis vectors
        let defocus_radius = focus_dist * f64::tan(degrees_to_radians(defocus_angle / 2.0));
        let defocus_disk_u = u * defocus_radius;
        let defocus_disk_v = v * defocus_radius;

        Self {
            samples_per_pixel,
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
            background_color,
            thread_num,
        }
    }

    /// Computes the color for a given ray by tracing it through the scene.
    ///
    /// # Arguments
    ///
    /// * `ray` - The ray to trace.
    /// * `world` - The scene represented as a BVH node.
    /// * `depth` - The remaining recursion depth.
    /// * `rng` - A random number generator.
    ///
    /// # Returns
    ///
    /// The computed color as a `Color3`.
    fn ray_color(&self, ray: &Ray, world: &Hittables, depth: i32, rng: &mut ThreadRng) -> Color3 {
        if depth <= 0 {
            return Color3::zero();
        }

        if let Some(hr) = world.hit(ray, Interval::new(0.001, INF)) {
            let color_from_emission = hr.mat.emitted(hr.u, hr.v, &hr.point);

            if let Some((attenuation, scattered)) = hr.mat.scatter(ray, &hr, rng) {
                let color_from_scatter =  attenuation * self.ray_color(&scattered, world, depth - 1, rng);
                color_from_emission + color_from_scatter
            } else {
                color_from_emission
            }
        } else {
            self.background_color
        }
    }

    /// Generates a random sample within a unit square.
    ///
    /// # Arguments
    ///
    /// * `rng` - A random number generator.
    ///
    /// # Returns
    ///
    /// A `Vec3` representing the random sample.
    fn sample_square(rng: &mut ThreadRng) -> Vec3 {
        Vec3::new(gen_01(rng) - 0.5, gen_01(rng) - 0.5, 0.0)
    }

    /// Samples a random point on the defocus disk for depth of field effects.
    ///
    /// # Arguments
    ///
    /// * `rng` - A random number generator.
    ///
    /// # Returns
    ///
    /// A `Point3` representing the sampled point.
    fn defocus_disk_sample(&self, rng: &mut ThreadRng) -> Point3 {
        let p = Vec3::random_disk(rng);
        self.center + (p.x * self.defocus_disk_u) + (p.y * self.defocus_disk_v)
    }

    /// Generates a ray for a specific pixel in the image.
    ///
    /// # Arguments
    ///
    /// * `i` - The horizontal pixel index.
    /// * `j` - The vertical pixel index.
    /// * `rng` - A random number generator.
    ///
    /// # Returns
    ///
    /// A `Ray` originating from the camera and passing through the pixel.
    fn get_ray(&self, i: usize, j: usize, rng: &mut ThreadRng) -> Ray {
        let offset = Self::sample_square(rng);
        let pixel_sample = self.pixel00_loc
            + ((i as f64 + offset.x) * self.pixel_delta_u)
            + ((j as f64 + offset.y) * self.pixel_delta_v);

        let ray_origin = if self.defocus_angle <= 0. {
            self.center
        } else {
            self.defocus_disk_sample(rng)
        };
        let ray_direction = pixel_sample - ray_origin;

        let ray_time = gen_01(rng);

        Ray::new_time(ray_origin, ray_direction, ray_time)
    }

    /// Renders a single scan line of the image.
    ///
    /// # Arguments
    ///
    /// * `world` - The scene represented as a BVH node.
    /// * `j` - The vertical index of the scan line.
    /// * `rng` - A random number generator.
    ///
    /// # Returns
    ///
    /// A vector of `Color3` values representing the colors of the pixels in the scan line.
    fn render_line(&self, world: &Hittables, j: usize, rng: &mut ThreadRng) -> Vec<Color3> {
        let mut scan_line = Vec::new();
        for i in 0..self.image_width {
            let mut pixel_color = Color3::new(0.0, 0.0, 0.0);
            for _ in 0..self.samples_per_pixel {
                let r = self.get_ray(i, j, rng);
                pixel_color = pixel_color + self.ray_color(&r, world, self.max_depth, rng);
            }
            scan_line.push(pixel_color * self.pixel_samples_scale);
        }
        scan_line
    }

    /// Renders the entire image and writes it to a file.
    ///
    /// # Arguments
    ///
    /// * `world` - The Hittables object the scene is rendering.
    /// * `path` - The file path to save the rendered image.
    ///
    /// # Returns
    ///
    /// A `Result` indicating success or failure.
    pub fn render(&self, world: Hittables, path: &str) -> Result<(), Error> {
        let lines = self.calculate_img(world);
        self.write_pixels(lines, path)
    }

    pub fn calculate_img(&self, world: Hittables) -> Vec<Vec<Color3>> {
        let mut handles = vec![];

        let progress_bar = Arc::new(ProgressBar::new(self.image_height as u64));
        progress_bar.set_style(
            ProgressStyle::with_template(
                "[{elapsed_precise}] [{eta_precise}] [{bar:40.green/red}] {pos}/{len} {msg}",
            )
            .unwrap()
            .progress_chars("=>-"),
        );

        println!(
            "Creating a {} x {} image",
            self.image_width, self.image_height
        );

        for thread in 0..self.thread_num {
            let progress_bar = Arc::clone(&progress_bar);
            let camera = self.clone();
            let world = world.clone();

            let lines_per_thread = self.image_height / self.thread_num;
            let lines_to_do = if thread == self.thread_num - 1 {
                self.image_height - (thread * lines_per_thread)
            } else {
                lines_per_thread
            };

            let handle = thread::spawn(move || {
                let mut rng = rand::thread_rng();
                let mut local_results = Vec::with_capacity(lines_to_do);

                for j in 0..lines_to_do {
                    let line_idx = lines_per_thread * thread + j;
                    let scan_line = camera.render_line(&world, line_idx, &mut rng);

                    progress_bar.inc(1);

                    local_results.push((line_idx, scan_line));
                }
                //println!("Thread {} finished", thread);
                local_results
            });
            handles.push(handle);
        }

        let mut lines: Vec<Vec<Color3>> = vec![Vec::new(); self.image_height];
        // Wait for all threads to complete
        for handle in handles {
            for (idx, line) in handle.join().unwrap() {
                lines[idx] = line;
            }
        }

        progress_bar.finish_with_message("All done!");
        lines
    }

    fn write_pixels(&self, img : Vec<Vec<Color3>>, path : &str) -> Result<(), Error> {
        // Creates or overwrites the file
        println!("Writing to File");
        let mut file = File::create(path)?;
        // Write data as bytes
        file.write_all(
            format!("P3\n{} {}\n255\n", self.image_width, self.image_height).as_bytes(),
        )?;

        for line in img.iter() {
            for pixel in line.iter() {
                pixel.writeln_color(&mut file)?;
            }
        }
        println!("Done!");
        Ok(())
    }
}
