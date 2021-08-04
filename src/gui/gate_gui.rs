use druid::{Env, PaintCtx, Point, Rect, Size};

use crate::base_gate::BaseGate;
use crate::gate::Gate;
use crate::gui::circuit_drawer::{CIRCUIT_MARGIN, DEFAULT_GATE_WIDTH, GATE_WIDTH, QBITS_SEPARATION};
use crate::gui::circuit_gui::GuiWidget;
use crate::gui::gate_gui::SpecificGateGuiLayout::{Not, Swap, BoxWithLetter};
use num_traits::ToPrimitive;

pub enum SpecificGateGuiLayout {
    BoxWithLetter(BoxWithLetterLayout),
    Not(NotLayout),
    Swap(SwapLayout),
}

pub struct GateGuiLayout {
    pub width: f64,
    pub center: Point,
    pub margin: f64,
    pub specific: SpecificGateGuiLayout,
}

pub struct BoxWithLetterLayout {
    pub letter: String,
    pub rect: Rect,
}

pub struct NotLayout {
    pub pos: Point,
}

pub struct SwapLayout {
    pub pos1: Point,
    pub pos2: Point,
}

pub struct GateGui {
    pub layout: Option<GateGuiLayout>,
    pub gate: BaseGate,
    pub control_bits: Vec<u8>,
}

impl From<&Gate> for GateGui {
    fn from(gate: &Gate) -> Self {
        GateGui { layout: None, gate: gate.get_gate(), control_bits: gate.get_control_bits().to_owned() }
    }
}

impl From<&GateGui> for Gate {
    fn from(gate: &GateGui) -> Self {
        Gate::new(gate.gate, gate.control_bits.clone())
    }
}

impl GateGuiLayout {
    fn full_width(&self) -> f64 {
        self.margin*2.+self.width
    }
}

impl GuiWidget for GateGui {
    fn update_layout(&mut self, env: &Env) {
        let qbit_sep = env.get(QBITS_SEPARATION);
        let margin = env.get(CIRCUIT_MARGIN);
        let width = env.get(GATE_WIDTH);

        let x_center = margin + width * 0.5;

        let size = Size { width, height: qbit_sep };
        let center_point = |q: u8| -> Point { Point::new(x_center, qbit_sep * q as f64) };

        let (layout,center) = {
            match self.gate {
                BaseGate::Swap(q1, q2) => GateGui::swap_specific_layout(q1, q2, size, center_point),
                BaseGate::Not(q) => GateGui::not_specific_layout(q, size, center_point),
                BaseGate::X(q) => GateGui::box_with_letter("X", q, size, center_point),
                BaseGate::Y(q) => GateGui::box_with_letter("Y", q, size, center_point),
                BaseGate::Z(q) => GateGui::box_with_letter("Z", q, size, center_point),
                BaseGate::Hadamard(q) => GateGui::box_with_letter("H", q, size, center_point),
            }
        };

        self.layout = Some(GateGuiLayout { width, margin,center, specific: layout });
    }


    fn full_width(&self) -> f64 {
        self.layout.as_ref().map_or(0.0, |l| { l.full_width() })
    }


    fn paint(&self, ctx: &mut PaintCtx, env: &Env) {
        if let Some(layout) = &self.layout {
            match &layout.specific {
                BoxWithLetter(l) => {}
                Not(l) => {}
                Swap(l) => {}
            }

        }
    }
}

impl GateGui {

    fn swap_specific_layout<F>(q1:u8,q2:u8, _size:Size, center_point:F) -> (SpecificGateGuiLayout,Point)
        where F:Fn(u8) -> Point {
        let pos1 = center_point(q1);
        let pos2 = center_point(q2);
        let layout = SwapLayout{pos1,pos2};
        (Swap(layout),pos1.midpoint(pos2))
    }

    fn not_specific_layout<F>(q:u8, _size:Size,center_point:F) -> (SpecificGateGuiLayout,Point)
        where F:Fn(u8) -> Point {
        let pos = center_point(q);
        (Not(NotLayout{pos }), pos)
    }

    fn box_with_letter<F>(letter:&str, q:u8, size:Size, center_point:F) -> (SpecificGateGuiLayout,Point)
        where F:Fn(u8) -> Point {
        let letter = letter.to_string();
        let center = center_point(q);
        let rect = Rect::from_center_size(center,size);
        (BoxWithLetter(BoxWithLetterLayout{letter,rect}),center)
    }

}
