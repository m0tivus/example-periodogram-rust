use nalgebra::{DVector, DMatrix};
use std::f32::consts::PI as PI;

pub fn gaussian_kernel_fn(kernel_size: f32) -> impl Fn(f32, f32) -> f32{
    move |x: f32, y: f32| (-0.5*((x-y)/kernel_size).powi(2)).exp()
}

pub fn periodic_kernel_fn(kernel_size: f32, frequency: f32) -> impl Fn(f32, f32) -> f32 {
    move |x: f32, y: f32| (-2.0*(((x-y)*PI*frequency).sin()/kernel_size).powi(2)).exp()
}

pub fn lower_triangular_gram_matrix(x: &DVector<f32>, kernel_fn: impl Fn(f32, f32) -> f32) -> DMatrix<f32> {
    let n: usize = x.len();
    let compute_kernel = |i:usize, j:usize| -> f32{
        let mut g = 1.0_f32;
        if i>j {
            g = kernel_fn(x[i], x[j]);
        }
        g
    };
    return DMatrix::<f32>::from_fn(n, n, compute_kernel);
}


