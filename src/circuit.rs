use crate::gate::ExecutionContext;
use crate::operation::{QOp, QuantumOperation};
use crate::state::QuantumState;
use serde::{Serialize,Deserialize};

#[derive(Clone, Serialize, Deserialize)]
pub struct Circuit {
    pub nb_qbits:u8,
    pub operations:Vec<QuantumOperation>,
}

pub struct Executable<'a>(pub &'a QuantumOperation);

impl Circuit {
    pub fn execute(&self, initial_state:&QuantumState) -> ExecutionContext {
        Execution(&self).execute(initial_state)
    }
}

struct Execution<'a>(pub &'a Circuit);

impl<'a> Execution<'a> {

    fn execute(&self, initial_state:&QuantumState) -> ExecutionContext {
        let mut context = ExecutionContext::initialize(&initial_state);
        self.0.apply(&mut context);
        return context;
    }

}