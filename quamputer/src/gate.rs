use crate::gate::Gate::{Not, X, Hadamard};
use crate::state::QuantumState;
use crate::operations::{apply_controlled_not, apply_controlled_hadamard};
use crate::QDimension;
use std::collections::HashMap;
use crate::gate::State::NOT_MEASURED;
use crate::measure::Measure;

pub trait QuantumOperation {

    /// Return the maximal index of the qbits
    /// involved in this gate operation
    /// Used to check if the gate operation
    /// can be used with a given quantum computer
    fn max_qbit_idx(&self) -> u8;

    /// Apply the current gate operation to the provided state
    /// and return the result.
    fn apply(&self, context:&mut ExecutionContext);

}

pub enum State {
    MEASURED(usize),
    NOT_MEASURED
}

#[derive(Debug,Copy, Clone)]
pub struct MeasureCount {
    pub nb_zero:u32,
    pub nb_one:u32,
}

pub struct ExecutionContext {
    pub current_state:QuantumState,
    pub state:State,
    pub count:HashMap<String,MeasureCount>
}

impl ExecutionContext {

    pub (crate) fn initialize(initial_state:&QuantumState) -> Self {
        Self{current_state:QuantumState::from(initial_state), state:NOT_MEASURED, count: HashMap::new()}
    }

    pub (crate) fn mask(&self, qbit_idx: u8) -> usize {
        self.current_state.mask(qbit_idx)
    }

    pub (crate) fn control_mask(&self, control_qbits: &[u8]) -> usize {
        self.current_state.control_mask(control_qbits)
    }
    pub (crate) fn nb_amplitudes(&self) -> usize {
        self.current_state.len()
    }

}

impl QDimension for ExecutionContext {
    fn nb_qbits(&self) -> u8 {
        self.current_state.nb_qbits()
    }
}


///
/// Gate without any control qbits.
///
#[derive(Copy, Clone)]
pub enum Gate {
    Not(u8),
    X(u8),
    // Y(u8),
    // Z(u8),
    Hadamard(u8),
}

///
/// Add some control qbits to a Gate.
/// For instance the Toffoli gate is obtained with
///
/// ```
/// use quamputer::gate::Gate::Not;
/// let toffoli = Not(2).with_two_controls(0,1);
/// ```
pub struct ControlledGate {
    gate: Gate,
    controls: Vec<u8>,
}

impl Gate {

    /// Create a ControlledGate from this gate
    /// that uses only one control qbit
    ///
    /// # Examples
    ///
    /// ```
    /// use quamputer::gate::Gate::Not;
    /// let not = Not(2); // create a Not Gate on qbit(2)
    /// let cnot = not.with_one_control(0); // create a CNot gate. Control is qbit(0) and target qbit(2)
    /// let toffoli = not.with_two_controls(0,1); // create a Toffoli
    /// ```
    pub fn with_one_control(&self, control: u8) -> ControlledGate {
        ControlledGate { gate: self.clone(), controls:vec![control] }
    }

    /// Create a ControlledGate from this gate
    /// that uses two control qbits (like Toffoli)
    ///
    /// # Examples
    ///
    /// ```
    /// use quamputer::gate::Gate::Not;
    /// let not = Not(2); // create a Not Gate on qbit(2)
    /// let toffoli = not.with_two_controls(0,1); // create a Toffoli gate
    /// ```
    pub fn with_two_controls(&self, control1: u8, control2: u8) -> ControlledGate {
        ControlledGate { gate: self.clone(), controls:vec![control1,control2] }
    }

    /// Create a ControlledGate from this gate
    /// that uses multiple control qbits
    pub fn with_multi_control(&self, controls: &[u8]) -> ControlledGate {
        ControlledGate { gate: self.clone(), controls:Vec::from(controls) }
    }

    fn apply_controlled(&self, control_qbits:&[u8], context:&mut ExecutionContext) {
        match self {
            Not(target) => apply_controlled_not(control_qbits, *target, context),
            X(target) => apply_controlled_not(control_qbits, *target, context),
            Hadamard(target) => apply_controlled_hadamard(control_qbits, *target, context)
        }
    }

}


impl QuantumOperation for Gate {

    fn max_qbit_idx(&self) -> u8 {
        match self {
            Not(target) => *target,
            X(target) => *target,
            Hadamard(target) => *target
        }
    }

    fn apply(&self, state: &mut ExecutionContext)  {
        return self.apply_controlled(&[],state);
    }
}

impl QuantumOperation for ControlledGate {

    fn max_qbit_idx(&self) -> u8 {
        let max_qbit_gate = self.gate.max_qbit_idx();
        let max_qbit_control = self.controls.iter().max().cloned().unwrap_or(0);
        return max_qbit_gate.max(max_qbit_control);
    }

    fn apply(&self, input: &mut ExecutionContext) {
        self.gate.apply_controlled(self.controls.as_slice(),input)
    }
}



pub fn cnot(control: u8, target: u8) -> ControlledGate {
    Not(target).with_one_control(control)
}

pub fn toffoli(control1: u8, control2: u8, target: u8) -> ControlledGate {
    Not(target).with_two_controls(control1, control2)
}
