use crate::math::{colour::Colour, matrix::Matrix4x4, point::Point};

use super::{
    comp::Comp,
    intersection::{Intersection, IntersectionHit},
    material::Material,
    point_light::PointLight,
    ray::Ray,
    sphere::Sphere,
};

pub struct World {
    objects: Vec<Sphere>,
    lights: Vec<PointLight>,
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
            lights: vec![light],
        }
    }

    pub fn objects(mut self, objects: Vec<Sphere>) -> Self {
        self.objects = objects;
        self
    }

    pub fn colour_at(&self, r: &Ray) -> Colour {
        let is = self.intersect(r);
        if let Some(hit) = is.hit() {
            let c = Comp::new(hit, *r);
            self.shade_hit(&c)
        } else {
            Colour::BLACK
        }
    }

    fn intersect(&self, r: &Ray) -> Vec<Intersection> {
        let mut is: Vec<Intersection> = Vec::new();
        for sphere in self.objects.as_slice() {
            let mut inners = r.intersections(sphere);
            is.append(&mut inners);
        }
        // safe if no NaN
        is.sort_by(|a, b| a.t.partial_cmp(&b.t).unwrap());
        is
        // filtered might not be needed
        //let mut filtered = is.into_iter()
        //    .filter(|i| i.t >= 0.)
        //    .collect::<Vec<_>>();
        //filtered.sort_by(|a, b| a.t.partial_cmp(&b.t).unwrap());
        //filtered
    }

    fn shade_hit(&self, c: &Comp) -> Colour {
        self.lights.iter().fold(Colour::BLACK, |acc, light| {
            acc + c
                .intersection
                .object
                .material
                .lightning(*light, c.point, c.eye, c.normal)
        })
    }
}

#[cfg(test)]
mod tests {
    use crate::math::{round::Round, vector::Vector};

    use super::*;

    #[test]
    fn colour_at_inter_behind_ray() -> () {
        let w = World::default();

        let outer = w.objects[0];
        let new_outer_m = outer.material.ambient(1.);
        let new_outer = outer.material(new_outer_m);

        let inner = w.objects[1];
        let new_inner_m = inner.material.ambient(1.);
        let new_inner = inner.material(new_inner_m);

        let new_world = w.objects(vec![new_outer, new_inner]);

        let ray = Ray::new(Point::new(0., 0., 0.75), Vector::new(0., 0., -1.));
        let c = new_world.colour_at(&ray);
        assert_eq!(c, new_inner.material.colour);
    }

    #[test]
    fn colour_at_hit() -> () {
        let w = World::default();
        let ray = Ray::new(Point::new(0., 0., -5.), Vector::new(0., 0., 1.));
        let c = w.colour_at(&ray);
        assert_eq!(c.rounded(5), vec![0.38066, 0.47583, 0.2855]);
    }

    #[test]
    fn colour_at_miss() -> () {
        let w = World::default();
        let ray = Ray::new(Point::new(0., 0., -5.), Vector::new(0., 1., 0.));
        let c = w.colour_at(&ray);
        assert_eq!(c, Colour::BLACK);
    }

    #[test]
    fn shade_inside() -> () {
        let mut w = World::default();
        w.lights = vec![PointLight::new(Point::new(0., 0.25, 0.), Colour::WHITE)];
        let ray = Ray::new(Point::ORIGIN, Vector::new(0., 0., 1.));
        let s = w.objects[1];
        let i = Intersection::new(0.5, s);
        let c = Comp::new(i, ray);
        let res = w.shade_hit(&c);
        assert_eq!(res.rounded(5), vec![0.90498, 0.90498, 0.90498]);
    }

    #[test]
    fn shade() -> () {
        let w = World::default();
        let ray = Ray::new(Point::new(0., 0., -5.), Vector::new(0., 0., 1.));
        let s = w.objects[0];
        let i = Intersection::new(4., s);
        let c = Comp::new(i, ray);
        let res = w.shade_hit(&c);
        assert_eq!(res.rounded(5), vec![0.38066, 0.47583, 0.2855]);
    }

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
        assert_eq!(w.lights[0], light);
    }
}
