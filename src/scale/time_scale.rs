
use scale::interpolate;
use chrono::{DateTime as ChDateTime, Utc, TimeZone};

/// Similar to chrono::DateTime<Utc>
#[derive(Clone, Debug, PartialEq)]
pub struct DateTime(pub f64);

impl DateTime {
    pub fn timestamp_millis(&self) -> i64 {
        self.0 as i64
    }
}

impl Default for DateTime {
    fn default() -> DateTime {
        Utc.timestamp(0, 0).into()
    }
}

impl From<ChDateTime<Utc>> for DateTime {
    fn from(date_time: ChDateTime<Utc>) -> Self {
        DateTime(date_time.timestamp_millis() as f64)
    }
}

impl From<f64> for DateTime {
    fn from(millis: f64) -> Self {
        DateTime(millis)
    }
}

#[derive(Debug, Builder, Default, PartialEq)]
#[builder(setter(into))]
pub struct TimeScale {
    #[builder(default = "[DateTime(0.0), DateTime(1000.0)]")]
    pub domain:[DateTime; 2],
    #[builder(default = "[0.0, 1.0]")]
    pub range: [f64; 2],
    #[builder(default)]
    pub clamp: bool,
    #[builder(default)]
    pub round: bool
}

impl TimeScale {

    pub fn call(&self, data: &[DateTime]) -> Vec<f64> {
        let domain = [self.domain[0].0, self.domain[1].0];
        let data_millis: Vec<f64> = data.iter().map(|x| x.0).collect();
        interpolate(&data_millis, &domain, &self.range, self.clamp, self.round, |x| {x}, |x| {x}, |x| {x})
    }

    pub fn invert(&self, data: &[f64]) -> Vec<DateTime> {
        let domain = [self.domain[0].0, self.domain[1].0];
        let millis = interpolate(data, &self.range, &domain, self.clamp, self.round, |x| {x}, |x| {x}, |x| {x});
        millis.iter().map(|x| DateTime(*x)).collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn builder_works() {
        let scale: TimeScale = TimeScaleBuilder::default()
            .clamp(false)
            .build().unwrap();
        assert_eq!(scale, TimeScale{
                range: [0.0, 1.0], 
                domain: [DateTime(0.0), DateTime(1000.0)], 
                clamp: false, 
                round: false
        });
    }

    #[test]
    fn call_works() {
        let scale: TimeScale = TimeScaleBuilder::default()
            .domain([DateTime(1.0), DateTime(2.0)])
            .range([0.0, 100.0])
            .build().unwrap();

        assert_eq!( 
            scale.call(&[DateTime(1.0), DateTime(2.0), DateTime(3.0), DateTime(4.0), DateTime(5.0)]),
            [0.0, 100.0, 200.0, 300.0, 400.0] 
        );
    }

    #[test]
    fn invert_works() {
        let scale: TimeScale = TimeScaleBuilder::default()
            .domain([DateTime(1.0), DateTime(2.0)])
            .range([10.0, 20.0])
            .build().unwrap();

        assert_eq!(
            scale.invert(&[10.0, 20.0, 30.0, 40.0, 50.0]),
            [DateTime(1.0), DateTime(2.0), DateTime(3.0), DateTime(4.0), DateTime(5.0)]
        );
    }


    #[test]
    fn clamp_works() {
        let scale: TimeScale = TimeScaleBuilder::default()
            .domain([DateTime(1.0), DateTime(2.0)])
            .range([10.0, 20.0])
            .clamp(true)
            .build().unwrap();

        assert_eq!( 
            scale.call(&[DateTime(0.0), DateTime(1.0), DateTime(2.0), DateTime(3.0)]), 
            [10.0, 10.0, 20.0, 20.0] 
        );
        assert_eq!( 
            scale.invert(&[0.0, 10.0, 20.0, 30.0]),
            &[DateTime(1.0), DateTime(1.0), DateTime(2.0), DateTime(2.0)] 
        );
    }

    #[test]
    fn round_works() {
        let scale: TimeScale = TimeScaleBuilder::default()
            .domain([DateTime(100.0), DateTime(200.0)])
            .range([0.0, 7.0])
            .round(true)
            .build().unwrap();

        assert_eq!(
            scale.call(&[DateTime(0.0), DateTime(10.0), DateTime(100.0), DateTime(150.0)]), 
            [-7.0, -6.0, 0.0, 4.0] 
        );
        assert_eq!(
            scale.invert(&[-7.0, -6.3, 0.0, 3.5]),
            [DateTime(0.0), DateTime(10.0), DateTime(100.0), DateTime(150.0)]
        );
    }
}
