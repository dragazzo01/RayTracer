use crate::vec3::{Point3, Color3, Vec3};

pub struct Ray {
    origin : Point3,
    direction : Vec3,
}

impl Ray {
    pub fn new(origin : Point3, direction : Vec3) -> Self {
        Ray {origin, direction}
    }

    pub fn at(&self, t : f64) -> Point3 {
        self.origin + t*self.direction
    }

    fn hit_sphere(&self, center : Point3, radius : f64) -> bool {
        let oc = center - self.origin;
        let a = self.direction.norm();
        let b = -2.0 * self.direction.dot(oc);
        let c = oc.norm() - radius*radius;
        let discriminant = b*b - 4.0*a*c;
        discriminant >= 0.0
    }

    pub fn ray_color(&self) -> Color3 {
        if self.hit_sphere(Vec3::new(0.0, 0.0, -1.0), 0.5) {
            return Color3::new(1.0, 0.0, 0.0);
        } 

        let unit_direction = self.direction.normalize();
        let a = 0.5*(unit_direction.y + 1.0);
        return (1.0-a)*Color3::new(1.0, 1.0, 1.0) + a*Color3::new(0.5, 0.7, 1.0);
    }
}