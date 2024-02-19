#[macro_use]
extern crate ray;
enum Ip {
    V4(V4),
    V6(V6),
}
struct V4(u8, u8);
impl TryFrom<Ip> for V4 {
    type Error = Ip;
    fn try_from(other: Ip) -> Result<Self, Self::Error> {
        match other {
            Ip::V4(v) => Ok(v),
            o => Err(o),
        }
    }
}
struct V6(String);
impl TryFrom<Ip> for V6 {
    type Error = Ip;
    fn try_from(other: Ip) -> Result<Self, Self::Error> {
        match other {
            Ip::V6(v) => Ok(v),
            o => Err(o),
        }
    }
}
pub fn main() {}