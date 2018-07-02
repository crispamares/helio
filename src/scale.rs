use std::ops::{Sub, Mul, Div};
use std::iter::{FromIterator};
use std::convert::{From};

#[derive(Debug, Builder, Default, PartialEq)]
pub struct Scale<D, R> {
    pub domain: Vec<D>,
    pub range: Vec<R>
}

impl<D, R> Scale<D, R> where 
    D: Sub<Output=D> + Mul<Output=D> + Div<Output=D> + Copy,
    R: Sub<Output=R> + Mul<Output=R> + Div<Output=R> + Copy,
    Vec<R>: FromIterator<D>,
    D: From<R>
{
    pub fn call(&self, data: &[D]) -> Vec<R> {
        data.iter()
            .map(|&x| x 
                * (self.range[1] - self.range[0]).into()
                / (self.domain[1] - self.domain[0]) )
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
            .build().unwrap();

        assert_eq!(scale, Scale{range: vec![10.0, 20.0], domain: vec![1.0, 2.0]});
    }

    #[test]
    fn call_works() {
        let scale: Scale<f32, f32> = ScaleBuilder::default()
            .domain(vec![1.0, 2.0])
            .range(vec![10.0, 20.0])
            .build().unwrap();

        assert_eq!( scale.call(&[1.0, 2.0, 3.0, 4.0, 5.0]), vec![10.0, 20.0, 30.0, 40.0, 50.0] );
    }
}