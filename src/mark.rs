use std::rc::Rc;

use crate::core::Style;

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
    #[builder(default)]
    pub style: Rc<Style>
}

#[derive(Debug, Builder)]
#[builder(setter(into))]
pub struct Segment {
    pub x: f64,
    pub y: f64,
    pub x2: f64,
    pub y2: f64,
    #[builder(default)]
    pub style: Rc<Style>
}

#[derive(Debug, Builder)]
#[builder(setter(into))]
pub struct Line {
    pub x: Vec<f64>,
    pub y: Vec<f64>,
    #[builder(default)]
    pub style: Rc<Style>
}

impl Line {
    pub fn add_x(&mut self, x: f64) {
        self.x.push(x);
    }
    pub fn add_y(&mut self, y: f64) {
        self.y.push(y);
    }
}

#[derive(Debug, Clone)]
pub enum TextAlign { Start, Center, End, }

#[derive(Debug, Builder)]
#[builder(setter(into))]
pub struct Text {
    pub x: f64,
    pub y: f64,
    pub text: String,
    #[builder(default = "TextAlign::Start")]
    pub align: TextAlign,
    #[builder(default)]
    pub style: Rc<Style>
}
