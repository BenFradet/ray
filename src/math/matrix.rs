use std::ops::{Index, IndexMut, Mul};

use super::{
    matrix_const::MatrixConst, matrix_from_iter::MatrixFromIter, matrix_size::MatrixSize,
    point::Point, vector::Vector,
};

// new type required to impl into iterator
pub struct Matrix<M: MatrixSize + Index<(usize, usize), Output = f64>> {
    pub m: M,
}

// can't do Output: f64 because of https://github.com/rust-lang/rust/issues/52662
impl<T: MatrixSize + Index<(usize, usize), Output = f64>> IntoIterator for Matrix<T> {
    type Item = f64;
    type IntoIter = std::vec::IntoIter<Self::Item>;
    fn into_iter(self) -> Self::IntoIter {
        //unsafe {
        //    transmute::<[[f64; 4]; 4], [f64; 16]>(self.m).to_vec().into_iter()
        //}
        let mut v = Vec::with_capacity(T::SIZE * T::SIZE);
        for r in 0..T::SIZE {
            for c in 0..T::SIZE {
                v.push(self.m[(r, c)]);
            }
        }
        v.into_iter()
    }
}

// todo: use nalgebra when done
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Matrix4x4 {
    pub m: [[f64; 4]; 4],
}

impl Matrix4x4 {
    pub fn new(
        m00: f64,
        m01: f64,
        m02: f64,
        m03: f64,
        m10: f64,
        m11: f64,
        m12: f64,
        m13: f64,
        m20: f64,
        m21: f64,
        m22: f64,
        m23: f64,
        m30: f64,
        m31: f64,
        m32: f64,
        m33: f64,
    ) -> Self {
        Self {
            m: [
                [m00, m01, m02, m03],
                [m10, m11, m12, m13],
                [m20, m21, m22, m23],
                [m30, m31, m32, m33],
            ],
        }
    }

    pub fn view_transform(eye: Point, to: Point, up: Vector) -> Self {
        let forward = (to - eye).norm();
        let left = forward.cross(up.norm());
        let true_up = left.cross(forward);
        let orientation = Self {
            m: [
                [left.x, left.y, left.z, 0.],
                [true_up.x, true_up.y, true_up.z, 0.],
                [-forward.x, -forward.y, -forward.z, 0.],
                [0., 0., 0., 1.],
            ],
        };
        orientation * Matrix4x4::translation(-eye.x, -eye.y, -eye.z)
    }

    pub fn translation(x: f64, y: f64, z: f64) -> Self {
        let mut res = Self::ID;
        res[(0, 3)] = x;
        res[(1, 3)] = y;
        res[(2, 3)] = z;
        res
    }

    pub fn translate(self, x: f64, y: f64, z: f64) -> Self {
        Self::translation(x, y, z) * self
    }

    pub fn scaling(x: f64, y: f64, z: f64) -> Self {
        let mut res = Self::ID;
        res[(0, 0)] = x;
        res[(1, 1)] = y;
        res[(2, 2)] = z;
        res
    }

    pub fn scale(self, x: f64, y: f64, z: f64) -> Self {
        Self::scaling(x, y, z) * self
    }

    pub fn rotation_x(r: f64) -> Self {
        let mut res = Self::ID;
        let (s, c) = r.sin_cos();
        res[(1, 1)] = c;
        res[(1, 2)] = -s;
        res[(2, 1)] = s;
        res[(2, 2)] = c;
        res
    }

    pub fn rotate_x(self, r: f64) -> Self {
        Self::rotation_x(r) * self
    }

    pub fn rotation_y(r: f64) -> Self {
        let mut res = Self::ID;
        let (s, c) = r.sin_cos();
        res[(0, 0)] = c;
        res[(0, 2)] = s;
        res[(2, 0)] = -s;
        res[(2, 2)] = c;
        res
    }

    pub fn rotate_y(self, r: f64) -> Self {
        Self::rotation_y(r) * self
    }

    pub fn rotation_z(r: f64) -> Self {
        let mut res = Self::ID;
        let (s, c) = r.sin_cos();
        res[(0, 0)] = c;
        res[(0, 1)] = -s;
        res[(1, 0)] = s;
        res[(1, 1)] = c;
        res
    }

    pub fn rotate_z(self, r: f64) -> Self {
        Self::rotation_z(r) * self
    }

    pub fn shearing(xy: f64, xz: f64, yx: f64, yz: f64, zx: f64, zy: f64) -> Self {
        let mut res = Self::ID;
        res[(0, 1)] = xy;
        res[(0, 2)] = xz;
        res[(1, 0)] = yx;
        res[(1, 2)] = yz;
        res[(2, 0)] = zx;
        res[(2, 1)] = zy;
        res
    }

    pub fn shear(self, xy: f64, xz: f64, yx: f64, yz: f64, zx: f64, zy: f64) -> Self {
        Self::shearing(xy, xz, yx, yz, zx, zy) * self
    }

    fn multiply(&self, x: f64, y: f64, z: f64, w: f64) -> (f64, f64, f64, f64) {
        (
            self[(0, 0)] * x + self[(0, 1)] * y + self[(0, 2)] * z + self[(0, 3)] * w,
            self[(1, 0)] * x + self[(1, 1)] * y + self[(1, 2)] * z + self[(1, 3)] * w,
            self[(2, 0)] * x + self[(2, 1)] * y + self[(2, 2)] * z + self[(2, 3)] * w,
            self[(3, 0)] * x + self[(3, 1)] * y + self[(3, 2)] * z + self[(3, 3)] * w,
        )
    }
}

impl IndexMut<(usize, usize)> for Matrix4x4 {
    fn index_mut(&mut self, index: (usize, usize)) -> &mut Self::Output {
        &mut self.m[index.0][index.1]
    }
}

impl Index<(usize, usize)> for Matrix4x4 {
    type Output = f64;

    fn index(&self, index: (usize, usize)) -> &Self::Output {
        &self.m[index.0][index.1]
    }
}

impl Mul<Matrix4x4> for Matrix4x4 {
    type Output = Matrix4x4;

    fn mul(self, rhs: Matrix4x4) -> Self::Output {
        let mut res = Matrix4x4::repeat(0.);
        for row in 0..4 {
            for col in 0..4 {
                res.m[row][col] = self.m[row][0] * rhs.m[0][col]
                    + self.m[row][1] * rhs.m[1][col]
                    + self.m[row][2] * rhs.m[2][col]
                    + self.m[row][3] * rhs.m[3][col]
            }
        }
        res
    }
}

impl Mul<Vector> for Matrix4x4 {
    type Output = Vector;

    fn mul(self, rhs: Vector) -> Self::Output {
        let (x, y, z, w) = self.multiply(rhs.x, rhs.y, rhs.z, rhs.w);
        Vector { x, y, z, w }
    }
}

impl Mul<Point> for Matrix4x4 {
    type Output = Point;

    fn mul(self, rhs: Point) -> Self::Output {
        let (x, y, z, w) = self.multiply(rhs.x, rhs.y, rhs.z, rhs.w);
        Point { x, y, z, w }
    }
}

#[cfg(test)]
mod tests4x4 {
    use std::f64::consts::{FRAC_PI_2, FRAC_PI_4};

    use crate::math::{matrix_invert::MatrixInvert, point::Point, round::Round};

    use super::*;

    #[test]
    fn view_transform_arb() -> () {
        let eye = Point::new(1., 3., 2.);
        let looking_at = Point::new(4., -2., 8.);
        let up = Vector::new(1., 1., 0.);
        let res = Matrix4x4::view_transform(eye, looking_at, up);
        assert_eq!(
            Matrix { m: res }.rounded(5),
            vec![
                -0.50709, 0.50709, 0.67612, -2.36643,
                0.76772, 0.60609, 0.12122, -2.82843,
                -0.35857, 0.59761, -0.71714, 0.,
                0., 0., 0., 1.
            ]
        );
    }

    #[test]
    fn view_transform_moves_world() -> () {
        let eye = Point::new(0., 0., 8.);
        let looking_at = Point::ORIGIN;
        let up = Vector::new(0., 1., 0.);
        let res = Matrix4x4::view_transform(eye, looking_at, up);
        assert_eq!(res, Matrix4x4::translation(0., 0., -8.));
    }

    #[test]
    fn view_transform_mirror() -> () {
        let eye = Point::ORIGIN;
        let looking_at = Point::new(0., 0., 1.);
        let up = Vector::new(0., 1., 0.);
        let res = Matrix4x4::view_transform(eye, looking_at, up);
        assert_eq!(res, Matrix4x4::scaling(-1., 1., -1.));
    }

    #[test]
    fn view_transform_id() -> () {
        let eye = Point::ORIGIN;
        let looking_at = Point::new(0., 0., -1.);
        let up = Vector::new(0., 1., 0.);
        let res = Matrix4x4::view_transform(eye, looking_at, up);
        assert_eq!(res, Matrix4x4::ID);
    }

    #[test]
    fn shearing() -> () {
        let p = Point::new(2., 3., 4.);
        assert_eq!(
            Matrix4x4::shearing(1., 0., 0., 0., 0., 0.) * p,
            Point::new(5., 3., 4.)
        );
        assert_eq!(
            Matrix4x4::shearing(0., 1., 0., 0., 0., 0.) * p,
            Point::new(6., 3., 4.)
        );
        assert_eq!(
            Matrix4x4::shearing(0., 0., 1., 0., 0., 0.) * p,
            Point::new(2., 5., 4.)
        );
        assert_eq!(
            Matrix4x4::shearing(0., 0., 0., 1., 0., 0.) * p,
            Point::new(2., 7., 4.)
        );
        assert_eq!(
            Matrix4x4::shearing(0., 0., 0., 0., 1., 0.) * p,
            Point::new(2., 3., 6.)
        );
        assert_eq!(
            Matrix4x4::shearing(0., 0., 0., 0., 0., 1.) * p,
            Point::new(2., 3., 7.)
        );
        assert_eq!(
            Matrix4x4::ID.shear(0., 0., 0., 0., 0., 1.) * p,
            Point::new(2., 3., 7.)
        );
    }

    #[test]
    fn rotation_z() -> () {
        let p = Point::new(0., 1., 0.);
        let eighth = Matrix4x4::rotation_z(FRAC_PI_4);
        let quarter = Matrix4x4::rotation_z(FRAC_PI_2);
        let sqrt = 2f64.sqrt() / 2.;
        assert_eq!(
            (eighth * p).rounded(5),
            Point::new(-sqrt, sqrt, 0.).rounded(5)
        );
        assert_eq!((quarter * p).rounded(5), Point::new(-1., 0., 0.).rounded(5));
        assert_eq!(
            (Matrix4x4::ID.rotate_z(FRAC_PI_2) * p).rounded(5),
            Point::new(-1., 0., 0.).rounded(5)
        );
    }

    #[test]
    fn rotation_y() -> () {
        let p = Point::new(0., 0., 1.);
        let eighth = Matrix4x4::rotation_y(FRAC_PI_4);
        let quarter = Matrix4x4::rotation_y(FRAC_PI_2);
        let sqrt = 2f64.sqrt() / 2.;
        assert_eq!(
            (eighth * p).rounded(5),
            Point::new(sqrt, 0., sqrt).rounded(5)
        );
        assert_eq!((quarter * p).rounded(5), Point::new(1., 0., 0.).rounded(5));
        assert_eq!(
            (Matrix4x4::ID.rotate_y(FRAC_PI_2) * p).rounded(5),
            Point::new(1., 0., 0.).rounded(5)
        );
    }

    #[test]
    fn rotation_x_inv() -> () {
        let p = Point::new(0., 1., 0.);
        let eighth = Matrix4x4::rotation_x(FRAC_PI_4);
        let eighthi = eighth.invert();
        assert!(eighthi.is_some());
        let sqrt = 2f64.sqrt() / 2.;
        assert_eq!(
            (eighthi.unwrap() * p).rounded(5),
            Point::new(0., sqrt, -sqrt).rounded(5)
        );
    }

    #[test]
    fn rotation_x() -> () {
        let p = Point::new(0., 1., 0.);
        let eighth = Matrix4x4::rotation_x(FRAC_PI_4);
        let quarter = Matrix4x4::rotation_x(FRAC_PI_2);
        let sqrt = 2f64.sqrt() / 2.;
        assert_eq!(
            (eighth * p).rounded(5),
            Point::new(0., sqrt, sqrt).rounded(5)
        );
        assert_eq!((quarter * p).rounded(5), Point::new(0., 0., 1.).rounded(5));
        assert_eq!(
            (Matrix4x4::ID.rotate_x(FRAC_PI_2) * p).rounded(5),
            Point::new(0., 0., 1.).rounded(5)
        );
    }

    #[test]
    fn reflection() -> () {
        let s = Matrix4x4::scaling(-1., 1., 1.);
        let p = Point::new(2., 3., 4.);
        let res = s * p;
        assert_eq!(res, Point::new(-2., 3., 4.));
    }

    #[test]
    fn scaling_vec_inv() -> () {
        let s = Matrix4x4::scaling(2., 3., 4.);
        let si = s.invert();
        assert!(si.is_some());
        let sip = si.unwrap();
        let v = Vector::new(-4., 6., 8.);
        let res = sip * v;
        assert_eq!(res, Vector::new(-2., 2., 2.));
    }

    #[test]
    fn scaling_vec() -> () {
        let s = Matrix4x4::scaling(2., 3., 4.);
        let v = Vector::new(-4., 6., 8.);
        let res = s * v;
        assert_eq!(res, Vector::new(-8., 18., 32.));
    }

    #[test]
    fn scaling_point() -> () {
        let s = Matrix4x4::scaling(2., 3., 4.);
        let p = Point::new(-4., 6., 8.);
        let res = s * p;
        assert_eq!(res, Point::new(-8., 18., 32.));
    }

    #[test]
    fn translation_vec() -> () {
        let t = Matrix4x4::translation(5., -3., 2.);
        let v = Vector::new(-3., 4., 5.);
        let res = t * v;
        assert_eq!(res, v);
        assert_eq!(Matrix4x4::ID.translate(-5., -3., 2.) * v, v);
    }

    #[test]
    fn translation_point_inv() -> () {
        let t = Matrix4x4::translation(5., -3., 2.);
        let ti = t.invert();
        assert!(ti.is_some());
        let tip = ti.unwrap();
        let p = Point::new(-3., 4., 5.);
        let res = tip * p;
        assert_eq!(res, Point::new(-8., 7., 3.));
    }

    #[test]
    fn translation_point() -> () {
        let t = Matrix4x4::translation(5., -3., 2.);
        let p = Point::new(-3., 4., 5.);
        let res = t * p;
        assert_eq!(res, Point::new(2., 1., 7.));
    }

    #[test]
    fn index() -> () {
        let mut m = Matrix4x4::ID;
        assert_eq!(m[(0, 0)], 1.);
        m[(0, 0)] = 2.;
        assert_eq!(m[(0, 0)], 2.);
    }

    #[test]
    fn mul_vector() -> () {
        let m = Matrix4x4::new(
            1., 2., 3., 4., 2., 4., 4., 2., 8., 6., 4., 1., 0., 0., 0., 1.,
        );
        let v = Vector {
            x: 1.,
            y: 2.,
            z: 3.,
            w: 1.,
        };
        assert_eq!(
            m * v,
            Vector {
                x: 18.,
                y: 24.,
                z: 33.,
                w: 1.
            }
        );
    }

    #[test]
    fn mul() -> () {
        let m1 = Matrix4x4::new(
            1., 2., 3., 4., 5., 6., 7., 8., 9., 8., 7., 6., 5., 4., 3., 2.,
        );
        let m2 = Matrix4x4::new(
            -2., 1., 2., 3., 3., 2., 1., -1., 4., 3., 6., 5., 1., 2., 7., 8.,
        );
        let ex = Matrix4x4::new(
            20., 22., 50., 48., 44., 54., 114., 108., 40., 58., 110., 102., 16., 26., 46., 42.,
        );
        assert_eq!(m1 * m2, ex);
    }

    #[test]
    fn new() -> () {
        let m = Matrix4x4::new(
            1., 2., 3., 4., 5.5, 6.5, 7.5, 8.5, 9., 10., 11., 12., 13.5, 14.5, 15.5, 16.5,
        );
        assert_eq!(m.m[0][0], 1.);
        assert_eq!(m.m[0][3], 4.);
        assert_eq!(m.m[1][0], 5.5);
        assert_eq!(m.m[1][2], 7.5);
        assert_eq!(m.m[2][2], 11.);
        assert_eq!(m.m[3][0], 13.5);
        assert_eq!(m.m[3][2], 15.5);
    }

    #[test]
    fn eq() -> () {
        let v = 2.22;
        let m1 = Matrix4x4::repeat(v);
        let m2 = Matrix4x4::repeat(v);
        assert_eq!(m1, m2);
    }

    #[test]
    fn neq() -> () {
        let m1 = Matrix4x4::repeat(2.22);
        let m2 = Matrix4x4::repeat(2.21);
        assert_ne!(m1, m2);
    }
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Matrix2x2 {
    pub m: [[f64; 2]; 2],
}

impl Matrix2x2 {
    pub fn new(m00: f64, m01: f64, m10: f64, m11: f64) -> Self {
        Self {
            m: [[m00, m01], [m10, m11]],
        }
    }
}

impl IndexMut<(usize, usize)> for Matrix2x2 {
    fn index_mut(&mut self, index: (usize, usize)) -> &mut Self::Output {
        &mut self.m[index.0][index.1]
    }
}

impl Index<(usize, usize)> for Matrix2x2 {
    type Output = f64;

    fn index(&self, index: (usize, usize)) -> &Self::Output {
        &self.m[index.0][index.1]
    }
}

#[cfg(test)]
mod tests2x2 {
    use super::*;

    #[test]
    fn index() -> () {
        let mut m = Matrix2x2::ID;
        assert_eq!(m[(0, 0)], 1.);
        m[(0, 0)] = 2.;
        assert_eq!(m[(0, 0)], 2.);
    }

    #[test]
    fn new() -> () {
        let m = Matrix2x2::new(-3., 5., 1., -2.);
        assert_eq!(m.m[0][0], -3.);
        assert_eq!(m.m[0][1], 5.);
        assert_eq!(m.m[1][0], 1.);
        assert_eq!(m.m[1][1], -2.);
    }
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Matrix3x3 {
    pub m: [[f64; 3]; 3],
}

impl Matrix3x3 {
    pub fn new(
        m00: f64,
        m01: f64,
        m02: f64,
        m10: f64,
        m11: f64,
        m12: f64,
        m20: f64,
        m21: f64,
        m22: f64,
    ) -> Self {
        Self {
            m: [[m00, m01, m02], [m10, m11, m12], [m20, m21, m22]],
        }
    }
}

impl IndexMut<(usize, usize)> for Matrix3x3 {
    fn index_mut(&mut self, index: (usize, usize)) -> &mut Self::Output {
        &mut self.m[index.0][index.1]
    }
}

impl Index<(usize, usize)> for Matrix3x3 {
    type Output = f64;

    fn index(&self, index: (usize, usize)) -> &Self::Output {
        &self.m[index.0][index.1]
    }
}

#[cfg(test)]
mod tests3x3 {
    use super::*;

    #[test]
    fn index() -> () {
        let mut m = Matrix3x3::ID;
        assert_eq!(m[(0, 0)], 1.);
        m[(0, 0)] = 2.;
        assert_eq!(m[(0, 0)], 2.);
    }

    #[test]
    fn new() -> () {
        let m = Matrix3x3::new(-3., 5., 0., 1., -2., -7., 0., 1., 1.);
        assert_eq!(m.m[0][0], -3.);
        assert_eq!(m.m[1][1], -2.);
        assert_eq!(m.m[2][2], 1.);
    }
}
