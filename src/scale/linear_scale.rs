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


#[derive(Debug, Builder, Default, PartialEq)]
#[builder(setter(into))]
pub struct LinearScale {
    pub domain:[f64; 2],
    pub range: [f64; 2],
    #[builder(default)]
    pub clap: bool
}

impl LinearScale {
    pub fn call(&self, data: &[f64]) -> Vec<f64> {
        data.iter()
            .map(|&x| {
                let mut val = ( x - self.domain[0] ) 
                    * (self.range[1] - self.range[0])
                    / (self.domain[1] - self.domain[0]);
                val = val + self.range[0];
                if self.clap { self.range[1].min(val).max(self.range[0]) } else { val }
            })
            .collect()
    }

    pub fn invert(&self, data: &[f64]) -> Vec<f64> {
        data.iter()
            .map(|&x| { 
                let mut val = ( x - self.range[0] ) 
                    * (self.domain[1] - self.domain[0])
                    / (self.range[1] - self.range[0]);
                val = val + self.domain[0];
                if self.clap { self.domain[1].min(val).max(self.domain[0]) } else { val }
                })
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn builder_works() {
        let scale: LinearScale = LinearScaleBuilder::default()
            .domain([1.0, 2.0])
            .range([10.0, 20.0])
            .clap(false)
            .build().unwrap();
        assert_eq!(scale, LinearScale{range: [10.0, 20.0], domain: [1.0, 2.0], clap: false});
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
    fn clap_works() {
        let scale: LinearScale = LinearScaleBuilder::default()
            .domain([1.0, 2.0])
            .range([10.0, 20.0])
            .clap(true)
            .build().unwrap();

        assert_eq!( scale.call(&[0.0, 1.0, 2.0, 3.0]), [10.0, 10.0, 20.0, 20.0] );
        assert_eq!( scale.invert(&[0.0, 10.0, 20.0, 30.0]), &[1.0, 1.0, 2.0, 2.0] );
    }


    #[test]
    fn clap_mut() {
        let mut scale: LinearScale = LinearScaleBuilder::default()
            .domain([1.0, 2.0])
            .range([10.0, 20.0])
            .clap(true)
            .build().unwrap();

        scale.clap = false;
        scale.domain = [10.0, 20.0];
        
        assert_eq!( scale.clap, false);
        assert_eq!( scale.domain, [10.0, 20.0]);
    }  
}