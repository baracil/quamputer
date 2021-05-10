use crate::gate::Gate::{Not, X, Hadamard, Swap, Z, Y, CNot, Toffoli, CSwap, Fredkin};
use crate::state::QuantumState;
use crate::QDimension;
use std::collections::HashMap;
use crate::gate::State::{NotMeasured, Measured};

use crate::gate_op::pauli::{apply_controlled_pauli_x, apply_controlled_pauli_y, apply_controlled_pauli_z, apply_controlled_not};
use crate::gate_op::hadamard::apply_controlled_hadamard;
use crate::gate_op::swap::apply_controlled_swap;
use num_complex::{Complex64};
use std::ops::{Sub, Mul};
use num_traits::{One, MulAdd};


pub trait QuantumOperation {
    /// Return the maximal index of the qbits
    /// involved in this gate operation
    /// Used to check if the gate operation
    /// can be used with a given quantum computer
    fn max_qbit_idx(&self) -> u8;

    /// Apply the current gate operation to the provided state
    /// and return the result.
    fn apply(&self, context: &mut ExecutionContext);
}


///
/// Gate without any control qbits.
///
#[derive(Copy, Clone)]
pub enum Gate {
    Not(u8),
    X(u8),
    Y(u8),
    Z(u8),
    Swap(u8,u8),
    Hadamard(u8),
    CNot(u8, u8),
    Toffoli(u8, u8,u8),
    CSwap(u8, u8, u8),
    Fredkin(u8,u8, u8),
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
        ControlledGate { gate: self.clone(), controls: vec![control] }
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
        ControlledGate { gate: self.clone(), controls: vec![control1, control2] }
    }

    /// Create a ControlledGate from this gate
    /// that uses multiple control qbits
    pub fn with_multi_control(&self, controls: &[u8]) -> ControlledGate {
        ControlledGate { gate: self.clone(), controls: Vec::from(controls) }
    }

    fn apply_controlled(&self, control_qbits: &[u8], context: &mut ExecutionContext) {
        match self {
            Not(target) => apply_controlled_not(control_qbits, *target, context),
            X(target) => apply_controlled_pauli_x(control_qbits, *target, context),
            Y(target) => apply_controlled_pauli_y(control_qbits, *target, context),
            Z(target) => apply_controlled_pauli_z(control_qbits, *target, context),
            Hadamard(target) => apply_controlled_hadamard(control_qbits, *target, context),
            Swap(target1, target2) => apply_controlled_swap(control_qbits, *target1, *target2, context),
            CNot(target, control) => apply_controlled_not(&[*control], *target, context),
            Toffoli(target, control1, control2) => apply_controlled_not(&[*control1, *control2], *target, context),
            CSwap(target1, target2, control) => apply_controlled_swap(&[*control], *target1, *target2, context),
            Fredkin(target1, target2, control) => apply_controlled_swap(&[*control], *target1, *target2, context)
        }
    }
}


impl QuantumOperation for Gate {
    fn max_qbit_idx(&self) -> u8 {
        match self {
            Not(target) => *target,
            X(target) => *target,
            Y(target) => *target,
            Z(target) => *target,
            Hadamard(target) => *target,
            Swap(target1, target2) => *target1.max(target2),
            CNot(target, control) => *target.max(control),
            Toffoli(target, control1, control2) => *target.max(&control1).max(control2),
            CSwap(target1,target2,control) => *target1.max(target2).max(control),
            Fredkin(target1,target2,control) => *target1.max(target2).max(control),
        }
    }

    fn apply(&self, state: &mut ExecutionContext) {
        return self.apply_controlled(&[], state);
    }
}

impl QuantumOperation for ControlledGate {
    fn max_qbit_idx(&self) -> u8 {
        let max_qbit_gate = self.gate.max_qbit_idx();
        let max_qbit_control = self.controls.iter().max().cloned().unwrap_or(0);
        return max_qbit_gate.max(max_qbit_control);
    }

    fn apply(&self, input: &mut ExecutionContext) {
        self.gate.apply_controlled(self.controls.as_slice(), input)
    }
}


// pub fn cnot(control: u8, target: u8) -> ControlledGate {
//     Not(target).with_one_control(control)
// }
//
// pub fn toffoli(control1: u8, control2: u8, target: u8) -> ControlledGate {
//     Not(target).with_two_controls(control1, control2)
// }
//
// pub fn cswap(control:u8, target1:u8,target2:u8) -> ControlledGate {
//     Swap(target1,target2).with_one_control(control)
// }
// pub fn fredkin(control:u8, target1:u8,target2:u8) -> ControlledGate {
//     cswap(control,target1,target2)
// }

pub enum State {
    Measured(usize),
    NotMeasured,
}

#[derive(Debug, Copy, Clone)]
pub struct MeasureCount {
    pub nb_zero: u32,
    pub nb_one: u32,
}

pub struct ExecutionContext {
    current_state: QuantumState,
    state: State,
    count: HashMap<String, MeasureCount>,
}

impl ExecutionContext {
    pub(crate) fn increase_zero(&mut self, id: &String) {
        self.increase_count(id, |c| c.nb_zero += 1)
    }

    pub(crate) fn increase_one(&mut self, id: &String) {
        self.increase_count(id, |c| c.nb_one += 1)
    }

    fn increase_count(&mut self, id: &String, action: fn(&mut MeasureCount) -> ()) {
        match self.count.get_mut(id) {
            Some(c) => (action)(c),
            None => {
                let mut count = MeasureCount { nb_one: 0, nb_zero: 0 };
                (action)(&mut count);
                self.count.insert(id.clone(), count);
            }
        }
    }

    pub(crate) fn set_measurement(&mut self, select_state: usize) {
        let mut output = QuantumState::nil(self.nb_qbits());
        output[select_state] = Complex64::one();

        self.current_state = output;
        self.state = Measured(select_state);
    }

    pub(crate) fn pick_on_state(&self) -> usize {
        let mut target = 1.0 - (rand::random::<f64>());
        for (index, amplitude) in self.current_state.iter().enumerate() {
            target -= amplitude.norm_sqr();
            if target <= 0.0 {
                return index;
            }
        };
        self.current_state.len() - 1
    }

    pub fn current_state(&self) -> &QuantumState {
        &self.current_state
    }

    pub(crate) fn current_amplitude_at(&self, idx: usize) -> Complex64 {
        self.current_state[idx]
    }

    pub(crate) fn _norm_of_diff(&self, idx: usize, reference: Complex64) -> f64 {
        self.current_state[idx].sub(reference).norm()
    }

    pub(crate) fn set_current_state(&mut self, new_state: QuantumState) {
        self.current_state = new_state;
        self.state = NotMeasured;
    }

    pub(crate) fn initialize(initial_state: &QuantumState) -> Self {
        Self { current_state: QuantumState::from(initial_state), state: NotMeasured, count: HashMap::new() }
    }

    pub(crate) fn mask(&self, qbit_idx: u8) -> usize {
        self.current_state.mask(qbit_idx)
    }

    pub(crate) fn control_mask(&self, control_qbits: &[u8]) -> usize {
        self.current_state.control_mask(control_qbits)
    }
    pub(crate) fn nb_amplitudes(&self) -> usize {
        self.current_state.len()
    }


    pub fn get_count(&self, variable: &str) -> Option<&MeasureCount> {
        self.count.get(variable)
    }

    pub fn get_nb_zero(&self, variable: &str) -> u32 {
        match self.count.get(variable) {
            Some(c) => c.nb_zero,
            None => 0
        }
    }

    pub fn get_nb_one(&self, variable: &str) -> u32 {
        match self.count.get(variable) {
            Some(c) => c.nb_one,
            None => 0
        }
    }
}

impl QDimension for ExecutionContext {
    fn nb_qbits(&self) -> u8 {
        self.current_state.nb_qbits()
    }
}

