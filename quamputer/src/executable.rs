use crate::circuit::QuantumCircuit;
use crate::state::QuantumState;

pub struct Executable {
    circuit: QuantumCircuit,
}

impl Executable {

    pub fn new(circuit:&QuantumCircuit) -> Self {
        Self{circuit:circuit.clone()}
    }

    pub fn execute(&self, initial_state:&QuantumState) -> QuantumState {
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

    pub fn execute(&self, initial_state:&QuantumState) -> QuantumState {
        let mut current = QuantumState::from(initial_state);
        for gate in self.circuit.iter() {
            current = gate.apply(&current);
        };
        return current;
    }

}