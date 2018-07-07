use std::ops::{Sub, Mul, Div, };
use std::iter::{FromIterator};
use std::convert::{From};


#[derive(Debug, Builder, Default, PartialEq)]
pub struct Scale<D, R> {
    pub domain: Vec<D>,
    pub range: Vec<R>,
    #[builder(default)]
    pub clap: bool
}

impl<D, R> Scale<D, R> where
    D: From<R> + Sub<Output=D> + Mul<Output=D> + Div<Output=D> + Copy,
    R: From<D> + Sub<Output=R> + Mul<Output=R> + Div<Output=R> + Copy,
    Vec<D>: FromIterator<R>,
    Vec<R>: FromIterator<D>
{
    pub fn call(&self, data: &[D]) -> Vec<R> {
        data.iter()
            .map(|&x| {
                let val = x 
                    * (self.range[1] - self.range[0]).into()
                    / (self.domain[1] - self.domain[0]); 
                if self.clap { self.range[1].min(val.into()).max(self.range[0]) } else { val.into() }
            })
            .collect()
    }

    pub fn invert(&self, data: &[R]) -> Vec<D> {
        data.iter()
            .map(|&x| { 
                let val = x 
                    * (self.domain[1] - self.domain[0]).into()
                    / (self.range[1] - self.range[0]);
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
            .domain(vec![1.0, 2.0])
            .range(vec![10.0, 20.0])
            .clap(false)
            .build().unwrap();
        assert_eq!(scale, Scale{range: vec![10.0, 20.0], domain: vec![1.0, 2.0], clap: false});
    }

    #[test]
    fn call_works() {
        let scale: Scale<f32, f32> = ScaleBuilder::default()
            .domain(vec![1.0, 2.0])
            .range(vec![10.0, 20.0])
            .build().unwrap();

        assert_eq!( scale.call(&[1.0, 2.0, 3.0, 4.0, 5.0]), [10.0, 20.0, 30.0, 40.0, 50.0] );
    }

    #[test]
    fn invert_works() {
        let scale: Scale<f32, f32> = ScaleBuilder::default()
            .domain(vec![1.0, 2.0])
            .range(vec![10.0, 20.0])
            .build().unwrap();

        assert_eq!( scale.invert(&[10.0, 20.0, 30.0, 40.0, 50.0]), [1.0, 2.0, 3.0, 4.0, 5.0]);
    }
}