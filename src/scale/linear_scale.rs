
use scale::interpolate;

#[derive(Debug, Builder, Default, PartialEq)]
#[builder(setter(into))]
pub struct LinearScale {
    #[builder(default = "[0.0, 1.0]")]
    pub domain:[f64; 2],
    #[builder(default = "[0.0, 1.0]")]
    pub range: [f64; 2],
    #[builder(default)]
    pub clamp: bool,
    #[builder(default)]
    pub round: bool
}

impl LinearScale {

    pub fn call(&self, data: &[f64]) -> Vec<f64> {
        interpolate(data, &self.domain, &self.range, self.clamp, self.round, |x| {x}, |x| {x}, |x| {x})
    }

    pub fn invert(&self, data: &[f64]) -> Vec<f64> {
        interpolate(data, &self.range, &self.domain, self.clamp, self.round, |x| {x}, |x| {x}, |x| {x})
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn builder_works() {
        let scale: LinearScale = LinearScaleBuilder::default()
            .clamp(false)
            .build().unwrap();
        assert_eq!(scale, LinearScale{range: [0.0, 1.0], domain: [0.0, 1.0], clamp: false, round: false});
    }

    #[test]
    fn call_works() {
        let scale: LinearScale = LinearScaleBuilder::default()
            .domain([1.0, 2.0])
            .range([0.0, 100.0])
            .build().unwrap();

        assert_eq!( scale.call(&[1.0, 2.0, 3.0, 4.0, 5.0]), [0.0, 100.0, 200.0, 300.0, 400.0] );
    }

    #[test]
    fn invert_works() {
        let scale: LinearScale = LinearScaleBuilder::default()
            .domain([1.0, 2.0])
            .range([10.0, 20.0])
            .build().unwrap();

        assert_eq!( scale.invert(&[10.0, 20.0, 30.0, 40.0, 50.0]), [1.0, 2.0, 3.0, 4.0, 5.0]);
    }


    #[test]
    fn clamp_works() {
        let scale: LinearScale = LinearScaleBuilder::default()
            .domain([1.0, 2.0])
            .range([10.0, 20.0])
            .clamp(true)
            .build().unwrap();

        assert_eq!( scale.call(&[0.0, 1.0, 2.0, 3.0]), [10.0, 10.0, 20.0, 20.0] );
        assert_eq!( scale.invert(&[0.0, 10.0, 20.0, 30.0]), &[1.0, 1.0, 2.0, 2.0] );
    }


    #[test]
    fn clamp_mut() {
        let mut scale: LinearScale = LinearScaleBuilder::default()
            .domain([1.0, 2.0])
            .range([10.0, 20.0])
            .clamp(true)
            .build().unwrap();

        scale.clamp = false;
        scale.domain = [10.0, 20.0];
        
        assert_eq!( scale.clamp, false);
        assert_eq!( scale.domain, [10.0, 20.0]);
    }  

    #[test]
    fn round_works() {
        let scale: LinearScale = LinearScaleBuilder::default()
            .domain([1.0, 2.0])
            .range([10.0, 20.0])
            .round(true)
            .build().unwrap();

        assert_eq!( scale.call(&[0.1, 1.1, 2.1, 3.1]), [1.0, 11.0, 21.0, 31.0] );
        assert_eq!( scale.invert(&[0.1, 10.1, 20.1, 30.1]), &[0.0, 1.0, 2.0, 3.0] );
    }
}