use crate::circuit::QuantumCircuit;
use crate::state::QuantumState;
use crate::executable::Executable;

pub struct QuantumComputer {
    nb_qbits:u8,
}

impl QuantumComputer {
    pub fn new(nb_qbits:u8) -> Self {
        Self{nb_qbits}
    }

    pub fn compile(&self, circuit:&QuantumCircuit) -> Executable {
        Executable::new(circuit)
    }

    pub fn new_circuit(&self) -> QuantumCircuit {
        QuantumCircuit::new(self.nb_qbits)
    }

    pub fn zero_state(&self) -> QuantumState {
        QuantumState::zero(self.nb_qbits)
    }

    pub fn same_amplitude(&self, qbit_idx:&[usize]) -> QuantumState {
        QuantumState::same_amplitude(self.nb_qbits, qbit_idx)
    }
}
