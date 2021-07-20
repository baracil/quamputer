use std::ops::Sub;

use num_complex::Complex64;
use num_traits::One;
use serde::{Deserialize, Serialize};

use crate::gate_op::hadamard::apply_controlled_hadamard;
use crate::gate_op::pauli::{apply_controlled_not, apply_controlled_pauli_x, apply_controlled_pauli_y, apply_controlled_pauli_z};
use crate::gate_op::swap::apply_controlled_swap;
use crate::gate_without_control::GateWithoutControl;
use crate::operation::CircuitElement;
use crate::state::QuantumState;

#[derive(Clone, Serialize, Deserialize)]
pub struct Gate {
    pub gate:GateWithoutControl,
    pub control_bits:Vec<u8>,
}

impl Gate {
    pub fn new(gate:GateWithoutControl, control_bits:Vec<u8>) -> Self {
        Gate{ gate, control_bits}
    }
}

