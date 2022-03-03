use std::fs::File;
use std::io::Read;
use std::path::Path;
//use std::error::Error;

use serde::{Deserialize};
//use serde_json::Result;
//use rand::prelude::*;

use wasm_bindgen::prelude::*;

use nalgebra::{DVector};
pub mod kernels;
pub mod information_theory;

fn renyi_periodogram(time: &DVector<f32>, magn: &DVector<f32>, freq_start: f32, freq_end: f32, freq_step: f32, alpha: f32){
    let n = time.len();
    let nfreq = ((freq_end - freq_start)/freq_step) as u32;
    let gram_magn = kernels::lower_triangular_gram_matrix(magn, kernels::gaussian_kernel_fn(1.0));
    let entropy_magn = information_theory::renyi_gram_entropy(gram_magn.clone(), n as f32, alpha);
    let compute_single_frequency = | freq_idx: u32 | -> f32 {
        let freq = freq_start + freq_step*(freq_idx as f32);
        let phase = DVector::<f32>::from_fn(n, |i, _j| (time[i]%(1.0/freq))*freq);
        let gram_phase = kernels::lower_triangular_gram_matrix(&phase, kernels::periodic_kernel_fn(1.0, 1.0));
        let entropy_phase = information_theory::renyi_gram_entropy(gram_phase.clone(), n as f32, alpha);
        let joint_entropy = information_theory::renyi_gram_joint_entropy(gram_phase, gram_magn.clone(), n as f32, alpha);
        entropy_magn + entropy_phase - joint_entropy
    };   

    let periodogram = (0..nfreq).map(compute_single_frequency).collect::<Vec<f32>>();
    println!("{:?}", periodogram);
}

#[derive(Debug, Deserialize)]
struct LightCurve {
    mjd: Vec<f32>,
    mag: Vec<f32>,
    err: Vec<f32>,
}

fn read_json_light_curve_file<P: AsRef<Path>>(path: P) -> LightCurve {
    let mut s = String::new();
    File::open(path).unwrap().read_to_string(&mut s).unwrap();
    serde_json::from_str(&s).unwrap()
}

/*
fn periodogram_random(){
    let mut rng = thread_rng();
    let mjd = DVector::<f64>::from_fn(10, |_i,_j| rng.gen::<f64>());
    let mag = DVector::<f64>::from_fn(10, |_i,_j| rng.gen::<f64>());
}
*/

#[wasm_bindgen]
pub fn periodogram_json(freq_start: f32, freq_end: f32, freq_step: f32, alpha: f32){
    let path = Path::new("example.json");
    let light_curve = read_json_light_curve_file(path);
    let mjd = DVector::<f32>::from_vec(light_curve.mjd);
    let mag = DVector::<f32>::from_vec(light_curve.mag);
    let _err = DVector::<f32>::from_vec(light_curve.err);
    renyi_periodogram(&mjd, &mag, freq_start, freq_end, freq_step, alpha)
}
