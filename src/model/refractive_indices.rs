use std::rc::Rc;

use crate::{math::vector::Vector, shapes::shape::Shape};

use super::intersection::Intersection;

#[derive(PartialEq, Debug, Copy, Clone)]
pub struct RefractiveIndices {
    n1: f64, // refractive index 1: material being exited at intersection
    n2: f64, // refractive index 2: material being entered at intersection
    pub ratio: f64,
    pub cos1: f64,
    pub cos2: f64,
    sin2_2: f64,
}

impl RefractiveIndices {
    fn from(n1: f64, n2: f64) -> Self {
        Self {
            n1,
            n2,
            ratio: n1 / n2,
            cos1: 0.,
            cos2: 0.,
            sin2_2: 0.,
        }
    }

    pub fn new(hit: &Intersection, is: &[Intersection]) -> Self {
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
        Self::from(n1, n2)
    }

    // https://en.wikipedia.org/wiki/Snell%27s_law
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

    // https://en.wikipedia.org/wiki/Schlick%27s_approximation
    pub fn reflectance(&self) -> f64 {
        if self.total_internal_reflection() {
            1.
        } else {
            let cos = if self.ratio > 1. {
                self.cos2
            } else {
                self.cos1
            };
            let r0 = ((self.n1 - self.n2) / (self.n1 + self.n2)).powf(2.);
            r0 + (1. - r0) * (1. - cos).powf(5.)
        }
    }
}

#[cfg(test)]
mod tests {
    use std::f64::consts::SQRT_2;

    use crate::{
        math::{matrix::Matrix4x4, point::Point},
        model::{comp::Comp, material::Material, ray::Ray},
    };

    use super::*;

    fn glass() -> Shape {
        Shape::id_sphere().material(Material::default().transparency(1.).refractive_index(1.5))
    }

    #[test]
    fn reflectance_small_angle() -> () {
        let s = Rc::new(glass());
        let r = Ray::new(Point::new(0., 0.99, -2.), Vector::Z);
        let is = vec![Intersection::new(Rc::clone(&s), 1.8589)];
        let c = Comp::new(is[0].clone(), r, &is);
        let res = c.indices.reflectance();
        assert_eq!((res * 100000.).round(), 0.48873 * 100000.);
    }

    #[test]
    fn reflectance_perpendicular() -> () {
        let s = Rc::new(glass());
        let r = Ray::new(Point::ORIGIN, Vector::Y);
        let is = vec![
            Intersection::new(Rc::clone(&s), -1.),
            Intersection::new(Rc::clone(&s), 1.),
        ];
        let c = Comp::new(is[1].clone(), r, &is);
        let res = c.indices.reflectance();
        assert_eq!((res * 100000.).round(), 0.04 * 100000.);
    }

    #[test]
    fn reflectance_tir() -> () {
        let s = Rc::new(glass());
        let s2 = SQRT_2 / 2.;
        let r = Ray::new(Point::new(0., 0., s2), Vector::Y);
        let is = vec![
            Intersection::new(Rc::clone(&s), -s2),
            Intersection::new(Rc::clone(&s), s2),
        ];
        let c = Comp::new(is[1].clone(), r, &is);
        let res = c.indices.reflectance();
        assert_eq!(res, 1.);
    }

    #[test]
    fn total_internal_reflection() -> () {
        let indices = RefractiveIndices::from(2., 1.);
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
}
