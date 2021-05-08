use crate::circuit::QuantumCircuit;
use crate::state::State;
use crate::gate::GateOp;

pub struct QuantumComputer {
    nb_qbits:u8,
}

impl QuantumComputer {
    pub fn new_circuit(&self) -> QuantumCircuit {
        QuantumCircuit::new(self.nb_qbits)
    }

    pub fn zero_state(&self) -> State {
        State::zero(self.nb_qbits)
    }
    pub fn same_amplitude(&self, idx:&[usize]) -> State {
        State::same_amplitude(self.nb_qbits,idx)
    }

}

pub struct Executable<'a> {
    computer: &'a QuantumComputer,
    circuit: QuantumCircuit,
}


impl Executable<'_> {

    pub fn launch(&self, initial_state:&State) -> State {
        let mut current = State::from(initial_state);
        for gate in self.circuit.iter() {
            current = gate.apply(&current);
        };
        return current;
    }

}

impl QuantumComputer {
    pub fn new(nb_qbits:u8) -> Self {
        Self{nb_qbits}
    }

    pub fn compile(&self, circuit:&QuantumCircuit) -> Executable {
        Executable{computer:self,circuit:circuit.clone()}
    }

}