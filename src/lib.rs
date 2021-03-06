//! This is utility crate for testing crates using rust-ndarray
//!
//! Contents
//! ----------
//! - [assertions](assert/index.html)
//! - [distributions for real/complex numbers](random/index.html)

extern crate rand;
extern crate ndarray;
extern crate ndarray_rand;
extern crate float_cmp;
extern crate num_complex;

pub mod prelude;
pub mod assert;
pub mod random;
pub mod random_init;
