use crate::gui::{Drawable, DrawingPar};
use raylib::prelude::*;
use crate::circuit::Circuit;

impl Drawable for Circuit {

    fn draw(&self, drawer: &mut impl RaylibDraw, pos:Vector2, parameter:&DrawingPar) -> Vector2 {
        let mut pos = pos.clone();
        for operation in self.operations.iter() {
            pos = operation.draw(drawer,pos, parameter);
        };
        pos
    }
}
