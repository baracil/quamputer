use serde::{Deserialize, Serialize};

use crate::execution::ExecutionContext;
use crate::base_gate::BaseGate;
use crate::operation::{CircuitElement, QuantumOperation};

#[derive(Clone, Serialize, Deserialize)]
pub struct Gate {
    gate: BaseGate,
    control_bits: Vec<u8>,
}

impl Gate {
    pub fn new(gate: BaseGate, control_bits: Vec<u8>) -> Self {
        Gate { gate, control_bits }
    }

    pub fn get_control_bits(&self) -> &Vec<u8> {
        &self.control_bits
    }

    pub fn get_gate(&self) -> BaseGate {
        self.gate
    }
}

impl Into<CircuitElement> for Gate {
    fn into(self) -> CircuitElement {
        CircuitElement::Gate(self)
    }
}

impl QuantumOperation for Gate {
    fn apply(&self, context: &mut ExecutionContext) {
        self.gate.apply_controlled(self.control_bits.as_slice(), context)
    }

    fn max_qbit_idx(&self) -> u8 {
        let max_qbit_idx = self.gate.max_qbit_idx();
        self.control_bits
            .iter()
            .max()
            .cloned()
            .unwrap_or(0)
            .max(max_qbit_idx)
    }

    fn check_validity(&self, nb_qbits: u8) -> Result<(), String> {
        let qbit_indices = self.gate.get_involved_qbits(self.control_bits.as_slice());
        for qbit_index in qbit_indices.iter() {
            if *qbit_index >= nb_qbits {
                return Err(format!("Index to high {}", qbit_index));
            }
        }
        check_for_no_duplicate(qbit_indices)
    }
}

fn check_for_no_duplicate(bits: Vec<u8>) -> Result<(), String> {
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
