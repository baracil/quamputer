use std::collections::VecDeque;
use std::ops::Deref;
use std::rc::Rc;

use quick_xml::DeError;
use quick_xml::se::to_string;

use crate::gate::ExecutionContext;
use crate::operation::{CircuitPar, Condition, LoopPar, QuantumOperation};
use crate::operation::QuantumOperation::{Circuit, Loop};
use crate::QDimension;

pub struct QuantumCircuitBuilder {
    nb_qbits: u8,
    operations: Vec<QuantumOperation>,
    loops: VecDeque<QLoopData>,
}

struct QLoopData {
    nb_qbits: u8,
    operations: Vec<QuantumOperation>,
    end_condition: Condition,
}

impl QLoopData {
    fn build_loop(&self) -> QuantumOperation {
        let operation = Circuit(CircuitPar{nb_qbit:self.nb_qbits, operations: self.operations.clone()});
        Loop(LoopPar {operation:Box::new(operation),end_condition:self.end_condition.clone()})
    }
}



impl QuantumCircuitBuilder {

    pub(crate) fn new(nb_qbits: u8) -> Self {
        return Self {nb_qbits, operations:Vec::new(),loops:VecDeque::new()};
    }

    pub fn build(&mut self) -> Result<QuantumOperation, String> {
        if !self.loops.is_empty() {
            return Err("Some loops have not been closed".to_string());
        }
        let circuit = Circuit(CircuitPar{nb_qbit:self.nb_qbits,operations:self.operations.clone()});
        Ok(circuit)
    }

    pub fn start_loop(&mut self, nb_iterations: u32) -> &mut QuantumCircuitBuilder {
        self.start_advanced_loop(todo!())
    }

    pub fn start_advanced_loop(&mut self, condition: Condition) -> &mut QuantumCircuitBuilder
    {
        let loop_data = QLoopData { nb_qbits: self.nb_qbits, operations: Vec::with_capacity(10), end_condition: condition};
        self.loops.push_back(loop_data);
        self
    }

    pub fn end_loop(&mut self) -> Result<&mut QuantumCircuitBuilder,String> {
        let data = self.loops.pop_back().map(|d| d.build_loop());
        match (data, self.loops.back_mut()) {
            (Some(circuit), Some(outer_loop)) => {
                outer_loop.operations.push(circuit);
                Ok(self)
            }
            (Some(circuit), None) => {
                self.operations.push(circuit);
                Ok(self)
            }
            (None,_) => Err("No more loop to end".to_owned())
        }
    }

    pub fn push(&mut self, operation: impl Into<QuantumOperation>) -> Result<&mut QuantumCircuitBuilder, String> {
        let operation = operation.into();
        if operation.max_qbit_idx() >= self.nb_qbits {
            return Err("Invalid operation : some qbit indices are too high".to_string());
        }

        let valid = operation.check_validity();
        if valid.is_err() {
            return Err(valid.err().unwrap());
        }

        match self.loops.back_mut() {
            Some(loop_data) => loop_data.operations.push(operation),
            None => self.operations.push(operation)
        }
        Ok(self)
    }
}

