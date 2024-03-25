use crate::math::{colour::Colour, point::Point};

use super::pattern_at::PatternAt;

#[derive(PartialEq, Debug, Copy, Clone)]
pub struct RadialGradient {
    a: Colour,
    b: Colour,
}

impl RadialGradient {
    pub fn new(a: Colour, b: Colour) -> Self {
        Self { a, b }
    }
}

impl PatternAt for RadialGradient {
    fn pattern_at(&self, p: Point) -> Colour {
        let pxsq = p.x.powf(2.);
        let pzsq = p.z.powf(2.);
        let sum_sqrt = (pxsq + pzsq).sqrt();
        self.a + (self.b - self.a) * (sum_sqrt - sum_sqrt.floor())
    }
}

#[cfg(test)]
mod tests {
    use crate::math::round::Round;

    use super::*;

    #[test]
    fn pattern_at() -> () {
        let rg = RadialGradient::new(Colour::WHITE, Colour::BLACK);
        assert_eq!(rg.pattern_at(Point::ORIGIN), Colour::WHITE);
        assert_eq!(
            rg.pattern_at(Point::new(0.25, 0., 0.25)).rounded(5),
            vec![0.64645, 0.64645, 0.64645]
        );
        assert_eq!(
            rg.pattern_at(Point::new(0.5, 0., 0.5)).rounded(5),
            vec![0.29289, 0.29289, 0.29289]
        );
        assert_eq!(
            rg.pattern_at(Point::new(0.75, 0., 0.75)).rounded(5),
            vec![0.93934, 0.93934, 0.93934]
        );
    }

    #[test]
    fn new() -> () {
        let rg = RadialGradient::new(Colour::WHITE, Colour::BLACK);
        assert_eq!(rg.a, Colour::WHITE);
        assert_eq!(rg.b, Colour::BLACK);
    }
}
