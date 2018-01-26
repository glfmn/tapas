//! # Generators
//!
//! Quasi-random number generators for several different commonly used quasi-random number
//! sequences:
//!
//! - [`Halton`]
//!
//! [`Halton`]: halton/struct.Halton.html

// Ensure implementation is equal to known sequence within machine precision
#[cfg(test)]
macro_rules! test_known {
    (@as_items $($i:item)*) => ($($i)*);
    {
        $(
            $(#[$m:meta])*
            fn $fn_name:ident($($init:tt)*) {
                $($seq:tt)*
            }
        )*
    } => (
        test_known! {
            @as_items
            $(
                #[test]
                $(#[$m])*
                fn $fn_name() {
                    use rand::Rng;
                    use std::f64::EPSILON as EPS;
                    let mut sampler = $($init)*;

                    let seq = $($seq)*;

                    for s in seq.iter() {
                        let sampled: f64 = sampler.gen();
                        abs_err_eq!(s == sampled ~ EPS, "sampled value {} != {}",sampled,s);
                    }
                }
            )*
        }
    )
}

pub mod halton;

pub use self::halton::Halton;

use rand::Rng;

/// Interleave different [`Rng`]s
///
/// Enables, for example, mixing of halton generators of different bases.
///
/// ```
/// # use tapas::rng::Halton;
/// # use tapas::rng::Interleave;
/// let mut gen = Interleave::new(&[Halton::new(1, 13), Halton::new(1, 17)]);
///
/// assert!(gen.next_f64() == 1./13. && gen.next_f64() == 1./17.);
/// ```
///
/// [`Rng`]: /rand.html
#[derive(Clone)]
pub struct Interleave<R: Rng> {
    generators: Vec<R>,
    current: usize,
}

macro_rules! interleave_next {
    ($func:ident, $type:ident) => {
        fn $func(&mut self) -> $type {
            let next = self.generators[self.current].$func();

            self.current = (self.current + 1) % self.generators.len();

            next
        }
    };
    (pub $func:ident, $type:ident) => {
        pub fn $func(&mut self) -> $type {
            let next = self.generators[self.current].$func();

            self.current = (self.current + 1) % self.generators.len();

            next
        }
    };
}

impl<R: Rng> Interleave<R> {
    /// Create a new interleaved generator from a slice of generators
    pub fn new(generators: &[R]) -> Interleave<R>
        where R: Clone {

        debug_assert!(generators.len() > 0, "{} generators provided", generators.len());

        Interleave {
            generators: generators.to_vec(),
            current: 0,
        }
    }

    /// Grab next u32 value from the current underlying generator
    interleave_next!(pub next_u32, u32);

    /// Grab next u64 value from the current underlying generator
    interleave_next!(pub next_u64, u64);

    /// Grab next f32 value from the current underlying generator
    interleave_next!(pub next_f32, f32);

    /// Grab next f64 value from the current underlying generator
    interleave_next!(pub next_f64, f64);
}

impl<R: Rng> Rng for Interleave<R> {
    interleave_next!(next_u32, u32);
    interleave_next!(next_u64, u64);
    interleave_next!(next_f32, f32);
    interleave_next!(next_f64, f64);
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    test_known! {
        // Ensure that interleaved tests wrap by interleaving known halton 2 and 3 sequences
        fn interleave_wrap(Interleave::new(&[Halton::new(1,2), Halton::new(1,3)])) {
            [// base 2 | base 3
                1./2.,   1./3.,
                1./4.,   2./3.,
                3./4.,   1./9.,
                1./8.,   4./9.,
                5./8.,   7./9.,
                3./8.,   2./9.,
                7./8.,   5./9.,
                1./16.,  8./9.,
                9./16.,  1./27.
            ]
        }
    }
}
