use std::rc::Rc;
pub use color::Color;

#[derive(Debug, Default)]
pub struct Canvas {
    pub width: u32,
    pub height: u32,
    pub background: Color
}

#[derive(Debug, Builder)]
#[builder(setter(into))]
pub struct Style {
    #[builder(default)]
    pub fill: Option<Color>,
    #[builder(default)]
    pub stroke: Option<Color>,
    #[builder(default)]
    pub stroke_width: f64
}

impl Default for Style {
    fn default() -> Style { 
        Style {
            fill: Some(Color{r: 0, g: 0, b: 0, a: 1.0}), 
            stroke: None,
            stroke_width: 1.0
        }
    }
}

#[derive(Default)]
pub struct Scene<T> {
    pub canvas: Canvas,
    pub glyphs: Vec<Box<Glyph<Context=T>>>
}

impl<T> Scene<T> {
    pub fn new(canvas: Canvas) -> Scene<T> {
        Scene{
            canvas: canvas,
            glyphs: vec![]
        }
    }

    pub fn add(&mut self, glyph: Box<Glyph<Context=T>>) {
        self.glyphs.push(glyph);
    }
}


pub trait Glyph {
    type Context;
    fn draw(& self, ctx: &mut Self::Context);
}

#[derive(Debug, Builder, Default)]
#[builder(setter(into))]
pub struct Circle {
    pub x: f64,
    pub y: f64,
    pub radius: f64,
    #[builder(default)]
    pub style: Rc<Style>
}

#[derive(Debug, Builder)]
#[builder(setter(into))]
pub struct Rect {
    pub x: f64,
    pub y: f64,
    pub width: f64,
    pub height: f64,
    pub style: Rc<Style>
}

#[derive(Debug, Builder)]
#[builder(setter(into))]
pub struct Segment {
    pub x: f64,
    pub y: f64,
    pub x2: f64,
    pub y2: f64,
    pub style: Rc<Style>
}

#[derive(Debug, Builder)]
#[builder(setter(into))]
pub struct Line {
    pub x: Vec<f64>,
    pub y: Vec<f64>,
    pub style: Rc<Style>
}

pub trait Mark {
    fn x() -> f64;
    fn y() -> f64;
    fn x2() -> f64;
    fn y2() -> f64;
    fn xc() -> f64;
    fn yc() -> f64;
    fn width() -> f64;
    fn height() -> f64;
    fn style() -> Style;
}
