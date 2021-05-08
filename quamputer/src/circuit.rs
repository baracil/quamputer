use crate::gate::GateOp;
use crate::gate::gate::Gate;
use crate::QDimension;
use std::ops::Deref;
use std::borrow::Borrow;
use std::os::linux::raw::time_t;
use std::rc::Rc;

#[derive(Clone)]
pub struct QuantumCircuit {
    nb_qbits: u8,
    gates: Vec<Rc<dyn GateOp>>,
}


impl Deref for QuantumCircuit {
    type Target = Vec<Rc<dyn GateOp>>;

    fn deref(&self) -> &Self::Target {
        &self.gates
    }
}


impl QuantumCircuit {
    pub fn new(nb_qbits: u8) -> Self {
        return Self { nb_qbits, gates: Vec::with_capacity(10) };
    }

    pub fn push(&mut self, gate: impl GateOp + 'static) -> &mut QuantumCircuit {
        self.push_safe(gate).unwrap()
    }

    pub fn push_safe(&mut self, gate: impl GateOp + 'static) -> Result<&mut QuantumCircuit, &str> {
        if gate.max_qbit_idx() >= self.nb_qbits {
            return Err("Invalid gate : some qbit indices are too high");
        }
        self.gates.push(Rc::new(gate));
        Ok(self)
    }
}

impl QDimension for QuantumCircuit {
    fn nb_qbits(&self) -> u8 {
        self.nb_qbits
    }
}