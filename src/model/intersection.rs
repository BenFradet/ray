use crate::shape::shape::Shape;

#[derive(PartialEq, Debug, Clone)]
pub struct Intersection<'a> {
    pub shape: &'a Shape,
    pub t: f64,
}

impl<'a> Intersection<'a> {
    // the intersection takes ownership of the shape
    // might need to revisit later
    pub fn new(shape: &'a Shape, t: f64) -> Self {
        Self { shape, t }
    }
}

pub trait IntersectionHit<'a> {
    fn hit(self) -> Option<Intersection<'a>>;
}

impl<'a, I: IntoIterator<Item = Intersection<'a>>> IntersectionHit<'a> for I {
    fn hit(self) -> Option<Intersection<'a>> {
        self.into_iter().fold(None, |acc, incoming| {
            if incoming.t < 0. {
                acc
            } else {
                match acc {
                    None => Some(incoming),
                    Some(ref existing) => {
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
        let i1 = Intersection::new(&s, 5.);
        let i2 = Intersection::new(&s, 7.);
        let i3 = Intersection::new(&s, -3.);
        let i4 = Intersection::new(&s, 2.);
        let is = vec![i1, i2, i3, i4.clone()];
        assert_eq!(is.hit(), Some(i4));
    }

    #[test]
    fn hit_all_neg() -> () {
        let s = Shape::id_sphere();
        let i1 = Intersection::new(&s, -1.);
        let i2 = Intersection::new(&s, -2.);
        let is = vec![i1, i2];
        assert_eq!(is.hit(), None);
    }

    #[test]
    fn hit_some_neg() -> () {
        let s = Shape::id_sphere();
        let i1 = Intersection::new(&s, -1.);
        let i2 = Intersection::new(&s, 1.);
        let is = vec![i1, i2.clone()];
        assert_eq!(is.hit(), Some(i2));
    }

    #[test]
    fn hit_all_pos() -> () {
        let s = Shape::id_sphere();
        let i1 = Intersection::new(&s, 1.);
        let i2 = Intersection::new(&s, 2.);
        let is = vec![i1.clone(), i2];
        assert_eq!(is.hit(), Some(i1));
    }

    #[test]
    fn new() -> () {
        let s = Shape::id_sphere();
        let i = Intersection::new(&s, 0.);
        assert_eq!(i.t, 0.);
        assert_eq!(i.shape, &s);
    }
}
