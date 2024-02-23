use crate::base::colour::Colour;

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

    // no new canvas to avoid re-allocating storage
    pub fn update(mut self, x: usize, y: usize, c: Colour) -> Option<Canvas> {
        let idx = self.idx(x, y);
        if idx < self.storage.len() {
            self.storage[idx] = c;
            Some(self)
        } else {
            None
        }
    }

    pub fn at(&self, x: usize, y: usize) -> Option<Colour> {
        self.storage.get(self.idx(x, y)).copied()
    }

    pub fn to_ppm(&self) -> String {
        let (_, data) = self.storage.iter().enumerate().fold((0, String::new()), |(j, mut acc), (i, c)| {
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
        format!("P3
{} {}
255
{}", self.width, self.height, data)
    }

    fn idx(&self, x: usize, y: usize) -> usize {
        self.width * y + x
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn to_ppm_split() -> () {
        let c = Canvas::new(10, 2, Colour::new(1.0, 0.8, 0.6));
        let ppm = c.to_ppm();
        assert_eq!(ppm, "P3
10 2
255
255 204 153 255 204 153 255 204 153 255 204 153 255 204 153
255 204 153 255 204 153 255 204 153 255 204 153 255 204 153
255 204 153 255 204 153 255 204 153 255 204 153 255 204 153
255 204 153 255 204 153 255 204 153 255 204 153 255 204 153
")
    }

    #[test]
    fn to_ppm_body() -> () {
        fn res() -> Option<Canvas> {
            let c = Canvas::black(5, 3);
            let c1 = c.update(0, 0, Colour::new(1.5, 0.0, 0.0))?;
            let c2 = c1.update(2, 1, Colour::new(0.0, 0.5, 0.0))?;
            c2.update(4, 2, Colour::new(-0.5, 0.0, 1.0))
        }
        if let Some(r) = res() {
            let ppm = r.to_ppm();
            assert_eq!(ppm, "P3
5 3
255
255 0 0 0 0 0 0 0 0 0 0 0 0 0 0
0 0 0 0 0 0 0 128 0 0 0 0 0 0 0
0 0 0 0 0 0 0 0 0 0 0 0 0 0 255
")
        } else {
            panic!("canvas couldn't be modified")
        }
    }

    #[test]
    fn to_ppm_header() -> () {
        let c = Canvas::black(5, 3);
        let res = c.to_ppm();
        assert!(res.starts_with("P3\n5 3\n255"));
    }

    #[test]
    fn update_at() -> () {
        let c = Canvas::black(10, 20);
        let r = Colour::new(1.0, 0.0, 0.0);
        if let Some(res) = c.update(2, 3, r) {
            let at = res.at(2, 3);
            assert_eq!(at, Some(r));
        } else {
            panic!("update returned none")
        }
    }

    #[test]
    fn new_canvas() -> () {
        let c = Canvas::black(10, 20);
        assert_eq!(c.width, 10);
        assert_eq!(c.height, 20);
        assert!(c.storage.iter().all(|c| *c == Colour::BLACK));
    }
}