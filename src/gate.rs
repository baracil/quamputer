use std::collections::HashMap;
use std::ops::Sub;

use num_complex::Complex64;
use num_traits::One;

use crate::gate::Gate::{Hadamard, Not, Swap, X, Y, Z};
use crate::gate::State::{Measured, NotMeasured};
use crate::gate_op::hadamard::apply_controlled_hadamard;
use crate::gate_op::pauli::{apply_controlled_not, apply_controlled_pauli_x, apply_controlled_pauli_y, apply_controlled_pauli_z};
use crate::gate_op::swap::apply_controlled_swap;
use crate::state::QuantumState;
use serde::{Serialize,Deserialize};
use crate::operation::{CircuitElement};




#[derive(Copy, Clone, Serialize, Deserialize)]
pub enum GateWithoutControl {
    Not(u8),
    X(u8),
    Y(u8),
    Z(u8),
    Swap(u8, u8),
    Hadamard(u8),
}

impl GateWithoutControl {

    pub fn get_involved_qbits(&self, others: &[u8]) -> Vec<u8> {
        let mut result = Vec::new();
        result.extend_from_slice(others);
        match self {
            GateWithoutControl::Not(t) => result.push(*t),
            GateWithoutControl::X(t) => result.push(*t),
            GateWithoutControl::Y(t) => result.push(*t),
            GateWithoutControl::Z(t) => result.push(*t),
            GateWithoutControl::Swap(t1, t2) => {
                result.push(*t1);
                result.push(*t2);
            }
            GateWithoutControl::Hadamard(t) => result.push(*t),
        };
        result
    }
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
    Swap(u8, u8),
    Hadamard(u8),
    CNot(u8, [u8; 1]),
    Toffoli(u8, [u8; 2]),
    CSwap(u8, u8, [u8; 1]),
    Fredkin(u8, u8, [u8; 1]),
}

impl Into<crate::operation::Gate> for Gate {
    fn into(self) -> crate::operation::Gate {
        match self {
            Not(t) => crate::operation::Gate { gate: GateWithoutControl::Not(t), control_bits: vec![] },
            X(t) => crate::operation::Gate { gate: GateWithoutControl::X(t), control_bits: vec![] },
            Y(t) => crate::operation::Gate { gate: GateWithoutControl::Y(t), control_bits: vec![] },
            Z(t) => crate::operation::Gate { gate: GateWithoutControl::Z(t), control_bits: vec![] },
            Swap(t1, t2) => crate::operation::Gate { gate: GateWithoutControl::Swap(t1, t2), control_bits: vec![] },
            Hadamard(t) => crate::operation::Gate { gate: GateWithoutControl::Hadamard(t), control_bits: vec![] },
            Gate::CNot(t, c) => crate::operation::Gate { gate: GateWithoutControl::Not(t), control_bits: Vec::from(c) },
            Gate::Toffoli(t, c) => crate::operation::Gate { gate: GateWithoutControl::Not(t), control_bits: Vec::from(c) },
            Gate::CSwap(t1, t2, c) => crate::operation::Gate { gate: GateWithoutControl::Swap(t1, t2), control_bits: Vec::from(c) },
            Gate::Fredkin(t1, t2, c) => crate::operation::Gate { gate: GateWithoutControl::Swap(t1, t2), control_bits: Vec::from(c) },
        }
    }
}

impl Into<CircuitElement> for Gate {
    fn into(self) -> CircuitElement {
        return CircuitElement::Gate(self.into());
    }
}

impl From<&Gate> for crate::operation::Gate {
    fn from(gate: &Gate) -> Self {
        match gate {
            Not(t) => crate::operation::Gate { gate: GateWithoutControl::Not(*t), control_bits: vec![] },
            X(t) => crate::operation::Gate { gate: GateWithoutControl::X(*t), control_bits: vec![] },
            Y(t) => crate::operation::Gate { gate: GateWithoutControl::Y(*t), control_bits: vec![] },
            Z(t) => crate::operation::Gate { gate: GateWithoutControl::Z(*t), control_bits: vec![] },
            Swap(t1, t2) => crate::operation::Gate { gate: GateWithoutControl::Swap(*t1, *t2), control_bits: vec![] },
            Hadamard(t) => crate::operation::Gate { gate: GateWithoutControl::Hadamard(*t), control_bits: vec![] },
            Gate::CNot(t, c) => crate::operation::Gate { gate: GateWithoutControl::Not(*t), control_bits: Vec::from(*c) },
            Gate::Toffoli(t, c) => crate::operation::Gate { gate: GateWithoutControl::Not(*t), control_bits: Vec::from(*c) },
            Gate::CSwap(t1, t2, c) => crate::operation::Gate { gate: GateWithoutControl::Swap(*t1, *t2), control_bits: Vec::from(*c) },
            Gate::Fredkin(t1, t2, c) => crate::operation::Gate { gate: GateWithoutControl::Swap(*t1, *t2), control_bits: Vec::from(*c) },
        }
    }
}

///
/// Add some control qbits to a Gate.
/// For instance the Toffoli gate is obtained with
///
/// ```
/// use quamputer::gate::Gate::Not;
/// let toffoli = Not(2).with_two_controls(0,1);
/// ```
#[derive(Clone)]
pub struct ControlledGate {
    gate: GateWithoutControl,
    controls: Vec<u8>,
}

impl Into<CircuitElement> for ControlledGate {
    fn into(self) -> CircuitElement {
        let par = crate::operation::Gate { gate: self.gate, control_bits: self.controls };
        crate::operation::CircuitElement::Gate(par)
    }
}


pub(crate) fn check_for_no_duplicate(bits: Vec<u8>) -> Result<(), String> {
    if bits.len() <= 1 {
        return Ok(());
    }
    for i in 0..bits.len() - 1 {
        for j in i + 1..bits.len() {
            if bits[i] == bits[j] {
                return Err(format!("Duplicate qbit : {} ", bits[i]));
            }
        }
    }
    Ok(())
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
        let gate_par:crate::operation::Gate = self.into();
        let mut controls = gate_par.control_bits.clone();
        controls.push(control);
        ControlledGate { gate: gate_par.gate, controls }
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
        let gate_par:crate::operation::Gate = self.into();
        let mut controls = gate_par.control_bits.clone();
        controls.push(control1);
        controls.push(control2);
        ControlledGate { gate: gate_par.gate, controls }
    }


    /// Create a ControlledGate from this gate
    /// that uses multiple control qbits
    pub fn with_multi_control(&self, controls: &[u8]) -> ControlledGate {
        let gate_par:crate::operation::Gate = self.into();
        let mut c = gate_par.control_bits.clone();
        c.extend_from_slice(controls);
        ControlledGate { gate: gate_par.gate, controls:c }
    }

}

impl GateWithoutControl {
    pub fn max_qbit_idx(&self) -> u8 {
        match self {
            GateWithoutControl::Not(target) => *target,
            GateWithoutControl::X(target) => *target,
            GateWithoutControl::Y(target) => *target,
            GateWithoutControl::Z(target) => *target,
            GateWithoutControl::Hadamard(target) => *target,
            GateWithoutControl::Swap(target1, target2) => *target1.max(target2),
        }
    }

    pub(crate) fn apply_controlled(&self, control_qbits: &[u8], context: &mut ExecutionContext) {
        match self {
            GateWithoutControl::Not(target) => apply_controlled_not(control_qbits, *target, context),
            GateWithoutControl::X(target) => apply_controlled_pauli_x(control_qbits, *target, context),
            GateWithoutControl::Y(target) => apply_controlled_pauli_y(control_qbits, *target, context),
            GateWithoutControl::Z(target) => apply_controlled_pauli_z(control_qbits, *target, context),
            GateWithoutControl::Hadamard(target) => apply_controlled_hadamard(control_qbits, *target, context),
            GateWithoutControl::Swap(target1, target2) => apply_controlled_swap(control_qbits, *target1, *target2, context),
        }
    }

}

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

impl ExecutionContext {
    pub fn nb_qbits(&self) -> u8 {
        self.current_state.nb_qbits()
    }
}

