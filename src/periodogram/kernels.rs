use nalgebra::{DVector, DMatrix};
use std::f64::consts::PI as PI;

pub fn gaussian_kernel_fn(kernel_size: f64) -> impl Fn(f64, f64) -> f64{
    move |x: f64, y: f64| (-0.5*((x-y)/kernel_size).powi(2)).exp()
}

pub fn periodic_kernel_fn(kernel_size: f64, frequency: f64) -> impl Fn(f64, f64) -> f64 {
    move |x: f64, y: f64| (-2.0*(((x-y)*PI*frequency).sin()/kernel_size).powi(2)).exp()
}

pub fn lower_triangular_gram_matrix(x: &DVector<f64>, kernel_fn: impl Fn(f64, f64) -> f64) -> DMatrix<f64> {
    let n: usize = x.len();
    let compute_kernel = |i:usize, j:usize| -> f64{
        let mut g = 1.0_f64;
        if i>j {
            g = kernel_fn(x[i], x[j]);
        }
        g
    };
    return DMatrix::<f64>::from_fn(n, n, compute_kernel);
}


