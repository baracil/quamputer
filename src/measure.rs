use serde::{Deserialize, Serialize};

use crate::execution::ExecutionContext;
use crate::operation::{CircuitElement, QuantumOperation};

#[derive(Debug, Copy, Clone)]
pub struct MeasureCount {
    pub nb_zero: u32,
    pub nb_one: u32,
}

#[derive(Clone, Serialize, Deserialize)]
pub struct Measure {
    /// Uniq value used to identify the measurement
    pub id: String,
    /// the qbit to measure
    pub qbit_target: u8,
}

impl Into<CircuitElement> for Measure {
    fn into(self) -> CircuitElement {
        CircuitElement::Measure(self)
    }
}

impl QuantumOperation for Measure {
    fn apply(&self, context: &mut ExecutionContext) {
        let mask = context.mask(self.qbit_target);
        let select_state = context.pick_on_state();

        context.set_measurement(select_state);
        let measured_one = (select_state & mask) == mask;

        match measured_one {
            true => context.increase_one(&self.id),
            false => context.increase_zero(&self.id),
        }
    }

    fn max_qbit_idx(&self) -> u8 {
        self.qbit_target
    }

    fn check_validity(&self, nb_qbits: u8) -> Result<(), String> {
        if self.qbit_target >= nb_qbits {
            return Err(format!("Index to high {}", self.qbit_target));
        }
        Ok(())
    }
}
