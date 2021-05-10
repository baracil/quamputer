use crate::QDimension;
use std::ops::Deref;


use std::rc::{Rc};
use std::collections::{VecDeque};

use crate::gate::{QuantumOperation, ExecutionContext};

use quick_xml::se::to_string;
use quick_xml::DeError;

#[derive(Clone)]
pub struct QuantumCircuit {
    nb_qbits: u8,
    operations: Vec<Rc<dyn QuantumOperation>>,
}

pub struct QuantumLoop {
    circuit:QuantumCircuit,
    predicate: Rc<dyn Fn(u32,&ExecutionContext) -> bool>,
}

impl Deref for QuantumCircuit {
    type Target = Vec<Rc<dyn QuantumOperation>>;

    fn deref(&self) -> &Self::Target {
        &self.operations
    }
}


impl QuantumCircuit {

    pub fn to_xml(&self) -> Result<String, DeError> {
        todo!()
    }

}

impl QuantumOperation for QuantumCircuit {
    fn max_qbit_idx(&self) -> u8 {
        return self.nb_qbits - 1;
    }

    fn apply(&self, context: &mut ExecutionContext) {
        self.operations.iter().for_each(|op| op.apply(context))
    }

    fn check_validity(&self) -> Result<(),String> {
        for operation in self.operations.iter() {
            let op_validity = operation.check_validity();
            if op_validity.is_err() {
                return op_validity;
            }
        }
        Ok(())
    }
}

impl QuantumOperation for QuantumLoop {
    fn max_qbit_idx(&self) -> u8 {
        self.circuit.max_qbit_idx()
    }

    fn apply(&self, context: &mut ExecutionContext) {
        let i = 0;
        while !(self.predicate)(i,&context) {
            self.circuit.apply(context)
        }
    }

    fn check_validity(&self) -> Result<(),String> {
        self.circuit.check_validity()
    }
}



pub struct QuantumCircuitBuilder {
    nb_qbits: u8,
    operations: Vec<Rc<dyn QuantumOperation>>,
    loops: VecDeque<QLoopData>,
}

struct QLoopData {
    nb_qbits: u8,
    operations: Vec<Rc<dyn QuantumOperation>>,
    predicate: Rc<dyn Fn(u32,&ExecutionContext) -> bool>,
}

impl QLoopData {
    fn build_loop(&self) -> QuantumLoop {
        QuantumLoop {circuit: QuantumCircuit { nb_qbits: self.nb_qbits, operations: self.operations.clone() }, predicate:self.predicate.clone()}
    }
}



impl QuantumCircuitBuilder {
    pub(crate) fn new(nb_qbits: u8) -> Self {
        return Self {nb_qbits, operations:Vec::new(),loops:VecDeque::new()};
    }

    pub fn build(&mut self) -> Result<QuantumCircuit, String> {
        if !self.loops.is_empty() {
            return Err("Some loops have not been closed".to_string());
        }
        Ok(QuantumCircuit{nb_qbits:self.nb_qbits,operations:self.operations.clone()})
    }

    pub fn start_loop(&mut self, nb_iterations: u32) -> &mut QuantumCircuitBuilder {
        self.start_advanced_loop(move |i,_m|  i>=nb_iterations)
    }

    pub fn start_advanced_loop<F>(&mut self, predicate: F) -> &mut QuantumCircuitBuilder
        where  F : Fn(u32,&ExecutionContext) -> bool + 'static
    {
        let loop_data = QLoopData { nb_qbits: self.nb_qbits, operations: Vec::with_capacity(10), predicate: Rc::new(predicate)};
        self.loops.push_back(loop_data);
        self
    }

    pub fn end_loop(&mut self) -> Result<&mut QuantumCircuitBuilder,String> {
        let data = self.loops.pop_back().map(|d| d.build_loop());
        match (data, self.loops.back_mut()) {
            (Some(circuit), Some(outer_loop)) => {
                outer_loop.operations.push(Rc::new(circuit));
                Ok(self)
            }
            (Some(circuit), None) => {
                self.operations.push(Rc::new(circuit));
                Ok(self)
            }
            (None,_) => Err("No more loop to end".to_owned())
        }
    }

    pub fn push(&mut self, operation: impl QuantumOperation + 'static) -> Result<&mut QuantumCircuitBuilder, String> {
        if operation.max_qbit_idx() >= self.nb_qbits {
            return Err("Invalid operation : some qbit indices are too high".to_string());
        }

        let valid = operation.check_validity();
        if valid.is_err() {
            return Err(valid.err().unwrap());
        }

        match self.loops.back_mut() {
            Some(loop_data) => loop_data.operations.push(Rc::new(operation)),
            None => self.operations.push(Rc::new(operation))
        }
        Ok(self)
    }
}


impl QDimension for QuantumCircuit {
    fn nb_qbits(&self) -> u8 {
        self.nb_qbits
    }
}