use crate::gate::gate::Gate::{Not, X, CNot, Toffoli, Hadamard};
use crate::gate::GateOp;
use crate::state::State;
use crate::gate::operations::{apply_not_gate, apply_cnot_gate, apply_toffoli_gate, apply_hadamard_gate};

#[derive(Copy, Clone)]
pub enum Gate {
    Not(u8),
    X(u8),
    // Y(u8),
    // Z(u8),
    CNot {control:u8,target:u8},
    Toffoli {control1:u8, control2:u8, target:u8},
    Hadamard(u8)
}


impl Gate {

    pub fn max_qbit_idx(&self) -> u8 {
        match self {
            Not(target) => *target,
            X(target) => *target,
            CNot {control,target} => *target.max(control),
            Toffoli {control1,control2,target}=> *target.max(control1).max(control2),
            Hadamard(target) => *target
        }
    }
}

impl GateOp for Gate {

    fn apply(&self, state: &State)  -> State {
        match self {
            Not(target) => apply_not_gate(*target, state),
            X(target) => apply_not_gate(*target, state),
            CNot {control, target} => apply_cnot_gate(*control,*target, state),
            Toffoli{control1,control2, target} => apply_toffoli_gate(*control1, *control2,*target, state),
            Hadamard(target) => apply_hadamard_gate(*target,state)

        }
    }

}


pub fn hadamard(target:u8) -> Gate {
    Hadamard(target)
}

pub fn pauli_x(target:u8) -> Gate {
    X(target)
}

pub fn cnot(control: u8, target: u8) -> Gate {
    CNot { control, target}
}

pub fn toffoli(control1: u8, control2: u8, target: u8) -> Gate {
    Toffoli { control1, control2, target}
}
