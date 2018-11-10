fn interpolate(
    data: &[f64], 
    domain: &[f64; 2], 
    range: &[f64; 2], 
    clamp: bool,
    round: bool,
    d_ease: impl Fn(f64) -> f64,
    r_ease: impl Fn(f64) -> f64,
    inv_ease: impl Fn(f64) -> f64
) -> Vec<f64> 
{
    data.iter()
        .map(|&x| {
            let d0 = d_ease(domain[0]); 
            let d1 = d_ease(domain[1]);
            let r0 = r_ease(range[0]);
            let r1 = r_ease(range[1]);
            let unit = (d_ease(x) - d0) / (d1 - d0);   // deinterpolate  f(x) -> t ; t € [0,1]
            let mut val = inv_ease((unit * (r1 - r0)) + r0);  // reinterpolate  f(t) -> y
            val = if clamp { range[1].min(val).max(range[0]) } else { val };
            if round { val.round() } else { val }
        })
        .collect()
}

pub mod linear_scale;
pub use self::linear_scale::{LinearScale, LinearScaleBuilder};

pub mod pow_scale;
pub use self::pow_scale::{PowScale, PowScaleBuilder};

pub mod time_scale;
pub use self::time_scale::{TimeScale, TimeScaleBuilder, DateTime};

pub mod ordinal_scale;
pub use self::ordinal_scale::{OrdinalScale, OrdinalScaleBuilder};