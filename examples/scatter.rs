extern crate helio;
extern crate csv;

use std::rc::Rc;
use std::error::Error;

use helio::core::{Color, Canvas, Circle, CircleBuilder, StyleBuilder, Scene};
use helio::scale::{Scale, ScaleBuilder, extend};
use helio::svg_backend;

fn main() -> Result<(), Box<Error>> {

    let width = 800;
    let height = 600;

    let x_column = "weight (lb)";
    let y_column = "displacement (cc)";

    // Data to use
    let mut x_data: Vec<f32> = Vec::new();
    let mut y_data: Vec<f32> = Vec::new();

    let mut rdr = csv::Reader::from_path("examples/data/cars.csv")?;
    println!("{:?}", rdr.headers()?);

    let x_data_idx = rdr.headers()?.iter().position(|x| x == x_column)
        .ok_or(format!("'{}' not in the dataset", x_column) )?;
    let y_data_idx = rdr.headers()?.iter().position(|x| x == y_column)
        .ok_or(format!("'{}' not in the dataset", y_column) )?;

    for result in rdr.records() {
        let record = result?;
        x_data.push(record.get(x_data_idx).unwrap().parse()?);
        y_data.push(record.get(y_data_idx).unwrap().parse()?);
    }

    let white = Color{r:0, g:0, b:0, a:1.0};
    let blue = Color{r:0, g:0, b:255, a:0.5};
    let style = Rc::new(
        StyleBuilder::default()
            .fill(Some(blue))
            .build()?
    );
    let canvas = Canvas {
        width: width,
        height: height,
        background: white
    };

    let x_scale: Scale<f32, f32> = ScaleBuilder::default()
        .domain(extend(&x_data))
        .range([0.0, width as f32])
        .build()?;

    let y_scale: Scale<f32, f32> = ScaleBuilder::default()
        .domain(extend(&y_data))
        .range([height as f32, 0.0])
        .build()?;

    print!("{:?} · ", &x_scale);
    let mut scene = Scene::new(canvas);
    for r in x_scale.call(&x_data).iter().zip(y_scale.call(&y_data).iter()) {

        let (x, y) = r;
        print!("{:?} · ", &x);
        let circle : Circle = CircleBuilder::default()
            .x(*x as u32)
            .y(*y as u32)
            .radius(10)
            .style(style.clone())
            .build()?;

        scene.add(Box::new(circle));
        
    }

    svg_backend::save("chart.svg", &scene);
    Ok(())
}