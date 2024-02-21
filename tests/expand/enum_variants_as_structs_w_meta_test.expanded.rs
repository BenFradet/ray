#[macro_use]
extern crate ray;
enum Ip {
    V4(V4),
    V6(V6),
}
#[automatically_derived]
impl ::core::fmt::Debug for Ip {
    #[inline]
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        match self {
            Ip::V4(__self_0) => {
                ::core::fmt::Formatter::debug_tuple_field1_finish(f, "V4", &__self_0)
            }
            Ip::V6(__self_0) => {
                ::core::fmt::Formatter::debug_tuple_field1_finish(f, "V6", &__self_0)
            }
        }
    }
}
struct V4(u8, u8);
#[automatically_derived]
impl ::core::fmt::Debug for V4 {
    #[inline]
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        ::core::fmt::Formatter::debug_tuple_field2_finish(f, "V4", &self.0, &&self.1)
    }
}
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
#[automatically_derived]
impl ::core::fmt::Debug for V6 {
    #[inline]
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        ::core::fmt::Formatter::debug_tuple_field1_finish(f, "V6", &&self.0)
    }
}
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
