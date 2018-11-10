use std::f64::{INFINITY, NEG_INFINITY};

pub trait Extend<T> {
    fn extend(vec: &[T]) -> [T; 2];
}

impl Extend<f64> for f64 {
    /// Computes the min and max of an array
    fn extend(vec: &[f64]) -> [f64; 2] {
        let mut min = INFINITY;
        let mut max = NEG_INFINITY;
        for &x in vec.iter() {
            min = x.min(min);
            max = x.max(max);
        }

        [min, max]
    }
}


// Stolen from https://crates.io/crates/itertools
#[macro_export]
macro_rules! izip {
    // @closure creates a tuple-flattening closure for .map() call. usage:
    // @closure partial_pattern => partial_tuple , rest , of , iterators
    // eg. izip!( @closure ((a, b), c) => (a, b, c) , dd , ee )
    ( @closure $p:pat => $tup:expr ) => {
        |$p| $tup
    };

    // The "b" identifier is a different identifier on each recursion level thanks to hygiene.
    ( @closure $p:pat => ( $($tup:tt)* ) , $_iter:expr $( , $tail:expr )* ) => {
        izip!(@closure ($p, b) => ( $($tup)*, b ) $( , $tail )*)
    };

    ( $first:expr $( , $rest:expr )* $(,)* ) => {
        std::iter::IntoIterator::into_iter($first)
            $(
                .zip($rest)
            )*
            .map(
                izip!(@closure a => (a) $( , $rest )*)
            )
    };
}
