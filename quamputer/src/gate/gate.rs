use crate::gate::gate::Gate::{Not, X, Hadamard};
use crate::gate::GateOp;
use crate::state::State;
use crate::gate::operations::{apply_controlled_not, apply_controlled_hadamard};

#[derive(Copy, Clone)]
pub enum Gate {
    Not(u8),
    X(u8),
    // Y(u8),
    // Z(u8),
    Hadamard(u8),
}

pub struct ControlledGate {
    gate: Gate,
    controls: Vec<u8>,
}

impl Gate {

    pub fn single_control(&self, control: u8) -> ControlledGate {
        ControlledGate { gate: self.clone(), controls:vec![control] }
    }

    pub fn bi_control(&self, control1: u8, control2: u8) -> ControlledGate {
        ControlledGate { gate: self.clone(), controls:vec![control1,control2] }
    }

    pub fn multi_control(&self, controls: &[u8]) -> ControlledGate {
        ControlledGate { gate: self.clone(), controls:Vec::from(controls) }
    }

    fn apply_controlled(&self, control_qbits:&[u8], state:&State) -> State {
        match self {
            Not(target) => apply_controlled_not(control_qbits,*target,state),
            X(target) => apply_controlled_not(control_qbits,*target,state),
            Hadamard(target) => apply_controlled_hadamard(control_qbits,*target,state)
        }
    }

}


impl GateOp for Gate {

    fn max_qbit_idx(&self) -> u8 {
        match self {
            Not(target) => *target,
            X(target) => *target,
            Hadamard(target) => *target
        }
    }

    fn apply(&self, state: &State) -> State {
        return self.apply_controlled(&[],state);
    }
}

impl GateOp for ControlledGate {

    fn max_qbit_idx(&self) -> u8 {
        let max_qbit_gate = self.gate.max_qbit_idx();
        return self.controls
            .iter()
            .reduce(|i1, i2| i1.max(i2))
            .map(|i| max_qbit_gate.max(*i))
            .unwrap_or(max_qbit_gate);
    }

    fn apply(&self, input: &State) -> State {
        self.gate.apply_controlled(self.controls.as_slice(),input)
    }
}


pub fn hadamard(target: u8) -> Gate {
    Hadamard(target)
}

pub fn pauli_x(target: u8) -> Gate {
    X(target)
}

pub fn cnot(control: u8, target: u8) -> ControlledGate {
    Not(target).single_control(control)
}

pub fn toffoli(control1: u8, control2: u8, target: u8) -> ControlledGate {
    Not(target).bi_control(control1,control2)
}
