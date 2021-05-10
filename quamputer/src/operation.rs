use crate::gate::{ExecutionContext, Gate, check_for_no_duplicate, ControlledGate, MeasureCount};

pub trait EndOfLoopPredicate {
    fn is_end_of_loop(&self, nb_iterations:u32, context: &ExecutionContext) -> bool ;
}


#[derive(Clone)]
pub enum Condition {
    MaxIteration(u32),
    MaxZeroSampling(String,u32),
    MaxOneSample(String,u32),
}

impl EndOfLoopPredicate for Condition {
    fn is_end_of_loop(&self, nb_iterations: u32, context: &ExecutionContext) -> bool {
        match self {
            Condition::MaxIteration(nb) => nb_iterations>=*nb,
            Condition::MaxZeroSampling(id, nb) => context.get_nb_zero(id)>=*nb,
            Condition::MaxOneSample(id, nb) => context.get_nb_one(id)>=*nb
        }
    }
}

#[derive(Clone)]
pub enum QuantumOperation {
    Circuit(CircuitPar),
    Loop(LoopPar),
    Gate(Gate),
    ControlledGate(ControlledGate),
    Measure(MeasurePar),
}

#[derive(Clone)]
pub struct MeasurePar {
    pub id:String,
    pub target:u8,
}

#[derive(Clone)]
pub struct CircuitPar {
    pub nb_qbit:u8,
    pub operations:Vec<QuantumOperation>,
}

#[derive(Clone)]
pub struct LoopPar {
    pub operation:Box<QuantumOperation>,
    pub end_condition:Condition,
}

impl MeasurePar {
    pub fn max_qbit_idx(&self) -> u8 {
        self.target
    }

    pub fn apply(&self, context: &mut ExecutionContext) {
        let mask = context.mask(self.target);
        let select_state = context.pick_on_state();

        context.set_measurement(select_state);
        let measured_one = (select_state & mask) == mask;

        match measured_one {
            true => context.increase_one(&self.id),
            false => context.increase_zero(&self.id),
        }
    }

    pub fn check_validity(&self) -> Result<(),String> {
        Ok(())
    }
}

impl CircuitPar {
    pub fn apply(&self, context: &mut ExecutionContext) {
        self.operations.iter().for_each(|op| op.apply(context))
    }

    pub fn max_qbit_idx(&self) -> u8 {
        self.nb_qbit-1
    }

    pub fn check_validity(&self) -> Result<(), String> {
        for operation in self.operations.iter() {
            let op_validity = operation.check_validity();
            if op_validity.is_err() {
                return op_validity;
            }
        }
        Ok(())
    }

}

impl LoopPar {
    pub fn apply(&self, context: &mut ExecutionContext) {
        let mut i = 0;
        while !(self.end_condition.is_end_of_loop(i,&context)) {
            self.operation.apply(context);
            i+=1;
        }
    }
    pub fn max_qbit_idx(&self) -> u8 {
        self.operation.max_qbit_idx()
    }
    pub fn check_validity(&self) -> Result<(), String> {
        self.operation.check_validity()
    }

}

// impl GatePar {
//     pub fn apply(&self, context: &mut ExecutionContext) {
//         self.gate.apply_controlled(self.control_bits,context);
//     }
//     pub fn max_qbit_idx(&self) -> u8 {
//         self.gate.get_involved_qbits(self.control_bits).iter().max().cloned().unwrap_or(0)
//     }
//     pub fn check_validity(&self) -> Result<(), String> {
//         check_for_no_duplicate(self.gate.get_involved_qbits(self.control_bits))
//     }
// }



impl QuantumOperation {
    /// Return the maximal index of the qbits
    /// involved in this gate operation
    /// Used to check if the gate operation
    /// can be used with a given quantum computer
    pub fn max_qbit_idx(&self) -> u8 {
        match self {
            QuantumOperation::Circuit(p) => p.max_qbit_idx(),
            QuantumOperation::Loop(p) => p.max_qbit_idx(),
            QuantumOperation::Gate(p) => p.max_qbit_idx(),
            QuantumOperation::ControlledGate(p) => p.max_qbit_idx(),
            QuantumOperation::Measure(p) => p.max_qbit_idx()
        }
    }

    /// Apply the current gate operation to the provided state
    /// and return the result.
    pub fn apply(&self, context: &mut ExecutionContext) {
        match self {
            QuantumOperation::Circuit(p) => p.apply(context),
            QuantumOperation::Loop(p) => p.apply(context),
            QuantumOperation::Gate(p) => p.apply(context),
            QuantumOperation::ControlledGate(p) => p.apply( context),
            QuantumOperation::Measure(p) => p.apply(context)
        }
    }

    pub fn check_validity(&self) -> Result<(), String> {
        match self {
            QuantumOperation::Circuit(p) => p.check_validity(),
            QuantumOperation::Loop(p) => p.check_validity(),
            QuantumOperation::Gate(p) => p.check_validity(),
            QuantumOperation::ControlledGate(p) => p.check_validity(),
            QuantumOperation::Measure(p) => p.check_validity()
        }
    }
}
