use crate::math::{matrix::Matrix4x4, point::Point, vector::Vector};

use super::{camera::Camera, intersection::Intersection, sphere::Sphere};

#[derive(PartialEq, Debug, Copy, Clone)]
pub struct Ray {
    origin: Point,
    pub direction: Vector,
}

impl Ray {
    pub fn new(origin: Point, direction: Vector) -> Self {
        Self { origin, direction }
    }

    pub fn for_pixel(camera: &Camera, px: usize, py: usize) -> Self {
        let pxf = px as f64;
        let pyf = py as f64;
        let x_offset = (pxf + 0.5) * camera.pixel_size;
        let y_offset = (pyf + 0.5) * camera.pixel_size;

        let world_x = camera.half_width - x_offset;
        let world_y = camera.half_height - y_offset;

        let pixel = camera.inv_t * Point::new(world_x, world_y, -1.);
        let origin = camera.inv_t * Point::ORIGIN;
        let direction = (pixel - origin).norm();
        Self::new(origin, direction)
    }

    pub fn position(&self, t: f64) -> Point {
        self.origin + self.direction * t
    }

    // https://en.wikipedia.org/wiki/Line%E2%80%93sphere_intersection
    pub fn intersections(&self, s: &Sphere) -> Vec<Intersection> {
        let t_ray = self.transform(s.inv_t);
        let sphere_to_ray = t_ray.origin - Point::ORIGIN;
        let a = t_ray.direction.dot(t_ray.direction);
        let b = 2. * t_ray.direction.dot(sphere_to_ray);
        let c = sphere_to_ray.dot(sphere_to_ray) - Sphere::RADIUS.powf(2.);
        let discriminant = b.powf(2.) - 4. * a * c;

        if discriminant < 0. {
            vec![]
        } else {
            // discriminant = 0 is one solution but we still output two
            let t1 = (-b - discriminant.sqrt()) / (2. * a);
            let t2 = (-b + discriminant.sqrt()) / (2. * a);
            vec![Intersection::new(t1, *s), Intersection::new(t2, *s)]
        }
    }

    pub fn transform(&self, t: Matrix4x4) -> Self {
        Self {
            origin: t * self.origin,
            direction: t * self.direction,
        }
    }
}

#[cfg(test)]
mod tests {
    use std::f64::consts::{FRAC_PI_2, FRAC_PI_4, SQRT_2};

    use crate::math::round::Round;

    use super::*;

    #[test]
    fn for_pixel_camera_transformed() -> () {
        let t = Matrix4x4::translation(0., -2., 5.).rotate_y(FRAC_PI_4);
        let c = Camera::new(201, 101, FRAC_PI_2).transform(t).unwrap();
        let res = Ray::for_pixel(&c, 100, 50);
        assert_eq!(res.origin, Point::new(0., 2., -5.));
        let s2 = SQRT_2 / 2.;
        assert_eq!(res.direction.rounded(5), vec![s2, 0., -s2, 0.].rounded(5));
    }

    #[test]
    fn for_pixel_corner() -> () {
        let c = Camera::new(201, 101, FRAC_PI_2);
        let res = Ray::for_pixel(&c, 0, 0);
        assert_eq!(res.origin, Point::ORIGIN);
        assert_eq!(res.direction.rounded(5), vec![0.66519, 0.33259, -0.66851, 0.]);
    }

    #[test]
    fn for_pixel_center() -> () {
        let c = Camera::new(201, 101, FRAC_PI_2);
        let res = Ray::for_pixel(&c, 100, 50);
        assert_eq!(res.origin, Point::ORIGIN);
        assert_eq!(res.direction.rounded(5), vec![0., 0., -1., 0.]);
    }

    #[test]
    fn intersect_translated_sphere() -> () {
        let r = Ray::new(Point::new(0., 0., -5.), Vector::new(0., 0., 1.));
        let s = Sphere::id().t(Matrix4x4::translation(5., 0., 0.)).unwrap();
        let res = r.intersections(&s);
        assert_eq!(res.len(), 0);
    }

    #[test]
    fn intersect_scaled_sphere() -> () {
        let r = Ray::new(Point::new(0., 0., -5.), Vector::new(0., 0., 1.));
        let s = Sphere::id().t(Matrix4x4::scaling(2., 2., 2.)).unwrap();
        let res = r.intersections(&s);
        assert_eq!(res.len(), 2);
        assert_eq!(res[0], Intersection::new(3., s));
        assert_eq!(res[1], Intersection::new(7., s));
    }

    #[test]
    fn scaling() -> () {
        let r = Ray::new(Point::new(1., 2., 3.), Vector::new(0., 1., 0.));
        let m = Matrix4x4::scaling(2., 3., 4.);
        let res = r.transform(m);
        assert_eq!(res.origin, Point::new(2., 6., 12.));
        assert_eq!(res.direction, Vector::new(0., 3., 0.));
    }

    #[test]
    fn translation() -> () {
        let dir = Vector::new(0., 1., 0.);
        let r = Ray::new(Point::new(1., 2., 3.), dir);
        let m = Matrix4x4::translation(3., 4., 5.);
        let res = r.transform(m);
        assert_eq!(res.origin, Point::new(4., 6., 8.));
        assert_eq!(res.direction, dir);
    }

    #[test]
    fn intersect_after_sphere() -> () {
        let r = Ray::new(Point::new(0., 0., 5.), Vector::new(0., 0., 1.));
        let s = Sphere::id();
        let res = r.intersections(&s);
        assert_eq!(res.len(), 2);
        assert_eq!(res[0], Intersection::new(-6., s));
        assert_eq!(res[1], Intersection::new(-4., s));
    }

    #[test]
    fn intersect_inside_sphere() -> () {
        let r = Ray::new(Point::new(0., 0., 0.), Vector::new(0., 0., 1.));
        let s = Sphere::id();
        let res = r.intersections(&s);
        assert_eq!(res.len(), 2);
        assert_eq!(res[0], Intersection::new(-1., s));
        assert_eq!(res[1], Intersection::new(1., s));
    }

    #[test]
    fn intersect_no_points() -> () {
        let r = Ray::new(Point::new(0., 2., -5.), Vector::new(0., 0., 1.));
        let s = Sphere::id();
        let res = r.intersections(&s);
        assert_eq!(res.len(), 0);
    }

    #[test]
    fn intersect_same_point() -> () {
        let r = Ray::new(Point::new(0., 1., -5.), Vector::new(0., 0., 1.));
        let s = Sphere::id();
        let res = r.intersections(&s);
        assert_eq!(res.len(), 2);
        assert_eq!(res[0], Intersection::new(5., s));
        assert_eq!(res[1], Intersection::new(5., s));
    }

    #[test]
    fn intersect_2_points() -> () {
        let r = Ray::new(Point::new(0., 0., -5.), Vector::new(0., 0., 1.));
        let s = Sphere::id();
        let res = r.intersections(&s);
        assert_eq!(res.len(), 2);
        assert_eq!(res[0], Intersection::new(4., s));
        assert_eq!(res[1], Intersection::new(6., s));
    }

    #[test]
    fn position() -> () {
        let p = Point::new(2., 3., 4.);
        let r = Ray::new(p, Vector::new(1., 0., 0.));
        assert_eq!(r.position(0.), p);
        assert_eq!(r.position(1.), Point::new(3., 3., 4.));
        assert_eq!(r.position(-1.), Point::new(1., 3., 4.));
        assert_eq!(r.position(2.5), Point::new(4.5, 3., 4.));
    }

    #[test]
    fn new() -> () {
        let o = Point::new(1., 2., 3.);
        let d = Vector::new(4., 5., 6.);
        let r = Ray::new(o, d);
        assert_eq!(r.origin, o);
        assert_eq!(r.direction, d);
    }
}
