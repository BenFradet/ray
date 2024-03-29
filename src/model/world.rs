use std::rc::Rc;

use crate::{
    math::{colour::Colour, matrix::Matrix4x4, point::Point},
    shapes::shape::Shape,
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
            let c = Comp::new(hit, *r, &is);
            self.shade_hit(&c, remaining)
        } else {
            Colour::BLACK
        }
    }

    fn intersect(&self, r: &Ray) -> Vec<Intersection> {
        let mut is: Vec<Intersection> = Vec::new();
        // TODO: rework, shouldn't need to clone shapes
        for shape in self.shapes.as_slice() {
            let rc = Rc::new(shape.clone());
            let mut inners = Intersection::intersections(Rc::clone(&rc), r);
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
            let shape = &c.intersection.shape;
            acc + shape.material.lightning(
                Rc::clone(shape),
                *light,
                c.over_point,
                c.eye,
                c.normal,
                is_shadowed,
            )
        });
        let reflected = self.reflected_colour(c, remaining);
        let refracted = self.refracted_colour(c, remaining);
        surface + reflected + refracted
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

    fn refracted_colour(&self, c: &Comp, remaining: u8) -> Colour {
        if remaining < 1 || c.indices.total_internal_reflection() {
            Colour::BLACK
        } else {
            let transparency = c.intersection.shape.material.transparency;
            let direction = c.normal * (c.indices.ratio * c.indices.cos1 - c.indices.cos2)
                - c.eye * c.indices.ratio;
            let refract_ray = Ray::new(c.under_point, direction);
            self.colour_at(&refract_ray, remaining - 1) * transparency
        }
    }

    // this rule hinders readability
    #[allow(clippy::match_like_matches_macro)]
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

impl Default for World {
    fn default() -> Self {
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
}

#[cfg(test)]
mod tests {
    use std::{f64::consts::SQRT_2, rc::Rc};

    use crate::{
        math::{round::Round, vector::Vector},
        patterns::{pattern::Pattern, pattern_kind::PatternKind},
    };

    use super::*;

    #[test]
    fn shade_hit_transparent_mat() -> () {
        let p = Shape::new_plane(Matrix4x4::translation(0., -1., 0.))
            .unwrap()
            .material(Material::default().transparency(0.5).refractive_index(1.5));
        let s = Shape::new_sphere(Matrix4x4::translation(0., -3.5, -0.5))
            .unwrap()
            .material(
                Material::default()
                    .colour(Colour::new(1., 0., 0.))
                    .ambient(0.5),
            );
        let w = World::default().add_shape(p.clone()).add_shape(s);
        let s2 = SQRT_2 / 2.;
        let r = Ray::new(Point::new(0., 0., -3.), Vector::new(0., -s2, s2));
        let i = Intersection::new(Rc::new(p), s2 * 2.);
        let c = Comp::new(i.clone(), r, &vec![i]);
        let res = w.shade_hit(&c, 5);
        assert_eq!(res.rounded(5), vec![0.93643, 0.68643, 0.68643]);
    }

    #[test]
    fn refracted_colour_refracted_ray() -> () {
        let w = World::default();
        let a = w.shapes[0].clone().material(
            Material::default()
                .ambient(1.)
                .pattern(Pattern::id(PatternKind::Test)),
        );
        let b = w.shapes[1]
            .clone()
            .material(Material::default().transparency(1.).refractive_index(1.5));
        let r = Ray::new(Point::new(0., 0., 0.1), Vector::new(0., 1., 0.));
        let new_w = w.shapes(vec![a.clone(), b.clone()]);
        let a_rc = Rc::new(a);
        let b_rc = Rc::new(b);
        let is = vec![
            Intersection::new(Rc::clone(&a_rc), -0.9899),
            Intersection::new(Rc::clone(&b_rc), -0.4899),
            Intersection::new(Rc::clone(&b_rc), 0.4899),
            Intersection::new(Rc::clone(&a_rc), 0.9899),
        ];
        let c = Comp::new(is[2].clone(), r, &is);
        let res = new_w.refracted_colour(&c, 5);
        assert_eq!(res.rounded(5), vec![0., 0.99887, 0.04722]);
    }

    #[test]
    fn refracted_colour_tir() -> () {
        let w = World::default();
        let shapes = w.shapes.clone();
        let s = shapes[0]
            .clone()
            .material(Material::default().transparency(1.).refractive_index(1.5));
        let s2 = SQRT_2 / 2.;
        let r = Ray::new(Point::new(0., 0., s2), Vector::new(0., 1., 0.));
        let new_w = w.shapes(vec![s.clone(), shapes[1].clone()]);
        let rc = Rc::new(s);
        let is = vec![
            Intersection::new(Rc::clone(&rc), -s2),
            Intersection::new(Rc::clone(&rc), s2),
        ];
        let c = Comp::new(is[1].clone(), r, &is);
        let res = new_w.refracted_colour(&c, 1);
        assert_eq!(res, Colour::BLACK);
    }

    #[test]
    fn refracted_colour_exhausted() -> () {
        let w = World::default();
        let shapes = w.shapes.clone();
        let s = shapes[0]
            .clone()
            .material(Material::default().transparency(1.).refractive_index(1.5));
        let r = Ray::new(Point::new(0., 0., -5.), Vector::new(0., 0., 1.));
        let new_w = w.shapes(vec![s.clone(), shapes[1].clone()]);
        let is = vec![Intersection::new(Rc::new(s), 4.)];
        let c = Comp::new(is[0].clone(), r, &is);
        let res = new_w.refracted_colour(&c, 0);
        assert_eq!(res, Colour::BLACK);
    }

    #[test]
    fn refracted_colour_nontransparent_mat() -> () {
        let w = World::default();
        let s = Rc::new(w.shapes[0].clone());
        let r = Ray::new(Point::new(0., 0., -5.), Vector::new(0., 0., 1.));
        let is = vec![
            Intersection::new(Rc::clone(&s), 4.),
            Intersection::new(Rc::clone(&s), 6.),
        ];
        let c = Comp::new(is[0].clone(), r, &is);
        let res = w.refracted_colour(&c, 1);
        assert_eq!(res, Colour::BLACK);
    }

    #[test]
    fn shade_hit_reflective() -> () {
        let m = Material::default().reflective(0.5);
        let s = Shape::new_plane(Matrix4x4::translation(0., -1., 0.))
            .unwrap()
            .material(m);
        let w = World::default().add_shape(s.clone());
        let s2 = SQRT_2 / 2.;
        let r = Ray::new(Point::new(0., 0., -3.), Vector::new(0., -s2, s2));
        let i = Intersection::new(Rc::new(s), s2 * 2.);
        let c = Comp::new(i.clone(), r, &vec![i]);
        let res = w.shade_hit(&c, 1);
        assert_eq!(res.rounded(5), vec![0.87676, 0.92434, 0.82917]);
    }

    #[test]
    fn reflected_colour_exhausted() -> () {
        let w = World::default();
        let r = Ray::new(Point::ORIGIN, Vector::new(0., 0., 1.));
        let i = Intersection::new(Rc::new(w.shapes[0].clone()), 1.);
        let c = Comp::new(i.clone(), r, &vec![i]);
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
        let i = Intersection::new(Rc::new(s), s2 * 2.);
        let c = Comp::new(i.clone(), r, &vec![i]);
        let res = w.reflected_colour(&c, 1);
        assert_eq!(res.rounded(5), vec![0.19033, 0.23792, 0.14275]);
    }

    #[test]
    fn reflected_colour_nonreflective_mat() -> () {
        let w = World::default();
        let s1 = w.shapes[0].clone();
        let s2 = w.shapes[1].clone();
        let new_s2 = s2.material(Material::default().ambient(1.));
        let new_w = w.shapes(vec![s1, new_s2.clone()]);
        let r = Ray::new(Point::ORIGIN, Vector::new(0., 0., 1.));
        let i = Intersection::new(Rc::new(new_s2), 1.);
        let c = Comp::new(i.clone(), r, &vec![i]);
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
        let i = Intersection::new(Rc::new(s2), 4.);
        let c = Comp::new(i.clone(), r, &vec![i]);
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
        let w = World::default().lights(vec![PointLight::new(
            Point::new(0., 0.25, 0.),
            Colour::WHITE,
        )]);
        let ray = Ray::new(Point::ORIGIN, Vector::new(0., 0., 1.));
        let s = Rc::new(w.shapes[1].clone());
        let i = Intersection::new(s, 0.5);
        let c = Comp::new(i.clone(), ray, &vec![i]);
        let res = w.shade_hit(&c, 1);
        assert_eq!(res.rounded(5), vec![0.90498, 0.90498, 0.90498]);
    }

    #[test]
    fn shade() -> () {
        let w = World::default();
        let ray = Ray::new(Point::new(0., 0., -5.), Vector::new(0., 0., 1.));
        let s = Rc::new(w.shapes[0].clone());
        let i = Intersection::new(s, 4.);
        let c = Comp::new(i.clone(), ray, &vec![i]);
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
