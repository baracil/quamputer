use crate::gui::{Drawable, DrawingPar};
use raylib::prelude::*;
use crate::gui::gui_circuit::{GuiCircuit, GuiRoot};


impl GuiRoot {

    pub fn layout(&mut self, parameter: &DrawingPar) {
        let width = self.circuit.layout(parameter);
        let height = parameter.full_circuit_height();
        self.texture = None;
        self.width = width.max(0.0).round() as u32;
        self.height = height.max(0.0).round() as u32;
    }


    pub fn clear_texture(&mut self) {
        self.texture = None;
    }

    pub fn draw_texture(&mut self, parameter: &DrawingPar, mut rdh : &mut RaylibHandle, rt:&RaylibThread) {
        if self.texture.is_some() {
            return;
        }

        let loading_result =  rdh.load_render_texture(rt,self.width,self.height);
        if loading_result.is_err() {
            self.texture = None;
        } else {
            let mut texture = loading_result.unwrap();
            {
                let mut tm = rdh.begin_texture_mode(rt, &mut texture);
                self.circuit.draw(&mut tm, Vector2::new(0.0,parameter.register_spacing), &parameter, true);
            }
            self.texture = Some(texture)
        }

    }


    pub fn draw(&self, drawer: &mut impl RaylibDraw, pos: Vector2, parameter: &DrawingPar) {
        match &self.texture {
            None => self.circuit.draw(drawer,pos,parameter, false),
            Some(t) => {
                let mut texture_pos = pos.clone();
                texture_pos.y -= parameter.register_spacing;
                drawer.draw_texture_ex(t,texture_pos,0.0,1.0,Color::WHITE)
            }
        }
    }
}

impl Drawable for GuiCircuit {
    fn layout(&mut self, parameter: &DrawingPar) -> f32 {
        let width = self.elements.iter_mut()
            .map(|o| o.layout(parameter))
            .sum();
        self.gui_data.width = width;
        width
    }

    fn draw(&self, drawer: &mut impl RaylibDraw, pos: Vector2, parameter: &DrawingPar, flipped:bool) {
        let mut pos = pos.clone();
        for element in self.elements.iter() {
            element.draw(drawer, pos, parameter,flipped);
            pos.x += element.gui_data().width
        };
    }
}
