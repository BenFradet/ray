use crate::{
    math::{colour::Colour, point::Point, vector::Vector},
    pattern::pattern::Pattern,
    shape::shape::Shape,
};

use super::point_light::PointLight;

#[derive(PartialEq, Debug, Clone)]
pub struct Material {
    pub colour: Colour,
    ambient: f64,
    diffuse: f64,
    specular: f64,
    shininess: f64,
    pattern: Option<Pattern>,
}

impl Material {
    pub fn new(c: Colour, ambient: f64, diffuse: f64, specular: f64) -> Self {
        Self {
            colour: c,
            ambient,
            diffuse,
            specular,
            shininess: 200.,
            pattern: None,
        }
    }

    pub fn default() -> Self {
        Self {
            colour: Colour::WHITE,
            ambient: 0.1,
            diffuse: 0.9,
            specular: 0.9,
            shininess: 200.,
            pattern: None,
        }
    }

    // https://en.wikipedia.org/wiki/Phong_reflection_model
    pub fn lightning(
        &self,
        shape: &Shape,
        light: PointLight,
        p: Point,
        eye: Vector,
        normal: Vector,
        in_shadow: bool,
    ) -> Colour {
        let colour = match &self.pattern {
            Some(pat) => pat.at_shape(&shape, p),
            None => self.colour,
        };

        let effective_colour = colour * light.intensity;

        let ambient = effective_colour * self.ambient;
        if in_shadow {
            ambient
        } else {
            let mut diffuse = Colour::BLACK;
            let mut specular = Colour::BLACK;

            let light_norm = (light.position - p).norm();
            let light_normal_cos = light_norm.dot(normal);

            // light and normal are on the same side
            if light_normal_cos >= 0. {
                diffuse = effective_colour * self.diffuse * light_normal_cos;

                let light_normal_reflected = (-light_norm).reflect(normal);
                let reflect_eye_cos = light_normal_reflected.dot(eye);

                // light reflects away from the eye means specular is null
                if reflect_eye_cos <= 0. {
                    specular = Colour::BLACK;
                } else {
                    let factor = reflect_eye_cos.powf(self.shininess);
                    specular = light.intensity * self.specular * factor;
                }
            }

            ambient + diffuse + specular
        }

        // specular
        //let light_reflected_v = (-light_norm).reflect(normal);
        //let reflected_eye_cos = light_reflected_v.dot(eye);
        //let specular = if reflected_eye_cos <= 0. {
        //    Colour::BLACK
        //} else {
        //    effective_colour * self.specular * reflected_eye_cos.powf(self.shininess)
        //};

        //// diffuse
        //let light_normal_cos = light_norm.dot(normal);
        //let diffuse = effective_colour * self.diffuse * light_normal_cos;
    }

    pub fn colour(mut self, c: Colour) -> Self {
        self.colour = c;
        self
    }

    pub fn ambient(mut self, a: f64) -> Self {
        self.ambient = a.abs();
        self
    }

    pub fn diffuse(mut self, d: f64) -> Self {
        self.diffuse = d.abs();
        self
    }

    pub fn specular(mut self, s: f64) -> Self {
        self.specular = s.abs();
        self
    }

    pub fn shininess(mut self, s: f64) -> Self {
        self.shininess = s.abs();
        self
    }

    pub fn pattern(mut self, p: Pattern) -> Self {
        self.pattern = Some(p);
        self
    }
}

#[cfg(test)]
mod tests {
    use std::f64::consts::SQRT_2;

    use crate::math::round::Round;

    use super::*;

    #[test]
    fn lightning_with_pattern() -> () {
        let m = Material::default()
            .ambient(1.)
            .diffuse(0.)
            .specular(0.)
            .pattern(Pattern::id_stripe(Colour::WHITE, Colour::BLACK));
        let eye = Vector::new(0., 0., -1.);
        let normal = Vector::new(0., 0., -1.);
        let light = PointLight::new(Point::new(0., 0., -10.), Colour::WHITE);
        let c1 = m.lightning(
            &Shape::id_sphere(),
            light,
            Point::new(0.9, 0., 0.),
            eye,
            normal,
            false,
        );
        assert_eq!(c1, Colour::WHITE);
        let c2 = m.lightning(
            &Shape::id_sphere(),
            light,
            Point::new(1.1, 0., 0.),
            eye,
            normal,
            false,
        );
        assert_eq!(c2, Colour::BLACK);
    }

    #[test]
    fn lighting_in_shadow() -> () {
        let m = Material::default();
        let p = Point::ORIGIN;
        let eye = Vector::new(0., 0., -1.);
        let normal = Vector::new(0., 0., -1.);
        let light = PointLight::new(Point::new(0., 0., -10.), Colour::WHITE);
        let res = m.lightning(&Shape::id_sphere(), light, p, eye, normal, true);
        assert_eq!(res, Colour::new(0.1, 0.1, 0.1));
    }

    #[test]
    fn lightning_eye_light_normal_aligned() -> () {
        let m = Material::default();
        let p = Point::ORIGIN;
        let eye = Vector::new(0., 0., -1.);
        let normal = Vector::new(0., 0., -1.);
        let light = PointLight::new(Point::new(0., 0., -10.), Colour::WHITE);
        let res = m.lightning(&Shape::id_sphere(), light, p, eye, normal, false);
        assert_eq!(res.rounded(5), vec![1.9, 1.9, 1.9]);
    }

    #[test]
    fn lightning_eye_between_light_surface() -> () {
        let m = Material::default();
        let p = Point::ORIGIN;
        let s2 = SQRT_2 / 2.;
        let eye = Vector::new(0., s2, -s2);
        let normal = Vector::new(0., 0., -1.);
        let light = PointLight::new(Point::new(0., 0., -10.), Colour::WHITE);
        let res = m.lightning(&Shape::id_sphere(), light, p, eye, normal, false);
        assert_eq!(res, Colour::WHITE);
    }

    #[test]
    fn lightning_eye_in_normal() -> () {
        let m = Material::default();
        let p = Point::ORIGIN;
        let eye = Vector::new(0., 0., -1.);
        let normal = eye;
        let light = PointLight::new(Point::new(0., 10., -10.), Colour::WHITE);
        let res = m.lightning(&Shape::id_sphere(), light, p, eye, normal, false);
        assert_eq!(res.rounded(4), vec![0.7364, 0.7364, 0.7364]);
    }

    #[test]
    fn lightning_eye_in_reflect() -> () {
        let m = Material::default();
        let p = Point::ORIGIN;
        let s2 = SQRT_2 / 2.;
        let eye = Vector::new(0., -s2, -s2);
        let normal = Vector::new(0., 0., -1.);
        let light = PointLight::new(Point::new(0., 10., -10.), Colour::WHITE);
        let res = m.lightning(&Shape::id_sphere(), light, p, eye, normal, false);
        assert_eq!(res.rounded(4), vec![1.6364, 1.6364, 1.6364]);
    }

    #[test]
    fn lightning_behind() -> () {
        let m = Material::default();
        let p = Point::ORIGIN;
        let eye = Vector::new(0., 0., -1.);
        let normal = Vector::new(0., 0., -1.);
        let light = PointLight::new(Point::new(0., 0., 10.), Colour::WHITE);
        let res = m.lightning(&Shape::id_sphere(), light, p, eye, normal, false);
        assert_eq!(res.rounded(5), vec![0.1, 0.1, 0.1]);
    }

    #[test]
    fn shininess() -> () {
        let m = Material::default();
        assert_eq!(m.shininess, 200.);
        let s = 100.;
        let new_m = m.shininess(-s);
        assert_eq!(new_m.shininess, s);
    }

    #[test]
    fn specular() -> () {
        let m = Material::default();
        assert_eq!(m.specular, 0.9);
        let s = 1.;
        let new_m = m.specular(-s);
        assert_eq!(new_m.specular, s);
    }

    #[test]
    fn diffuse() -> () {
        let m = Material::default();
        assert_eq!(m.diffuse, 0.9);
        let d = 1.;
        let new_m = m.diffuse(-d);
        assert_eq!(new_m.diffuse, d);
    }

    #[test]
    fn ambient() -> () {
        let m = Material::default();
        assert_eq!(m.ambient, 0.1);
        let a = 0.2;
        let new_m = m.ambient(-a);
        assert_eq!(new_m.ambient, a);
    }

    #[test]
    fn colour() -> () {
        let m = Material::default();
        assert_eq!(m.colour, Colour::WHITE);
        let c = Colour::BLACK;
        let new_m = m.colour(c);
        assert_eq!(new_m.colour, c);
    }

    #[test]
    fn new() -> () {
        let m = Material::default();
        assert_eq!(m.colour, Colour::WHITE);
        assert_eq!(m.ambient, 0.1);
        assert_eq!(m.diffuse, 0.9);
        assert_eq!(m.specular, 0.9);
        assert_eq!(m.shininess, 200.);
    }
}
