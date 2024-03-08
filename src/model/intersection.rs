use super::sphere::Sphere;

#[derive(PartialEq, Debug, Copy, Clone)]
pub struct Intersection {
    t: f64,
    object: Sphere,
}

impl Intersection {
    // the intersection takes ownership of the object
    // might need to revisit later
    pub fn new(t: f64, sphere: Sphere) -> Self {
        Self { t, object: sphere, }
    }
}