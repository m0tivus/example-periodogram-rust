use std::fs::File;
use std::io::Read;
use std::path::Path;
//use std::error::Error;

use serde::{Deserialize};
//use serde_json::Result;
//use rand::prelude::*;

use wasm_bindgen::prelude::*;

use nalgebra::{DVector};
mod kernels;
mod information_theory;
mod utils;

#[derive(Debug, Deserialize)]
struct LightCurve {
    mjd: Vec<f32>,
    mag: Vec<f32>,
    err: Vec<f32>,
}

fn renyi_periodogram(light_curve: LightCurve, freq_start: f32, freq_end: f32, freq_step: f32, alpha: f32) -> Vec<f32>{
    let mjd = DVector::<f32>::from_vec(light_curve.mjd);
    let mag = DVector::<f32>::from_vec(light_curve.mag);
    let _err = DVector::<f32>::from_vec(light_curve.err);
    let n = mjd.len();
    let nfreq = ((freq_end - freq_start)/freq_step) as u32;
    let gram_mag = kernels::lower_triangular_gram_matrix(&mag, kernels::gaussian_kernel_fn(1.0));
    let entropy_mag = information_theory::renyi_gram_entropy(gram_mag.clone(), n as f32, alpha);
    let compute_single_frequency = | freq_idx: u32 | -> f32 {
        let freq = freq_start + freq_step*(freq_idx as f32);
        let phase = DVector::<f32>::from_fn(n, |i, _j| (mjd[i]%(1.0/freq))*freq);
        let gram_phase = kernels::lower_triangular_gram_matrix(&phase, kernels::periodic_kernel_fn(1.0, 1.0));
        let entropy_phase = information_theory::renyi_gram_entropy(gram_phase.clone(), n as f32, alpha);
        let joint_entropy = information_theory::renyi_gram_joint_entropy(gram_phase, gram_mag.clone(), n as f32, alpha);
        entropy_mag + entropy_phase - joint_entropy
    };   

    (0..nfreq).map(compute_single_frequency).collect::<Vec<f32>>()
}

/*
fn periodogram_random(){
    let mut rng = thread_rng();
    let mjd = DVector::<f64>::from_fn(10, |_i,_j| rng.gen::<f64>());
    let mag = DVector::<f64>::from_fn(10, |_i,_j| rng.gen::<f64>());
}
*/

#[wasm_bindgen]
pub fn main(light_curve: &JsValue, freq_start: f32, freq_end: f32, freq_step: f32, alpha: f32) -> String {
    utils::set_panic_hook();
    let light_curve: LightCurve = light_curve.into_serde().unwrap();
    let periodogram: Vec<f32> = renyi_periodogram(light_curve, freq_start, freq_end, freq_step, alpha);
    format!("{:?}", periodogram)
    
}

fn read_json_light_curve_file<P: AsRef<Path>>(path: P) -> LightCurve {
    let mut s = String::new();
    File::open(path).unwrap().read_to_string(&mut s).unwrap();
    serde_json::from_str(&s).unwrap()
}

#[wasm_bindgen]
pub fn periodogram_json(file_name: String, freq_start: f32, freq_end: f32, freq_step: f32, alpha: f32) -> String{
    utils::set_panic_hook();
    let path = Path::new(&file_name);
    let light_curve = read_json_light_curve_file(path);
    let periodogram: Vec<f32> = renyi_periodogram(light_curve, freq_start, freq_end, freq_step, alpha);
    format!("{:?}", periodogram)
}
