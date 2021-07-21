use serde::{Deserialize, Serialize};

use crate::execution::ExecutionContext;
use crate::gate::Gate;
use crate::gate_op::hadamard::apply_controlled_hadamard;
use crate::gate_op::pauli::{apply_controlled_not, apply_controlled_pauli_x, apply_controlled_pauli_y, apply_controlled_pauli_z};
use crate::gate_op::swap::apply_controlled_swap;
use crate::operation::CircuitElement;

#[derive(Copy, Clone, Serialize, Deserialize)]
pub enum GateWithoutControl {
    Not(u8),
    X(u8),
    Y(u8),
    Z(u8),
    Swap(u8, u8),
    Hadamard(u8),
    // Todo
    // Phase(f64,u8),
    // S(u8),
    // T(u8),
    // Rx(f64,u8)
    // Ry(f64,u8)
    // Rz(f64,u8)
}

impl Into<Gate> for GateWithoutControl {
    fn into(self) -> Gate {
        Gate::new(self, vec![])
    }
}

impl Into<CircuitElement> for GateWithoutControl {
    fn into(self) -> CircuitElement {
        CircuitElement::Gate(self.into())
    }
}

impl GateWithoutControl {
    /// Create a [`Gate`] from this gate
    /// that uses only one control qbit
    ///
    /// # Examples
    ///
    /// ```
    /// use quamputer::gate_without_control::GateWithoutControl::Not;
    /// let not = Not(2); // create a Not Gate on qbit(2)
    /// let cnot = not.with_one_control(0); // create a CNot gate. Control is qbit(0) and target qbit(2)
    /// ```
    ///
    pub fn with_one_control(self, control: u8) -> Gate {
        Gate::new(self, vec![control])
    }

    /// Create a ControlledGate from this gate
    /// that uses two control qbits (like Toffoli)
    ///
    /// # Examples
    ///
    /// ```
    /// use quamputer::gate_without_control::GateWithoutControl::Not;
    /// let not = Not(2); // create a Not Gate on qbit(2)
    /// let toffoli = not.with_two_controls(0,1); // create a Toffoli
    /// ```
    pub fn with_two_controls(self, control1: u8, control2: u8) -> Gate {
        Gate::new(self, vec![control1, control2])
    }


    /// Create a ControlledGate from this gate
    /// that uses multiple control qbits
    pub fn with_multi_control(self, controls: &[u8]) -> Gate {
        Gate::new(self, controls.to_vec())
    }

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
            GateWithoutControl::Not(target) => apply_controlled_not(*target, control_qbits, context),
            GateWithoutControl::X(target) => apply_controlled_pauli_x(*target, control_qbits, context),
            GateWithoutControl::Y(target) => apply_controlled_pauli_y(*target, control_qbits, context),
            GateWithoutControl::Z(target) => apply_controlled_pauli_z(*target, control_qbits, context),
            GateWithoutControl::Hadamard(target) => apply_controlled_hadamard(control_qbits, *target, context),
            GateWithoutControl::Swap(target1, target2) => apply_controlled_swap(control_qbits, *target1, *target2, context),
        }
    }
}
