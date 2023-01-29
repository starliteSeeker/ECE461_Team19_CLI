use crate::metrics::Metrics;

pub struct Github {}

impl Metrics for Github {
    fn get_correctness() -> f64 {
        1.0
    }
}
