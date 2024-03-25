use std::rc::Rc;

use crate::shapes::{intersect::Intersect, shape::Shape};

use super::ray::Ray;

#[derive(PartialEq, Debug, Clone)]
pub struct Intersection {
    pub shape: Rc<Shape>,
    pub t: f64,
}

impl Intersection {
    pub fn new(shape: Rc<Shape>, t: f64) -> Self {
        Self { shape, t }
    }

    pub fn intersections(shape: Rc<Shape>, r: &Ray) -> Vec<Intersection> {
        let t_ray = r.transform(shape.inv_t);
        let ts = shape.underlying.intersect(&t_ray);
        ts.iter()
            .map(|t| Intersection::new(Rc::clone(&shape), *t))
            .collect()
    }
}

pub trait IntersectionHit {
    fn hit(&self) -> Option<Intersection>;
}

impl IntersectionHit for Vec<Intersection> {
    // into_iter takes ownership
    // can't find the generic for iter()
    fn hit(&self) -> Option<Intersection> {
        self.iter().fold(None, |acc, incoming| {
            if incoming.t < 0. {
                acc
            } else {
                match acc {
                    None => Some(incoming.clone()),
                    Some(ref existing) => {
                        if incoming.t < existing.t {
                            Some(incoming.clone())
                        } else {
                            acc
                        }
                    }
                }
            }
        })
    }
}

#[cfg(test)]
mod tests {
    use crate::math::{matrix::Matrix4x4, point::Point, vector::Vector};

    use super::*;

    #[test]
    fn intersections_translated_shape() -> () {
        let r = Ray::new(Point::new(0., 0., -5.), Vector::new(0., 0., 1.));
        let s = Shape::new_sphere(Matrix4x4::translation(5., 0., 0.)).unwrap_or(Shape::id_sphere());
        let res = Intersection::intersections(Rc::new(s), &r);
        assert_eq!(res, vec![]);
    }

    #[test]
    fn intersections_scaled_shape() -> () {
        let r = Ray::new(Point::new(0., 0., -5.), Vector::new(0., 0., 1.));
        let s = Shape::new_sphere(Matrix4x4::scaling(2., 2., 2.)).unwrap_or(Shape::id_sphere());
        let res = Intersection::intersections(Rc::new(s), &r);
        assert_eq!(res[0].t, 3.);
        assert_eq!(res[1].t, 7.);
    }

    #[test]
    fn hit_smallest_non_neg() -> () {
        let s = Rc::new(Shape::id_sphere());
        let i1 = Intersection::new(Rc::clone(&s), 5.);
        let i2 = Intersection::new(Rc::clone(&s), 7.);
        let i3 = Intersection::new(Rc::clone(&s), -3.);
        let i4 = Intersection::new(Rc::clone(&s), 2.);
        let is = vec![i1, i2, i3, i4.clone()];
        assert_eq!(is.hit(), Some(i4));
    }

    #[test]
    fn hit_all_neg() -> () {
        let s = Rc::new(Shape::id_sphere());
        let i1 = Intersection::new(Rc::clone(&s), -1.);
        let i2 = Intersection::new(Rc::clone(&s), -2.);
        let is = vec![i1, i2];
        assert_eq!(is.hit(), None);
    }

    #[test]
    fn hit_some_neg() -> () {
        let s = Rc::new(Shape::id_sphere());
        let i1 = Intersection::new(Rc::clone(&s), -1.);
        let i2 = Intersection::new(Rc::clone(&s), 1.);
        let is = vec![i1, i2.clone()];
        assert_eq!(is.hit(), Some(i2));
    }

    #[test]
    fn hit_all_pos() -> () {
        let s = Rc::new(Shape::id_sphere());
        let i1 = Intersection::new(Rc::clone(&s), 1.);
        let i2 = Intersection::new(Rc::clone(&s), 2.);
        let is = vec![i1.clone(), i2];
        assert_eq!(is.hit(), Some(i1));
    }

    #[test]
    fn new() -> () {
        let s = Rc::new(Shape::id_sphere());
        let i = Intersection::new(Rc::clone(&s), 0.);
        assert_eq!(i.t, 0.);
        assert_eq!(i.shape, s);
    }
}
