use std::iter::{FromIterator};
use std::convert::{From, Into};
use num_traits::{Float};

pub fn extend<F>(vec: &[F]) -> [F; 2] 
    where F: Float 
{
    let mut min: F = Float::infinity();
    let mut max: F = Float::neg_infinity();
    for &x in vec.iter() {
        min = x.min(min);
        max = x.max(max);
    }

    [min, max]
}


#[derive(Debug, Builder, Default, PartialEq)]
pub struct Scale<D, R> where 
    D: Float,
    R: Float
{
    pub domain:[D; 2],
    pub range: [R; 2],
    #[builder(default)]
    pub clap: bool
}

impl<D, R> Scale<D, R> where
    D: From<R> + Float,
    R: From<D> + Float,
    Vec<D>: FromIterator<R>,
    Vec<R>: FromIterator<D>
{
    pub fn call(&self, data: &[D]) -> Vec<R> {
        data.iter()
            .map(|&x| {
                let mut val = ( x - self.domain[0] ) 
                    * (self.range[1] - self.range[0]).into()
                    / (self.domain[1] - self.domain[0]);
                val = val + self.range[0].into();
                if self.clap { self.range[1].min(val.into()).max(self.range[0]) } else { val.into() }
            })
            .collect()
    }

    pub fn invert(&self, data: &[R]) -> Vec<D> {
        data.iter()
            .map(|&x| { 
                let mut val = ( x - self.range[0] ) 
                    * (self.domain[1] - self.domain[0]).into()
                    / (self.range[1] - self.range[0]);
                val = val + self.domain[0].into();
                if self.clap { self.domain[1].min(val.into()).max(self.domain[0]) } else { val.into() }
                })
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn builder_works() {
        let scale: Scale<f32, f32> = ScaleBuilder::default()
            .domain([1.0, 2.0])
            .range([10.0, 20.0])
            .clap(false)
            .build().unwrap();
        assert_eq!(scale, Scale{range: [10.0, 20.0], domain: [1.0, 2.0], clap: false});
    }

    #[test]
    fn call_works() {
        let scale: Scale<f32, f32> = ScaleBuilder::default()
            .domain([1.0, 2.0])
            .range([0.0, 100.0])
            .build().unwrap();

        assert_eq!( scale.call(&[1.0, 2.0, 3.0, 4.0, 5.0]), [0.0, 100.0, 200.0, 300.0, 400.0] );
    }

    #[test]
    fn invert_works() {
        let scale: Scale<f32, f32> = ScaleBuilder::default()
            .domain([1.0, 2.0])
            .range([10.0, 20.0])
            .build().unwrap();

        assert_eq!( scale.invert(&[10.0, 20.0, 30.0, 40.0, 50.0]), [1.0, 2.0, 3.0, 4.0, 5.0]);
    }


    #[test]
    fn clap_works() {
        let scale: Scale<f32, f32> = ScaleBuilder::default()
            .domain([1.0, 2.0])
            .range([10.0, 20.0])
            .clap(true)
            .build().unwrap();

        assert_eq!( scale.call(&[0.0, 1.0, 2.0, 3.0]), [10.0, 10.0, 20.0, 20.0] );
        assert_eq!( scale.invert(&[0.0, 10.0, 20.0, 30.0]), &[1.0, 1.0, 2.0, 2.0] );
    }


    #[test]
    fn clap_mut() {
        let mut scale: Scale<f32, f32> = ScaleBuilder::default()
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