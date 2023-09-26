//! A simplex complex number implementation for Rust.

mod arithmetic;
mod polar;
mod trigonometric;

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub struct Complex<F>
where
    F: num::Float,
{
    pub re: F,
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
    pub fn new(re: F, im: F) -> Self {
        Self { re, im }
    }

    pub fn i() -> Self {
        Self {
            re: F::zero(),
            im: F::one(),
        }
    }

    pub fn origin() -> Self {
        Self {
            re: F::zero(),
            im: F::zero(),
        }
    }

    pub fn from_polar(r: F, theta: F) -> Self {
        Self {
            re: r * theta.cos(),
            im: r * theta.sin(),
        }
    }

    pub fn re(&self) -> F {
        self.re
    }

    pub fn im(&self) -> F {
        self.im
    }

    pub fn norm(&self) -> F {
        self.re * self.re + self.im * self.im
    }

    pub fn arg(&self) -> F {
        self.im.atan2(self.re)
    }

    pub fn conj(&self) -> Self {
        Self {
            re: self.re,
            im: -self.im,
        }
    }

    pub fn polar(&self) -> (F, F) {
        (self.norm().sqrt(), self.arg())
    }

    pub fn exp(&self) -> Self {
        Self::from_polar(F::exp(self.re), self.im)
    }

    pub fn ln(&self) -> Self {
        Self::new(self.norm().ln(), self.arg())
    }

    pub fn sqrt(&self) -> Self {
        let (r, theta) = self.polar();

        Self::from_polar(r.sqrt(), theta / F::from(2).unwrap())
    }

    pub fn sin(&self) -> Self {
        Self::new(
            self.re.sin() * self.im.cosh(),
            self.re.cos() * self.im.sinh(),
        )
    }
}
