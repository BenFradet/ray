use crate::math::{matrix::Matrix4x4, matrix_const::MatrixConst, matrix_invert::MatrixInvert};

#[derive(PartialEq, Debug, Copy, Clone)]
pub struct Camera {
    hsize: usize,
    vsize: usize,
    fov: f64,
    transform: Matrix4x4,
    pub inv_t: Matrix4x4,
    pub pixel_size: f64,
    pub half_width: f64,
    pub half_height: f64,
}

impl Camera {
    pub fn new(hsize: usize, vsize: usize, fov: f64) -> Self {
        let half_view = f64::tan(fov / 2.);
        let hsize_f = hsize as f64;
        let aspect_ratio = hsize_f / vsize as f64;
        let (half_width, half_height) = if aspect_ratio >= 1. {
            (half_view, half_view / aspect_ratio)
        } else {
            (half_view * aspect_ratio, half_view)
        };
        let pixel_size = half_width * 2. / hsize_f;

        Self {
            hsize,
            vsize,
            fov,
            transform: Matrix4x4::ID,
            inv_t: Matrix4x4::ID,
            pixel_size,
            half_width,
            half_height,
        }
    }

    pub fn transform(mut self, t: Matrix4x4) -> Option<Self> {
        self.transform = t;
        t.invert().map(|inv_t| {
            self.inv_t = inv_t;
            self
        })
    }
}

#[cfg(test)]
mod tests {
    use std::f64::consts::FRAC_PI_2;

    use super::*;

    #[test]
    fn pixel_size_v() -> () {
        let c = Camera::new(125, 200, FRAC_PI_2);
        assert_eq!((c.pixel_size * 10000.).round(), 100.); // 0.01
    }

    #[test]
    fn pixel_size_h() -> () {
        let c = Camera::new(200, 125, FRAC_PI_2);
        assert_eq!((c.pixel_size * 10000.).round(), 100.);
    }

    #[test]
    fn new () -> () {
        let w = 160;
        let h = 120;
        let fov = FRAC_PI_2;
        let c = Camera::new(w, h, fov);
        assert_eq!(c.hsize, w);
        assert_eq!(c.vsize, h);
        assert_eq!(c.fov, fov);
        assert_eq!(c.transform, Matrix4x4::ID);
    }
}