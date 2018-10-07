
use svg;
use svg::{Document, Node};
use svg::node::element;
use core::{Scene, Circle, Glyph, Color, Rect, Segment, Line, Style};

pub struct SVGContext {
    pub doc: Document
}

fn set_style<T:Node>(elem: &mut T, style: & Style) {
    elem.assign("fill", Color::rgba(&style.fill));
    elem.assign("stroke", Color::rgba(&style.stroke));
    elem.assign("stroke-width", style.stroke_width);
}

impl Glyph for Circle {
    type Context =  SVGContext;

    fn draw(& self, ctx: &mut Self::Context) {
        let mut e = element::Circle::new()
            .set("cx", self.x)
            .set("cy", self.y)
            .set("r", self.radius);
        set_style(&mut e, &self.style);
        ctx.doc.append(e);
    }
}


impl Glyph for Rect {
    type Context =  SVGContext;

    fn draw(& self, ctx: &mut Self::Context) {
        let mut e = element::Rectangle::new()
            .set("x", self.x)
            .set("y", self.y)
            .set("width", self.width)
            .set("height", self.height);
        set_style(&mut e, &self.style);
        ctx.doc.append(e);
    }
}

impl Glyph for Segment {
    type Context =  SVGContext;

    fn draw(& self, ctx: &mut Self::Context) {
        let mut e = element::Line::new()
            .set("x1", self.x)
            .set("y1", self.y)
            .set("x2", self.x2)
            .set("y2", self.y2);
        set_style(&mut e, &self.style);
        ctx.doc.append(e);
    }
}

impl Glyph for Line {
    type Context =  SVGContext;

    fn draw(& self, ctx: &mut Self::Context) {
        let points: Vec<String> = izip!(&self.x, &self.y)
            .map(|(x, y)| { format!("{},{}", x, y) })
            .collect();
        
        let mut e = element::Polyline::new()
            .set("points", points.join(","));
        set_style(&mut e, &self.style);
        ctx.doc.append(e);
    }
}

pub fn save (path: &str, scene: & Scene<SVGContext>) {
    let mut context = SVGContext{
        doc: Document::new()
                .set("viewBox", (0, 0, scene.canvas.width, scene.canvas.height))
                .set("style", format!("background: {};", scene.canvas.background.to_string()))
    };

    for glyph in & scene.glyphs {
        glyph.draw(&mut context);
    }

    svg::save(path, &context.doc).unwrap();
}