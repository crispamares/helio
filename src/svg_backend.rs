
use svg;
use svg::{Document, Node};
use svg::node::element;
use core::{Scene, Circle, Glyph};

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
            .set("fill", match &self.style.fill { Some(c) => c.to_string(), Node => "none".into() })
            .set("stroke", match &self.style.stroke { Some(c) => c.to_string(), Node => "none".into() });
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