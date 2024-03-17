use super::DOT_SIZE;
use nannou::draw::Draw;
use nannou::prelude::*;

pub trait DrawExt {
    fn point(&self, x: f32, y: f32, color: Rgb);
}

impl DrawExt for Draw {
    fn point(&self, x: f32, y: f32, color: Rgb) {
        self.ellipse()
            .w_h(DOT_SIZE, DOT_SIZE)
            .x_y(x, y)
            .color(color);
    }
}
