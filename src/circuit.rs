use serde::{Deserialize, Serialize};

use crate::execution::ExecutionContext;
use crate::operation::{CircuitElement, QuantumOperation};
use crate::state::QuantumState;

#[derive(Clone, Serialize, Deserialize)]
pub struct Circuit {
    pub nb_qbits: u8,
    pub elements: Vec<CircuitElement>,
}


impl Circuit {
    pub fn execute(&self, initial_state: &QuantumState) -> ExecutionContext {
        let mut context = ExecutionContext::initialize(&initial_state);
        self.apply(&mut context);
        return context;
    }

    pub fn to_string(&self) -> serde_json::error::Result<String> {
        serde_json::to_string(self)
    }

    pub fn from_string(serialized_operation: &str) -> serde_json::error::Result<Self> {
        serde_json::from_str::<Circuit>(serialized_operation)
    }
}


impl QuantumOperation for Circuit {
    fn apply(&self, context: &mut ExecutionContext) {
        self.elements.iter().for_each(|op| op.apply(context))
    }

    fn max_qbit_idx(&self) -> u8 {
        self.nb_qbits - 1
    }

    fn check_validity(&self, nb_qbits: u8) -> Result<(), String> {
        for operation in self.elements.iter() {
            let op_validity = operation.check_validity(nb_qbits);
            if op_validity.is_err() {
                return op_validity;
            }
        }
        Ok(())
    }
}
