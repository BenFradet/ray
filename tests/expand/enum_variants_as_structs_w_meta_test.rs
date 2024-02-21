#[macro_use]
extern crate ray;

enum_variants_as_structs! {
    #[derive(Debug)]
    enum Ip {
        #[derive(Debug)]
        V4(u8, u8),
        #[derive(Debug)]
        V6(String),
    }
}

pub fn main() {}