use crate::prelude::*;
use image::ImageReader;
use image::ImageBuffer;
use image::Rgb;

#[derive(Debug, Clone)]
pub enum Textures {
    Checkered(CheckerTexture),
    Solid(Solid),
    Img(ImageTexture)
}

impl Textures {
    pub fn solid_color(albedo : Color3) -> Arc<Self> {
        Arc::new(Self::Solid(Solid {albedo}))
    }

    pub fn rgb(red : f64, green : f64, blue : f64) -> Arc<Self> {
        Self::solid_color(Color3::new(red, green, blue))
    }

    pub fn checker(scale: f64, even: Arc<Textures>, odd: Arc<Textures>) -> Arc<Self> {
        Arc::new(Self::Checkered(CheckerTexture {inv_scale: 1. / scale, even, odd}))
    }

    pub fn image(path : &str) -> Arc<Self> {
        let img= ImageTexture::new(path);

        match img {
            Ok(x) => Arc::new(Self::Img(x)),
            _ => Self::rgb(0., 1., 1.),
        }
    }

    pub fn value(&self, u : f64, v : f64, p : &Point3) -> Color3 {
        match self {
            Self::Checkered(c) => c.value(u, v, p),
            Self::Solid(s) => s.value(),
            Self::Img(i) => i.value(u, v),
        }
    }
}
#[derive(Debug, Clone)]
pub struct Solid {
    albedo : Color3
}

impl Solid {
    pub fn value(&self) -> Color3 {
        self.albedo
    }
}
#[derive(Debug, Clone)]
pub struct CheckerTexture {
    inv_scale: f64,
    even: Arc<Textures>,
    odd: Arc<Textures>,
}

impl CheckerTexture {
    pub fn value(&self, u : f64, v : f64, p : &Point3) -> Color3 { 
        let x  = (self.inv_scale * p.x).floor() as i32;
        let y  = (self.inv_scale * p.y).floor() as i32;
        let z  = (self.inv_scale * p.z).floor() as i32;

        if (x + y + z) % 2 == 0 {
            self.even.value(u, v, p)
        } else {
            self.odd.value(u, v, p)
        }
    }
}

#[derive(Debug, Clone)]    
pub struct ImageTexture {
    image : ImageBuffer<Rgb<f32>, Vec<f32>>
}

impl ImageTexture {
    pub fn new(path : &str) -> Result<Self, Error> {
        let image = ImageReader::open(path)
        .expect("Failed to open image")
        .decode()
        .expect("Failed to decode image")
        .to_rgb8();

        let (width, height) = image.dimensions();
        let mut buf = ImageBuffer::new(width, height);

        for (x, y, pixel) in image.enumerate_pixels() {
            let linear = Rgb([
                Self::srgb_to_linear_f32(pixel[0]),
                Self::srgb_to_linear_f32(pixel[1]),
                Self::srgb_to_linear_f32(pixel[2]),
            ]);
            buf.put_pixel(x, y, linear);
        }

        Ok(Self {image : buf})

    }

    fn srgb_to_linear_f32(c: u8) -> f32 {
        let c = c as f32 / 255.0;
        if c <= 0.04045 {
            c / 12.92
        } else {
            ((c + 0.055) / 1.055).powf(2.4)
        }
    }

    pub fn value(&self, u : f64, v : f64) -> Color3 { 
        let image = &self.image;
        // If we have no texture data, then return solid cyan as a debugging aid.
        if image.height() <= 0 {return Color3::new(0.,1.,1.)};

        // Clamp input texture coordinates to [0,1] x [1,0]
        let bounds = Interval::new(0., 1.);
        let u = bounds.clamp(u);
        let v = 1. - bounds.clamp(v);  // Flip V to image coordinates

        let i = ((u * image.width() as f64).min(image.width() as f64 - 1.0)) as u32;
        let j = ((v * image.height() as f64).min(image.height() as f64 - 1.0)) as u32;
        let pixel = image.get_pixel(i,j);

        Color3::new(pixel[0].into(), 
                    pixel[1].into(), 
                    pixel[2].into())
    }
}