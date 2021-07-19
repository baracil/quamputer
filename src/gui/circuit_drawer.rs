use crate::gui::{Drawable, DrawingPar};
use raylib::prelude::*;
use crate::gui::gui_circuit::{GuiCircuit, GuiRoot};
use rsgui::gui::GuiData;
use crate::gui::gui_drawer::GuiDrawer;
use num_traits::Inv;

const SCALE:u32 = 1;

impl GuiRoot {

    pub fn layout(&mut self, parameter: &DrawingPar) {
        let width = self.circuit.layout(parameter);
        let height = parameter.full_circuit_height();
        self.texture = None;
        self.width = width.max(0.0).round() as u32;
        self.height = height.max(0.0).round() as u32;
    }

    pub fn draw<T:RaylibDraw>(&self, drawer: &mut GuiDrawer<T>, pos: Vector2, parameter: &DrawingPar) {
        match &self.texture {
            None => self.circuit.draw(drawer, parameter),
            Some(t) => {
                let mut texture_pos = pos.clone();
                texture_pos.y -= parameter.register_spacing;
                drawer.draw_texture_ex(t,texture_pos,0.0,(SCALE as f32).inv(),Color::VIOLET)
            }
        }
    }

    pub fn clear_texture(&mut self) {
        self.texture = None;
    }

    pub fn draw_texture(&mut self, parameter: &DrawingPar, mut raylib_handle: &mut RaylibHandle, raylib_thread:&RaylibThread) {
        if self.texture.is_some() {
            return;
        }

        let loading_result =  raylib_handle.load_render_texture(raylib_thread, self.width*SCALE, self.height*SCALE);
        if loading_result.is_err() {
            self.texture = None;
        } else {
            let mut texture = loading_result.unwrap();
            {
                let mut raylib_draw = raylib_handle.begin_texture_mode(raylib_thread, &mut texture);
                let mut gui_drawer = GuiDrawer::for_texture(&mut raylib_draw,parameter,SCALE);
                self.circuit.draw(&mut gui_drawer, &parameter);
            }
            self.texture = Some(texture)
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

    fn draw<T:RaylibDraw>(&self, drawer: &mut GuiDrawer<T>, parameter: &DrawingPar) {
        drawer.push_offset();
        for element in self.elements.iter() {
            element.draw(drawer,  parameter);
            drawer.shift_by(element.gui_data().width);
        };
        drawer.pop_offset();
    }
}
