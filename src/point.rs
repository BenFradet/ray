struct Point {
    x: f32,
    y: f32,
    z: f32,
    w: f32,
}

impl Point {
    pub fn new(x: f32, y: f32, z: f32) -> Point {
        Point {
            x,
            y,
            z,
            w: 1.0,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() -> () {
        let p = Point::new(4.3, -4.2, 3.1);
        assert_eq!(p.x, 4.3);
        assert_eq!(p.y, -4.2);
        assert_eq!(p.z, 3.1);
        assert_eq!(p.w, 1.0);
    }
}