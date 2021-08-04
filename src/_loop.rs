use serde::{Deserialize, Serialize};

use crate::circuit::Circuit;
use crate::condition::StopCondition;
use crate::execution::ExecutionContext;
use crate::operation::{CircuitElement, QuantumOperation};

#[derive(Clone, Serialize, Deserialize)]
pub struct Loop {
    /// the circuit that makes the content of the loop
    pub(crate) circuit: Circuit,
    /// the condition used to stop the loop
    pub(crate) stop_condition: StopCondition,
}

impl Loop {
    pub fn new(circuit:impl Into<Circuit>, stop_condition:&StopCondition) -> Self {
        Loop{circuit:circuit.into(), stop_condition:stop_condition.clone()}
    }
}

impl Into<CircuitElement> for Loop {
    fn into(self) -> CircuitElement {
        CircuitElement::Loop(self)
    }
}

impl QuantumOperation for Loop {
    fn apply(&self, context: &mut ExecutionContext) {
        let mut i = 0;
        while !(self.stop_condition.is_end_of_loop(i, &context)) {
            self.circuit.apply(context);
            i += 1;
        }
    }
    fn max_qbit_idx(&self) -> u8 {
        self.circuit.max_qbit_idx()
    }
    fn check_validity(&self, nb_qbits: u8) -> Result<(), String> {
        self.circuit.check_validity(nb_qbits)
    }
}
