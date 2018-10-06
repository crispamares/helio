
use scale::interpolate;

#[derive(Debug, Builder, Default, PartialEq)]
#[builder(setter(into))]
pub struct PowScale {
    pub domain:[f64; 2],
    pub range: [f64; 2],
    #[builder(default)]
    pub clamp: bool,
    #[builder(default)]
    pub round: bool,
    #[builder(default = "2")]
    pub exponent: i32,
}

impl PowScale {

    pub fn call(&self, data: &[f64]) -> Vec<f64> {
        interpolate(data, &self.domain, &self.range, self.clamp, self.round, |x| {x.powi(self.exponent)})
    }

    pub fn invert(&self, data: &[f64]) -> Vec<f64> {
        interpolate(data, &self.range, &self.domain, self.clamp, self.round, |x| {x.powf(1.0/ self.exponent as f64)})
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn builder_works() {
        let scale: PowScale = PowScaleBuilder::default()
            .domain([1.0, 2.0])
            .range([10.0, 20.0])
            .clamp(false)
            .build().unwrap();
        assert_eq!(scale, PowScale{range: [10.0, 20.0], domain: [1.0, 2.0], clamp: false, round: false, exponent: 2});
    }

    #[test]
    fn call_works() {
        let mut scale: PowScale = PowScaleBuilder::default()
            .domain([0.0, 1.0])
            .range([0.0, 100.0])
            .build().unwrap();

        assert_eq!( scale.call(&[1.0, 2.0, 3.0, 4.0, 5.0]), [100.0, 400.0, 900.0, 1600.0, 2500.0] );

        scale.domain = [1.0, 2.0];
        scale.range = [1.0, 2.0];

        assert_eq!( scale.call(&[0.5,1.0,2.0,4.0,5.0]), [ 0.75, 1.0, 2.0, 6.0, 9.0 ]);
    }

    #[test]
    fn invert_works() {
        let mut scale: PowScale = PowScaleBuilder::default()
            .domain([0.0, 1.0])
            .range([0.0, 100.0])
            .build().unwrap();

        assert_eq!( scale.invert(&[100.0, 400.0, 900.0, 1600.0, 2500.0]), [1.0, 2.0, 3.0, 4.0, 5.0]);

        scale.domain = [1.0, 2.0];
        scale.range = [1.0, 2.0];

        assert_eq!( scale.invert(&[ 0.75, 1.0, 2.0, 6.0, 9.0 ]), [0.5,1.0,2.0,4.0,5.0]);
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
            .domain([1.0, 2.0])
            .range([10.0, 20.0])
            .round(true)
            .build().unwrap();

        assert_eq!( scale.call(&[0.1, 1.1, 2.1, 3.1]), [1.0, 11.0, 21.0, 31.0] );
        assert_eq!( scale.invert(&[0.1, 10.1, 20.1, 30.1]), &[0.0, 1.0, 2.0, 3.0] );
    }
}