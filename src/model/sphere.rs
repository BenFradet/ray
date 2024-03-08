use crate::math::point::Point;

#[derive(PartialEq, Debug, Copy, Clone)]
pub struct Sphere {
    pub center: Point,
    pub radius: f64,
}

impl Sphere {
    pub fn new() -> Self {
        Sphere {
            center: Point::new(0., 0., 0.),
            radius: 1.,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new() -> () {
        let s = Sphere::new();
        assert_eq!(s.center, Point::new(0., 0., 0.));
        assert_eq!(s.radius, 1.);
    }
}