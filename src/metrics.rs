pub mod github;
pub trait Metrics {
    // calculate metrics of module
    // returns value in range [0, 1]

    // ramp up time for engineers to learn module
    fn ramp_up_time(&self) -> f64;

    // correctness of module
    fn correctness(&self) -> f64;

    // whether there are enough maintainers for module
    fn bus_factor(&self) -> f64;

    // responsiveness of module maintainers fixing bugs
    fn responsiveness(&self) -> f64;

    // whether module is compatible with LGPLv2.1
    fn compatibility(&self) -> f64;
}
