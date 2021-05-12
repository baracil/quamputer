use crate::gate::{ExecutionContext, ControlledGate, check_for_no_duplicate, GateWithoutControl};
use crate::gate::Gate::{Swap, Not};


use crate::operation::QuantumOperation::{Circuit, Loop, Measure, Gate};
use crate::condition::{EndOfLoopPredicate, Condition};
use serde::{Serialize,Deserialize};


#[derive(Clone, Serialize, Deserialize)]
pub enum QuantumOperation {
    Circuit(CircuitPar),
    Loop(LoopPar),
    Gate(GatePar),
    Measure(MeasurePar),
}

impl From<CircuitPar> for QuantumOperation {
    fn from(p: CircuitPar) -> Self {
        Circuit(p)
    }
}
impl From<LoopPar> for QuantumOperation {
    fn from(p: LoopPar) -> Self {
        Loop(p)
    }
}
impl From<GatePar> for QuantumOperation {
    fn from(p: GatePar) -> Self {
        Gate(p)
    }
}
impl From<MeasurePar> for QuantumOperation {
    fn from(p: MeasurePar) -> Self {
        Measure(p)
    }
}

impl QuantumOperation {
    pub fn to_string(&self) -> serde_json::error::Result<String> {
        serde_json::to_string(self)
    }

    pub fn from_string(serialized_operation:&str) -> serde_json::error::Result<Self> {
        serde_json::from_str::<QuantumOperation>(serialized_operation)
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
pub struct MeasurePar {
    pub id:String,
    pub target:u8,
}

#[derive(Clone, Serialize, Deserialize)]
pub struct CircuitPar {
    pub nb_qbit:u8,
    pub operations:Vec<QuantumOperation>,
}

#[derive(Clone, Serialize, Deserialize)]
pub struct LoopPar {
    pub operation:Box<QuantumOperation>,
    pub stop_condition:Condition,
}


#[derive(Clone, Serialize, Deserialize)]
pub struct GatePar {
    pub gate:GateWithoutControl,
    pub control_bits:Vec<u8>
}



impl QOp for  MeasurePar {
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

impl QOp for  CircuitPar {
    fn max_qbit_idx(&self) -> u8 {
        self.nb_qbit-1
    }

    fn apply(&self, context: &mut ExecutionContext) {
        self.operations.iter().for_each(|op| op.apply(context))
    }

    fn check_validity(&self, nb_qbits:u8) -> Result<(), String> {
        for operation in self.operations.iter() {
            let op_validity = operation.check_validity(nb_qbits);
            if op_validity.is_err() {
                return op_validity;
            }
        }
        Ok(())
    }

}

impl QOp for LoopPar {
    fn max_qbit_idx(&self) -> u8 {
        self.operation.max_qbit_idx()
    }
    fn apply(&self, context: &mut ExecutionContext) {
        let mut i = 0;
        while !(self.stop_condition.is_end_of_loop(i, &context)) {
            self.operation.apply(context);
            i+=1;
        }
    }
    fn check_validity(&self, nb_qbits:u8) -> Result<(), String> {
        self.operation.check_validity(nb_qbits)
    }
}

impl QOp for GatePar {
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


impl QOp for QuantumOperation {

    /// Return the maximal index of the qbits
    /// involved in this gate operation
    /// Used to check if the gate operation
    /// can be used with a given quantum computer
    fn max_qbit_idx(&self) -> u8 {
        match self {
            QuantumOperation::Circuit(p) => p.max_qbit_idx(),
            QuantumOperation::Loop(p) => p.max_qbit_idx(),
            QuantumOperation::Gate(p) => p.max_qbit_idx(),
            QuantumOperation::Measure(p) => p.max_qbit_idx()
        }
    }

    /// Apply the current gate operation to the provided state
    /// and return the result.
    fn apply(&self, context: &mut ExecutionContext) {
        match self {
            QuantumOperation::Circuit(p) => p.apply(context),
            QuantumOperation::Loop(p) => p.apply(context),
            QuantumOperation::Gate(p) => p.apply(context),
            QuantumOperation::Measure(p) => p.apply(context)
        }
    }

    fn check_validity(&self, nb_qbits:u8) -> Result<(), String> {
        match self {
            QuantumOperation::Circuit(p) => p.check_validity(nb_qbits),
            QuantumOperation::Loop(p) => p.check_validity(nb_qbits),
            QuantumOperation::Gate(p) => p.check_validity(nb_qbits),
            QuantumOperation::Measure(p) => p.check_validity(nb_qbits)
        }
    }
}

pub trait QOp {
    fn max_qbit_idx(&self) -> u8;
    fn apply(&self, context: &mut ExecutionContext);
    fn check_validity(&self, nb_qbits:u8) -> Result<(), String>;
}