extern crate helio;

use std::rc::Rc;
use std::error::Error;

use helio::core::{Color, Canvas, Circle, CircleBuilder, StyleBuilder, Scene, Rect};
use helio::svg_backend;

fn main() -> Result<(), Box<Error>> {
    let white = Color{r:0, g:0, b:0, a:1.0};
    let blue = Color{r:0, g:0, b:255, a:0.5};
    let style = Rc::new(
        StyleBuilder::default()
            .fill(Some(blue))
            .build()?
    );
    let canvas = Canvas {
        width: 800,
        height: 600,
        background: white
    };

    let circle : Circle = CircleBuilder::default()
        .x(canvas.width as f64 / 2.0)
        .y(canvas.height as f64 / 2.0)
        .radius(40)
        .style(style.clone())
        .build()?;

    let rect = Rect {
        x: canvas.width as f64 / 4.0,
        y: canvas.height as f64 / 4.0,
        width: 40.0,
        height: 40.0,
        style: style.clone()
    };

    let mut scene = Scene::new(canvas);
    scene.add(Box::new(circle));
    scene.add(Box::new(rect));

    svg_backend::save("chart.svg", &scene);
    Ok(())
}