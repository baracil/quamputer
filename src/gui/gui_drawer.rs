use raylib::{RaylibHandle, RaylibThread};
use raylib::drawing::RaylibDraw;
use crate::gui::DrawingPar;
use raylib::prelude::{Color, Rectangle, Vector2, RenderTexture2D};
use rs_gui::font::FontInfo;
use std::panic::panic_any;
use rs_gui::size::Size;
use std::path::Prefix::DeviceNS;
use std::collections::LinkedList;
use std::net::Shutdown::Read;

pub struct GuiDrawer<'a, T: RaylibDraw> {
    raylib_draw: &'a mut T,
    full_height: f32,
    flipped: bool,
    point1: Vector2,
    point2: Vector2,
    rectangle: Rectangle,
    offset: Vector2,
    offset_queue: LinkedList<Vector2>,
}

impl<'a, T: RaylibDraw> GuiDrawer<'a, T> {}

impl<'a, T: RaylibDraw> GuiDrawer<'a, T> {
    pub(crate) fn push_offset(&mut self) {
        self.offset_queue.push_back(self.offset)
    }

    pub(crate) fn pop_offset(&mut self) {
        match self.offset_queue.pop_back() {
            None => self.offset = Vector2::new(0.0, 0.0),
            Some(o) => self.offset = o
        }
    }
}

impl<'a, T: RaylibDraw> GuiDrawer<'a, T> {
    pub fn shift_by(&mut self, width: f32) {
        self.offset.x += width;
    }

    fn transform_thickness(&self, length: &i32) -> i32 {
        return *length;
    }

    fn transform_length(&self, length: &f32) -> f32 {
        return *length;
    }

    fn transform_vector(&self, reference: &Vector2) -> Vector2 {
        let mut result = reference.clone();
        self.transform_vector_in_place(&mut result);
        result
    }

    fn transform_vector_in_place(&self, target: &mut Vector2) {
        let mut x = target.x + self.offset.x;
        let mut y = target.y + self.offset.y;
        if self.flipped {
            y = self.full_height - y;
        }
        target.x = x;
        target.y = y;
    }

    fn transform_rectangle(&self, reference: &Rectangle) -> Rectangle {
        let mut result = reference.clone();
        result.x += self.offset.x;
        result.y += self.offset.y;

        if self.flipped {
            result.y = self.full_height - result.y - reference.height;
        }
        result
    }


    fn transform_rectangle_in_place(&self, reference: &mut Rectangle) {
        let x = reference.x + self.offset.x;
        let mut y = reference.y + self.offset.y;
        if self.flipped {
            y = self.full_height - y - reference.height;
        }
        reference.x = x;
        reference.y = y;
    }


    pub(crate) fn draw_text(&mut self, font: &FontInfo, text: &String, pos: &Vector2, size: &Size, color: Color) {
        let mut rec = Rectangle::new(pos.x, pos.y, size.width(), size.height());
        self.transform_rectangle_in_place(&mut rec);
        let pos = Vector2::new(rec.x, rec.y);
        font.draw_text(self.raylib_draw, text, &pos, 0.0, color);
    }

    pub(crate) fn draw_circle_sector_lines(&mut self, center: &Vector2, radius: f32, start_angle: i32, end_angle: i32, segments: i32, color: Color) {
        let radius = self.transform_length(&radius);
        let center = self.transform_vector(&center);
        self.raylib_draw.draw_circle_sector_lines(center, radius, start_angle, end_angle, segments, color)
    }

    pub(crate) fn draw_texture_ex(&mut self, texture: &RenderTexture2D, position: Vector2, rotation: f32, scale: f32, color: Color) {
        let position = self.transform_vector(&position);
        self.raylib_draw.draw_texture_ex(texture, position, rotation, scale, color)
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

    pub(crate) fn draw_all_registers(&mut self, parameter: &DrawingPar, width: f32) {
        let width = self.transform_length(&width);
        let mut pos_start = Vector2::zero();
        let mut pos_end = Vector2::zero();

        for i in 0..parameter.nb_qbits {
            pos_start.x = 0.0;
            pos_start.y = (i as f32) * parameter.register_spacing;

            pos_end.x = pos_start.x + width;
            pos_end.y = pos_start.y;

            self.transform_vector_in_place(&mut pos_start);
            self.transform_vector_in_place(&mut pos_end);

            self.raylib_draw.draw_line_ex(pos_start, pos_end, parameter.register_thickness, parameter.foreground_color);
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
}

impl<'a, T: RaylibDraw> GuiDrawer<'a, T> {
    pub fn default(raylib_draw: &'a mut T, parameter: &DrawingPar, position: Vector2) -> GuiDrawer<'a, T> {
        return GuiDrawer::new(raylib_draw, parameter.full_circuit_height(), position, false);
    }

    pub fn for_texture(raylib_draw: &'a mut T, parameter: &DrawingPar) -> GuiDrawer<'a, T> {
        let position = Vector2::new(0.0, parameter.register_spacing);
        return GuiDrawer::new(raylib_draw, parameter.full_circuit_height(), position, true);
    }

    fn new(raylib_draw: &'a mut T, full_height: f32, position: Vector2, flipped: bool) -> Self {
        Self { raylib_draw, full_height, flipped, point1: Default::default(), offset: position, point2: Default::default(), rectangle: Default::default(), offset_queue: LinkedList::new() }
    }
}