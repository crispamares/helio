#[macro_use] extern crate helio;
extern crate csv;

use std::error::Error;

use helio::core::{Color, Canvas, Circle, CircleBuilder, StyleBuilder, Scene};
use helio::scale::{LinearScale, LinearScaleBuilder, OrdinalScale, OrdinalScaleBuilder, PowScale, PowScaleBuilder, extend};
use helio::svg_backend;
use helio::color::{WHITE, STEELBLUE, RED, PERU};

fn main() -> Result<(), Box<Error>> {

    let width = 800;
    let height = 600;

    let x_column = "weight (lb)";
    let y_column = "displacement (cc)";
    let r_column = "year";
    let c_column = "cylinders";

    // Data to use
    let mut x_data: Vec<f64> = Vec::new();
    let mut y_data: Vec<f64> = Vec::new();
    let mut r_data: Vec<f64> = Vec::new();
    let mut c_data: Vec<String> = Vec::new();
    
    let mut rdr = csv::Reader::from_path("examples/data/cars.csv")?;
    println!("{:?}", rdr.headers()?);

    let x_data_idx = rdr.headers()?.iter().position(|x| x == x_column)
        .ok_or(format!("'{}' not in the dataset", x_column) )?;
    let y_data_idx = rdr.headers()?.iter().position(|x| x == y_column)
        .ok_or(format!("'{}' not in the dataset", y_column) )?;
    let r_data_idx = rdr.headers()?.iter().position(|x| x == r_column)
        .ok_or(format!("'{}' not in the dataset", r_column) )?;
    let c_data_idx = rdr.headers()?.iter().position(|x| x == c_column)
        .ok_or(format!("'{}' not in the dataset", c_column) )?;

    for result in rdr.records() {
        let record = result?;
        x_data.push(record.get(x_data_idx).unwrap().parse()?);
        y_data.push(record.get(y_data_idx).unwrap().parse()?);
        r_data.push(record.get(r_data_idx).unwrap().parse()?);
        c_data.push(record.get(c_data_idx).unwrap().parse()?);
    }

    let canvas = Canvas {
        width: width,
        height: height,
        background: WHITE
    };

    let x_scale: LinearScale = LinearScaleBuilder::default()
        .domain(extend(&x_data))
        .range([0.0, width as f64])
        .build()?;

    let y_scale: LinearScale = LinearScaleBuilder::default()
        .domain(extend(&y_data))
        .range([height as f64, 0.0])
        .build()?;

    let r_scale: PowScale = PowScaleBuilder::default()
        .domain(extend(&r_data))
        .range([5.0, 15.0])
        .exponent(2)
        .build()?;

    let c_scale: OrdinalScale<String, Color> = OrdinalScaleBuilder::default()
        .domain(vec!['6'.to_string(),'8'.to_string()])
        .range(vec![RED, PERU])
        .unknown(STEELBLUE)
        .build()?;

    let mut scene = Scene::new(canvas);
    // for r in x_scale.call(&x_data).iter().zip(y_scale.call(&y_data).iter()) {
    for row in izip!(x_scale.call(&x_data), 
                     y_scale.call(&y_data),
                     r_scale.call(&r_data),
                     c_scale.call(&c_data)) {

        let (x, y, r, c) = row;
           
        let style = StyleBuilder::default()
            .fill(Some(c))
            .build()?;

        let circle : Circle = CircleBuilder::default()
            .x(x)
            .y(y)
            .radius(r)
            .style(style)
            .build()?;

        scene.add(Box::new(circle));
    }

    svg_backend::save("chart.svg", &scene);
    Ok(())
}