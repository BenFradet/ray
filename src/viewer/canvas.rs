use std::fmt::{Display, Formatter};

use crate::{
    math::colour::Colour,
    model::{camera::Camera, ray::Ray, world::World},
};

use super::drawable::Drawable;

pub const SCALE: usize = 255;
pub const PPM_MAX_LINE_LENGTH: usize = 70;

pub struct Canvas {
    pub width: usize,
    pub height: usize,
    storage: Vec<Colour>,
}

impl Canvas {
    pub fn new(width: usize, height: usize, c: Colour) -> Canvas {
        Canvas {
            width,
            height,
            storage: vec![c; width * height],
        }
    }

    pub fn black(width: usize, height: usize) -> Canvas {
        Canvas::new(width, height, Colour::BLACK)
    }

    pub fn render(&mut self, c: &Camera, w: &World) -> () {
        for y in 0..c.vsize {
            for x in 0..c.hsize {
                let ray = Ray::for_pixel(c, x, y);
                let colour = w.colour_at(&ray, 3);
                self.update(x, y, colour);
            }
        }
    }

    // no new canvas to avoid re-allocating storage
    pub fn update(&mut self, x: usize, y: usize, c: Colour) -> () {
        let idx = self.idx(x, y);
        if idx < self.storage.len() {
            self.storage[idx] = c;
        }
    }

    pub fn at(&self, x: usize, y: usize) -> Option<Colour> {
        self.storage.get(self.idx(x, y)).copied()
    }

    fn idx(&self, x: usize, y: usize) -> usize {
        self.width * y + x
    }
}

impl Drawable for Canvas {
    fn draw(&self, frame: &mut [u8]) -> () {
        for (i, pixel) in frame.chunks_exact_mut(4).enumerate() {
            let x = i % self.width;
            let y = i / self.width;
            let rgba = match self.at(x, y).map(|c| c.scale(SCALE as u8)) {
                Some(colour) => [colour.0, colour.1, colour.2, 0xff],
                None => [0xff, 0xff, 0xff, 0xff],
            };
            pixel.copy_from_slice(&rgba)
        }
    }
}

impl Display for Canvas {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let (_, data) =
            self.storage
                .iter()
                .enumerate()
                .fold((0, String::new()), |(j, mut acc), (i, c)| {
                    let (r, g, b) = (*c).scale(SCALE);
                    let s = format!("{r} {g} {b}");
                    let s_len = s.len() + 1;
                    if j + s_len > PPM_MAX_LINE_LENGTH {
                        acc.pop();
                        (0, acc + "\n" + &s + " ")
                    } else if (i + 1) % self.width == 0 {
                        (0, acc + &s + "\n")
                    } else {
                        (j + s_len, acc + &s + " ")
                    }
                });
        write!(
            f,
            "P3
{} {}
255
{}",
            self.width, self.height, data
        )
    }
}

#[cfg(test)]
mod tests {
    use std::f64::consts::FRAC_PI_2;

    use crate::math::{matrix::Matrix4x4, point::Point, round::Round, vector::Vector};

    use super::*;

    #[test]
    fn render() -> () {
        let w = World::default();
        let eye = Point::new(0., 0., -5.);
        let to = Point::ORIGIN;
        let up = Vector::new(0., 1., 0.);
        let c = Camera::new(11, 11, FRAC_PI_2)
            .transform(Matrix4x4::view_transform(eye, to, up))
            .unwrap();
        let mut canvas = Canvas::new(c.hsize, c.vsize, Colour::BLACK);
        canvas.render(&c, &w);
        let res = canvas.at(5, 5);
        assert!(res.is_some());
        let resp = res.unwrap();
        assert_eq!(resp.rounded(5), vec![0.38066, 0.47583, 0.2855])
    }

    #[test]
    fn draw() -> () {
        let c = Canvas::new(1, 1, Colour::new(1.0, 0.8, 0.6));
        let mut vec = vec![0, 0, 0, 0];
        let slice = vec.as_mut_slice();
        c.draw(slice);
        assert_eq!(slice, vec![255, 204, 153, 255].as_mut_slice());
    }

    #[test]
    fn display_split() -> () {
        let c = Canvas::new(10, 2, Colour::new(1.0, 0.8, 0.6));
        let ppm = c.to_string();
        assert_eq!(
            ppm,
            "P3
10 2
255
255 204 153 255 204 153 255 204 153 255 204 153 255 204 153
255 204 153 255 204 153 255 204 153 255 204 153 255 204 153
255 204 153 255 204 153 255 204 153 255 204 153 255 204 153
255 204 153 255 204 153 255 204 153 255 204 153 255 204 153
"
        )
    }

    #[test]
    fn display_body() -> () {
        fn res() -> Canvas {
            let mut c = Canvas::black(5, 3);
            c.update(0, 0, Colour::new(1.5, 0.0, 0.0));
            c.update(2, 1, Colour::new(0.0, 0.5, 0.0));
            c.update(4, 2, Colour::new(-0.5, 0.0, 1.0));
            c
        }
        let ppm = res().to_string();
        assert_eq!(
            ppm,
            "P3
5 3
255
255 0 0 0 0 0 0 0 0 0 0 0 0 0 0
0 0 0 0 0 0 0 128 0 0 0 0 0 0 0
0 0 0 0 0 0 0 0 0 0 0 0 0 0 255
"
        )
    }

    #[test]
    fn display_header() -> () {
        let c = Canvas::black(5, 3);
        let res = c.to_string();
        assert!(res.starts_with("P3\n5 3\n255"));
    }

    #[test]
    fn update_at() -> () {
        let mut c = Canvas::black(10, 20);
        let r = Colour::new(1.0, 0.0, 0.0);
        c.update(2, 3, r);
        let at = c.at(2, 3);
        assert_eq!(at, Some(r));
    }

    #[test]
    fn new_canvas() -> () {
        let c = Canvas::black(10, 20);
        assert_eq!(c.width, 10);
        assert_eq!(c.height, 20);
        assert!(c.storage.iter().all(|c| *c == Colour::BLACK));
    }
}
