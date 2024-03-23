use std::{f64::consts::PI, mem::size_of};

use crate::math::{colour::Colour, point::Point, vector::Vector};

use super::{pattern_at::PatternAt, pattern_kind::PatternKind};

#[derive(PartialEq, Debug, Clone)]
pub struct Perlin {
    p: PatternKind,
    scale: f64,
}

impl Perlin {
    pub fn new(p: PatternKind, scale: f64) -> Self {
        Self { p, scale }
    }

    // https://en.wikipedia.org/wiki/Perlin_noise
    fn noise_w(p: Point) -> f64 {
        fn smootherstep(a0: f64, a1: f64, w: f64) -> f64 {
            if w < 0. {
                a0
            } else if w > 1. {
                a1
            } else {
                (a1 - a0) * ((w * (w * 6.0 - 15.0) + 10.0) * w * w * w) + a0
            }
        }

        fn random_gradient(iv: Vector) -> Vector {
            let w = size_of::<usize>();
            let s = w / 2;
            let mut a = iv.x as usize;
            let mut b = iv.y as usize;
            let mut c = iv.z as usize;
            // need to integrate iv.z somehow
            a = a.wrapping_mul(1444444447);
            b ^= a << s | a >> w - s;
            b = b.wrapping_mul(2000000011);
            a ^= b << s | b >> w - s;
            c = c.wrapping_mul(7202218937);
            a ^= c << s | c >> w - s;
            a = a.wrapping_mul(3553741211);
            let r = a as f64 * (PI / !(!0 as usize >> 1) as f64);
            Vector::new(r.cos(), r.sin(), r.tan())
        }

        fn dot_grid_gradient(iv: Vector, v: Vector) -> f64 {
            let gradient = random_gradient(iv);
            let distance = v - iv;
            distance.dot(gradient)
        }

        let v = Vector::new(p.x, p.y, p.z);

        let x0 = p.x.floor();
        let x1 = x0 + 1.;
        let y0 = p.y.floor();
        let y1 = y0 + 1.;
        let z0 = p.z.floor();
        let z1 = z0 + 1.;

        let sx = p.x - x0 as f64;
        let sy = p.y - y0 as f64;
        let sz = p.z - z0 as f64;

        let n000 = dot_grid_gradient(Vector::new(x0, y0, z0), v);
        let n100 = dot_grid_gradient(Vector::new(x1, y0, z0), v);
        let ix0 = smootherstep(n000, n100, sx);
        let n010 = dot_grid_gradient(Vector::new(x0, y1, z0), v);
        let n110 = dot_grid_gradient(Vector::new(x1, y1, z0), v);
        let ix1 = smootherstep(n010, n110, sx);
        let n001 = dot_grid_gradient(Vector::new(x0, y0, z1), v);
        let n101 = dot_grid_gradient(Vector::new(x1, y0, z1), v);
        let ix2 = smootherstep(n001, n101, sx);
        let n011 = dot_grid_gradient(Vector::new(x0, y1, z1), v);
        let n111 = dot_grid_gradient(Vector::new(x1, y1, z1), v);
        let ix3 = smootherstep(n011, n111, sx);

        let ix01 = smootherstep(ix0, ix1, sy);
        let ix23 = smootherstep(ix2, ix3, sy);
        smootherstep(ix01, ix23, sz).clamp(-1., 1.)
    }

    // https://mrl.cs.nyu.edu/~perlin/noise/
    fn noise_p(p: Point) -> f64 {
        fn lerp(t: f64, a: f64, b: f64) -> f64 {
            a + t * (b - a)
        }

        fn fade(t: f64) -> f64 {
            t * t * t * (t * (t * 6. - 15.) + 10.)
        }

        fn grad(hash: usize, x: f64, y: f64, z: f64) -> f64 {
            let h = hash & 15;
            let u = if h < 8 { x } else { y };
            let v = if h < 4 {
                y
            } else {
                if h == 12 || h == 14 {
                    x
                } else {
                    z
                }
            };

            let up = if (h & 1) == 0 { u } else { -u };
            let vp = if (h & 2) == 0 { v } else { -v };
            up + vp
        }

        let xu = p.x.floor() as usize & 255;
        let yu = p.y.floor() as usize & 255;
        let zu = p.z.floor() as usize & 255;

        let xf = p.x - p.x.floor();
        let yf = p.y - p.y.floor();
        let zf = p.z - p.z.floor();

        let u = fade(xf);
        let v = fade(yf);
        let w = fade(zf);

        let a = Self::PERMS[xu] + yu;
        let aa = Self::PERMS[a] + zu;
        let ab = Self::PERMS[a + 1] + zu;
        let b = Self::PERMS[xu + 1] + yu;
        let ba = Self::PERMS[b] + zu;
        let bb = Self::PERMS[b + 1] + zu;

        let aaba = lerp(
            u,
            grad(Self::PERMS[aa], xf, yf, zf),
            grad(Self::PERMS[ba], xf - 1., yf, zf),
        );
        let abbb = lerp(
            u,
            grad(Self::PERMS[ab], xf, yf - 1., zf),
            grad(Self::PERMS[bb], xf - 1., yf - 1., zf),
        );
        let lerp1 = lerp(v, aaba, abbb);

        let aaba_p = lerp(
            u,
            grad(Self::PERMS[aa + 1], xf, yf, zf - 1.),
            grad(Self::PERMS[ba + 1], xf - 1., yf, zf - 1.),
        );
        let abbb_p = lerp(
            u,
            grad(Self::PERMS[ab + 1], xf, yf - 1., zf - 1.),
            grad(Self::PERMS[bb + 1], xf - 1., yf - 1., zf - 1.),
        );
        let lerp2 = lerp(v, aaba_p, abbb_p);

        lerp(w, lerp1, lerp2).clamp(-1., 1.)
    }

    const PERMS: [usize; 512] = [
        151, 160, 137, 91, 90, 15, 131, 13, 201, 95, 96, 53, 194, 233, 7, 225, 140, 36, 103, 30,
        69, 142, 8, 99, 37, 240, 21, 10, 23, 190, 6, 148, 247, 120, 234, 75, 0, 26, 197, 62, 94,
        252, 219, 203, 117, 35, 11, 32, 57, 177, 33, 88, 237, 149, 56, 87, 174, 20, 125, 136, 171,
        168, 68, 175, 74, 165, 71, 134, 139, 48, 27, 166, 77, 146, 158, 231, 83, 111, 229, 122, 60,
        211, 133, 230, 220, 105, 92, 41, 55, 46, 245, 40, 244, 102, 143, 54, 65, 25, 63, 161, 1,
        216, 80, 73, 209, 76, 132, 187, 208, 89, 18, 169, 200, 196, 135, 130, 116, 188, 159, 86,
        164, 100, 109, 198, 173, 186, 3, 64, 52, 217, 226, 250, 124, 123, 5, 202, 38, 147, 118,
        126, 255, 82, 85, 212, 207, 206, 59, 227, 47, 16, 58, 17, 182, 189, 28, 42, 223, 183, 170,
        213, 119, 248, 152, 2, 44, 154, 163, 70, 221, 153, 101, 155, 167, 43, 172, 9, 129, 22, 39,
        253, 19, 98, 108, 110, 79, 113, 224, 232, 178, 185, 112, 104, 218, 246, 97, 228, 251, 34,
        242, 193, 238, 210, 144, 12, 191, 179, 162, 241, 81, 51, 145, 235, 249, 14, 239, 107, 49,
        192, 214, 31, 181, 199, 106, 157, 184, 84, 204, 176, 115, 121, 50, 45, 127, 4, 150, 254,
        138, 236, 205, 93, 222, 114, 67, 29, 24, 72, 243, 141, 128, 195, 78, 66, 215, 61, 156, 180,
        151, 160, 137, 91, 90, 15, 131, 13, 201, 95, 96, 53, 194, 233, 7, 225, 140, 36, 103, 30,
        69, 142, 8, 99, 37, 240, 21, 10, 23, 190, 6, 148, 247, 120, 234, 75, 0, 26, 197, 62, 94,
        252, 219, 203, 117, 35, 11, 32, 57, 177, 33, 88, 237, 149, 56, 87, 174, 20, 125, 136, 171,
        168, 68, 175, 74, 165, 71, 134, 139, 48, 27, 166, 77, 146, 158, 231, 83, 111, 229, 122, 60,
        211, 133, 230, 220, 105, 92, 41, 55, 46, 245, 40, 244, 102, 143, 54, 65, 25, 63, 161, 1,
        216, 80, 73, 209, 76, 132, 187, 208, 89, 18, 169, 200, 196, 135, 130, 116, 188, 159, 86,
        164, 100, 109, 198, 173, 186, 3, 64, 52, 217, 226, 250, 124, 123, 5, 202, 38, 147, 118,
        126, 255, 82, 85, 212, 207, 206, 59, 227, 47, 16, 58, 17, 182, 189, 28, 42, 223, 183, 170,
        213, 119, 248, 152, 2, 44, 154, 163, 70, 221, 153, 101, 155, 167, 43, 172, 9, 129, 22, 39,
        253, 19, 98, 108, 110, 79, 113, 224, 232, 178, 185, 112, 104, 218, 246, 97, 228, 251, 34,
        242, 193, 238, 210, 144, 12, 191, 179, 162, 241, 81, 51, 145, 235, 249, 14, 239, 107, 49,
        192, 214, 31, 181, 199, 106, 157, 184, 84, 204, 176, 115, 121, 50, 45, 127, 4, 150, 254,
        138, 236, 205, 93, 222, 114, 67, 29, 24, 72, 243, 141, 128, 195, 78, 66, 215, 61, 156, 180,
    ];
}

impl PatternAt for Perlin {
    fn pattern_at(&self, p: Point) -> Colour {
        let nx = Perlin::noise_w(p) * self.scale;
        let ny = Perlin::noise_w(Point::new(p.z, p.x, p.y)) * self.scale;
        let nz = Perlin::noise_w(Point::new(p.y, p.z, p.z)) * self.scale;
        self.p.pattern_at(Point::new(p.x + nx, p.y + ny, p.z + nz))
    }
}

#[cfg(test)]
mod tests {
    use crate::pattern::gradient::Gradient;

    use super::*;

    #[test]
    fn pattern_at() -> () {
        let p = Perlin::new(PatternKind::Gradient(Gradient::new(Colour::WHITE, Colour::BLACK)), 0.2);
        let res1 = p.pattern_at(Point::new(1.1, 2.1, 1.1));
        assert_eq!(res1, Colour::WHITE);
        let res2 = p.pattern_at(Point::new(1.1, 1.1, 2.1));
        assert_eq!(res2, Colour::WHITE);
        assert!(res1 != res2);
        let res3 = p.pattern_at(Point::new(1., 1., 2.));
        assert_eq!(res3, Colour::WHITE);
    }

    #[test]
    fn noise_w() -> () {
        let res1 = Perlin::noise_w(Point::new(1.1, 2.1, 1.1));
        assert_eq!(res1, -0.09032912721240051);
        let res2 = Perlin::noise_w(Point::new(1.1, 1.1, 2.1));
        assert_eq!(res2, -0.08908466962242434);
        assert!(res1 != res2);
        let res3 = Perlin::noise_w(Point::new(1., 1., 2.));
        assert_eq!(res3, 0.);
    }

    #[test]
    fn noise_p() -> () {
        let res1 = Perlin::noise_p(Point::new(1.1, 2.1, 1.1));
        assert_eq!(res1, 0.0016090406267904045);
        let res2 = Perlin::noise_p(Point::new(1.1, 1.1, 2.1));
        assert_eq!(res2, -0.013651408617984029);
        assert!(res1 != res2);
        let res3 = Perlin::noise_p(Point::new(1., 1., 2.));
        assert_eq!(res3, 0.);
    }

    #[test]
    fn new() -> () {
        let p = PatternKind::Gradient(Gradient::new(Colour::WHITE, Colour::BLACK));
        let perlin = Perlin::new(p.clone(), 0.2);
        assert_eq!(perlin.p, p);
        assert_eq!(perlin.scale, 0.2);
    }
}
