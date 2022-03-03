# Mutual Information periodogram in Rust

A periodogram for irregularly sampled time series based on the information theoretic concept of mutual information implemented in Rust

## Build and run

Build with

    ~/.cargo/bin/wasm-pack build --release --target no-modules --verbose

Then open index.html with a browser
  
## Notes

The mutual information implementation requires computing the eigen values of a dense Gram Matrix. This is done using the `nalgebra` Rust crate. There is an alternative version of this crate that uses lapack, see: https://www.nalgebra.org/docs/user_guide/decompositions_and_lapack/#lapack-integration. Note that for browser applications bindings to blas and lapack cannot be used, see:  https://www.nalgebra.org/docs/user_guide/wasm_and_embedded_targets
  
