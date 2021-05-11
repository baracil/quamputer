use crate::state::QuantumState;

use crate::gate::ExecutionContext;
use crate::operation::{QuantumOperation, QOp};

pub struct Executable<'a> {
    operation: &'a QuantumOperation,
}

impl<'a> Executable<'a> {

    pub fn new(operation: &'a QuantumOperation) -> Self {
        Self{operation }
    }

    pub fn execute(&self, initial_state:&QuantumState) -> ExecutionContext {
        Execution::new(&self.operation).execute(initial_state)
    }

}

pub struct Execution<'a> {
    operation:&'a QuantumOperation,
}

impl<'a> Execution<'a> {

    fn new(operation:&'a QuantumOperation) -> Execution<'a> {
        Self{operation}
    }

    pub fn execute(&self, initial_state:&QuantumState) -> ExecutionContext {
        let mut context = ExecutionContext::initialize(&initial_state);
        self.operation.apply(&mut context);
        return context;
    }

}