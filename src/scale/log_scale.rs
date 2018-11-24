use crate::scale::interpolate;
use crate::utils::InDelta;
use std::f64::consts::{E};

#[derive(Debug, Builder, Default, PartialEq)]
#[builder(setter(into))]
pub struct LogScale {
    #[builder(default = "[1.0, 10.0]")]
    pub domain:[f64; 2],
    #[builder(default = "[0.0, 1.0]")]
    pub range: [f64; 2],
    #[builder(default)]
    pub clamp: bool,
    #[builder(default)]
    pub round: bool,
    #[builder(default = "10.0")]
    pub base: f64,
}

impl LogScale {

    fn ease(&self) -> impl Fn(f64) -> f64 {
        let mut f: fn(x:f64, base:f64) -> f64 = |x, base| x.log(base);
        if self.base.in_delta(10.0) {
            f = |x, _base| x.log10();
        }
        if self.base.in_delta(E) {
            f = |x, _base| x.ln();
        }
        if self.base.in_delta(2.0) {
            f = |x, _base| x.log2()
        }
        let base = self.base;
        move |x: f64| f(x, base)
    }
    fn inv_ease(&self) -> impl Fn(f64) -> f64 { 
        let mut f: fn(x:f64, base:f64) -> f64 = |x, base| base.powf(x);
        if self.base.in_delta(E) {
            f = |x, _base| x.exp();
        }
        if self.base.in_delta(2.0) {
            f = |x, _base| x.exp2();
        }
        let base = self.base;
        move |x| f(x, base)
    }
   
    pub fn call(&self, data: &[f64]) -> Vec<f64> {
        interpolate(data, &self.domain, &self.range, self.clamp, self.round, self.ease(), |x| {x}, |x| {x})
    }

    pub fn invert(&self, data: &[f64]) -> Vec<f64> {
        interpolate(data, &self.range, &self.domain, self.clamp, self.round, |x| {x}, self.ease(), self.inv_ease())
    }
}



#[cfg(test)]
mod tests {
    use super::*;
    use std::f64::{ NAN, INFINITY };

    #[test]
    fn builder_works() {
        let scale: LogScale = LogScaleBuilder::default()
            .clamp(false)
            .build().unwrap();
        assert_eq!(scale, LogScale{range: [0.0, 1.0], domain: [1.0, 10.0], clamp: false, round: false, base: 10.0});
    }

    #[test]
    fn call_works() {
        let mut scale: LogScale = LogScaleBuilder::default()
            .domain([1.0, 10.0])
            .range([0.0, 1.0])
            .base(10.0)
            .build().unwrap();

        let res = scale.call(&[-1.0, 0.0, 0.1, 1.0, 10.0, 100.0, 1000.0]);
        let expect = [NAN, -INFINITY, -1.0, 0.0, 1.0, 2.0, 3.0];
        assert!(res[0].is_nan());
        assert_eq!(res[1..], expect[1..]);
        
        scale.domain = [1.0, 2.0];
        scale.base = 2.0;
        assert_eq!( scale.call(&[1.0,2.0,4.0,8.0,16.0,32.0]), [0.0, 1.0, 2.0, 3.0, 4.0, 5.0]);

        scale.domain = [1.0, E];
        scale.base = E;
        
        assert_eq!( scale.call(&[1.0,E,f64::exp(2.0),f64::exp(3.0),f64::exp(4.0)]), [0.0,1.0,2.0,3.0,4.0]);
        
        scale.domain = [1.0, 3.0];
        scale.base = 3.0;
        
        assert_eq!( scale.call(&[3.0_f64.powf(0.0), 3.0_f64.powf(1.0), 3.0_f64.powf(2.0), 3.0_f64.powf(3.0), 3.0_f64.powf(4.0)]), [0.0,1.0,2.0,3.0,4.0]);
    }

    #[test]
    fn invert_works() {
        let mut scale: LogScale = LogScaleBuilder::default()
            .build().unwrap();

        assert_eq!( scale.invert(&[-INFINITY,-10.0, -1.0, 0.0, 1.0, 2.0, 3.0]), [0.0, 0.0000000001, 0.1, 1.0, 10.0, 100.0, 1000.0]);

        scale.domain = [1.0, 2.0];
        scale.base = 2.0;

        assert_eq!( scale.invert(&[0.0, 1.0, 2.0, 3.0, 4.0, 5.0]), [1.0,2.0,4.0,8.0,16.0,32.0]);

        scale.domain = [1.0, E];
        scale.base = E;

        assert_eq!( scale.invert(&[0.0,1.0,2.0,3.0,4.0]), [1.0,E,f64::exp(2.0),f64::exp(3.0),f64::exp(4.0)]);

        scale.domain = [1.0, 3.0];
        scale.base = 3.0;

        assert_eq!( scale.invert(&[0.0,1.0,2.0,3.0,4.0]), [3.0_f64.powf(0.0), 3.0_f64.powf(1.0), 3.0_f64.powf(2.0), 3.0_f64.powf(3.0), 3.0_f64.powf(4.0)]);
    }


    #[test]
    fn clamp_works() {
        let mut scale: LogScale = LogScaleBuilder::default()
            .domain([1.0, 10.0])
            .range([0.0, 1.0])
            .base(10.0)
            .clamp(true)
            .build().unwrap();

        assert_eq!( scale.call(&[-1.0, 0.0, 0.1, 1.0, 5.0, 10.0, 100.0]), [0.0, 0.0, 0.0, 0.0, f64::log10(5.0), 1.0, 1.0] );

        scale.domain = [1.0, 2.0];
        scale.base = 2.0;
        assert_eq!( scale.invert(&[-1.0, 0.0, 0.5, 1.0, 2.0]), [1.0,1.0,f64::exp2(0.5),2.0,2.0]);
    }


    #[test]
    fn round_works() {
        let mut scale: LogScale = LogScaleBuilder::default()
            .round(false)
            .build().unwrap();

        assert_eq!( scale.call(&[0.1, 1.0, 2.0]), [ -1.0, 0.0, f64::log10(2.0)]);
        assert_eq!( scale.invert(&[-1.0, 0.0, 0.1, 0.2]), [0.1, 1.0, f64::powf(10.0, 0.1), f64::powf(10.0, 0.2)]);
        scale.round = true;
        assert_eq!( scale.call(&[0.1, 1.0, 2.0]), [ -1.0, 0.0, 0.0]);
        assert_eq!( scale.invert(&[-1.0, 0.0, 0.1, 0.2]), [0.0, 1.0, 1.0, 2.0]);
        // 
    }
}