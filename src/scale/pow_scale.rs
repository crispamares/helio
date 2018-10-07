
use scale::interpolate;

#[derive(Debug, Builder, Default, PartialEq)]
#[builder(setter(into))]
pub struct PowScale {
    #[builder(default = "[0.0, 1.0]")]
    pub domain:[f64; 2],
    #[builder(default = "[0.0, 1.0]")]
    pub range: [f64; 2],
    #[builder(default)]
    pub clamp: bool,
    #[builder(default)]
    pub round: bool,
    #[builder(default = "2.0")]
    pub exponent: f64,
}

fn raise(x: f64, exp: f64) -> f64 {
    if x < 0.0 {-(x.powf(exp))} else {x.powf(exp)}
}

impl PowScale {

    fn ease(&self) -> (impl Fn(f64) -> f64) { 
        let exp = self.exponent;
        move |x| raise(x, exp)
    }
    fn inv_ease(&self) -> (impl Fn(f64) -> f64) { 
        let exp = self.exponent;
        move |x| raise(x, 1.0 / exp)
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

    #[test]
    fn builder_works() {
        let scale: PowScale = PowScaleBuilder::default()
            .clamp(false)
            .build().unwrap();
        assert_eq!(scale, PowScale{range: [0.0, 1.0], domain: [0.0, 1.0], clamp: false, round: false, exponent: 2.0});
    }

    #[test]
    fn call_works() {
        let mut scale: PowScale = PowScaleBuilder::default()
            .domain([0.0, 1.0])
            .range([0.0, 100.0])
            .exponent(2.0)
            .build().unwrap();

        assert_eq!( scale.call(&[1.0, 2.0, 3.0, 4.0, 5.0]), [100.0, 400.0, 900.0, 1600.0, 2500.0] );

        scale.domain = [1.0, 3.0];
        scale.range = [1.0, 3.0];

        assert_eq!( scale.call(&[0.5,1.0,2.0,3.0,4.0]), [ 0.8125, 1.0, 1.75, 3.0, 4.75 ]);

        scale.domain = [0.0, 1.0];
        scale.range = [0.0, 1.0];
        
        assert_eq!( scale.call(&[-2.0,-1.0,0.0,1.0,2.0]), [-4.0,-1.0,0.0,1.0,4.0]);
    }

    #[test]
    fn invert_works() {
        let mut scale: PowScale = PowScaleBuilder::default()
            .domain([0.0, 1.0])
            .range([0.0, 100.0])
            .build().unwrap();

        assert_eq!( scale.invert(&[100.0, 400.0, 900.0, 1600.0, 2500.0]), [1.0, 2.0, 3.0, 4.0, 5.0]);

        scale.domain = [1.0, 3.0];
        scale.range = [1.0, 3.0];

        assert_eq!( scale.invert(&[ 0.8125, 1.0, 1.75, 3.0, 4.75 ]), [0.5,1.0,2.0,3.0,4.0]);
    }


    #[test]
    fn clamp_works() {
        let scale: PowScale = PowScaleBuilder::default()
            .domain([0.0, 1.0])
            .range([0.0, 100.0])
            .clamp(true)
            .build().unwrap();

        assert_eq!( scale.call(&[1.0, 2.0, 3.0, 4.0, 5.0]), [100.0, 100.0, 100.0, 100.0, 100.0] );
        assert_eq!( scale.invert(&[100.0, 400.0, 900.0, 1600.0, 2500.0]), [1.0, 1.0, 1.0, 1.0, 1.0]);
    }


    #[test]
    fn round_works() {
        let scale: PowScale = PowScaleBuilder::default()
            .domain([1.0, 3.0])
            .range([1.0, 3.0])
            .round(true)
            .build().unwrap();

        assert_eq!( scale.call(&[0.5,1.0,2.0,3.0,4.0]), [ 1.0, 1.0, 2.0, 3.0, 5.0 ]);
        assert_eq!( scale.invert(&[ 0.8125, 1.0, 1.75, 3.0, 4.75 ]), [1.0,1.0,2.0,3.0,4.0]);
    }
}