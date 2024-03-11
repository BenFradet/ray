pub trait Round {
    fn rounded(self, d: u32) -> Vec<f64>;
}

impl<T: IntoIterator<Item = f64>> Round for T {
    fn rounded(self, d: u32) -> Vec<f64> {
        fn round(i: f64, d: u32) -> f64 {
            let p = 10u32.pow(d) as f64;
            (i * p).round() / p
        }
        self.into_iter().map(|f| round(f, d)).collect()
    }
}
