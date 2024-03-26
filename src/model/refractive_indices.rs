use std::rc::Rc;

use crate::{math::vector::Vector, shapes::shape::Shape};

use super::intersection::Intersection;

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
    fn from(n1: f64, n2: f64) -> Self {
        Self {
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
}

#[cfg(test)]
mod tests {
    use super::*;

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
}