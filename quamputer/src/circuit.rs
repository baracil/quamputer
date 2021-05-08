
use crate::QDimension;
use std::ops::Deref;


use std::rc::Rc;
use crate::gate::QuantumOperation;

#[derive(Clone)]
pub struct QuantumCircuit {
    nb_qbits: u8,
    gates: Vec<Rc<dyn QuantumOperation>>,
}


impl Deref for QuantumCircuit {
    type Target = Vec<Rc<dyn QuantumOperation>>;

    fn deref(&self) -> &Self::Target {
        &self.gates
    }
}


impl QuantumCircuit {
    pub(crate) fn new(nb_qbits: u8) -> Self {
        return Self { nb_qbits, gates: Vec::with_capacity(10) };
    }

    pub fn push(&mut self, gate: impl QuantumOperation + 'static) -> &mut QuantumCircuit {
        self.push_safe(gate).unwrap()
    }

    pub fn push_safe(&mut self, gate: impl QuantumOperation + 'static) -> Result<&mut QuantumCircuit, &str> {
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