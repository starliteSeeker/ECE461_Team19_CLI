pub mod github;

pub trait Metrics {
    fn correctness(&self) -> f64;
}
