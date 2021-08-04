use std::borrow::{Borrow, BorrowMut};
use std::sync::{Arc, Mutex};

use druid::{Data, Env, Point, PaintCtx, RenderContext, Affine, Vec2};
use druid::env::Key;

use crate::circuit::Circuit;
use crate::gui::circuit_drawer::CIRCUIT_MARGIN;
use crate::gui::circuit_gui::CircuitElementGui::{Gate, Loop, Measure};
use crate::gui::gate_gui::GateGui;
use crate::gui::loop_gui::LoopGui;
use crate::gui::measure_gui::MeasureGui;
use crate::operation::CircuitElement;

pub trait GuiWidget {
    /// Update the width field of the gui element and return its value
    /// the width takes into account the margin
    fn update_layout(&mut self, env: &Env);

    fn full_width(&self) -> f64;

    fn paint(&self, ctx: &mut PaintCtx, env: &Env);
}


#[derive(Data, Clone)]
pub struct CircuitGui {
    pub margin: f64,
    pub width: f64,
    /// the number of qbits in this circuit
    pub nb_qbits: u8,
    /// the elements composing this circuit
    pub elements: Arc<Mutex<Vec<CircuitElementGui>>>,
}

pub enum CircuitElementGui {
    Loop(LoopGui),
    Gate(GateGui),
    Measure(MeasureGui),
}

impl From<Circuit> for CircuitGui {
    fn from(circuit: Circuit) -> Self {
        let elements = circuit.elements.iter().map(|ce| { CircuitElementGui::from(ce) }).collect::<Vec<CircuitElementGui>>();
        CircuitGui { width: 0., margin:0., nb_qbits: circuit.nb_qbits, elements: Arc::new(Mutex::new(elements)) }
    }
}

impl From<&Circuit> for CircuitGui {
    fn from(circuit: &Circuit) -> Self {
        let elements = circuit.elements.iter().map(|ce| { CircuitElementGui::from(ce) }).collect::<Vec<CircuitElementGui>>();
        CircuitGui { width: 0., margin:0., nb_qbits: circuit.nb_qbits, elements: Arc::new(Mutex::new(elements)) }
    }
}

impl From<&CircuitGui> for Circuit {
    fn from(circuit: &CircuitGui) -> Self {
        let elements = circuit.elements.lock().unwrap().iter().map(|ce| { ce.into() }).collect::<Vec<CircuitElement>>();
        Circuit { nb_qbits: circuit.nb_qbits, elements }
    }
}

impl GuiWidget for CircuitGui {
    fn update_layout(&mut self, env: &Env) {
        let width: f64 = self.elements
            .lock()
            .unwrap()
            .iter_mut()
            .map(|e| { e.update_layout(env);e.full_width() })
            .sum();

        let margin = env.get(CIRCUIT_MARGIN);
        self.margin = margin;
        self.width = width;
    }

    fn full_width(&self) -> f64 {
        self.width + 2.*self.margin
    }

    fn paint(&self, ctx: &mut PaintCtx, env: &Env) {
        ctx.with_save(|ctx| {
            for element in self.elements.lock().unwrap().iter() {
                element.paint(ctx, env);
                ctx.transform(Affine::translate(Vec2{x:element.full_width(),y:0.}));
            }
        });
    }
}

impl GuiWidget for CircuitElementGui {
    fn update_layout(&mut self, env: &Env) {
        match self {
            Loop(l) => l.update_layout(env),
            Gate(g) => g.update_layout(env),
            Measure(m) => m.update_layout(env)
        };
    }

    fn full_width(&self) -> f64 {
        match self {
            Loop(l) => l.full_width(),
            Gate(g) => g.full_width(),
            Measure(m) => m.full_width(),
        }
    }

    fn paint(&self, ctx: &mut PaintCtx, env: &Env) {
        match self {
            Loop(l) => l.paint(ctx,env),
            Gate(g) => g.paint(ctx,env),
            Measure(m) => m.paint(ctx,env),
        }
    }
}


impl From<&CircuitElement> for CircuitElementGui {
    fn from(element: &CircuitElement) -> Self {
        match element {
            CircuitElement::Loop(l) => Loop(l.into()),
            CircuitElement::Gate(g) => Gate(g.into()),
            CircuitElement::Measure(m) => Measure(m.into())
        }
    }
}

impl From<&CircuitElementGui> for CircuitElement {
    fn from(element: &CircuitElementGui) -> Self {
        match element {
            Loop(l) => CircuitElement::Loop(l.into()),
            Gate(g) => CircuitElement::Gate(g.into()),
            Measure(m) => CircuitElement::Measure(m.into()),
        }
    }
}

