pub use crate::color::Color;
pub use crate::mark::{Circle, Rect, Segment, Line,
                      CircleBuilder, RectBuilder, SegmentBuilder, LineBuilder};

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
    #[builder(default = "1.0")]
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
    pub glyphs: Vec<Box<dyn Glyph<Context=T>>>
}

impl<T> Scene<T> {
    pub fn new(canvas: Canvas) -> Scene<T> {
        Scene{
            canvas: canvas,
            glyphs: vec![]
        }
    }

    pub fn add(&mut self, glyph: Box<dyn Glyph<Context=T>>) {
        self.glyphs.push(glyph);
    }
}


pub trait Glyph {
    type Context;
    fn draw(& self, ctx: &mut Self::Context);
}