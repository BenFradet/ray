use crate::{
    math::{colour::Colour, matrix::Matrix4x4, point::Point},
    shape::shape::Shape,
};

use super::{
    comp::Comp,
    intersection::{Intersection, IntersectionHit},
    material::Material,
    point_light::PointLight,
    ray::Ray,
};

pub struct World {
    shapes: Vec<Shape>,
    lights: Vec<PointLight>,
}

impl World {
    pub fn new(shapes: Vec<Shape>, lights: Vec<PointLight>) -> Self {
        Self { shapes, lights }
    }

    pub fn default() -> Self {
        let light = PointLight::new(Point::new(-10., 10., -10.), Colour::WHITE);
        let material = Material::default()
            .colour(Colour::new(0.8, 1., 0.6))
            .diffuse(0.7)
            .specular(0.2);
        let sphere1 = Shape::id_sphere().material(material);
        let sphere2 =
            Shape::new_sphere(Matrix4x4::scaling(0.5, 0.5, 0.5)).unwrap_or(Shape::id_sphere());

        Self {
            shapes: vec![sphere1, sphere2],
            lights: vec![light],
        }
    }

    pub fn shapes(mut self, shapes: Vec<Shape>) -> Self {
        self.shapes = shapes;
        self
    }

    pub fn add_shape(mut self, shape: Shape) -> Self {
        self.shapes.push(shape);
        self
    }

    pub fn lights(mut self, lights: Vec<PointLight>) -> Self {
        self.lights = lights;
        self
    }

    pub fn colour_at(&self, r: &Ray, remaining: u8) -> Colour {
        let is = self.intersect(r);
        if let Some(hit) = is.hit() {
            let c = Comp::new(hit, *r);
            self.shade_hit(&c, remaining)
        } else {
            Colour::BLACK
        }
    }

    fn intersect(&self, r: &Ray) -> Vec<Intersection> {
        let mut is: Vec<Intersection> = Vec::new();
        for shape in self.shapes.as_slice() {
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

    fn shade_hit(&self, c: &Comp, remaining: u8) -> Colour {
        let surface = self.lights.iter().fold(Colour::BLACK, |acc, light| {
            let is_shadowed = self.is_shadowed(c.over_point, light);
            acc + c.intersection.shape.material.lightning(
                c.intersection.shape,
                *light,
                c.over_point,
                c.eye,
                c.normal,
                is_shadowed,
            )
        });
        let reflected = self.reflected_colour(&c, remaining);
        surface + reflected
    }

    fn reflected_colour(&self, c: &Comp, remaining: u8) -> Colour {
        if remaining < 1 {
            Colour::BLACK
        } else {
            let reflective = c.intersection.shape.material.reflective;
            if reflective == 0. {
                Colour::BLACK
            } else {
                let reflect_ray = Ray::new(c.over_point, c.reflect);
                let c = self.colour_at(&reflect_ray, remaining - 1);
                c * reflective
            }
        }
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
    use std::f64::consts::SQRT_2;

    use crate::math::{round::Round, vector::Vector};

    use super::*;

    #[test]
    fn shade_hit_reflective() -> () {
        let m = Material::default().reflective(0.5);
        let s = Shape::new_plane(Matrix4x4::translation(0., -1., 0.))
            .unwrap()
            .material(m);
        let w = World::default().add_shape(s.clone());
        let s2 = SQRT_2 / 2.;
        let r = Ray::new(Point::new(0., 0., -3.), Vector::new(0., -s2, s2));
        let i = Intersection::new(&s, s2 * 2.);
        let c = Comp::new(i, r);
        let res = w.shade_hit(&c, 1);
        assert_eq!(res.rounded(5), vec![0.87676, 0.92434, 0.82917]);
    }

    #[test]
    fn reflected_colour_exhausted() -> () {
        let w = World::default();
        let r = Ray::new(Point::ORIGIN, Vector::new(0., 0., 1.));
        let i = Intersection::new(&w.shapes[0], 1.);
        let c = Comp::new(i, r);
        let res = w.reflected_colour(&c, 0);
        assert_eq!(res, Colour::BLACK);
    }

    #[test]
    fn reflected_colour_reflective_mat() -> () {
        let m = Material::default().reflective(0.5);
        let s = Shape::new_plane(Matrix4x4::translation(0., -1., 0.))
            .unwrap()
            .material(m);
        let w = World::default().add_shape(s.clone());
        let s2 = SQRT_2 / 2.;
        let r = Ray::new(Point::new(0., 0., -3.), Vector::new(0., -s2, s2));
        let i = Intersection::new(&s, s2 * 2.);
        let c = Comp::new(i, r);
        let res = w.reflected_colour(&c, 1);
        assert_eq!(res.rounded(5), vec![0.19033, 0.23792, 0.14275]);
    }

    #[test]
    fn reflected_colour_nonreflective_mat() -> () {
        let w = World::default();
        let shapes = w.shapes.clone();
        let s1 = &shapes[0];
        let s2 = shapes[1].clone();
        let new_mat = s2.material.clone().ambient(1.);
        let new_s2 = s2.material(new_mat);
        let new_w = w.shapes(vec![s1.clone(), new_s2.clone()]);
        let r = Ray::new(Point::ORIGIN, Vector::new(0., 0., 1.));
        let i = Intersection::new(&new_s2, 1.);
        let c = Comp::new(i, r);
        let res = new_w.reflected_colour(&c, 1);
        assert_eq!(res, Colour::BLACK);
    }

    #[test]
    fn shade_hit_in_shadow() -> () {
        let s1 = Shape::id_sphere();
        let s2 =
            Shape::new_sphere(Matrix4x4::translation(0., 0., 10.)).unwrap_or(Shape::id_sphere());
        let w = World::default()
            .lights(vec![PointLight::new(
                Point::new(0., 0., -10.),
                Colour::WHITE,
            )])
            .shapes(vec![s1, s2.clone()]);
        let r = Ray::new(Point::new(0., 0., 5.), Vector::new(0., 0., 1.));
        let i = Intersection::new(&s2, 4.);
        let c = Comp::new(i, r);
        let res = w.shade_hit(&c, 1);
        assert_eq!(res, Colour::new(0.1, 0.1, 0.1));
    }

    #[test]
    fn is_shadowed_shape_behind_point() -> () {
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
    fn is_shadowed_point_behind_shape() -> () {
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

        let outer_m = w.shapes[0].clone().material;
        let new_outer_m = outer_m.ambient(1.);
        let new_outer = w.shapes[0].clone().material(new_outer_m);

        let inner_m = w.shapes[1].clone().material;
        let new_inner_m = inner_m.ambient(1.);
        let new_inner = w.shapes[1].clone().material(new_inner_m.clone());

        let new_world = w.shapes(vec![new_outer, new_inner]);

        let ray = Ray::new(Point::new(0., 0., 0.75), Vector::new(0., 0., -1.));
        let c = new_world.colour_at(&ray, 1);
        assert_eq!(c, new_inner_m.colour);
    }

    #[test]
    fn colour_at_hit() -> () {
        let w = World::default();
        let ray = Ray::new(Point::new(0., 0., -5.), Vector::new(0., 0., 1.));
        let c = w.colour_at(&ray, 1);
        assert_eq!(c.rounded(5), vec![0.38066, 0.47583, 0.2855]);
    }

    #[test]
    fn colour_at_miss() -> () {
        let w = World::default();
        let ray = Ray::new(Point::new(0., 0., -5.), Vector::new(0., 1., 0.));
        let c = w.colour_at(&ray, 1);
        assert_eq!(c, Colour::BLACK);
    }

    #[test]
    fn shade_inside() -> () {
        let mut w = World::default();
        w.lights = vec![PointLight::new(Point::new(0., 0.25, 0.), Colour::WHITE)];
        let ray = Ray::new(Point::ORIGIN, Vector::new(0., 0., 1.));
        let s = &w.shapes[1];
        let i = Intersection::new(s, 0.5);
        let c = Comp::new(i, ray);
        let res = w.shade_hit(&c, 1);
        assert_eq!(res.rounded(5), vec![0.90498, 0.90498, 0.90498]);
    }

    #[test]
    fn shade() -> () {
        let w = World::default();
        let ray = Ray::new(Point::new(0., 0., -5.), Vector::new(0., 0., 1.));
        let s = &w.shapes[0];
        let i = Intersection::new(s, 4.);
        let c = Comp::new(i, ray);
        let res = w.shade_hit(&c, 1);
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
        let sphere2 =
            Shape::new_sphere(Matrix4x4::scaling(0.5, 0.5, 0.5)).unwrap_or(Shape::id_sphere());
        let w = World::default();
        assert!(w.shapes.contains(&sphere1));
        assert!(w.shapes.contains(&sphere2));
        assert_eq!(w.lights[0], light);
    }
}
