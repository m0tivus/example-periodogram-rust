use nalgebra::{DMatrix, linalg};

pub fn renyi_gram_entropy(gram_matrix: DMatrix<f64>, n: f64, alpha: f64) -> f64 {
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

pub fn renyi_gram_joint_entropy(a: DMatrix<f64>, b: DMatrix<f64>, n: f64, alpha: f64) -> f64 {
    
    return renyi_gram_entropy(a.component_mul(&b), n, alpha);
}

