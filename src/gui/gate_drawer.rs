use std::cmp::Ordering;

use raylib::drawing::RaylibDraw;
use raylib::math::Vector2;
use raylib::prelude::Rectangle;
use rsgui::alignment::HAlignment;
use rsgui::size::Size;
use vec_tree::VecTree;

use crate::gate_without_control::GateWithoutControl;
use crate::gui::{Drawable, DrawingPar, HEIGHT_SPACING_RATIO};
use crate::gui::gui_circuit::{GuiCircuitElement, GuiGate, GuiGateData, HoverData};
use crate::gui::gui_drawer::GuiDrawer;

impl Drawable for GuiGate {
    fn layout(&self, nb_qbits: u8, parameter: &DrawingPar, _tree: &VecTree<GuiCircuitElement>) -> f32 {
        let gate_size = self.gate.width(parameter);
        let gate_y_center = self.gate.y_middle(parameter);
        let text = self.gate.text();

        let mut data = GuiGateData::default();
        data.text = text;
        data.outline.x = parameter.margin;
        data.outline.y = gate_y_center - gate_size * 0.5;
        data.outline.width = gate_size;
        data.outline.height = gate_size;

        match &data.text {
            None => {
                data.text_size = Size::new(0.0, 0.0);
                data.text_position = Vector2::default();
            }
            Some(t) => {
                let size = parameter.font.measure_text(t, 0.0);
                data.text_size = size;
                data.text_position.x = data.outline.x + (data.outline.width - size.width()) * 0.5;
                data.text_position.y = data.outline.y + (data.outline.height - size.height()) * 0.5;
            }
        }

        let width = 2.0 * parameter.margin + gate_size;
        data.width = width;
        data.center.x = parameter.margin + gate_size * 0.5;
        data.center.y = gate_y_center;
        data.gate_size = gate_size;

        self.gui_data.replace(data);
        width
    }

    fn draw<T: RaylibDraw>(&self, drawer: &mut GuiDrawer<T>, nb_qbits: u8, parameter: &DrawingPar, _tree: &VecTree<GuiCircuitElement>) -> Option<HoverData> {
        let width = self.gui_data.borrow().width;

        drawer.draw_all_registers(nb_qbits, parameter, width);

        let hover_control = self.draw_control_qbits(drawer, parameter, &self.control_bits);


        let hover_gate = self.gate.draw(drawer, parameter, &self.gui_data.borrow());

        let hover_result = if let Some(index) = self.index {
            match (hover_control, hover_gate) {
                (_, Some(target_qbit)) => Some(HoverData::for_gate_on_target_qbit(index, target_qbit)),
                (Some(control_qbit), _) => Some(HoverData::for_gate_on_control_qbit(index, control_qbit)),
                (None, None) => None
            }
        } else { None };

        hover_result
    }
}

impl GuiGate {
    fn draw_control_qbits<T: RaylibDraw>(&self, drawer: &mut GuiDrawer<T>, parameter: &DrawingPar, control_bits: &[u8]) -> Option<usize> {
        let mut cpos_end = self.gui_data.borrow().center.clone();
        let radius = parameter.register_spacing * 0.06;


        for (i, control_bit) in control_bits.iter().enumerate() {
            cpos_end.y = parameter.qbit_y_offset(*control_bit);
            drawer.draw_line_ex(&self.gui_data.borrow().center, &cpos_end, parameter.register_thickness, parameter.foreground_color);
        };

        let mut hover_result = None;
        for (i, control_bit) in control_bits.iter().enumerate() {
            cpos_end.y = parameter.qbit_y_offset(*control_bit);
            let hover = drawer.is_mouse_in_disk(&cpos_end, radius);
            if hover {
                hover_result = Some(i);
            }
            let color = hover.then(|| { parameter.hover_color }).unwrap_or(parameter.foreground_color);
            drawer.draw_circle_v(&cpos_end, radius, color);
        };
        hover_result
    }
}


impl GateWithoutControl {
    pub fn width(&self, parameter: &DrawingPar) -> f32 {
        let factor: f32 = match self {
            GateWithoutControl::Not(_) => 0.5,
            GateWithoutControl::Swap(_, _) => 0.5,
            GateWithoutControl::X(_) => 1.0,
            GateWithoutControl::Y(_) => 1.0,
            GateWithoutControl::Z(_) => 1.0,
            GateWithoutControl::Hadamard(_) => 1.0
        };
        return parameter.register_spacing * factor * HEIGHT_SPACING_RATIO;
    }

    pub fn text(&self) -> Option<String> {
        match self {
            GateWithoutControl::Not(_) => None,
            GateWithoutControl::X(_) => Some(String::from("X")),
            GateWithoutControl::Y(_) => Some(String::from("Y")),
            GateWithoutControl::Z(_) => Some(String::from("Z")),
            GateWithoutControl::Swap(_, _) => None,
            GateWithoutControl::Hadamard(_) => Some(String::from("H"))
        }
    }

    pub fn y_middle(&self, parameter: &DrawingPar) -> f32 {
        match self {
            GateWithoutControl::Not(t) => parameter.qbit_y_offset(*t),
            GateWithoutControl::X(t) => parameter.qbit_y_offset(*t),
            GateWithoutControl::Y(t) => parameter.qbit_y_offset(*t),
            GateWithoutControl::Z(t) => parameter.qbit_y_offset(*t),
            GateWithoutControl::Swap(t1, t2) => (parameter.qbit_y_offset(*t1) + parameter.qbit_y_offset(*t2)) * 0.5,
            GateWithoutControl::Hadamard(t) => parameter.qbit_y_offset(*t),
        }
    }

    fn draw<T: RaylibDraw>(&self, drawer: &mut GuiDrawer<T>, parameter: &DrawingPar, gui_data: &GuiGateData) -> Option<u8> {
        match self {
            GateWithoutControl::X(target) => draw_gate_with_text(drawer, parameter, gui_data).then(|| { *target }),
            GateWithoutControl::Y(target) => draw_gate_with_text(drawer, parameter, gui_data).then(|| { *target }),
            GateWithoutControl::Z(target) => draw_gate_with_text(drawer, parameter, gui_data).then(|| { *target }),
            GateWithoutControl::Hadamard(target) => draw_gate_with_text(drawer, parameter, gui_data).then(|| { *target }),

            GateWithoutControl::Not(target) => draw_not_gate(drawer, parameter, gui_data).then(|| { *target }),
            GateWithoutControl::Swap(target1, target2) => draw_swap_gate(drawer, parameter, gui_data, target1, target2),
        }
    }
}


fn draw_gate_with_text<T: RaylibDraw>(drawer: &mut GuiDrawer<T>, parameter: &DrawingPar, gui_data: &GuiGateData) -> bool {
    let mouse_position = drawer.mouse_info.world_pos;
    let transformed_outline = drawer.transform_rectangle(&gui_data.outline);

    let hover = transformed_outline.check_collision_point_rec(mouse_position);

    let color = if hover { parameter.hover_color } else { parameter.foreground_color };

    drawer.draw_rectangle_rec(&gui_data.outline, parameter.background_color);
    drawer.draw_rectangle_lines_ex(&gui_data.outline, parameter.register_thickness as i32, color);

    if let Some(text) = &gui_data.text {
        drawer.draw_text(&parameter.font, text, &gui_data.text_position, &gui_data.text_size, parameter.foreground_color);
    }

    hover
}

fn draw_not_gate<T: RaylibDraw>(drawer: &mut GuiDrawer<T>, parameter: &DrawingPar, gui_data: &GuiGateData) -> bool {
    let circle_radius = gui_data.gate_size * 0.5;
    let hover = drawer.is_mouse_in_disk(&gui_data.center, circle_radius);

    let color = if hover { parameter.hover_color } else { parameter.foreground_color };

    drawer.draw_circle_sector_lines(&gui_data.center, circle_radius, 0, 360, 32, color);

    let mut pos1 = Vector2::new(gui_data.center.x - circle_radius, gui_data.center.y);
    let mut pos2 = Vector2::new(gui_data.center.x + circle_radius, gui_data.center.y);
    drawer.draw_line_ex(&pos1, &pos2, parameter.register_thickness, color);
    pos1.x = gui_data.center.x;
    pos2.x = gui_data.center.x;
    pos1.y -= circle_radius;
    pos2.y += circle_radius;
    drawer.draw_line_ex(&pos1, &pos2, parameter.register_thickness, color);

    hover
}

fn draw_swap_gate<T: RaylibDraw>(drawer: &mut GuiDrawer<T>, parameter: &DrawingPar, gui_data: &GuiGateData, target1: &u8, target2: &u8) -> Option<u8> {
    let target_y_pos1 = parameter.qbit_y_offset(*target1);
    let target_y_pos2 = parameter.qbit_y_offset(*target2);
    let size = gui_data.gate_size * 0.5;

    let pos1 = Vector2::new(parameter.margin + size, target_y_pos1);
    let pos2 = Vector2::new(parameter.margin + size, target_y_pos2);

    drawer.draw_line_ex(&pos1, &pos2, parameter.register_thickness, parameter.foreground_color);

    let hover1 = draw_swap_cross(drawer, pos1, size, parameter);
    let hover2 = draw_swap_cross(drawer, pos2, size, parameter);

    hover1.then(|| { *target1 }).or_else(|| { hover2.then(|| { *target2 }) })
}

fn draw_swap_cross<T: RaylibDraw>(drawer: &mut GuiDrawer<T>, center: Vector2, size: f32, parameter: &DrawingPar) -> bool {
    let size = size * 0.5;
    let mut point1 = center.clone();
    let mut point2 = center.clone();
    let hover = drawer.is_mouse_in_disk(&center, size);

    let color = if hover { parameter.hover_color } else { parameter.foreground_color };

    point1.x -= size;
    point1.y -= size;
    point2.x += size;
    point2.y += size;
    drawer.draw_line_ex(&point1, &point2, parameter.register_thickness, color);

    point1.y += 2.0 * size;
    point2.y -= 2.0 * size;
    drawer.draw_line_ex(&point1, &point2, parameter.register_thickness, color);

    hover
}

