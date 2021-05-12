use crate::state::QuantumState;

use crate::gate::ExecutionContext;
use crate::operation::{QuantumOperation, QOp};

pub struct Executable<'a>(pub &'a QuantumOperation);

impl<'a> Executable<'a> {
    pub fn execute(&self, initial_state:&QuantumState) -> ExecutionContext {
        Execution(&self.0).execute(initial_state)
    }
}

struct Execution<'a>(pub &'a QuantumOperation);

impl<'a> Execution<'a> {

    fn execute(&self, initial_state:&QuantumState) -> ExecutionContext {
        let mut context = ExecutionContext::initialize(&initial_state);
        self.0.apply(&mut context);
        return context;
    }

}