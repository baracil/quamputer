use crate::gate::GateOp;
use crate::gate::gate::Gate;
use crate::QDimension;
use std::ops::Deref;

#[derive(Clone)]
pub struct QuantumCircuit {
    nb_qbits:u8,
    gates:Vec<Gate>,
}

impl Deref for QuantumCircuit {
    type Target = Vec<Gate>;

    fn deref(&self) -> &Self::Target {
        &self.gates
    }
}


impl QuantumCircuit {

    pub  fn new(nb_qbits:u8) -> Self {
        return Self{nb_qbits,gates:Vec::with_capacity(10)};
    }


    pub fn push(&mut self, gate:Gate) -> &mut QuantumCircuit {
        self.push_safe(gate).unwrap()
    }

    pub fn push_safe(&mut self, gate:Gate) -> Result<&mut QuantumCircuit,&str> {
        if gate.max_qbit_idx()>=self.nb_qbits {
            return Err("Invalid gate : some qbit indices are too high");
        }
        self.gates.push(gate);
        Ok(self)
    }
}

impl QDimension for QuantumCircuit {
    fn nb_qbits(&self) -> u8 {
        self.nb_qbits
    }
}