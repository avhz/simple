use crate::Complex;
use std::ops::{Add, Div, Mul, Sub};

impl<F> Add for Complex<F>
where
    F: num::Float,
{
    type Output = Complex<F>;

    fn add(self, other: Complex<F>) -> Complex<F> {
        Complex::new(self.re + other.re, self.im + other.im)
    }
}

impl<F> Sub for Complex<F>
where
    F: num::Float,
{
    type Output = Complex<F>;

    fn sub(self, other: Complex<F>) -> Complex<F> {
        Complex::new(self.re - other.re, self.im - other.im)
    }
}

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
