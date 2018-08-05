use std::collections::HashMap;
use std;
use std::cmp::Eq;
use std::hash::Hash;
use std::fmt::Debug;

#[derive(Debug, Builder, Default, PartialEq)]
#[builder(setter(into), build_fn(skip))]
pub struct OrdinalScale<D, R> where 
    D: Eq + Hash + Clone + Debug,
    R: Default + Clone + Debug
{
    pub domain: Vec<D>,
    pub range: Vec<R>,
    #[builder(default)]
    pub unknown: R,
    #[builder(setter(skip))]
    map: HashMap<D, R>
}

#[allow(dead_code)]
impl<D, R> OrdinalScaleBuilder<D, R> where
    D: Eq + Hash + Clone + Debug,
    R: Default + Clone + Debug
{
    pub fn build(&self) -> Result<OrdinalScale<D,R>, String> {
        let domain = Clone::clone(self.domain.as_ref()
                .ok_or("domain must be initialized")?);
        let range = Clone::clone(self.range.as_ref()
                .ok_or("range must be initialized")?);
        let unknown = Clone::clone(self.unknown.as_ref()
                .unwrap_or(&Default::default()));
        let map: HashMap<D, R> = izip!(domain.clone(), range.clone()).collect();
        
        Ok(OrdinalScale {
            domain: domain,
            range: range,
            unknown: unknown,
            map: map
        })
    }
}

impl<D, R> OrdinalScale<D, R> where
    D: Eq + Hash + Clone + Debug,
    R: Default + Clone + Debug
{

    pub fn call(&self, data: &[D]) -> Vec<R> {
        data.iter()
            .map(|x| { self.map.get(x).unwrap_or(&self.unknown).clone() })
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn builder_works() {
        let scale: OrdinalScale<&str, &str> = OrdinalScaleBuilder::default()
            .domain(vec!["a", "b", "c", "d"])
            .range(vec!["one", "two", "three", "four"])
            .build().unwrap();
        println!("{:?}", &scale);

        let map: HashMap<&str, &str> = izip!(scale.domain.clone(), scale.range.clone()).collect();
        println!("{:?}", &map);

        assert_eq!(scale.domain, ["a", "b", "c", "d"]);
        assert_eq!(scale.map.get(&"a").unwrap(), &"one");
    }

    #[test]
    fn call_works() {
        let mut scale: OrdinalScale<&str, &str> = OrdinalScaleBuilder::default()
            .domain(vec!["a", "b", "c", "d"])
            .range(vec!["one", "two", "three", "four"])
            .build().unwrap();

        assert_eq!( scale.call(&["d", "b", "c", "a", "r"]), ["four", "two", "three", "one", ""] );

        scale.unknown = "paco";
        assert_eq!( scale.call(&["d", "b", "c", "a", "r"]), ["four", "two", "three", "one", "paco"] );

    }
}