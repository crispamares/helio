
use svg;
use svg::{Document, Node};
use svg::node::element;
use core::{Scene, Circle, Glyph, Color, Rect};

pub struct SVGContext {
    pub doc: Document
}


impl Glyph for Circle {
    type Context =  SVGContext;

    fn draw(& self, ctx: &mut Self::Context) {
        let e = element::Circle::new()
            .set("cx", self.x)
            .set("cy", self.y)
            .set("r", self.radius)
            .set("fill", Color::rgba(&self.style.fill))
            .set("stroke", Color::rgba(&self.style.stroke))
            .set("stroke-width", self.style.stroke_width);
        ctx.doc.append(e);
    }
}


impl Glyph for Rect {
    type Context =  SVGContext;

    fn draw(& self, ctx: &mut Self::Context) {
        let e = element::Rectangle::new()
            .set("x", self.x)
            .set("y", self.y)
            .set("width", self.width)
            .set("height", self.height)
            .set("fill", Color::rgba(&self.style.fill))
            .set("stroke", Color::rgba(&self.style.stroke))
            .set("stroke-width", self.style.stroke_width);
        ctx.doc.append(e);
    }
}

pub fn save (path: &str, scene: & Scene<SVGContext>) {
    let mut context = SVGContext{
        doc: Document::new()
                .set("viewBox", (0, 0, scene.canvas.width, scene.canvas.height))
    };

    for glyph in & scene.glyphs {
        glyph.draw(&mut context);
    }

    svg::save(path, &context.doc).unwrap();
}