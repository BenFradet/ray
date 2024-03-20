use crate::shape::shape::Shape;

#[derive(PartialEq, Debug, Copy, Clone)]
pub struct Intersection {
    pub t: f64,
    pub shape: Shape,
}

impl Intersection {
    // the intersection takes ownership of the shape
    // might need to revisit later
    pub fn new(t: f64, shape: Shape) -> Self {
        Self { t, shape }
    }
}

pub trait IntersectionHit {
    fn hit(self) -> Option<Intersection>;
}

impl<I: IntoIterator<Item = Intersection>> IntersectionHit for I {
    fn hit(self) -> Option<Intersection> {
        self.into_iter().fold(None, |acc, incoming| {
            if incoming.t < 0. {
                acc
            } else {
                match acc {
                    None => Some(incoming),
                    Some(existing) => {
                        if incoming.t < existing.t {
                            Some(incoming)
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
    use super::*;

    #[test]
    fn hit_smallest_non_neg() -> () {
        let s = Shape::id_sphere();
        let i1 = Intersection::new(5., s);
        let i2 = Intersection::new(7., s);
        let i3 = Intersection::new(-3., s);
        let i4 = Intersection::new(2., s);
        let is = vec![i1, i2, i3, i4];
        assert_eq!(is.hit(), Some(i4));
    }

    #[test]
    fn hit_all_neg() -> () {
        let s = Shape::id_sphere();
        let i1 = Intersection::new(-1., s);
        let i2 = Intersection::new(-2., s);
        let is = vec![i1, i2];
        assert_eq!(is.hit(), None);
    }

    #[test]
    fn hit_some_neg() -> () {
        let s = Shape::id_sphere();
        let i1 = Intersection::new(-1., s);
        let i2 = Intersection::new(1., s);
        let is = vec![i1, i2];
        assert_eq!(is.hit(), Some(i2));
    }

    #[test]
    fn hit_all_pos() -> () {
        let s = Shape::id_sphere();
        let i1 = Intersection::new(1., s);
        let i2 = Intersection::new(2., s);
        let is = vec![i1, i2];
        assert_eq!(is.hit(), Some(i1));
    }

    #[test]
    fn new() -> () {
        let s = Shape::id_sphere();
        let i = Intersection::new(0., s);
        assert_eq!(i.t, 0.);
        assert_eq!(i.shape, s);
    }
}
