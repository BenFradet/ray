use crate::math::{colour::Colour, point::Point};

use super::{pattern_at::PatternAt, pattern_kind::PatternKind};

// can't have Copy for recursive data structures
#[derive(PartialEq, Debug, Clone)]
pub struct Nested {
    p: PatternKind,
    child: Option<Box<Nested>>,
}

impl Nested {
    pub fn new(p: PatternKind) -> Self {
        Self { p, child: None }
    }

    pub fn with_child(p: PatternKind, child: Nested) -> Self {
        Self {
            p,
            child: Some(Box::new(child)),
        }
    }

    fn rec(&self, p: Point) -> Colour {
        fn go(n: Nested, p: Point, colour: Colour, depth: usize) -> (Colour, usize) {
            let at = n.p.pattern_at(p);
            let new_colour = at + colour;
            match n.child {
                None => (new_colour, depth),
                Some(c) => go(*c, p, new_colour, depth + 1),
            }
        }
        let (colour, depth) = go(self.clone(), p, Colour::BLACK, 1);
        colour * (1. / depth as f64)
    }
}

impl PatternAt for Nested {
    fn pattern_at(&self, p: Point) -> Colour {
        self.rec(p)
    }
}

#[cfg(test)]
mod tests {
    use crate::pattern::{gradient::Gradient, solid::Solid};

    use super::*;

    #[test]
    fn pattern_at() -> () {
        let g = PatternKind::Gradient(Gradient::new(Colour::WHITE, Colour::BLACK));
        let s = PatternKind::Solid(Solid::new(Colour::WHITE));
        let nested = Nested::with_child(g, Nested::new(s));
        assert_eq!(nested.pattern_at(Point::ORIGIN), Colour::WHITE);
        assert_eq!(
            nested.pattern_at(Point::new(0.25, 0., 0.)),
            Colour::new(0.875, 0.875, 0.875)
        );
        assert_eq!(
            nested.pattern_at(Point::new(0.5, 0., 0.)),
            Colour::new(0.75, 0.75, 0.75)
        );
        assert_eq!(
            nested.pattern_at(Point::new(0.75, 0., 0.)),
            Colour::new(0.625, 0.625, 0.625)
        );
    }

    #[test]
    fn with_child() -> () {
        let p1 = PatternKind::Gradient(Gradient::new(Colour::WHITE, Colour::BLACK));
        let p2 = PatternKind::Solid(Solid::new(Colour::WHITE));
        let top = Nested::with_child(p1, Nested::new(p2));
        assert_eq!(top.p, p1);
        assert_eq!(top.child, Some(Box::new(Nested::new(p2))));
    }

    #[test]
    fn new() -> () {
        let p = PatternKind::Solid(Solid::new(Colour::WHITE));
        let n = Nested::new(p);
        assert_eq!(n.p, p);
        assert_eq!(n.child, None);
    }
}
