#![doc = include_str!("../README.md")]

pub use ttf_parser;

extern crate nalgebra as na;

use ttf_parser::{Face, GlyphId, OutlineBuilder};

use fdsm::{
    bezier::{Point, Segment},
    shape::{Contour, Shape},
};

#[cfg(test)]
mod tests;

/// Loads a glyph from a [`ttf_parser::Face`] as a [`Shape`].
pub fn load_shape_from_face(face: &Face, glyph_id: GlyphId) -> Option<Shape<Contour>> {
    let mut builder = ShapeBuilder {
        shape: Shape::default(),
        start_point: None,
        last_point: None,
    };
    face.outline_glyph(glyph_id, &mut builder)?;
    Some(builder.shape)
}

#[derive(Debug)]
struct ShapeBuilder {
    shape: Shape<Contour>,
    start_point: Option<Point>,
    last_point: Option<Point>,
}

impl OutlineBuilder for ShapeBuilder {
    fn move_to(&mut self, x: f32, y: f32) {
        // eprintln!("move_to {x} {y}");
        if let Some(contour) = self.shape.contours.last_mut() {
            if self.start_point != self.last_point {
                contour.segments.push(Segment::line(
                    self.last_point.unwrap(),
                    self.start_point.unwrap(),
                ));
            }
        }
        self.start_point = Some(Point::new(x.into(), y.into()));
        self.last_point = self.start_point;
        self.shape.contours.push(Contour::default());
    }

    fn line_to(&mut self, x: f32, y: f32) {
        // eprintln!("line_to {x} {y}");
        let next_point = Point::new(x.into(), y.into());
        self.shape
            .contours
            .last_mut()
            .unwrap()
            .segments
            .push(Segment::line(self.last_point.unwrap(), next_point));
        self.last_point = Some(next_point);
    }

    fn quad_to(&mut self, x1: f32, y1: f32, x: f32, y: f32) {
        // eprintln!("quad_to {x1} {y1} {x} {y}");
        let next_point = Point::new(x.into(), y.into());
        self.shape
            .contours
            .last_mut()
            .unwrap()
            .segments
            .push(Segment::quad(
                self.last_point.unwrap(),
                Point::new(x1.into(), y1.into()),
                next_point,
            ));
        self.last_point = Some(next_point);
    }

    fn curve_to(&mut self, x1: f32, y1: f32, x2: f32, y2: f32, x: f32, y: f32) {
        // eprintln!("curve_to {x1} {y1} {x2} {y2} {x} {y}");
        let next_point = Point::new(x.into(), y.into());
        self.shape
            .contours
            .last_mut()
            .unwrap()
            .segments
            .push(Segment::cubic(
                self.last_point.unwrap(),
                Point::new(x1.into(), y1.into()),
                Point::new(x2.into(), y2.into()),
                next_point,
            ));
        self.last_point = Some(next_point);
    }

    fn close(&mut self) {
        // eprintln!("close");
        if let Some(contour) = self.shape.contours.last_mut() {
            if self.start_point != self.last_point {
                contour.segments.push(Segment::line(
                    self.last_point.take().unwrap(),
                    self.start_point.take().unwrap(),
                ));
            }
        }
    }
}
