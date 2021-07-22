use std::collections::LinkedList;

use raylib::drawing::RaylibDraw;
use raylib::prelude::{Color, Rectangle, Vector2};
use rsgui::font::FontInfo;
use rsgui::size::Size;

use crate::gui::{Style};
use crate::gui::mouse_information::MouseInformation;
use crate::gui::gui_circuit::DrawableParameter;

pub struct GuiDrawer<'a, T: RaylibDraw> {
    raylib_draw: &'a mut T,
    pub mouse_info:MouseInformation,
    scale: u32,
    offset: Vector2,
    offset_queue: LinkedList<Vector2>,
}

impl<'a, T: RaylibDraw> GuiDrawer<'a, T> {

    pub(crate) fn is_mouse_in_disk(&self, center: &Vector2, radius: f32) -> bool {
        let mouse_position = self.inv_transform_vector(&self.mouse_info.world_pos);
        let mouse_distance = (mouse_position.x - center.x).hypot(mouse_position.y - center.y);
        mouse_distance <= radius
    }


    pub(crate) fn push_offset(&mut self) {
        self.offset_queue.push_back(self.offset)
    }

    pub(crate) fn pop_offset(&mut self) {
        match self.offset_queue.pop_back() {
            None => self.offset = Vector2::new(0.0, 0.0),
            Some(o) => self.offset = o
        }
    }

    pub fn shift_by(&mut self, width: f32) {
        self.offset.x += self.transform_length(&width);
    }

    fn transform_thickness(&self, length: &i32) -> i32 {
        return *length * (self.scale as i32);
    }

    fn transform_length(&self, length: &f32) -> f32 {
        return *length * (self.scale as f32);
    }

    pub fn transform_vector(&self, reference: &Vector2) -> Vector2 {
        let mut result = reference.clone();
        self.transform_vector_in_place(&mut result);
        result
    }

    pub fn inv_transform_vector(&self, reference: &Vector2) -> Vector2 {
        let mut result = reference.clone();
        self.inv_transform_vector_in_place(&mut result);
        result
    }

    pub fn transform_rectangle(&self, reference: &Rectangle) -> Rectangle {
        let mut result = reference.clone();
        self.transform_rectangle_in_place(&mut result);
        result
    }


    fn transform_vector_in_place(&self, target: &mut Vector2) {
        let x = target.x * (self.scale as f32) + self.offset.x;
        let y = target.y * (self.scale as f32) + self.offset.y;
        target.x = x;
        target.y = y;
    }

    pub fn inv_transform_vector_in_place(&self, target: &mut Vector2) {
        let x = (target.x - self.offset.x) / (self.scale as f32);
        let y = (target.y - self.offset.y)/ (self.scale as f32);
        target.x = x;
        target.y = y;
    }

    pub fn transform_rectangle_in_place(&self, reference: &mut Rectangle) {
        let x = reference.x * (self.scale as f32) + self.offset.x;
        let y = reference.y * (self.scale as f32) + self.offset.y;
        let width = reference.width * (self.scale as f32);
        let height = reference.height * (self.scale as f32);
        reference.x = x;
        reference.y = y;
        reference.width = width;
        reference.height = height;
    }

    pub(crate) fn draw_text(&mut self, font: &FontInfo, text: &String, pos: &Vector2, size: &Size, color: Color) {
        let mut rec = Rectangle::new(pos.x, pos.y, size.width(), size.height());
        self.transform_rectangle_in_place(&mut rec);
        let pos = Vector2::new(rec.x, rec.y);
        self.raylib_draw.draw_text_ex(&font.font.as_ref(), text, pos, font.size * (self.scale as f32), 0.0, color);
    }

    pub(crate) fn draw_circle_sector_lines(&mut self, center: &Vector2, radius: f32, start_angle: i32, end_angle: i32, segments: i32, color: Color) {
        let radius = self.transform_length(&radius);
        let center = self.transform_vector(&center);
        self.raylib_draw.draw_circle_sector_lines(center, radius, start_angle, end_angle, segments, color)
    }

    pub(crate) fn draw_circle_v(&mut self, center: &Vector2, radius: f32, color: Color) {
        let radius = self.transform_length(&radius);
        let center = self.transform_vector(&center);
        self.raylib_draw.draw_circle_v(center, radius, color);
    }

    pub(crate) fn draw_line_ex(&mut self, start: &Vector2, end: &Vector2, thickness: f32, color: Color) {
        let start = self.transform_vector(start);
        let end = self.transform_vector(end);
        let thickness = self.transform_length(&thickness);

        self.raylib_draw.draw_line_ex(start, end, thickness, color);
    }

    pub(crate) fn draw_all_registers(&mut self, parameter:&DrawableParameter, width: f32) {
        let width = self.transform_length(&width);
        let thickness = self.transform_length(&parameter.register_thickness);
        let mut pos_start = Vector2::zero();
        let mut pos_end = Vector2::zero();

        for i in 0..parameter.nb_qbits {
            pos_start.x = 0.0;
            pos_start.y = (i as f32) * parameter.register_spacing;

            pos_end.x = pos_start.x + width;
            pos_end.y = pos_start.y;

            self.transform_vector_in_place(&mut pos_start);
            self.transform_vector_in_place(&mut pos_end);

            self.raylib_draw.draw_line_ex(pos_start, pos_end, thickness, parameter.foreground_color);
        }
    }

    pub(crate) fn draw_rectangle_lines_ex(&mut self, rectangle: &Rectangle, thickness: i32, color: Color) {
        let rectangle = self.transform_rectangle(rectangle);
        let thickness = self.transform_thickness(&thickness);
        self.raylib_draw.draw_rectangle_lines_ex(rectangle, thickness, color)
    }

    pub(crate) fn draw_rectangle_rec(&mut self, rectangle: &Rectangle, color: Color) {
        let rectangle = self.transform_rectangle(rectangle);
        self.raylib_draw.draw_rectangle_rec(rectangle, color)
    }

    pub fn default(raylib_draw: &'a mut T, mouse_info:&MouseInformation, position: Vector2) -> GuiDrawer<'a, T> {
        return GuiDrawer::new(raylib_draw,  mouse_info, position, 1);
    }

    fn new(raylib_draw: &'a mut T, mouse_info:&MouseInformation, position: Vector2, scale: u32) -> Self {
        Self { raylib_draw,scale, offset: position, offset_queue: LinkedList::new(), mouse_info:mouse_info.clone() }
    }
}