use nalgebra::{DVector, DMatrix, linalg};

fn gaussian_kernel_fn(kernel_size: f64) -> impl Fn(f64, f64) -> f64{
    move |x: f64, y: f64| (-0.5*((x-y)/kernel_size).powi(2)).exp()
}

fn periodic_kernel_fn(kernel_size: f64, frequency: f64) -> impl Fn(f64, f64) -> f64 {
    move |x: f64, y: f64| (-2.0*(((x-y)*std::f64::consts::PI*frequency).sin()/kernel_size).powi(2)).exp()
}

fn lower_triangular_gram_matrix(x: &DVector<f64>, kernel_fn: impl Fn(f64, f64) -> f64) -> DMatrix<f64> {
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

fn renyi_gram_entropy(gram_matrix: DMatrix<f64>, n: f64, alpha: f64) -> f64 {
    let mut s = 0.0_f64;
    let eigenvalues = linalg::SymmetricEigen::new(gram_matrix/n).eigenvalues;
    for i in 0usize..eigenvalues.len() {
        let eigval = eigenvalues[i];
        if eigval > 1e-32 {
            s += eigval.powf(alpha);
        }        
    }
    return s.log2()/(1.0-alpha);    
}

fn renyi_gram_joint_entropy(a: DMatrix<f64>, b: DMatrix<f64>, n: f64, alpha: f64) -> f64 {
    
    return renyi_gram_entropy(a.component_mul(&b), n, alpha);
}

pub(crate) fn renyi_periodogram(time: &DVector<f64>, magn: &DVector<f64>, freq_start: f64, freq_end: f64, freq_step: f64, alpha: f64){
    let n = time.len();
    let nfreq = ((freq_end - freq_start)/freq_step) as u32;
    let gram_magn = lower_triangular_gram_matrix(magn, gaussian_kernel_fn(1.0));
    let entropy_magn = renyi_gram_entropy(gram_magn.clone(), n as f64, alpha);
    let compute_single_frequency = | freq_idx: u32 | -> f64 {
        let freq = freq_start + freq_step*(freq_idx as f64);
        let phase = DVector::<f64>::from_fn(n, |i, _j| (time[i]%(1.0/freq))*freq);
        let gram_phase = lower_triangular_gram_matrix(&phase, periodic_kernel_fn(1.0, 1.0));
        let entropy_phase = renyi_gram_entropy(gram_phase.clone(), n as f64, alpha);
        let joint_entropy = renyi_gram_joint_entropy(gram_phase, gram_magn.clone(), n as f64, alpha);
        entropy_magn + entropy_phase - joint_entropy
    };   

    let periodogram = (0..nfreq).map(compute_single_frequency).collect::<Vec<f64>>();
    println!("{:?}", periodogram);
}

