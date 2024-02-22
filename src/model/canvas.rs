use crate::base::colour::Colour;

struct Canvas {
    pub width: usize,
    pub height: usize,
    storage: Vec<Colour>,
}

impl Canvas {
    fn new(width: usize, height: usize) -> Canvas {
        Canvas {
            width,
            height,
            storage: vec![Colour::BLACK; width * height]
        }
    }

    // no new canvas to avoid re-allocating storage
    pub fn update(mut self, x: usize, y: usize, c: Colour) -> Option<Canvas> {
        let idx = self.idx(x, y);
        if idx < self.storage.len() {
            self.storage.insert(idx, c);
            Some(self)
        } else {
            None
        }
    }

    pub fn at(&self, x: usize, y: usize) -> Option<Colour> {
        self.storage.get(self.idx(x, y)).copied()
    }

    fn idx(&self, x: usize, y: usize) -> usize {
        self.width * y + x
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn update_at() -> () {
        let c = Canvas::new(10, 20);
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
        let c = Canvas::new(10, 20);
        assert_eq!(c.width, 10);
        assert_eq!(c.height, 20);
        assert!(c.storage.iter().all(|c| *c == Colour::BLACK));
    }
}