use crate::gate::{ExecutionContext, ControlledGate, check_for_no_duplicate, GateWithoutControl};
use crate::gate::Gate::{Swap, Not};
use crate::condition::{StopCondition};
use serde::{Serialize,Deserialize};
use crate::circuit::Circuit;


#[derive(Clone, Serialize, Deserialize)]
pub enum CircuitElement {
    Loop(Loop),
    Gate(Gate),
    Measure(Measure),
}

impl From<Loop> for CircuitElement {
    fn from(p: Loop) -> Self {
        CircuitElement::Loop(p)
    }
}

impl From<Gate> for CircuitElement {
    fn from(p: Gate) -> Self {
        CircuitElement::Gate(p)
    }
}
impl From<Measure> for CircuitElement {
    fn from(p: Measure) -> Self {
        CircuitElement::Measure(p)
    }
}


pub fn cnot(target:u8, control:u8) -> ControlledGate {
    Not(target).with_one_control(control)
}
pub fn toffoli(target:u8, control1:u8, control2:u8) -> ControlledGate {
    Not(target).with_two_controls(control1, control2)
}
pub fn cswap(target1:u8, target2:u8, control:u8) -> ControlledGate {
    Swap(target1,target2).with_one_control(control)
}
pub fn fredkin(target1:u8, target2:u8, control:u8) -> ControlledGate {
    Swap(target1,target2).with_one_control(control)
}


#[derive(Clone, Serialize, Deserialize)]
pub struct Measure {
    pub id:String,
    pub target:u8,
}

#[derive(Clone, Serialize, Deserialize)]
pub struct Loop {
    pub circuit:Circuit,
    pub stop_condition: StopCondition,
}


#[derive(Clone, Serialize, Deserialize)]
pub struct Gate {
    pub gate:GateWithoutControl,
    pub control_bits:Vec<u8>,
}

impl Gate {
    pub fn new(gate:GateWithoutControl, control_bits:Vec<u8>) -> Self {
        Gate{
            gate,
            control_bits
        }
    }
}

impl QuantumOperation for Measure {
    fn max_qbit_idx(&self) -> u8 {
        self.target
    }

    fn apply(&self, context: &mut ExecutionContext) {
        let mask = context.mask(self.target);
        let select_state = context.pick_on_state();

        context.set_measurement(select_state);
        let measured_one = (select_state & mask) == mask;

        match measured_one {
            true => context.increase_one(&self.id),
            false => context.increase_zero(&self.id),
        }
    }

    fn check_validity(&self, nb_qbits:u8) -> Result<(),String> {
        if self.target >= nb_qbits {
            return Err(format!("Index to high {}",self.target))
        }
        Ok(())
    }
}

impl QuantumOperation for  Circuit {
    fn max_qbit_idx(&self) -> u8 {
        self.nb_qbits-1
    }

    fn apply(&self, context: &mut ExecutionContext) {
        self.elements.iter().for_each(|op| op.apply(context))
    }

    fn check_validity(&self, nb_qbits:u8) -> Result<(), String> {
        for operation in self.elements.iter() {
            let op_validity = operation.check_validity(nb_qbits);
            if op_validity.is_err() {
                return op_validity;
            }
        }
        Ok(())
    }

}

impl QuantumOperation for Loop {
    fn max_qbit_idx(&self) -> u8 {
        self.circuit.max_qbit_idx()
    }
    fn apply(&self, context: &mut ExecutionContext) {
        let mut i = 0;
        while !(self.stop_condition.is_end_of_loop(i, &context)) {
            self.circuit.apply(context);
            i+=1;
        }
    }
    fn check_validity(&self, nb_qbits:u8) -> Result<(), String> {
        self.circuit.check_validity(nb_qbits)
    }
}

impl QuantumOperation for Gate {
    fn max_qbit_idx(&self) -> u8 {
        let max_qbit_idx = self.gate.max_qbit_idx();
        self.control_bits
            .iter()
            .max()
            .cloned()
            .unwrap_or(0)
            .max(max_qbit_idx)
    }

    fn apply(&self, context: &mut ExecutionContext) {
        self.gate.apply_controlled(self.control_bits.as_slice(),context)
    }

    fn check_validity(&self, nb_qbits:u8) -> Result<(), String> {
        let qbit_indices = self.gate.get_involved_qbits(self.control_bits.as_slice());
        for qbit_index in qbit_indices.iter() {
            if *qbit_index >= nb_qbits {
                return Err(format!("Index to high {}",qbit_index))
            }
        }
        check_for_no_duplicate(qbit_indices)
    }
}


impl QuantumOperation for CircuitElement {

    /// Return the maximal index of the qbits
    /// involved in this gate operation
    /// Used to check if the gate operation
    /// can be used with a given quantum computer
    fn max_qbit_idx(&self) -> u8 {
        match self {
            CircuitElement::Loop(p) => p.max_qbit_idx(),
            CircuitElement::Gate(p) => p.max_qbit_idx(),
            CircuitElement::Measure(p) => p.max_qbit_idx()
        }
    }

    /// Apply the current gate operation to the provided state
    /// and return the result.
    fn apply(&self, context: &mut ExecutionContext) {
        match self {
            CircuitElement::Loop(p) => p.apply(context),
            CircuitElement::Gate(p) => p.apply(context),
            CircuitElement::Measure(p) => p.apply(context)
        }
    }

    fn check_validity(&self, nb_qbits:u8) -> Result<(), String> {
        match self {
            CircuitElement::Loop(p) => p.check_validity(nb_qbits),
            CircuitElement::Gate(p) => p.check_validity(nb_qbits),
            CircuitElement::Measure(p) => p.check_validity(nb_qbits)
        }
    }
}

pub trait QuantumOperation {
    fn max_qbit_idx(&self) -> u8;
    fn apply(&self, context: &mut ExecutionContext);
    fn check_validity(&self, nb_qbits:u8) -> Result<(), String>;
}