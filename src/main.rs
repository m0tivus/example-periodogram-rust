use nalgebra::{DVector};
use rand::prelude::*;
mod periodogram;

fn main() {

    let mut rng = thread_rng();
    let time = DVector::<f64>::from_fn(10, |_i,_j| rng.gen::<f64>());
    let magn = DVector::<f64>::from_fn(10, |_i,_j| rng.gen::<f64>());
    periodogram::renyi_periodogram(&time, &magn, 0.0, 1.0, 0.1, 1.01);
}
