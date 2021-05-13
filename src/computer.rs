use crate::builder::{QuantumCircuitBuilder};
use crate::state::QuantumState;
use crate::executable::Executable;
use crate::operation::QuantumOperation;

pub struct QuantumComputer {
    nb_qbits:u8,
}

impl QuantumComputer {

    /// Create a new computer
    pub fn new(nb_qbits:u8) -> Self {
        Self{nb_qbits}
    }




    /// Create a new circuit builder to create
    /// circuit this computer can run
    pub fn new_circuit_builder(&self) -> QuantumCircuitBuilder {
        QuantumCircuitBuilder::new(self.nb_qbits)
    }

    /// Compile an executable that can be launch
    /// with a initial state
    pub fn compile<'a>(&self, circuit:&'a QuantumOperation) -> Executable<'a> {
        Executable(circuit)
    }


    pub fn zero_state(&self) -> QuantumState {
        QuantumState::zero(self.nb_qbits)
    }

    pub fn same_amplitude(&self, qbit_idx:&[usize]) -> QuantumState {
        QuantumState::same_amplitude(self.nb_qbits, qbit_idx)
    }
    pub fn nb_qbits(&self) -> u8 {
        self.nb_qbits
    }
}
