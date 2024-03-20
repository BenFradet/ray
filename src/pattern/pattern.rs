use super::stripe::Stripe;

#[derive(PartialEq, Debug, Copy, Clone)]
pub enum Pattern {
    S(Stripe),
}
