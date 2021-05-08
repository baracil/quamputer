use crate::QDimension;
use std::ops::Deref;


use std::rc::{Rc, Weak};
use crate::gate::{QuantumOperation, ExecutionContext};
use std::collections::VecDeque;
use std::borrow::Borrow;

#[derive(Clone)]
pub struct QuantumCircuit {
    nb_qbits: u8,
    operations: Vec<Rc<dyn QuantumOperation>>,
}

pub struct QuantumLoop {
    circuit:QuantumCircuit,
    nb_iterations:u32,
}

impl Deref for QuantumCircuit {
    type Target = Vec<Rc<dyn QuantumOperation>>;

    fn deref(&self) -> &Self::Target {
        &self.operations
    }
}

impl QuantumOperation for QuantumCircuit {
    fn max_qbit_idx(&self) -> u8 {
        return self.nb_qbits - 1;
    }

    fn apply(&self, context: &mut ExecutionContext) {
        self.operations.iter().for_each(|op| op.apply(context))
    }
}

impl QuantumOperation for QuantumLoop {
    fn max_qbit_idx(&self) -> u8 {
        self.circuit.max_qbit_idx()
    }

    fn apply(&self, context: &mut ExecutionContext) {
        for i in 0..self.nb_iterations {
            self.circuit.apply(context)
        }
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
    nb_iterations: u32,
}

impl QLoopData {
    fn build_loop(&self) -> QuantumLoop {
        QuantumLoop {circuit: QuantumCircuit { nb_qbits: self.nb_qbits, operations: self.operations.clone() }, nb_iterations:self.nb_iterations}
    }
}



impl QuantumCircuitBuilder {
    pub(crate) fn new(nb_qbits: u8) -> Self {
        return Self {nb_qbits, operations:Vec::new(),loops:VecDeque::new()};
    }

    pub fn build(&mut self) -> Result<QuantumCircuit, &str> {
        if !self.loops.is_empty() {
            return Err("Some loops have not been closed");
        }
        Ok(QuantumCircuit{nb_qbits:self.nb_qbits,operations:self.operations.clone()})
    }

    pub fn start_loop(&mut self, nb_iterations: u32) -> &mut QuantumCircuitBuilder {
        let loop_data = QLoopData { nb_qbits: self.nb_qbits, operations: Vec::with_capacity(10), nb_iterations };
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
            (None,_) => Err("No more loop to end".to_string())
        }
    }

    pub fn push(&mut self, gate: impl QuantumOperation + 'static) -> &mut QuantumCircuitBuilder {
        self.push_safe(gate).unwrap()
    }

    pub fn push_safe(&mut self, operation: impl QuantumOperation + 'static) -> Result<&mut QuantumCircuitBuilder, &str> {
        if operation.max_qbit_idx() >= self.nb_qbits {
            return Err("Invalid gate : some qbit indices are too high");
        }
        match self.loops.back_mut() {
            Some(loop_data) => loop_data.operations.push(Rc::new(operation)),
            None => self.operations.push(Rc::new(operation))
        }
        Ok(self)
    }
}


// pub trait QLoopStarter {
//     fn start_loop(&mut self) -> QuantumCircuitLoop<Self>;
// }
//
// impl<'a> QLoopStarter for QuantumCircuitBuilder {
//     fn start_loop(&'a mut self) -> QuantumCircuitLoop<'a, Self> {
//         self.start_loop()
//     }
// }


impl QDimension for QuantumCircuit {
    fn nb_qbits(&self) -> u8 {
        self.nb_qbits
    }
}