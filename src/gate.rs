



use serde::{Deserialize, Serialize};




use crate::gate_without_control::GateWithoutControl;



#[derive(Clone, Serialize, Deserialize)]
pub struct Gate {
    pub gate:GateWithoutControl,
    pub control_bits:Vec<u8>,
}

impl Gate {
    pub fn new(gate:GateWithoutControl, control_bits:Vec<u8>) -> Self {
        Gate{ gate, control_bits}
    }
}

