use std::f64::{INFINITY, NEG_INFINITY};

/// Computes the min and max of an array
pub fn extend(vec: &[f64]) -> [f64; 2] {
    let mut min = INFINITY;
    let mut max = NEG_INFINITY;
    for &x in vec.iter() {
        min = x.min(min);
        max = x.max(max);
    }

    [min, max]
}

fn interpolate(
    data: &[f64], 
    domain: &[f64; 2], 
    range: &[f64; 2], 
    clamp: bool,
    round: bool,
    interpolator: impl Fn(f64) -> f64
) -> Vec<f64> 
{
    data.iter()
        .map(|&x| {
            let mut val = ( interpolator(x) - interpolator(domain[0]) ) 
                * (range[1] - range[0])
                / (interpolator(domain[1]) - interpolator(domain[0]));
            val = val + range[0];
            let result = if clamp { range[1].min(val).max(range[0]) } else { val };
            if round { result.round() } else { result }
        })
        .collect()
}

pub mod linear_scale;
pub use self::linear_scale::{LinearScale, LinearScaleBuilder};

pub mod pow_scale;
pub use self::pow_scale::{PowScale, PowScaleBuilder};


pub mod ordinal_scale;
pub use self::ordinal_scale::{OrdinalScale, OrdinalScaleBuilder};