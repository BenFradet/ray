use crate::math::{colour::Colour, matrix::Matrix4x4, point::Point};

use super::{intersection::Intersection, material::Material, point_light::PointLight, ray::Ray, sphere::Sphere};

pub struct World {
    objects: Vec<Sphere>,
    light: PointLight,
}

impl World {
    pub fn default() -> World {
        let light = PointLight::new(Point::new(-10., 10., -10.), Colour::WHITE);
        let material = Material::new()
            .colour(Colour::new(0.8, 1., 0.6))
            .diffuse(0.7)
            .specular(0.2);
        let sphere1 = Sphere::id().material(material);
        let sphere2 = Sphere::new(Matrix4x4::scaling(0.5, 0.5, 0.5)).unwrap_or(Sphere::id());

        World {
            objects: vec![sphere1, sphere2],
            light,
        }
    }

    pub fn intersect(&self, r: &Ray) -> Vec<Intersection> {
        let mut is: Vec<Intersection> = Vec::new();
        for sphere in self.objects.as_slice() {
            let mut inners = r.intersections(sphere);
            is.append(&mut inners);
        }
        // safe if no NaN
        is.sort_by(|a, b| a.t.partial_cmp(&b.t).unwrap());
        is
    }
}

#[cfg(test)]
mod tests {
    use crate::math::vector::Vector;

    use super::*;

    #[test]
    fn intersect() -> () {
        let w = World::default();
        let ray = Ray::new(Point::new(0., 0., -5.), Vector::new(0., 0., 1.));
        let is = w.intersect(&ray);
        assert_eq!(is.len(), 4);
        assert_eq!(is[0].t, 4.);
        assert_eq!(is[1].t, 4.5);
        assert_eq!(is[2].t, 5.5);
        assert_eq!(is[3].t, 6.);
    }

    #[test]
    fn default() -> () {
        let light = PointLight::new(Point::new(-10., 10., -10.), Colour::WHITE);
        let material = Material::new()
            .colour(Colour::new(0.8, 1., 0.6))
            .diffuse(0.7)
            .specular(0.2);
        let sphere1 = Sphere::id().material(material);
        let sphere2 = Sphere::new(Matrix4x4::scaling(0.5, 0.5, 0.5)).unwrap_or(Sphere::id());
        let w = World::default();
        assert!(w.objects.contains(&sphere1));
        assert!(w.objects.contains(&sphere2));
        assert_eq!(w.light, light);
    }
}