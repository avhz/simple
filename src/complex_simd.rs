use std::simd::{LaneCount, Simd, SupportedLaneCount};

use crate::Complex;

/// A complex number vector.
#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub struct ComplexSIMD<F, const LANES: usize>
where
    F: num::Float + std::simd::SimdElement,
    LaneCount<LANES>: SupportedLaneCount,
{
    /// The real part of the complex number.
    pub re: Simd<F, LANES>,
    /// The imaginary part of the complex number.
    pub im: Simd<F, LANES>,
}

/// Take a `Vec<Complex<F>>` (`&[Complex<F>]` more generally)
/// and return a `Vec<ComplexSIMD<F, LANES>>` with the same data,
/// but chunked into vectors of size `LANES`.
pub fn chunk_complex<F, const LANES: usize>(complex: &[Complex<F>]) -> Vec<ComplexSIMD<F, LANES>>
where
    F: num::Float + std::simd::SimdElement,
    LaneCount<LANES>: SupportedLaneCount,
{
    let length = complex.len();

    assert!(length % LANES == 0, "Length must be a multiple of {LANES}");

    let mut output = Vec::with_capacity(length / LANES + 1);

    let mut re: [F; LANES] = [F::zero(); LANES];
    let mut im: [F; LANES] = [F::zero(); LANES];

    complex.chunks(LANES).for_each(|chunk| {
        re = [F::zero(); LANES];
        im = [F::zero(); LANES];

        for (i, c) in chunk.iter().enumerate() {
            re[i] = c.re;
            im[i] = c.im;
        }

        output.push(ComplexSIMD::<F, LANES>::new(re, im));
    });

    output
}

impl<F, const LANES: usize> ComplexSIMD<F, LANES>
where
    F: num::Float + std::simd::SimdElement,
    LaneCount<LANES>: SupportedLaneCount,
{
    /// Create a new complex number SIMD type.
    pub fn new(re: [F; LANES], im: [F; LANES]) -> Self {
        Self {
            re: Simd::from_array(re),
            im: Simd::from_array(im),
        }
    }

    // /// Sum the values in the SIMD vector.
    // pub fn sum(&self) -> Complex<F> {
    //     let mut out = Complex {
    //         re: F::zero(),
    //         im: F::zero(),
    //     };

    //     for i in 0..LANES {
    //         out.re += self.re..extract(i);
    //         out.im += self.im.extract(i);
    //     }

    //     out
    // }
}

// Macro to implement ComplexSIMD for all supported lane counts.
macro_rules! impl_complex_simd {
    ($($lanes:literal),*) => {
        $(
            // impl<F> ComplexSIMD<F, $lanes>
            // where
            //     F: num::Float + std::simd::SimdElement,
            //     LaneCount<$lanes>: SupportedLaneCount,
            // {
            //     /// Create a new complex number vector.
            //     pub fn new(re: [F; $lanes], im: [F; $lanes]) -> Self {
            //         Self {
            //             re: Simd::from_array(re),
            //             im: Simd::from_array(im),
            //         }
            //     }
            // }

            // Implement Add for ComplexSIMD.
            impl<F> std::ops::Add for ComplexSIMD<F, $lanes>
            where
                F: num::Float + std::simd::SimdElement,
                LaneCount<$lanes>: SupportedLaneCount,
                Simd<F, $lanes>: std::ops::Add<Output = Simd<F, $lanes>>
            {
                type Output = Self;

                fn add(self, rhs: Self) -> Self::Output {
                    Self {
                        re: self.re + rhs.re,
                        im: self.im + rhs.im,
                    }
                }
            }

            // Implement Sub for ComplexSIMD.
            impl<F> std::ops::Sub for ComplexSIMD<F, $lanes>
            where
                F: num::Float + std::simd::SimdElement,
                LaneCount<$lanes>: SupportedLaneCount,
                Simd<F, $lanes>: std::ops::Sub<Output = Simd<F, $lanes>>
            {
                type Output = Self;

                fn sub(self, rhs: Self) -> Self::Output {
                    Self {
                        re: self.re - rhs.re,
                        im: self.im - rhs.im,
                    }
                }
            }

            // Implement Mul for ComplexSIMD.
            impl<F> std::ops::Mul for ComplexSIMD<F, $lanes>
            where
                F: num::Float + std::simd::SimdElement,
                LaneCount<$lanes>: SupportedLaneCount,
                Simd<F, $lanes>: std::ops::Mul<Output = Simd<F, $lanes>>
                                + std::ops::Sub<Output = Simd<F, $lanes>>
                                + std::ops::Add<Output = Simd<F, $lanes>>
            {
                type Output = Self;

                fn mul(self, rhs: Self) -> Self::Output {
                    Self {
                        re: self.re * rhs.re - self.im * rhs.im,
                        im: self.re * rhs.im + self.im * rhs.re,
                    }
                }
            }

            // Implement Div for ComplexSIMD.
            impl<F> std::ops::Div for ComplexSIMD<F, $lanes>
            where
                F: num::Float + std::simd::SimdElement,
                LaneCount<$lanes>: SupportedLaneCount,
                Simd<F, $lanes>: std::ops::Div<Output = Simd<F, $lanes>>
                                + std::ops::Mul<Output = Simd<F, $lanes>>
                                + std::ops::Sub<Output = Simd<F, $lanes>>
                                + std::ops::Add<Output = Simd<F, $lanes>>
            {
                type Output = Self;

                fn div(self, rhs: Self) -> Self::Output {
                    let denom = rhs.re * rhs.re + rhs.im * rhs.im;
                    Self {
                        re: (self.re * rhs.re + self.im * rhs.im) / denom,
                        im: (self.im * rhs.re - self.re * rhs.im) / denom,
                    }
                }
            }


        )*
    };
}

impl_complex_simd!(1, 2, 4, 8, 16, 32, 64);

#[cfg(test)]
mod tests {
    use std::time::Instant;

    use super::*;

    #[test]
    fn test_complex_simd() {
        let re = [1.0, 2.0, 3.0, 4.0];
        let im = [5.0, 6.0, 7.0, 8.0];

        let a = ComplexSIMD::<f32, 4>::new(re, im);
        let b = ComplexSIMD::<f32, 4>::new(re, im);

        assert_eq!(a, b);
        assert_eq!(
            a + b,
            ComplexSIMD::<f32, 4>::new([2.0, 4.0, 6.0, 8.0], [10.0, 12.0, 14.0, 16.0])
        );
    }

    #[test]
    fn test_time_comparison() {
        use std::time::Instant;

        const N: usize = 2_usize.pow(10);

        let complex = crate::complex_float::Complex { re: 0.0, im: 0.0 };

        let real_parts = (0..N).map(|x| x as f32).collect::<Vec<_>>();
        let imag_parts = (0..N).map(|x| x as f32).collect::<Vec<_>>();

        let simd_sum = 0;
        let scalar_sum = 0;

        // let start = Instant::now();
        // // Iterate over chunks of 4.
        // for

        // for chunk in 0..N {
        //     let a = ComplexSIMD::<f32, 4>::new(re, im);
        //     let b = ComplexSIMD::<f32, 4>::new(re, im);
        // }
        // let end = Instant::now();
        // println!("SIMD: {:?}", end - start);

        // let start = Instant::now();
        // for _ in 0..N {
        //     // let _ = a + b;
        // }
        // let end = Instant::now();
        // println!("Scalar: {:?}", end - start);
    }

    #[test]
    fn test_chunk_complex() {
        let complex = vec![
            Complex { re: 1.0, im: 5.0 },
            Complex { re: 2.0, im: 6.0 },
            Complex { re: 3.0, im: 7.0 },
            Complex { re: 4.0, im: 8.0 },
            Complex { re: 5.0, im: 9.0 },
            Complex { re: 6.0, im: 10.0 },
            Complex { re: 7.0, im: 11.0 },
            Complex { re: 8.0, im: 12.0 },
        ];

        let chunked = chunk_complex::<f32, 4>(&complex);

        assert_eq!(
            chunked,
            vec![
                ComplexSIMD::<f32, 4>::new([1.0, 2.0, 3.0, 4.0], [5.0, 6.0, 7.0, 8.0]),
                ComplexSIMD::<f32, 4>::new([5.0, 6.0, 7.0, 8.0], [9.0, 10.0, 11.0, 12.0]),
            ]
        );

        println!("{:?}", chunked);

        // Time how long it takes to sum the chunks for normal Complex
        // and ComplexSIMD.

        let start = Instant::now();
        let mut sum = Complex { re: 0.0, im: 0.0 };
        for c in &complex {
            sum = sum + *c;
        }
        let end = Instant::now();
        println!("Complex sum: {:?}", sum);
        println!("Complex time: {:?}", end - start);

        let start = Instant::now();
        let mut sum = ComplexSIMD::<f32, 4>::new([0.0; 4], [0.0; 4]);
        for c in &chunked {
            sum = sum + *c;
        }
        let end = Instant::now();
        println!("ComplexSIMD sum: {:?}", sum);
        println!("ComplexSIMD time: {:?}", end - start);
    }
}
