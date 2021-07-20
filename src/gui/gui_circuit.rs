use crate::condition::StopCondition;
use crate::gate::{GateWithoutControl};
use crate::circuit::Circuit;
use crate::operation::{CircuitElement, Loop, Gate, Measure};
use raylib::prelude::{Vector2, Color};
use std::ops::{Deref, DerefMut};
use raylib::math::Rectangle;
use rsgui::size::Size;
use generational_arena::Index;
use vec_tree::VecTree;
use std::cell::{ RefCell};

///Common data to all gui element
#[derive(Clone, Default)]
pub struct CommonGuiData {
    ///the total width the element takes
    pub width:f32,
    ///the center of the element
    pub center:Vector2,
}

///Graphical data for a circuit
#[derive(Clone, Default)]
pub struct GuiCircuitData {
    pub common:CommonGuiData,
}

///Graphical data for a loop element
#[derive(Clone, Default)]
pub struct GuiLoopData {
    ///the common data (width and position)
    pub common:CommonGuiData,
    ///the outline used to delimit the loop
    pub outline:Rectangle,
    ///the background color of the loop
    pub outline_background:Color,
    pub margin:f32,
}

///Graphical data for a gate element
#[derive(Clone,Default)]
pub struct GuiGateData {
    ///the common data (width and position)
    pub common:CommonGuiData,
    pub gate_size:f32,
    pub outline:Rectangle,
    ///optional displayed text
    pub text:Option<String>,
    ///the size of the text
    pub text_size:Size,
    ///the position of the text to display
    pub text_position:Vector2,
}

#[derive(Clone, Default)]
pub struct GuiMeasureData {
    pub common:CommonGuiData,
    pub outline:Rectangle,
}

pub struct GuiRoot {
    pub width:u32,
    pub height:u32,
    pub tree:VecTree<GuiCircuitElement>,
}

impl GuiRoot {

    pub fn new(circuit:&Circuit) -> Self {
        let gui_loop:GuiCircuitElement = circuit.into();
        let mut tree = VecTree::new();

        let root_index = tree.insert_root(gui_loop);
        tree.get_mut(root_index).unwrap().set_index(root_index);

        let mut root = GuiRoot{tree, width:0,height:0};

        for element in &circuit.elements {
            root.add(element,root_index);
        };

        root
    }

    fn add(&mut self,element:&CircuitElement,parent:Index) {
        let gui_element = element.into();
        let index = self.tree.insert(gui_element,parent);
        self.tree.get_mut(index).unwrap().set_index(index);
        match element {
            CircuitElement::Loop(l) => {
                for x in &l.circuit.elements {
                    self.add(x,index)
                }}
            CircuitElement::Gate(_) => {}
            CircuitElement::Measure(_) => {}
        };
    }

}

#[derive(Clone)]
pub struct GuiCircuit {
    pub gui_data:RefCell<GuiCircuitData>,
    pub nb_qbits:u8,
}

#[derive(Clone)]
pub struct GuiLoop {
    pub index:Option<Index>,
    pub gui_data:RefCell<GuiLoopData>,
    pub circuit:GuiCircuit,
    pub stop_condition:StopCondition,
    pub raw_circuit:bool,
}

#[derive(Clone)]
pub struct GuiGate {
    pub index:Option<Index>,
    pub gui_data:RefCell<GuiGateData>,
    pub gate:GateWithoutControl,
    pub control_bits:Vec<u8>,
}

#[derive(Clone)]
pub struct GuiMeasure {
    pub index:Option<Index>,
    pub gui_data:RefCell<GuiMeasureData>,
    ///A uniq identifier of the measurement
    pub id:String,
    ///the target qbit for the measurement
    pub target:u8,
}


#[derive(Clone)]
pub enum GuiCircuitElement {
    GuiLoop(GuiLoop),
    GuiGate(GuiGate),
    GuiMeasure(GuiMeasure),
}

impl GuiCircuitElement {
    pub(crate) fn width(&self) -> f32 {
        match self {
            GuiCircuitElement::GuiLoop(p) => p.gui_data.borrow().common.width,
            GuiCircuitElement::GuiGate(p) => p.gui_data.borrow().common.width,
            GuiCircuitElement::GuiMeasure(p) => p.gui_data.borrow().common.width,
        }
    }

    pub(crate) fn set_index(&mut self, index:Index) {
        match self {
            GuiCircuitElement::GuiLoop(p) => p.index = Some(index),
            GuiCircuitElement::GuiGate(p) => p.index = Some(index),
            GuiCircuitElement::GuiMeasure(p) => p.index = Some(index),
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

impl From<Circuit> for GuiCircuit {
    fn from(c: Circuit) -> Self {
        GuiCircuit{gui_data:RefCell::new(GuiCircuitData::default()),nb_qbits:c.nb_qbits}
    }
}
impl From<&Circuit> for GuiCircuit {
    fn from(c: &Circuit) -> Self {
        GuiCircuit{gui_data:RefCell::new(GuiCircuitData::default()),nb_qbits:c.nb_qbits}
    }
}


impl From<&CircuitElement> for GuiCircuitElement {
    fn from(element: &CircuitElement) -> Self {
        match element {
            CircuitElement::Loop(l) => l.into(),
            CircuitElement::Gate(g) => g.into(),
            CircuitElement::Measure(m) => m.into(),
        }
    }
}


impl From<&Loop> for GuiCircuitElement {
    fn from(l: &Loop) -> Self {
        let gui_circuit = l.circuit.clone().into();
        let gui_loop = GuiLoop{index:None,raw_circuit:false,gui_data:RefCell::new(GuiLoopData::default()),circuit:gui_circuit,stop_condition:l.stop_condition.clone()};
        return GuiCircuitElement::GuiLoop(gui_loop)
    }
}

impl From<Circuit> for GuiCircuitElement {
    fn from(c: Circuit) -> Self {
        let gui_loop = GuiLoop{index:None,raw_circuit:true,gui_data:RefCell::new(GuiLoopData::default()),circuit:c.into(),stop_condition:StopCondition::Once()};
        return GuiCircuitElement::GuiLoop(gui_loop)
    }
}

impl From<&Circuit> for GuiCircuitElement {
    fn from(c: &Circuit) -> Self {
        let gui_loop = GuiLoop{index:None,raw_circuit:true,gui_data:RefCell::new(GuiLoopData::default()),circuit:c.into(),stop_condition:StopCondition::Once()};
        return GuiCircuitElement::GuiLoop(gui_loop)
    }
}

impl From<&Gate> for GuiCircuitElement {
    fn from(g: &Gate) -> Self {
        let gui_gate = GuiGate{index:None,gui_data:RefCell::new(GuiGateData::default()),control_bits:g.control_bits.clone(),gate:g.gate};
        GuiCircuitElement::GuiGate(gui_gate)
    }
}

impl From<&Measure> for GuiCircuitElement {
    fn from(m: &Measure) -> Self {
        let gui_measure = GuiMeasure{index:None,gui_data:RefCell::new(GuiMeasureData::default()),target:m.qbit_target,id:m.id.clone()};
        GuiCircuitElement::GuiMeasure(gui_measure)
    }
}



//
//
// impl From<&Circuit> for GuiCircuit {
//     fn from(circuit: &Circuit) -> Self {
//         let elements = circuit.elements.iter().map(|i| {i.into()}).collect();
//         GuiCircuit{ gui_data:Default::default(),nb_qbits:circuit.nb_qbits, elements }
//     }
// }
// impl From<Circuit> for GuiCircuit {
//     fn from(circuit: Circuit) -> Self {
//         let elements = circuit.elements.iter().map(|i| {i.into()}).collect();
//         GuiCircuit{ gui_data:Default::default(),nb_qbits:circuit.nb_qbits, elements }
//     }
// }
// impl From<&Loop> for GuiCircuitElement {
//     fn from(l: &Loop) -> Self {
//         let gui_circuit = (&l.circuit).into();
//         GuiLoop{ gui_data:Default::default(),circuit:gui_circuit, stop_condition:l.stop_condition.clone()}.into()
//     }
// }
// impl From<&Gate> for GuiCircuitElement {
//     fn from(g: &Gate) -> Self {
//         GuiGate{ gui_data:Default::default(),gate:g.gate,control_bits:g.control_bits.clone()}.into()
//     }
// }
// impl From<&Measure> for GuiCircuitElement {
//     fn from(m: &Measure) -> Self {
//         GuiMeasure{ gui_data:Default::default(),id:m.id.clone(), target:m.qbit_target }.into()
//     }
// }
//
//
//
// impl From<GuiCircuit> for Circuit {
//     fn from(gc: GuiCircuit) -> Self {
//         todo!()
//     }
// }
// impl From<&GuiCircuit> for Circuit {
//     fn from(gc: &GuiCircuit) -> Self {
//         todo!()
//     }
// }
// impl From<&GuiCircuitElement> for CircuitElement {
//     fn from(element: &GuiCircuitElement) -> Self {
//         match element {
//             GuiCircuitElement::GuiLoop(l) => l.into(),
//             GuiCircuitElement::GuiGate(g) => g.into(),
//             GuiCircuitElement::GuiMeasure(m) => m.into(),
//         }
//     }
// }
// impl From<&GuiLoop> for CircuitElement {
//     fn from(l: &GuiLoop) -> Self {
//         Loop{circuit:(&l.circuit).into(),stop_condition:l.stop_condition.clone()}.into()
//     }
// }
// impl From<&GuiGate> for CircuitElement {
//     fn from(g: &GuiGate) -> Self {
//         Gate{gate:g.gate.clone(),control_bits:g.control_bits.clone()}.into()
//     }
// }
// impl From<&GuiMeasure> for CircuitElement {
//     fn from(m: &GuiMeasure) -> Self {
//         Measure{ qbit_target:m.target, id:m.id.clone()}.into()
//     }
// }
//


