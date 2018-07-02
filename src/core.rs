use std::rc::Rc;

#[derive(Debug, Clone, Copy, Default)]
pub struct Color {
    pub r: u8,
    pub g: u8,
    pub b: u8,
    pub a: f32
}

impl Color {
    pub fn to_string(&self) -> String {
        format!("rgba({:},{:},{:},{:})", self.r, self.g, self.b, self.a)
    }

    pub fn rgba(color: &Option<Self>) -> String {
        match color { 
            Some(c) => c.to_string(), 
            None => "none".into() 
        }
    }
}

#[derive(Debug, Default)]
pub struct Canvas {
    pub width: u32,
    pub height: u32,
    pub background: Color
}

#[derive(Debug, Builder)]
pub struct Style {
    pub fill: Option<Color>,
    pub stroke: Option<Color>,
    pub stroke_width: u32
}

impl Default for Style {
    fn default() -> Style { 
        Style {
            fill: Some(Color{r: 0, g: 0, b: 0, a: 1.0}), 
            stroke: None,
            stroke_width: 1
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
pub struct Circle {
    pub x: u32,
    pub y: u32,
    pub radius: u32,
    #[builder(default)]
    pub style: Rc<Style>
}

#[derive(Debug, Builder)]
pub struct Rect {
    pub x: u32,
    pub y: u32,
    pub width: u32,
    pub height: u32,
    pub style: Rc<Style>
}

pub trait Mark {
    fn x() -> u32;
    fn y() -> u32;
    fn x2() -> u32;
    fn y2() -> u32;
    fn xc() -> u32;
    fn yc() -> u32;
    fn width() -> u32;
    fn height() -> u32;
    fn style() -> Style;
}
