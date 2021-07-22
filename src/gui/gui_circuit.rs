use std::cell::RefCell;
use std::fmt::{Display, Formatter};
use std::ops::{Deref, DerefMut};

use generational_arena::Index;
use raylib::core::drawing::RaylibDraw;
use raylib::math::Rectangle;
use raylib::prelude::{Color, Vector2};
use rsgui::size::Size;

use crate::_loop::Loop;
use crate::circuit::Circuit;
use crate::condition::StopCondition;
use crate::gate::Gate;
use crate::gate_without_control::GateWithoutControl;
use crate::gui::{Drawable, Style};
use crate::gui::gui_drawer::GuiDrawer;
use crate::measure::Measure;
use crate::operation::CircuitElement;
use crate::gui::id_generator::IdGenerator;
use std::process::id;

/// Information about hoover gate/control point
pub enum HoverData {
    Loop(u32),
    Measure(u32),
    Gate(u32, Option<u8>, Option<usize>),
}

impl Display for HoverData {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            HoverData::Loop(index) => write!(f, "Loop {:?}", index),
            HoverData::Measure(index) => write!(f, "Measure {:?}", index),
            HoverData::Gate(index, target, control) => write!(f, "Gate {:?} t:{:?} c:{:?}", index, target, control),
        }
    }
}

impl HoverData {
    pub fn for_measure(id: u32) -> Self {
        HoverData::Measure(id)
    }

    pub fn for_loop(id: u32) -> Self {
        HoverData::Loop(id)
    }

    pub fn for_gate_on_target_qbit(id: u32, target: u8) -> Self {
        HoverData::Gate(id, Some(target), None)
    }

    pub fn for_gate_on_control_qbit(id: u32, control: usize) -> Self {
        HoverData::Gate(id, None, Some(control))
    }
}


///Common data to all gui element
#[derive(Clone, Default)]
pub struct CommonGuiData {
    ///the total width the element takes
    pub width: f32,
    ///the center of the element
    pub center: Vector2,
}

///Graphical data for a circuit
#[derive(Clone, Default)]
pub struct GuiCircuitData {
    pub common: CommonGuiData,
}

///Graphical data for a loop element
#[derive(Clone, Default)]
pub struct GuiLoopData {
    ///the common data (width and position)
    pub common: CommonGuiData,
    ///the outline used to delimit the loop
    pub outline: Rectangle,
    ///the background color of the loop
    pub outline_background: Color,
    pub margin: f32,
}

///Graphical data for a gate element
#[derive(Clone, Default)]
pub struct GuiGateData {
    ///the common data (width and position)
    pub common: CommonGuiData,
    pub gate_size: f32,
    pub outline: Rectangle,
    ///optional displayed text
    pub text: Option<String>,
    ///the size of the text
    pub text_size: Size,
    ///the position of the text to display
    pub text_position: Vector2,
}

#[derive(Clone, Default)]
pub struct GuiMeasureData {
    pub common: CommonGuiData,
    pub outline: Rectangle,
}

pub struct GuiRoot {
    pub position: Vector2,
    pub circuit: GuiCircuit,
    pub parameter: DrawableParameter,
}

pub struct DrawableParameter {
    pub nb_qbits: u8,
    pub style: Style,
}

impl Deref for GuiRoot {
    type Target = DrawableParameter;
    fn deref(&self) -> &Self::Target {
        &self.parameter
    }
}

impl DerefMut for GuiRoot {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.parameter
    }
}

impl Deref for DrawableParameter {
    type Target = Style;
    fn deref(&self) -> &Self::Target {
        &self.style
    }
}

impl GuiRoot {
    pub fn new(circuit: &Circuit, reference: &Style) -> Self {
        let mut id_generator= IdGenerator::default();
        let gui_circuit = GuiCircuit::new(circuit, &mut id_generator);

        let parameter = DrawableParameter { nb_qbits: circuit.nb_qbits, style: reference.clone() };
        let mut root = GuiRoot { position: Vector2::default(), parameter, circuit:gui_circuit };

        root
    }

}

#[derive(Clone)]
pub struct GuiCircuit {
    pub id: u32,
    pub gui_data: GuiCircuitData,
    pub nb_qbits: u8,
    pub gui_elements: Vec<GuiCircuitElement>,
}

#[derive(Clone)]
pub struct GuiLoop {
    pub id: u32,
    pub gui_data: GuiLoopData,
    pub circuit: GuiCircuit,
    pub stop_condition: StopCondition,
}

#[derive(Clone)]
pub struct GuiGate {
    pub id: u32,
    pub gui_data: GuiGateData,
    pub gate: GateWithoutControl,
    pub control_bits: Vec<u8>,
}

#[derive(Clone)]
pub struct GuiMeasure {
    pub id: u32,
    pub gui_data: GuiMeasureData,
    ///A uniq identifier of the measurement
    pub measure_id: String,
    ///the target qbit for the measurement
    pub target: u8,
}


#[derive(Clone)]
pub enum GuiCircuitElement {
    GuiLoop(GuiLoop),
    GuiGate(GuiGate),
    GuiMeasure(GuiMeasure),
}

impl Display for GuiCircuitElement {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            GuiCircuitElement::GuiLoop(_) => f.write_str("GuiLoop"),
            GuiCircuitElement::GuiGate(_p) => f.write_str("GuiGate "),
            GuiCircuitElement::GuiMeasure(_) => f.write_str("GuiMeasure"),
        }
    }
}

impl GuiCircuitElement {
    pub(crate) fn width(&self) -> f32 {
        match self {
            GuiCircuitElement::GuiLoop(p) => p.gui_data.common.width,
            GuiCircuitElement::GuiGate(p) => p.gui_data.common.width,
            GuiCircuitElement::GuiMeasure(p) => p.gui_data.common.width,
        }
    }
}

impl Deref for GuiCircuitData {
    type Target = CommonGuiData;
    fn deref(&self) -> &Self::Target {
        &self.common
    }
}

impl DerefMut for GuiCircuitData {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.common
    }
}

impl Deref for GuiGateData {
    type Target = CommonGuiData;
    fn deref(&self) -> &Self::Target {
        &self.common
    }
}

impl DerefMut for GuiGateData {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.common
    }
}

impl Deref for GuiLoopData {
    type Target = CommonGuiData;
    fn deref(&self) -> &Self::Target {
        &self.common
    }
}

impl DerefMut for GuiLoopData {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.common
    }
}

impl Deref for GuiMeasureData {
    type Target = CommonGuiData;
    fn deref(&self) -> &Self::Target {
        &self.common
    }
}

impl DerefMut for GuiMeasureData {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.common
    }
}

impl Drawable for GuiCircuit {
    fn layout(&mut self, parameter: &DrawableParameter) -> f32 {
        let width = self.gui_elements.iter_mut().map(|l| { l.layout(parameter) }).sum();
        self.gui_data.width = width;
        width
    }

    fn draw<T: RaylibDraw>(&self, drawer: &mut GuiDrawer<T>, parameter: &DrawableParameter) -> Option<HoverData> {
        drawer.push_offset();
        let mut hoover_result = None;
        for element in &self.gui_elements {
            let child_hoover = element.draw(drawer, parameter);
            hoover_result = hoover_result.or(child_hoover);
            drawer.shift_by(element.width());
        };
        drawer.pop_offset();
        hoover_result
    }
}


impl GuiCircuit {
    pub fn new(circuit: &Circuit, id_generator: &mut IdGenerator) -> Self {
        let id = id_generator.get_and_increment();
        let gui_elements = circuit.elements.iter()
            .map(|element| { to_gui(element,id_generator)}).collect();

        GuiCircuit {
            id,
            nb_qbits: circuit.nb_qbits,
            gui_data: GuiCircuitData::default(),
            gui_elements
        }
    }
}

fn to_gui(element: &CircuitElement, id_generator: &mut IdGenerator) -> GuiCircuitElement {
    match element {
        CircuitElement::Loop(l) => GuiLoop::new(l, id_generator),
        CircuitElement::Gate(g) => GuiGate::new(g, id_generator),
        CircuitElement::Measure(m) => GuiMeasure::new(m, id_generator)
    }
}

impl GuiLoop {
    pub fn new(loop_element: &Loop, id_generator: &mut IdGenerator) -> GuiCircuitElement {
        let stop_condition = loop_element.stop_condition.clone();
        let circuit = GuiCircuit::new(&loop_element.circuit, id_generator);
        let gui_loop = GuiLoop { id:id_generator.get_and_increment(), stop_condition, circuit, gui_data: GuiLoopData::default() };
        GuiCircuitElement::GuiLoop(gui_loop)
    }
}

impl GuiGate {
    pub fn new(gate: &Gate, id_generator: &mut IdGenerator) -> GuiCircuitElement {
        let gui_gate = GuiGate { id:id_generator.get_and_increment(), control_bits: gate.get_control_bits().clone(), gate: gate.get_gate(), gui_data: GuiGateData::default() };
        GuiCircuitElement::GuiGate(gui_gate)
    }
}

impl GuiMeasure {
    pub fn new(measure: &Measure, id_generator: &mut IdGenerator) -> GuiCircuitElement {
        let gui_measure = GuiMeasure { id:id_generator.get_and_increment(), target: measure.qbit_target, measure_id: measure.id.clone(), gui_data: GuiMeasureData::default() };
        GuiCircuitElement::GuiMeasure(gui_measure)
    }
}


// impl From<Circuit> for GuiCircuit {
//     fn from(c: Circuit) -> Self {
//         GuiCircuit { id:0,gui_data: RefCell::new(GuiCircuitData::default()), nb_qbits: c.nb_qbits }
//     }
// }
//
// impl From<&Circuit> for GuiCircuit {
//     fn from(c: &Circuit) -> Self {
//         GuiCircuit { id:0,gui_data: RefCell::new(GuiCircuitData::default()), nb_qbits: c.nb_qbits }
//     }
// }
//
//
// impl From<&CircuitElement> for GuiCircuitElement {
//     fn from(element: &CircuitElement) -> Self {
//         match element {
//             CircuitElement::Loop(l) => l.into(),
//             CircuitElement::Gate(g) => g.into(),
//             CircuitElement::Measure(m) => m.into(),
//         }
//     }
// }
//
// impl From<Circuit> for GuiCircuitElement {
//     fn from(c: Circuit) -> Self {
//         let gui_loop = GuiLoop { index: None, raw_circuit: true, gui_data: RefCell::new(GuiLoopData::default()), circuit: c.into(), stop_condition: StopCondition::Once() };
//         return GuiCircuitElement::GuiLoop(gui_loop);
//     }
// }
//
// impl From<&Circuit> for GuiCircuitElement {
//     fn from(c: &Circuit) -> Self {
//         let gui_loop = GuiLoop { index: None, raw_circuit: true, gui_data: RefCell::new(GuiLoopData::default()), circuit: c.into(), stop_condition: StopCondition::Once() };
//         return GuiCircuitElement::GuiLoop(gui_loop);
//     }
// }
//
//
//
//
// impl From<&Loop> for GuiCircuitElement {
//     fn from(l: &Loop) -> Self {
//         let gui_circuit = l.circuit.clone().into();
//         let gui_loop = GuiLoop { index: None, raw_circuit: false, gui_data: RefCell::new(GuiLoopData::default()), circuit: gui_circuit, stop_condition: l.stop_condition.clone() };
//         return GuiCircuitElement::GuiLoop(gui_loop);
//     }
// }
//
// impl From<&Gate> for GuiCircuitElement {
//     fn from(g: &Gate) -> Self {
//         let gui_gate = GuiGate { index: None, gui_data: RefCell::new(GuiGateData::default()), control_bits: g.get_control_bits().clone(), gate: g.get_gate() };
//         GuiCircuitElement::GuiGate(gui_gate)
//     }
// }
//
// impl From<&Measure> for GuiCircuitElement {
//     fn from(m: &Measure) -> Self {
//         let gui_measure = GuiMeasure { index: None, gui_data: RefCell::new(GuiMeasureData::default()), target: m.qbit_target, id: m.id.clone() };
//         GuiCircuitElement::GuiMeasure(gui_measure)
//     }
// }
