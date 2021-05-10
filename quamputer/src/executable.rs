use crate::circuit::QuantumCircuit;
use crate::state::QuantumState;

use crate::gate::ExecutionContext;

pub struct Executable<'a> {
    circuit: &'a QuantumCircuit,
}

impl<'a> Executable<'a> {

    pub fn new(circuit: &'a QuantumCircuit) -> Self {
        Self{circuit }
    }

    pub fn execute(&self, initial_state:&QuantumState) -> ExecutionContext {
        Execution::new(&self.circuit).execute(initial_state)
    }

}


pub struct Execution<'a> {
    circuit:&'a QuantumCircuit,
}

impl<'a> Execution<'a> {

    fn new(circuit:&'a QuantumCircuit) -> Execution<'a> {
        Self{circuit}
    }

    pub fn execute(&self, initial_state:&QuantumState) -> ExecutionContext {
        let mut context = ExecutionContext::initialize(&initial_state);
        for gate in self.circuit.iter() {
            gate.apply(&mut context);
        };
        return context;
    }

}