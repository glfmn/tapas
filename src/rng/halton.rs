use rand::Rng;
use std::iter::Iterator;

/// Incrementally calculated Halton sequence
///
/// Calculates the halton sequence incrementally from a given base with minimal machine error.
///
/// Since the halton sequence's base is trivial to find, the halton sequence is far from
/// cryptographically secure.  Only use this for sampling applications or other non-cryptography
/// contexts.
///
/// Memory consumption for the generator is _inversely_ proportional to the base; in other words,
/// larger bases consume less memory because we require less digits to represent larger numbers.
///
/// # Examples
///
/// We can do quasi-monte-carlo sampling using the halton sequence to estimate pi from the area of
/// a circle, and compare the precision to standard random number generation.
///
/// ```
/// # extern crate rand;
/// # extern crate tapas;
/// # use tapas::rng::Halton;
/// use std::f64::consts::PI;
/// use rand::Rng;
/// use rand::distributions::{IndependentSample, Range};
///
/// # fn main() {
/// // Define a function to take two random number generators from which to sample
/// fn monte_carlo_pi<R: Rng>(points: usize, mut s1: R, mut s2: R) -> f64 {
///
///     let between = Range::new(-1f64, 1.);
///     let mut in_circle = 0;
///
///     for _ in 0..points {
///         let a = between.ind_sample(&mut s1);
///         let b = between.ind_sample(&mut s2);
///         if a*a + b*b <= 1. {
///             in_circle += 1;
///         }
///     }
///
///     4. * (in_circle as f64) / (points as f64)
/// }
///
/// // Create two halton sequence generators with prime bases 17 and 19
/// let h_est = monte_carlo_pi(10_000, Halton::new(1,17), Halton::new(1,19));
///
/// // Estimate using standard thread_rng
/// let r_est = monte_carlo_pi(1_000_000, rand::thread_rng(), rand::thread_rng());
///
/// // The error is less than the standard number generator with 100 times less points
/// assert!((h_est-PI).abs() < (r_est-PI).abs());
/// # } // close main
/// ```
///
/// We can also use Halton as an iterator, but it will never end by itself, so be sure to use the
/// `take` method to determine how many numbers you want from the sequence, if a finite number is
/// desired.
///
/// ```
/// # use tapas::rng::Halton;
/// let seq: Vec<f64> = Halton::new(1,17).take(10).collect();
/// ```
///
/// # References
/// - Kolar, M., O'Shea, S. F., Fast, Portable, and Reliable Algorithm for the Clalculation of
///   Halton Numbers
#[derive(Debug, Clone)]
pub struct Halton {
    /// Vector of remainders used to quickly calculate next halton number
    rem: Vec<f64>,
    /// Digits in base-b notation for the halton sequence for index i
    dig: Vec<u32>,
    /// Base-b used to determine which base-b notation to use
    base: u32,
    /// Latest value generated from the halton sequence
    state: f64,
}

impl Halton {
    /// Generate a new Halton sequence starting at index `i` with base `b`
    ///
    /// The first number generated is always the value at index `i`; for example:
    ///
    /// ```
    /// # extern crate rand;
    /// # extern crate tapas;
    /// # use tapas::rng::Halton;
    /// # fn main() {
    /// use rand::Rng;
    /// use std::f64::EPSILON;
    ///
    /// let mut sampler = Halton::new(1,23);
    /// let first: f64 = sampler.gen();
    /// assert!((1./23. - first).abs() < EPSILON); // Equal within machine precision
    /// # }
    /// ```
    pub fn new(i: u32, b: u32) -> Halton {
        // Pre-set size of the halton sequence to ensure we can get to at least the millionth
        // index before the vectors have to resize
        let size = 1_000_000f64.log(b as f64).ceil() as usize;
        let mut remainders: Vec<f64> = Vec::with_capacity(size);
        let mut digits: Vec<u32> = Vec::with_capacity(size);

        // Ensure we have mutable access for a safe value
        let mut i = if i < 1 {0} else {i-1};

        // Ensure a sensible base
        let b = if b < 2 {2} else {b};

        // Convert number to digits in the given base
        while i >= b {
            digits.push(i.wrapping_rem(b));
            i = i / b;
        }
        digits.push(i);

        // Calculate remainders in reverse order for each digit
        let base = b as f64;
        remainders.push(0.);
        for d in (&digits).iter().rev() {
            if remainders.len() < digits.len() {
                let last = remainders.last().cloned().unwrap();
                remainders.push((*d as f64 + last) / base)
            }
        }

        // Produce new Halton generator with the first set of digits and remainders
        Halton {
            base: b,
            rem: remainders,
            dig: digits,
            state: 0.,
        }
    }

    /// Advance the state of the halton number generator to the next in the sequence
    #[inline]
    fn advance(&mut self) {

        debug_assert!(!self.dig.is_empty(), "Empty digit vector in {:?}", self);
        debug_assert!(!self.rem.is_empty(), "Empty remainder vector in {:?}", self);

        // Efficient calculation of the next number with minimal error is performed using Kolar
        // and O'Shea's method for calculating elements of the Halton Sequence.
        if self.dig[0] == self.base-1 {
            let mut i = 0;
            // Perform carry operation
            while i < self.dig.len() && self.dig[i] == self.base-1 {
                self.dig[i] = 0;
                i += 1;
            }
            // Bounds check when updating element
            if i < self.dig.len() {
                self.dig[i] += 1;
            } else {
                self.dig.push(1);
                self.rem.push(0.); // keep number of digits and remainders the same
            }

            // Update remainders
            let len = self.rem.len();
            let b = self.base as f64;
            self.rem[len-i] = (self.dig[i] as f64 + self.rem[len-i-1]) / b;
            if i >= 2 {
                for i in len-i..len-1 {
                    self.rem[i+1] = self.rem[i] / b;
                }
            }

            // Calculate new state
            self.state = self.rem.last().unwrap() / b;
        } else {
            // Calculate new state
            self.dig[0] += 1;
            self.state = (self.dig[0] as f64 + self.rem.last().unwrap()) / self.base as f64;
        }
    }

    /// Skip a desired number of elements from the halton sequence
    ///
    /// In some applications, it's preferred to sample only the 100th element or so.
    pub fn skip(&mut self, size: usize) {
        for _ in 0..size {
            self.advance()
        }
    }

    /// Get the next value in the halton sequence as an f64 value between `0` and `1`
    #[inline]
    fn sample_f64(&mut self) -> f64 {
        self.advance();
        self.state
    }

    /// Get the next value in the halton sequence as a u64 between `0` and `u64::MAX`
    #[inline]
    fn sample_u64(&mut self) -> u64 {
        self.advance();
        (self.state * u64::max_value() as f64).floor() as u64
    }

    /// Get the next value in the halton sequence as a u64 between `0` and `u32::MAX`
    #[inline]
    fn sample_u32(&mut self) -> u32 {
        self.advance();
        (self.state * u32::max_value() as f64).floor() as u32
    }
}

impl Rng for Halton {
    #[inline]
    fn next_u32(&mut self) -> u32 {
        self.sample_u32()
    }

    #[inline]
    fn next_u64(&mut self) -> u64 {
        self.sample_u64()
    }

    #[inline]
    fn next_f32(&mut self) -> f32 {
        self.sample_f64() as f32
    }

    #[inline]
    fn next_f64(&mut self) -> f64 {
        self.sample_f64()
    }
}

impl Iterator for Halton {
    type Item = f64;

    fn next(&mut self) -> Option<Self::Item> {
        self.advance();
        Some(self.state)
    }
}

#[cfg(test)]
mod test {
    use super::*;

    // Brute force calculate element `i` in Halton base `b`
    fn brute_force(index: u32, base: u32) -> f64 {
        let mut i = if index == 0 {1} else {index};
        let base = if base < 2 {2} else {base};
        let mut f = 1f64;
        let mut r = 0f64;
        while i > 0 {
            f /= base as f64;
            r = r + f * (i.wrapping_rem(base)) as f64;
            i /= base;
        }

        r
    }

    quickcheck! {
        // Ensure implementation is equal to brute force within 2 times machine precision
        fn compare_to_brute_force(index: u32, base: u32) -> bool {
            use std::f64::EPSILON;

            let i = if index == 0 {1} else {index};
            let b = if base < 2 {2} else {base};

            let mut sampler = Halton::new(i,b);

            sampler.gen::<f64>();
            let bf = brute_force(i,b);
            // Check brute force against implementation
            (sampler.state - bf).abs() < EPSILON * 2.
        }

        // Ensure implementation is equal to brute force for a range of values to ensure that the
        // advance method actually calculates the expected values
        fn compare_to_range(base: u32, start: u32, end: u32) -> bool {
            use std::f64::EPSILON;

            // Ensure random arguments won't break brute force method and can create a valid range
            let (start, end) = if start > end {(end, start)} else {(start,end)};
            let start = if start == 0 {1} else {start};
            let base = if base < 2 {2} else {base};

            // Initialize the sampler with the given start index and base
            let mut sampler = Halton::new(start,base);

            let mut all = true;
            for i in start..end {
                sampler.gen::<f64>();
                let bf = brute_force(i,base);
                if (sampler.state - bf).abs() > EPSILON * 2. {
                    all = false;
                    println!("{:?} != {}",sampler, bf)
                }
            }

            all
        }
    }

    test_known! {
        fn compare_to_known_base_2(Halton::new(1,2)) {
            [1./2., 1./4., 3./4., 1./8., 5./8., 3./8., 7./8.,1./16., 9./16.]
        }

        fn compare_to_known_base_3(Halton::new(1,3)) {
            [1./3., 2./3., 1./9., 4./9., 7./9., 2./9., 5./9., 8./9., 1./27.]
        }
    }
}
