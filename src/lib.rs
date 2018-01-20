//! # Tapas: Quasi-Random Smapling
//!
//! Extension for the `rand` crate adds generators for random number sequences.

extern crate rand;
#[cfg(test)]
#[macro_use]
extern crate quickcheck;


/// Absolute error assert for floating-point valued functions
#[cfg(test)]
macro_rules! abs_err_eq {
    ($test:tt == $base:tt ~ $range:tt, $($tail:tt)*) => {
        assert!(($test - $base).abs() < $range, $($tail)*);
    };
    ($test:tt == [$lower:expr,$upper:expr], $($tail:tt)*) => {
        assert!($test <= $upper && $test >= $lower, $($tail)*);
    };
    ($test:tt == ($lower:expr,$upper:expr), $($tail:tt)*) => {
        assert!($test < $upper && $test > $lower, $($tail)*);
    };
    ($test:tt == (|$lower:expr,$upper:expr), $($tail:tt)*) => {
       assert!($test < $upper && $test >= $lower, $($tail:tt)*);
    };
    ($test:tt == ($lower:expr,|$upper:expr), $($tail:tt)*) => {
       assert!($test <= $upper && $test >= $lower, $($tail:tt)*);
    };
}


pub mod rng;
