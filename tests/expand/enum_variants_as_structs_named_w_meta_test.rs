#[macro_use]
extern crate ray;

enum_variants_as_structs! {
    #[derive(Debug)]
    enum Ip {
        #[derive(Debug)]
        V4 { a1: u8, a2: u8 },
        #[derive(Debug)]
        V6 { addr: String },
    }
}

pub fn main() {}