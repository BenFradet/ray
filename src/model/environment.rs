use std::fmt::{Display, Formatter, Result};

use crate::base::vector::Vector;

#[derive(PartialEq, Debug, Copy, Clone)]
pub struct Environment {
    pub gravity: Vector,
    pub wind: Vector,
}

impl Display for Environment {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(
            f,
            "gravity: {}, wind: {}",
            self.gravity.to_string(),
            self.wind.to_string()
        )
    }
}
