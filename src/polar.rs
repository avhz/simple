/// Polar form of a complex number.
pub struct Polar<F>
where
    F: num::Float,
{
    /// The radius of the complex number.
    pub r: F,
    /// The angle of the complex number from the positive real axis.
    pub theta: F,
}
