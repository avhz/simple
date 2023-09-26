use crate::complex_float::Complex;
use std::ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Neg, Sub, SubAssign};

// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
// Negation
// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

impl<F> Neg for Complex<F>
where
    F: num::Float,
{
    type Output = Complex<F>;

    fn neg(self) -> Complex<F> {
        Complex::new(-self.re, -self.im)
    }
}

// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
// Addition
// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

impl<F> Add for Complex<F>
where
    F: num::Float,
{
    type Output = Complex<F>;

    fn add(self, other: Complex<F>) -> Complex<F> {
        Complex::new(self.re + other.re, self.im + other.im)
    }
}

impl<F> AddAssign for Complex<F>
where
    F: num::Float,
{
    fn add_assign(&mut self, other: Complex<F>) {
        *self = Complex::new(self.re + other.re, self.im + other.im);
    }
}

// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
// Subtraction
// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

impl<F> Sub for Complex<F>
where
    F: num::Float,
{
    type Output = Complex<F>;

    fn sub(self, other: Complex<F>) -> Complex<F> {
        Complex::new(self.re - other.re, self.im - other.im)
    }
}

impl<F> SubAssign for Complex<F>
where
    F: num::Float,
{
    fn sub_assign(&mut self, other: Complex<F>) {
        *self = Complex::new(self.re - other.re, self.im - other.im);
    }
}

// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
// Multiplication
// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

impl<F> Mul for Complex<F>
where
    F: num::Float,
{
    type Output = Complex<F>;

    fn mul(self, other: Complex<F>) -> Complex<F> {
        Complex::new(
            self.re * other.re - self.im * other.im,
            self.re * other.im + self.im * other.re,
        )
    }
}

impl<F> MulAssign for Complex<F>
where
    F: num::Float,
{
    fn mul_assign(&mut self, other: Complex<F>) {
        *self = Complex::new(
            self.re * other.re - self.im * other.im,
            self.re * other.im + self.im * other.re,
        );
    }
}

// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
// Division
// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

impl<F> Div for Complex<F>
where
    F: num::Float,
{
    type Output = Complex<F>;

    fn div(self, other: Complex<F>) -> Complex<F> {
        let denom = other.re * other.re + other.im * other.im;

        Complex::new(
            (self.re * other.re + self.im * other.im) / denom,
            (self.im * other.re - self.re * other.im) / denom,
        )
    }
}

impl<F> DivAssign for Complex<F>
where
    F: num::Float,
{
    fn div_assign(&mut self, other: Complex<F>) {
        let denom = other.re * other.re + other.im * other.im;

        *self = Complex::new(
            (self.re * other.re + self.im * other.im) / denom,
            (self.im * other.re - self.re * other.im) / denom,
        );
    }
}
