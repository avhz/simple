use crate::polar::*;

/// A complex number implementation for floats.
#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub struct Complex<F>
where
    F: num::Float,
{
    /// The real part of the complex number.
    pub re: F,
    /// The imaginary part of the complex number.
    pub im: F,
}

impl<F> Default for Complex<F>
where
    F: num::Float,
{
    fn default() -> Self {
        Self {
            re: F::zero(),
            im: F::zero(),
        }
    }
}

impl<F> Complex<F>
where
    F: num::Float,
{
    /// Create a new complex number.
    pub fn new(re: F, im: F) -> Self {
        Self { re, im }
    }

    /// Return the complex number `i = sqrt(-1) = 0 + 1i`.
    pub fn i() -> Self {
        Self {
            re: F::zero(),
            im: F::one(),
        }
    }

    /// Return the complex number `0 + 0i` (the origin).
    pub fn origin() -> Self {
        Self {
            re: F::zero(),
            im: F::zero(),
        }
    }

    /// Convert a polar form to a complex number.
    pub fn from_polar(r: F, theta: F) -> Self {
        Self {
            re: r * theta.cos(),
            im: r * theta.sin(),
        }
    }

    /// Return the real part of the complex number.
    /// Re(z) = Re(a + bi) = a
    pub fn re(&self) -> F {
        self.re
    }

    /// Return the imaginary part of the complex number.
    /// Im(z) = Im(a + bi) = b
    pub fn im(&self) -> F {
        self.im
    }

    /// Returns the norm of the complex number.
    pub fn norm(&self) -> F {
        self.re * self.re + self.im * self.im
    }

    /// Returns the argument of the complex number.
    pub fn arg(&self) -> F {
        self.im.atan2(self.re)
    }

    /// Returns the complex conjugate of the complex number.
    /// conj(z) = conj(a + bi) = a - bi
    pub fn conj(&self) -> Self {
        Self {
            re: self.re,
            im: -self.im,
        }
    }

    /// Returns the polar form of the complex number.
    pub fn polar(&self) -> Polar<F> {
        Polar {
            r: self.norm().sqrt(),
            theta: self.arg(),
        }
    }

    /// Complex exponential function.
    pub fn exp(&self) -> Self {
        Self::from_polar(F::exp(self.re), self.im)
    }

    /// Complex logarithm function.
    pub fn ln(&self) -> Self {
        Self::new(self.norm().ln(), self.arg())
    }

    /// Complex square-root function.
    pub fn sqrt(&self) -> Self {
        let polar = self.polar();

        Self::from_polar(polar.r.sqrt(), polar.theta / F::from(2).unwrap())
    }

    /// Complex sine function.
    pub fn sin(&self) -> Self {
        Self::new(
            self.re.sin() * self.im.cosh(),
            self.re.cos() * self.im.sinh(),
        )
    }
}
