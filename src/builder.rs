

use crate::operation::{Measure, QuantumOperation};
use crate::operation::{Loop, CircuitElement};

use crate::condition::{StopCondition};
use crate::circuit::Circuit;

pub struct QuantumCircuitBuilder {
    nb_qbits: u8,
    operations: Vec<CircuitElement>,
}

impl QuantumCircuitBuilder {

    pub (crate) fn new(nb_qbits: u8) -> Self {
        return Self {nb_qbits, operations:Vec::new()};
    }

    pub fn build(&self) -> Result<Circuit, String> {
        let circuit = Circuit{nb_qbits:self.nb_qbits,operations:self.operations.clone()};
        circuit.check_validity(self.nb_qbits).map(|()| circuit)
    }

    pub fn apply_sub_circuit(&mut self, circuit:impl Into<Circuit>, loop_condition: StopCondition) -> &mut QuantumCircuitBuilder {
        self.apply(Loop{circuit:circuit.into(), loop_condition})
    }

    pub fn measure(&mut self, id:&str, target:u8) -> &mut QuantumCircuitBuilder {
        self.apply(Measure{id:id.to_string(),target})
    }

    pub fn apply(&mut self, operation: impl Into<CircuitElement>) -> &mut QuantumCircuitBuilder {
        self.operations.push(operation.into());
        self
    }
}

impl From<QuantumCircuitBuilder> for Circuit {
    fn from(b: QuantumCircuitBuilder) -> Self {
        return b.build().unwrap()
    }
}

impl From<&mut QuantumCircuitBuilder> for Circuit {
    fn from(b: &mut QuantumCircuitBuilder) -> Self {
        return b.build().unwrap()
    }
}