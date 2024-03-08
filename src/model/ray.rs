use crate::math::{point::Point, vector::Vector};

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
    pub fn intersect(&self, s: &Sphere) -> Vec<Intersection> {
        let sphere_to_ray = self.origin - s.center;
        let a = self.direction.dot(self.direction);
        let b = 2. * self.direction.dot(sphere_to_ray);
        let c = sphere_to_ray.dot(sphere_to_ray) - s.radius.powf(2.);
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
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn intersect_after_sphere() -> () {
        let r = Ray::new(Point::new(0., 0., 5.), Vector::new(0., 0., 1.));
        let s = Sphere::new();
        let res = r.intersect(&s);
        assert_eq!(res.len(), 2);
        assert_eq!(res[0], Intersection::new(-6., s));
        assert_eq!(res[1], Intersection::new(-4., s));
    }

    #[test]
    fn intersect_inside_sphere() -> () {
        let r = Ray::new(Point::new(0., 0., 0.), Vector::new(0., 0., 1.));
        let s = Sphere::new();
        let res = r.intersect(&s);
        assert_eq!(res.len(), 2);
        assert_eq!(res[0], Intersection::new(-1., s));
        assert_eq!(res[1], Intersection::new(1., s));
    }

    #[test]
    fn intersect_no_points() -> () {
        let r = Ray::new(Point::new(0., 2., -5.), Vector::new(0., 0., 1.));
        let s = Sphere::new();
        let res = r.intersect(&s);
        assert_eq!(res.len(), 0);
    }

    #[test]
    fn intersect_same_point() -> () {
        let r = Ray::new(Point::new(0., 1., -5.), Vector::new(0., 0., 1.));
        let s = Sphere::new();
        let res = r.intersect(&s);
        assert_eq!(res.len(), 2);
        assert_eq!(res[0], Intersection::new(5., s));
        assert_eq!(res[1], Intersection::new(5., s));
    }

    #[test]
    fn intersect_2_points() -> () {
        let r = Ray::new(Point::new(0., 0., -5.), Vector::new(0., 0., 1.));
        let s = Sphere::new();
        let res = r.intersect(&s);
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