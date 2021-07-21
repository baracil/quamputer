use crate::gate::Gate;
use crate::gate_without_control::GateWithoutControl;
use crate::operation::CircuitElement;
use crate::standard_gate::StandardGate::{CNot, CSwap, Fredkin, Hadamard, Not, Swap, Toffoli, X, Y, Z};

///
/// Gate without any control qbits.
///
#[derive(Copy, Clone)]
pub enum StandardGate {
    Not(u8),
    X(u8),
    Y(u8),
    Z(u8),
    Swap(u8, u8),
    Hadamard(u8),
    CNot(u8, [u8; 1]),
    Toffoli(u8, [u8; 2]),
    CSwap(u8, u8, [u8; 1]),
    Fredkin(u8, u8, [u8; 1]),
}


impl Into<Gate> for StandardGate {
    fn into(self) -> Gate {
        match self {
            Not(t) => crate::gate::Gate::new(GateWithoutControl::Not(t), vec![]),
            X(t) => crate::gate::Gate::new(GateWithoutControl::X(t), vec![]),
            Y(t) => crate::gate::Gate::new(GateWithoutControl::Y(t), vec![]),
            Z(t) => crate::gate::Gate::new(GateWithoutControl::Z(t), vec![]),
            Swap(t1, t2) => crate::gate::Gate::new(GateWithoutControl::Swap(t1, t2), vec![]),
            Hadamard(t) => crate::gate::Gate::new(GateWithoutControl::Hadamard(t), vec![]),
            CNot(t, c) => crate::gate::Gate::new(GateWithoutControl::Not(t), Vec::from(c)),
            Toffoli(t, c) => crate::gate::Gate::new(GateWithoutControl::Not(t), Vec::from(c)),
            CSwap(t1, t2, c) => crate::gate::Gate::new(GateWithoutControl::Swap(t1, t2), Vec::from(c)),
            Fredkin(t1, t2, c) => crate::gate::Gate::new(GateWithoutControl::Swap(t1, t2), Vec::from(c)),
        }
    }
}

impl Into<CircuitElement> for StandardGate {
    fn into(self) -> CircuitElement {
        return CircuitElement::Gate(self.into());
    }
}

