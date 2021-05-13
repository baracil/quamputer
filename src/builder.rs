

use crate::operation::{MeasurePar, QOp};
use crate::operation::{CircuitPar, LoopPar, QuantumOperation};

use crate::operation::QuantumOperation::{Circuit};
use crate::condition::{Condition};

pub struct QuantumCircuitBuilder {
    nb_qbits: u8,
    operations: Vec<QuantumOperation>,
}

impl QuantumCircuitBuilder {

    pub (crate) fn new(nb_qbits: u8) -> Self {
        return Self {nb_qbits, operations:Vec::new()};
    }

    pub fn build(&self) -> Result<QuantumOperation, String> {
        let circuit = Circuit(CircuitPar{nb_qbit:self.nb_qbits,operations:self.operations.clone()});
        circuit.check_validity(self.nb_qbits).map(|()| circuit)
    }

    pub fn push_loop(&mut self, stop_condition:Condition, operation:QuantumOperation) -> &mut QuantumCircuitBuilder {
        self.apply(LoopPar{operation:Box::new(operation), stop_condition })
    }

    pub fn measure(&mut self, id:&str, target:u8) -> &mut QuantumCircuitBuilder {
        self.apply(MeasurePar{id:id.to_string(),target})
    }

    pub fn apply(&mut self, operation: impl Into<QuantumOperation>) -> &mut QuantumCircuitBuilder {
        self.operations.push(operation.into());
        self
    }
}

impl Into<QuantumOperation> for QuantumCircuitBuilder {
    fn into(self) -> QuantumOperation {
        self.build().unwrap()
    }
}
