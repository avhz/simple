//! A simple complex number implementation for Rust.

#![feature(portable_simd)]
#![forbid(missing_docs)]

/// Arithmetic implementations for complex numbers.
pub mod ops;
pub use ops::*;

/// Complex number implementation for floats.
pub mod complex_float;
pub use complex_float::*;

/// Complex number implementation for SIMD floats.
pub mod complex_simd;
pub use complex_simd::*;

/// Polar form implementations for complex numbers.
pub mod polar;
pub use polar::*;
