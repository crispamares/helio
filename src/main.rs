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
        .x(canvas.width / 2)
        .y(canvas.height / 2)
        .radius(40)
        .style(style.clone())
        .build()?;

    let rect = Rect {
        x: canvas.width / 4,
        y: canvas.height / 4,
        width: 40,
        height: 40,
        style: style.clone()
    };

    let mut scene = Scene::new(canvas);
    scene.add(Box::new(circle));
    scene.add(Box::new(rect));

    svg_backend::save("chart.svg", &scene);
    Ok(())
}