#[macro_use]
extern crate ray;

enum_variants_as_structs! {
    enum Ip {
        V4 { a1: u8, a2: u8 },
        V6 { addr: String },
    }
}

pub fn main() {}