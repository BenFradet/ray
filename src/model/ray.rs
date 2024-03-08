use crate::math::{matrix::Matrix4x4, matrix_invert::MatrixInvert, point::Point, vector::Vector};

use super::{intersection::Intersection, sphere::Sphere};

#[derive(PartialEq, Debug, Copy, Clone)]
pub struct Ray {
    origin: Point,
    direction: Vector,
}

impl Ray {
    pub fn new(origin: Point, direction: Vector) -> Self {
        Self { origin, direction, }
    }

    pub fn position(&self, t: f64) -> Point {
        self.origin + self.direction * t
    }

    // https://en.wikipedia.org/wiki/Line%E2%80%93sphere_intersection
    pub fn intersections(&self, s: Sphere) -> Vec<Intersection> {
        let t_ray = match s.t.invert() {
            None => *self,
            Some(inv) => self.transform(inv),
        };
        let sphere_to_ray = t_ray.origin - s.center;
        let a = t_ray.direction.dot(t_ray.direction);
        let b = 2. * t_ray.direction.dot(sphere_to_ray);
        let c = sphere_to_ray.dot(sphere_to_ray) - s.radius.powf(2.);
        let discriminant = b.powf(2.) - 4. * a * c;

        if discriminant < 0. {
            vec![]
        } else {
            // discriminant = 0 is one solution but we still output two
            let t1 = (-b - discriminant.sqrt()) / (2. * a);
            let t2 = (-b + discriminant.sqrt()) / (2. * a);
            vec![Intersection::new(t1, s), Intersection::new(t2, s)]
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
    use super::*;

    #[test]
    fn intersect_translated_sphere() -> () {
        let r = Ray::new(Point::new(0., 0., -5.), Vector::new(0., 0., 1.));
        let s = Sphere::new().t(Matrix4x4::translation(5., 0., 0.));
        let res = r.intersections(s);
        assert_eq!(res.len(), 0);
    }

    #[test]
    fn intersect_scaled_sphere() -> () {
        let r = Ray::new(Point::new(0., 0., -5.), Vector::new(0., 0., 1.));
        let s = Sphere::new().t(Matrix4x4::scaling(2., 2., 2.));
        let res = r.intersections(s);
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
        let s = Sphere::new();
        let res = r.intersections(s);
        assert_eq!(res.len(), 2);
        assert_eq!(res[0], Intersection::new(-6., s));
        assert_eq!(res[1], Intersection::new(-4., s));
    }

    #[test]
    fn intersect_inside_sphere() -> () {
        let r = Ray::new(Point::new(0., 0., 0.), Vector::new(0., 0., 1.));
        let s = Sphere::new();
        let res = r.intersections(s);
        assert_eq!(res.len(), 2);
        assert_eq!(res[0], Intersection::new(-1., s));
        assert_eq!(res[1], Intersection::new(1., s));
    }

    #[test]
    fn intersect_no_points() -> () {
        let r = Ray::new(Point::new(0., 2., -5.), Vector::new(0., 0., 1.));
        let s = Sphere::new();
        let res = r.intersections(s);
        assert_eq!(res.len(), 0);
    }

    #[test]
    fn intersect_same_point() -> () {
        let r = Ray::new(Point::new(0., 1., -5.), Vector::new(0., 0., 1.));
        let s = Sphere::new();
        let res = r.intersections(s);
        assert_eq!(res.len(), 2);
        assert_eq!(res[0], Intersection::new(5., s));
        assert_eq!(res[1], Intersection::new(5., s));
    }

    #[test]
    fn intersect_2_points() -> () {
        let r = Ray::new(Point::new(0., 0., -5.), Vector::new(0., 0., 1.));
        let s = Sphere::new();
        let res = r.intersections(s);
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