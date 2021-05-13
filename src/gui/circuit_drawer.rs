use crate::gui::{Drawable, DrawingPar};
use crate::operation::CircuitPar;
use raylib::prelude::*;

impl Drawable for CircuitPar {

    fn draw(&self, drawer: &mut impl RaylibDraw, pos:Vector2, parameter:&DrawingPar) -> Vector2 {
        let mut pos = pos.clone();
        for operation in self.operations.iter() {
            pos = operation.draw(drawer,pos, parameter);
        };
        pos
    }
}
