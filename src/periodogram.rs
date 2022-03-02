use nalgebra::{DVector};

pub mod kernels;
pub mod information_theory;

pub fn renyi_periodogram(time: &DVector<f64>, magn: &DVector<f64>, freq_start: f64, freq_end: f64, freq_step: f64, alpha: f64){
    let n = time.len();
    let nfreq = ((freq_end - freq_start)/freq_step) as u32;
    let gram_magn = kernels::lower_triangular_gram_matrix(magn, kernels::gaussian_kernel_fn(1.0));
    let entropy_magn = information_theory::renyi_gram_entropy(gram_magn.clone(), n as f64, alpha);
    let compute_single_frequency = | freq_idx: u32 | -> f64 {
        let freq = freq_start + freq_step*(freq_idx as f64);
        let phase = DVector::<f64>::from_fn(n, |i, _j| (time[i]%(1.0/freq))*freq);
        let gram_phase = kernels::lower_triangular_gram_matrix(&phase, kernels::periodic_kernel_fn(1.0, 1.0));
        let entropy_phase = information_theory::renyi_gram_entropy(gram_phase.clone(), n as f64, alpha);
        let joint_entropy = information_theory::renyi_gram_joint_entropy(gram_phase, gram_magn.clone(), n as f64, alpha);
        entropy_magn + entropy_phase - joint_entropy
    };   

    let periodogram = (0..nfreq).map(compute_single_frequency).collect::<Vec<f64>>();
    println!("{:?}", periodogram);
}

/*pub(crate) fn renyi_periodogram_json(freq_start: f64, freq_end: f64, freq_step: f64, alpha: f64){
    
}*/
