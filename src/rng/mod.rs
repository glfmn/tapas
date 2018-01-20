//! # Generators
//!
//! Quasi-random number generators for several different commonly used quasi-random number
//! sequences:
//!
//! - [`Halton`]
//!
//! [`Halton`]: halton/struct.Halton.html

pub mod halton;

pub use self::halton::Halton;
