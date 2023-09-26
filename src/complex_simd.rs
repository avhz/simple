/// A complex number vector.
#[derive(Debug, Clone, Eq, PartialEq)]
pub struct ComplexSIMD<F>
where
    F: std::simd::SimdFloat,
{
    /// The real part of the complex number.
    pub re: F,
    /// The imaginary part of the complex number.
    pub im: F,
}
