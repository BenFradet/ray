struct Vector {
    x: f32,
    y: f32,
    z: f32,
    w: f32,
}

impl Vector {
    pub fn new(x: f32, y: f32, z: f32) -> Vector {
        Vector {
            x,
            y,
            z,
            w: 0.0,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() -> () {
        let v = Vector::new(4.3, -4.2, 3.1);
        assert_eq!(v.x, 4.3);
        assert_eq!(v.y, -4.2);
        assert_eq!(v.z, 3.1);
        assert_eq!(v.w, 0.0);
    }
}