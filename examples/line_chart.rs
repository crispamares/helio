
use csv;



use std::error::Error;
use std::rc::Rc;
use chrono::{Utc, NaiveDate, DateTime as ChDateTime};
use itertools::Itertools;

use helio::core::{Color, Canvas, Line, StyleBuilder, Scene, TextBuilder, TextAlign};
use helio::scale::{OrdinalScale, OrdinalScaleBuilder, DateTime, TimeScale, TimeScaleBuilder, LinearScale, LinearScaleBuilder};
use helio::svg_backend;
use helio::color;
use helio::utils::{Extend};

fn main() -> Result<(), Box<dyn Error>> {

    let width = 800;
    let height = 300;
    let margin = (50.0, 70.0, 50.0, 70.0); // Top, Right, Bottom, Left

    let x_column = "date";
    let y_column = "price";
    let c_column = "symbol";

    // Data to use
    let mut x_data: Vec<DateTime> = Vec::new();
    let mut y_data: Vec<f64> = Vec::new();
    let mut c_data: Vec<String> = Vec::new();
    
    let mut rdr = csv::Reader::from_path("examples/data/stocks.csv")?;
    println!("{:?}", rdr.headers()?);

    let x_data_idx = rdr.headers()?.iter().position(|x| x == x_column)
        .ok_or(format!("'{}' not in the dataset", x_column) )?;
    let y_data_idx = rdr.headers()?.iter().position(|x| x == y_column)
        .ok_or(format!("'{}' not in the dataset", y_column) )?;
    let c_data_idx = rdr.headers()?.iter().position(|x| x == c_column)
        .ok_or(format!("'{}' not in the dataset", c_column) )?;

    for result in rdr.records() {
        let record = result?;
        let date = NaiveDate::parse_from_str(record.get(x_data_idx).unwrap(), "%b %e %Y")?.and_hms(0,0,0);
        let datetime = ChDateTime::<Utc>::from_utc(date, Utc);
        x_data.push(datetime.into());
        y_data.push(record.get(y_data_idx).unwrap().parse()?);
        c_data.push(record.get(c_data_idx).unwrap().parse()?);
    }

    let canvas = Canvas {
        width: width,
        height: height,
        background: color::WHITE
    };

    let x_scale: TimeScale = TimeScaleBuilder::default()
        .domain(DateTime::extend(&x_data))
        .range([0.0 + margin.3, width as f64 - margin.1])
        .build()?;

    let y_scale: LinearScale = LinearScaleBuilder::default()
        .domain(f64::extend(&y_data))
        .range([height as f64 - margin.0, 0.0 + margin.2])
        .build()?;

    let c_scale: OrdinalScale<String, Color> = OrdinalScaleBuilder::default()
        .domain(vec!["MSFT".to_string(),"AMZN".to_string(), "IBM".to_string(), "GOOG".to_string(), "AAPL".to_string()])
        .range(vec![color::STEELBLUE, color::YELLOW, color::DARKBLUE, color::RED, color::GREEN])
        .unknown(color::GRAY)
        .build()?;

    let mut scene = Scene::new(canvas);
    // for r in x_scale.call(&x_data).iter().zip(y_scale.call(&y_data).iter()) {

    let x = x_scale.call(&x_data);
    let y = y_scale.call(&y_data);

    for category in c_data.iter().unique() {
        let style = StyleBuilder::default()
            .stroke(Some(c_scale.call(&[category.clone()])[0]))
            .stroke_width(2)
            .build()?;

        let rows = c_data.iter()
            .enumerate()
            .filter(move |&(_i, c)| *c == *category)
            .map(|(i, _c)| i);

        let mut line = Line{x: vec![], y: vec![], style: Rc::new(style)};
        for row in rows {
            line.add_x(x[row]);
            line.add_y(y[row]);
        }

        scene.add(Box::new(line));
    }

    let title = TextBuilder::default()
        .x(width / 2)
        .y(40)
        .text("Stocks values")
        .align(TextAlign::Center)
        .build()?;

    scene.add(Box::new(title));

    svg_backend::save("chart.svg", &scene);
    Ok(())
}