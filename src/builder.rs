

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
        let circuit = Circuit{nb_qbits:self.nb_qbits, elements:self.operations.clone()};
        circuit.check_validity(self.nb_qbits).map(|()| circuit)
    }

    pub fn add_loop(&mut self, circuit:impl Into<Circuit>, loop_condition: StopCondition) -> &mut QuantumCircuitBuilder {
        self.add_operation(Loop{circuit:circuit.into(), stop_condition: loop_condition })
    }

    pub fn add_measure(&mut self, id:&str, target:u8) -> &mut QuantumCircuitBuilder {
        self.add_operation(Measure{id:id.to_string(),target})
    }

    pub fn add_operation(&mut self, operation: impl Into<CircuitElement>) -> &mut QuantumCircuitBuilder {
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