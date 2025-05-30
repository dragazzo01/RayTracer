use crate::prelude::*;

const POINT_COUNT: usize = 256;
#[derive(Debug, Clone)]
pub struct Perlin {
    randvec: [Vec3; POINT_COUNT],
    perm_x: [usize; POINT_COUNT],
    perm_y: [usize; POINT_COUNT],
    perm_z: [usize; POINT_COUNT],
}

impl Perlin {
    pub fn new(rng: &mut ThreadRng) -> Self {

        let mut randvec = [Vec3::zero(); POINT_COUNT];
        for i in 0..POINT_COUNT {
            randvec[i] = Vec3::random_bound(-1., 1., rng).normalize();
        }
        let perm_x = Self::perlin_generate_perm(rng);
        let perm_y = Self::perlin_generate_perm(rng);
        let perm_z = Self::perlin_generate_perm(rng);

        Self {
            randvec,
            perm_x,
            perm_y,
            perm_z,
        }
    }

    pub fn noise(&self, p: &Point3) -> f64 {
        let u = p.x - p.x.floor();
        let v = p.y - p.y.floor();
        let w = p.z - p.z.floor();

        let i = p.x.floor() as isize;
        let j = p.y.floor() as isize;
        let k = p.z.floor() as isize;

        let mut c= [[[Vec3::zero(); 2]; 2]; 2];

        for di in 0..2 {
            for dj in 0..2 {
                for dk in 0..2  {
                    let idx = self.perm_x[((i + di as isize) & 255) as usize]
                        ^ self.perm_y[((j + dj as isize) & 255) as usize]
                        ^ self.perm_z[((k + dk as isize) & 255) as usize];
                    c[di][dj][dk] = self.randvec[idx];
                }
            }
        }

        Self::trilinear_interp(c, u, v, w)
    }

    pub fn turb(&self, p: &Point3, depth: i32) -> f64 {
        let mut accum = 0.0;
        let mut temp_p = *p;
        let mut weight = 1.0;

        for _ in 0..depth {
            accum += weight * self.noise(&temp_p);
            weight *= 0.5;
            temp_p = 2. * temp_p;
        }

        accum.abs()
    }

    fn trilinear_interp(c: [[[Vec3; 2]; 2]; 2], u: f64, v: f64, w: f64) -> f64 {
        let uu = u*u*(3.-2.*u);
        let vv = v*v*(3.-2.*v);
        let ww = w*w*(3.-2.*w);

        let mut accum = 0.0;
        for i in 0..2 {
            for j in 0..2 {
                for k in 0..2  {
                    let fi = i as f64;
                    let fj = j as f64;
                    let fk = k as f64;
                    let weight_v = Vec3::new(u-fi, v-fj, w-fk);
                    accum += (fi*uu + (1.-fi)*(1.-uu))
                           * (fj*vv + (1.-fj)*(1.-vv))
                           * (fk*ww + (1.-fk)*(1.-ww))
                           * c[i][j][k].dot(&weight_v);
                }
            }
        }

        accum
    }

    fn perlin_generate_perm(rng: &mut ThreadRng) -> [usize; POINT_COUNT] {
        let mut p = [0; POINT_COUNT];
        for i in 0..POINT_COUNT {
            p[i] = i;
        }
        Self::permute(&mut p, rng);
        p
    }

    fn permute(p: &mut [usize; POINT_COUNT], rng: &mut ThreadRng) {
        for i in (1..POINT_COUNT).rev() {
            let target = rng.gen_range(0..=i);
            p.swap(i, target);
        }
    }
}