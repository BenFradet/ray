#[macro_use]
extern crate ray;

enum_variants_as_structs! {
    enum Ip {
        V4(u8, u8),
        V6(String),
    }
}

pub fn main() {}