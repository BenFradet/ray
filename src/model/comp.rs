use std::rc::Rc;

use crate::{
    math::{point::Point, vector::Vector},
    shapes::shape::Shape,
};

use super::{intersection::Intersection, ray::Ray};

#[derive(PartialEq, Debug, Copy, Clone)]
pub struct RefractiveIndices {
    //n1: f64, // refractive index 1: material being exited at intersection
    //n2: f64, // refractive index 2: material being entered at intersection
    pub ratio: f64,
    pub cos1: f64,
    pub cos2: f64,
    sin2_2: f64,
}

impl RefractiveIndices {
    // https://en.wikipedia.org/wiki/Snell%27s_law
    pub fn new(n1: f64, n2: f64) -> Self {
        Self {
            ratio: n1 / n2,
            cos1: 0.,
            cos2: 0.,
            sin2_2: 0.,
        }
    }

    pub fn refract(mut self, eye: Vector, normal: Vector) -> Self {
        // sin1 / sin2 = n2 / n1
        // sin2 = sin1 * n1 / n2
        // sin1 = sqrt(1 - cos1^2)
        // sin1^2 = 1 - cos1^2
        // sin2^2 = (1 - cos1^2) * (n1 / n2)^2
        self.cos1 = eye.dot(normal);
        self.sin2_2 = (1. - self.cos1.powf(2.)) * self.ratio.powf(2.);
        self.cos2 = (1. - self.sin2_2).sqrt();
        self
    }

    pub fn total_internal_reflection(&self) -> bool {
        self.sin2_2 > 1.
    }
}

#[derive(PartialEq, Debug, Clone)]
pub struct Comp {
    pub intersection: Intersection,
    pub point: Point,
    pub over_point: Point,
    pub under_point: Point,
    pub eye: Vector,
    pub normal: Vector,
    pub reflect: Vector,
    pub inside: bool,
    pub indices: RefractiveIndices,
}

impl Comp {
    const EPS: f64 = 0.00001;

    pub fn new(intersection: Intersection, ray: Ray, is: &[Intersection]) -> Self {
        let point = ray.position(intersection.t);
        let eye = -ray.direction;
        let mut normal = intersection.shape.normal_at(point);
        let inside = if normal.dot(eye) < 0. {
            normal = -normal;
            true
        } else {
            false
        };
        let over_point = point + normal * Self::EPS;
        let under_point = point - normal * Self::EPS;
        let reflect = ray.direction.reflect(normal);
        let indices = Self::refractive_indices(&intersection, is).refract(eye, normal);
        Self {
            intersection,
            point,
            over_point,
            under_point,
            eye,
            normal,
            reflect,
            inside,
            indices,
        }
    }

    fn refractive_indices(hit: &Intersection, is: &[Intersection]) -> RefractiveIndices {
        let mut n1 = 0.;
        let mut n2 = 0.;
        let mut containers: Vec<Rc<Shape>> = Vec::new();
        for i in is {
            if i == hit {
                match containers.last() {
                    None => n1 = 1.,
                    Some(last) => n1 = last.material.refractive_index,
                };
            }

            let i_shape = Rc::clone(&i.shape);

            match containers.iter().position(|s| *s == i_shape) {
                None => containers.push(i_shape),
                Some(idx) => {
                    containers.remove(idx);
                }
            }

            if i == hit {
                match containers.last() {
                    None => n2 = 1.,
                    Some(last) => n2 = last.material.refractive_index,
                };
                break;
            }
        }
        RefractiveIndices::new(n1, n2)
    }
}

#[cfg(test)]
mod tests {
    use std::f64::consts::SQRT_2;

    use crate::{math::matrix::Matrix4x4, model::material::Material, shapes::shape::Shape};

    use super::*;

    #[test]
    fn total_internal_reflection() -> () {
        let indices = RefractiveIndices::new(2., 1.);
        let eye = Vector::new(0., -1., 0.);
        let normal = Vector::new(-1., 0., 0.);
        let tir = indices.refract(eye, normal);
        assert!(tir.sin2_2 > 1.);
        let eye = Vector::new(0., -1., 0.);
        let normal = Vector::new(0., -1., 0.);
        let not_tir = indices.refract(eye, normal);
        assert!(not_tir.sin2_2 < 1.);
    }

    #[test]
    fn under_point() -> () {
        let r = Ray::new(Point::new(0., 0., -5.), Vector::new(0., 0., 1.));
        let s = Shape::new_sphere(Matrix4x4::translation(0., 0., 1.))
            .unwrap()
            .material(Material::default().transparency(1.).refractive_index(1.));
        let i = Intersection::new(Rc::new(s), 5.);
        let c = Comp::new(i.clone(), r, &vec![i]);
        assert!(c.under_point.z > Comp::EPS / 2.);
        assert!(c.under_point.z > c.point.z);
    }

    #[test]
    fn refractive_indices() -> () {
        vec![
            (0usize, 1., 1.5),
            (1usize, 1.5, 2.),
            (2usize, 2., 2.5),
            (3usize, 2.5, 2.5),
            (4usize, 2.5, 1.5),
            (5usize, 1.5, 1.),
        ]
        .iter()
        .for_each(|(idx, n1, n2)| {
            let a = Rc::new(
                Shape::new_sphere(Matrix4x4::scaling(2., 2., 2.))
                    .unwrap()
                    .material(Material::default().transparency(1.).refractive_index(1.5)),
            );
            let b = Rc::new(
                Shape::new_sphere(Matrix4x4::translation(0., 0., -0.25))
                    .unwrap()
                    .material(Material::default().transparency(1.).refractive_index(2.)),
            );
            let c = Rc::new(
                Shape::new_sphere(Matrix4x4::translation(0., 0., 0.25))
                    .unwrap()
                    .material(Material::default().transparency(1.).refractive_index(2.5)),
            );
            let r = Ray::new(Point::new(0., 0., -4.), Vector::new(1., 0., 0.));
            let is = vec![
                Intersection::new(Rc::clone(&a), 2.),
                Intersection::new(Rc::clone(&b), 2.75),
                Intersection::new(Rc::clone(&c), 3.25),
                Intersection::new(Rc::clone(&b), 4.75),
                Intersection::new(Rc::clone(&c), 5.25),
                Intersection::new(Rc::clone(&a), 6.),
            ];
            let c = Comp::new(is[*idx].clone(), r, &is);
            assert_eq!(c.indices.ratio, *n1 / *n2);
        });
    }

    #[test]
    fn reflect() -> () {
        let s = Shape::id_plane();
        let s2 = SQRT_2 / 2.;
        let r = Ray::new(Point::new(0., 1., -1.), Vector::new(0., -s2, s2));
        let i = Intersection::new(Rc::new(s), s2 * 2.);
        let c = Comp::new(i.clone(), r, &vec![i]);
        assert_eq!(c.reflect, Vector::new(0., s2, s2));
    }

    #[test]
    fn over_point() -> () {
        let r = Ray::new(Point::new(0., 0., -5.), Vector::new(0., 0., 1.));
        let s = Shape::new_sphere(Matrix4x4::translation(0., 0., 1.)).unwrap_or(Shape::id_sphere());
        let i = Intersection::new(Rc::new(s), 5.);
        let c = Comp::new(i.clone(), r, &vec![i]);
        assert!(c.over_point.z < -Comp::EPS / 2.);
        assert!(c.point.z > c.over_point.z);
    }

    #[test]
    fn inside() -> () {
        let r = Ray::new(Point::ORIGIN, Vector::new(0., 0., 1.));
        let s = Shape::id_sphere();
        let i = Intersection::new(Rc::new(s), 1.);
        let c = Comp::new(i.clone(), r, &vec![i]);
        assert_eq!(c.point, Point::new(0., 0., 1.));
        assert_eq!(c.eye, Vector::new(0., 0., -1.));
        assert_eq!(c.normal, Vector::new(0., 0., -1.));
        assert!(c.inside);
    }

    #[test]
    fn not_inside() -> () {
        let r = Ray::new(Point::new(0., 0., -5.), Vector::new(0., 0., 1.));
        let s = Shape::id_sphere();
        let i = Intersection::new(Rc::new(s), 4.);
        let c = Comp::new(i.clone(), r, &vec![i]);
        assert!(!c.inside);
    }

    #[test]
    fn new() -> () {
        let r = Ray::new(Point::new(0., 0., -5.), Vector::new(0., 0., 1.));
        let s = Rc::new(Shape::id_sphere());
        let i = Intersection::new(Rc::clone(&s), 4.);
        let it = i.t;
        let c = Comp::new(i.clone(), r, &vec![i]);
        assert_eq!(c.intersection.t, it);
        assert_eq!(c.intersection.shape, s);
        assert_eq!(c.point, Point::new(0., 0., -1.));
        assert_eq!(c.eye, Vector::new(0., 0., -1.));
        assert_eq!(c.normal, Vector::new(0., 0., -1.));
    }
}
