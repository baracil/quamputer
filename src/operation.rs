use serde::{Deserialize, Serialize};

use crate::_loop::Loop;
use crate::execution::ExecutionContext;
use crate::gate::Gate;
use crate::measure::Measure;

#[derive(Clone, Serialize, Deserialize)]
pub enum CircuitElement {
    Loop(Loop),
    Gate(Gate),
    Measure(Measure),
}


pub trait QuantumOperation {
    /// Apply the quantum operation by using the provided context
    fn apply(&self, context: &mut ExecutionContext);
    fn max_qbit_idx(&self) -> u8;
    fn check_validity(&self, nb_qbits: u8) -> Result<(), String>;
}


/// Dispatch trait to value of all variants of CircuitElement
impl QuantumOperation for CircuitElement {
    /// Return the maximal index of the qbits
    /// involved in this gate operation
    /// Used to check if the gate operation
    /// can be used with a given quantum computer
    fn max_qbit_idx(&self) -> u8 {
        match self {
            CircuitElement::Loop(p) => p.max_qbit_idx(),
            CircuitElement::Gate(p) => p.max_qbit_idx(),
            CircuitElement::Measure(p) => p.max_qbit_idx()
        }
    }

    /// Apply the current gate operation to the provided state
    /// and return the result.
    fn apply(&self, context: &mut ExecutionContext) {
        match self {
            CircuitElement::Loop(p) => p.apply(context),
            CircuitElement::Gate(p) => p.apply(context),
            CircuitElement::Measure(p) => p.apply(context)
        }
    }

    fn check_validity(&self, nb_qbits: u8) -> Result<(), String> {
        match self {
            CircuitElement::Loop(p) => p.check_validity(nb_qbits),
            CircuitElement::Gate(p) => p.check_validity(nb_qbits),
            CircuitElement::Measure(p) => p.check_validity(nb_qbits)
        }
    }
}

