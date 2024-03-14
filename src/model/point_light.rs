use crate::math::{colour::Colour, point::Point};

#[derive(PartialEq, Debug, Copy, Clone)]
pub struct PointLight {
    pub position: Point,
    pub intensity: Colour,
}

impl PointLight {
    pub fn new(p: Point, i: Colour) -> Self {
        Self {
            position: p,
            intensity: i,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new() -> () {
        let p = Point::new(0., 0., 0.);
        let i = Colour::WHITE;
        let pl = PointLight::new(p, i);
        assert_eq!(pl.position, p);
        assert_eq!(pl.intensity, i);
    }
}
