use svg::Document;
use svg::node::element::*;

use crate::transforms::get_translation;

pub struct Plot {
    pub width: f64,
    pub height: f64,
    top: f64,
    bottom: f64,
    left: f64,
    right: f64,
}

impl Plot {
    pub fn new() -> Self {
        Plot {
            width: 800.0,
            height: 600.0,
            top: 10.0,
            bottom: 10.0,
            left: 10.0,
            right: 10.0,
        }
    }

    pub fn set_width(&mut self, width: f64) -> &mut Self {
        self.width = width;
        self
    }

    pub fn set_height(&mut self, height: f64) -> &mut Self {
        self.height = height;
        self
    }

    pub fn set_margin(&mut self, (top, right, bottom, left): (f64, f64, f64, f64)) -> &mut Self {
        self.top = top;
        self.right = right;
        self.bottom = bottom;
        self.left = left;
        self
    }

    pub fn ef_w(&self) -> f64 {
        self.width - self.left - self.right
    }

    pub fn ef_h(&self) -> f64 {
        self.height - self.top - self.bottom
    }

    pub fn ef_r(&self) -> f64 {
        self.ef_w()/self.ef_h()
    }

    pub fn get_tgroup(&self) -> Group {

        let margstr = get_translation(self.left, self.top);

        let g = Group::new()
            .set("transform",margstr);

        g
    }

    pub fn get_docu(&self) -> Document {
        let document = Document::new()
                        .set("viewBox", (0, 0, self.width, self.height));

        document
    }
}
