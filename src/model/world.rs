use crate::{math::{colour::Colour, matrix::Matrix4x4, point::Point}, shape::{shape::Shape, shape_kind::ShapeKind, sphere::Sphere}};

use super::{
    comp::Comp,
    intersection::{Intersection, IntersectionHit},
    material::Material,
    point_light::PointLight,
    ray::Ray,
};

pub struct World {
    objects: Vec<Shape>,
    lights: Vec<PointLight>,
}

impl World {
    pub fn new(objects: Vec<Shape>, lights: Vec<PointLight>) -> Self {
        Self {
            objects,
            lights,
        }
    }

    pub fn default() -> Self {
        let light = PointLight::new(Point::new(-10., 10., -10.), Colour::WHITE);
        let material = Material::default()
            .colour(Colour::new(0.8, 1., 0.6))
            .diffuse(0.7)
            .specular(0.2);
        let sphere1 = Shape::id_sphere().material(material);
        let sphere2 = Shape::new_sphere(Matrix4x4::scaling(0.5, 0.5, 0.5))
            .unwrap_or(Shape::id_sphere());

        Self {
            objects: vec![sphere1, sphere2],
            lights: vec![light],
        }
    }

    pub fn objects(mut self, objects: Vec<Shape>) -> Self {
        self.objects = objects;
        self
    }

    pub fn lights(mut self, lights: Vec<PointLight>) -> Self {
        self.lights = lights;
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
        for shape in self.objects.as_slice() {
            let mut inners = shape.intersections(&r);
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
            let is_shadowed = self.is_shadowed(c.over_point, light);
            acc + c
                .intersection
                .object
                .material
                .lightning(*light, c.over_point, c.eye, c.normal, is_shadowed)
        })
    }

    fn is_shadowed(&self, p: Point, light: &PointLight) -> bool {
        let point_to_light = light.position - p;
        let dist = point_to_light.len();
        let direction = point_to_light.norm();
        let ray = Ray::new(p, direction);
        match self.intersect(&ray).hit() {
            Some(hit) if hit.t < dist => true,
            _ => false,
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::math::{round::Round, vector::Vector};

    use super::*;

    #[test]
    fn shade_hit_in_shadow() -> () {
        let s1 = Shape::id_sphere();
        let s2 = Shape::new_sphere(Matrix4x4::translation(0., 0., 10.))
            .unwrap_or(Shape::id_sphere());
        let w = World::default()
            .lights(vec![PointLight::new(Point::new(0., 0., -10.), Colour::WHITE)])
            .objects(vec![s1, s2]);
        let r = Ray::new(Point::new(0., 0., 5.), Vector::new(0., 0., 1.));
        let i = Intersection::new(4., s2);
        let c = Comp::new(i, r);
        let res = w.shade_hit(&c);
        assert_eq!(res, Colour::new(0.1, 0.1, 0.1));
    }

    #[test]
    fn is_shadowed_object_behind_point() -> () {
        let w = World::default();
        let p = Point::new(-2., 2., 2.);
        assert!(!w.is_shadowed(p, &w.lights[0]));
    }

    #[test]
    fn is_shadowed_behind_light() -> () {
        let w = World::default();
        let p = Point::new(-20., 20., 20.);
        assert!(!w.is_shadowed(p, &w.lights[0]));
    }

    #[test]
    fn is_shadowed_point_behind_object() -> () {
        let w = World::default();
        let p = Point::new(10., -10., 10.);
        assert!(w.is_shadowed(p, &w.lights[0]));
    }

    #[test]
    fn is_shadowed_nothing_colinear() -> () {
        let w = World::default();
        let p = Point::new(0., 10., 0.);
        assert!(!w.is_shadowed(p, &w.lights[0]));
    }

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
        let material = Material::default()
            .colour(Colour::new(0.8, 1., 0.6))
            .diffuse(0.7)
            .specular(0.2);
        let sphere1 = Shape::id_sphere().material(material);
        let sphere2 = Shape::new_sphere(Matrix4x4::scaling(0.5, 0.5, 0.5))
            .unwrap_or(Shape::id_sphere());
        let w = World::default();
        assert!(w.objects.contains(&sphere1));
        assert!(w.objects.contains(&sphere2));
        assert_eq!(w.lights[0], light);
    }
}
