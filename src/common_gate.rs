use crate::gate::Gate;
use crate::base_gate::BaseGate;
use crate::operation::CircuitElement;
use crate::common_gate::CommonGate::{CNot, CSwap, Fredkin, Hadamard, Not, Swap, Toffoli, X, Y, Z, CCNot};

///
/// Gate without any control qbits.
///
#[derive(Copy, Clone)]
pub enum CommonGate {
    Not(u8),
    X(u8),
    Y(u8),
    Z(u8),
    Swap(u8, u8),
    Hadamard(u8),
    CNot(u8, [u8; 1]),
    Toffoli(u8, [u8; 2]),
    CCNot(u8, [u8; 2]),
    CSwap(u8, u8, [u8; 1]),
    Fredkin(u8, u8, [u8; 1]),
}


impl Into<Gate> for CommonGate {
    fn into(self) -> Gate {
        match self {
            Not(t) => crate::gate::Gate::new(BaseGate::Not(t), vec![]),
            X(t) => crate::gate::Gate::new(BaseGate::X(t), vec![]),
            Y(t) => crate::gate::Gate::new(BaseGate::Y(t), vec![]),
            Z(t) => crate::gate::Gate::new(BaseGate::Z(t), vec![]),
            Swap(t1, t2) => crate::gate::Gate::new(BaseGate::Swap(t1, t2), vec![]),
            Hadamard(t) => crate::gate::Gate::new(BaseGate::Hadamard(t), vec![]),
            CNot(t, c) => crate::gate::Gate::new(BaseGate::Not(t), Vec::from(c)),
            Toffoli(t, c) => crate::gate::Gate::new(BaseGate::Not(t), Vec::from(c)),
            CCNot(t, c) => crate::gate::Gate::new(BaseGate::Not(t), Vec::from(c)),
            CSwap(t1, t2, c) => crate::gate::Gate::new(BaseGate::Swap(t1, t2), Vec::from(c)),
            Fredkin(t1, t2, c) => crate::gate::Gate::new(BaseGate::Swap(t1, t2), Vec::from(c)),
        }
    }
}

impl Into<CircuitElement> for CommonGate {
    fn into(self) -> CircuitElement {
        return CircuitElement::Gate(self.into());
    }
}

