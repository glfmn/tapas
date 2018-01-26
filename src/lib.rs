//! # Tapas: Quasi-Random Smapling
//!
//! Extension for the `rand` crate adds generators for random number sequences.

extern crate rand;
#[cfg(test)]
#[macro_use]
extern crate quickcheck;


/// Absolute error assert for floating-point valued functions
// see tests for examples
#[cfg(test)]
macro_rules! abs_err_eq {
    ($test:tt == $base:tt ~ $range:tt, $($tail:tt)*) => {
        assert!(($test - $base).abs() < $range, $($tail)*);
    };
    ($test:tt == $base:tt ~ $range:tt) => {
        assert!(($test - $base).abs() < $range);
    };
    ($test:tt == [$lower:expr,$upper:expr], $($tail:tt)*) => {
        assert!($test <= $upper && $test >= $lower, $($tail)*);
    };
    ($test:tt == [$lower:expr,$upper:expr]) => {
        assert!($test <= $upper && $test >= $lower);
    };
    ($test:tt == ($lower:expr,$upper:expr), $($tail:tt)*) => {
        assert!($test < $upper && $test > $lower, $($tail)*);
    };
    ($test:tt == ($lower:expr,$upper:expr)) => {
        assert!($test < $upper && $test > $lower);
    };
    ($test:tt == (=$lower:expr,$upper:expr), $($tail:tt)*) => {
       assert!($test < $upper && $test >= $lower, $($tail:tt)*);
    };
    ($test:tt == (=$lower:expr,$upper:expr)) => {
       assert!($test < $upper && $test >= $lower);
    };
    ($test:tt == ($lower:expr,=$upper:expr), $($tail:tt)*) => {
       assert!($test <= $upper && $test > $lower, $($tail:tt)*);
    };
    ($test:tt == ($lower:expr,=$upper:expr)) => {
       assert!($test <= $upper && $test > $lower);
    };
}


pub mod quasi;


mod test {
    #[test]
     fn abs_err() {
         abs_err_eq!(1.05f64 == 1.0 ~ 0.1);
     }

     #[test]
     fn in_range() {
         abs_err_eq!(0f64 == [0., 1.]);
     }

     #[test]
     #[should_panic]
     fn in_range_noninclusive() {
         abs_err_eq!(0f64 == (0., 1.));
     }

     #[test]
     fn in_range_include_lower() {
         abs_err_eq!( 0f64 == (=0f64, 0.1) );
     }

     #[test]
     fn in_range_include_upper() {
         abs_err_eq!(0f64 == (-1., =0.));
     }
}
