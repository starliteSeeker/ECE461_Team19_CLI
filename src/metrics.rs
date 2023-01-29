mod github;

pub trait Metrics {
    fn get_correctness() -> f64;
}
