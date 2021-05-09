

use crate::gate::{QuantumOperation, ExecutionContext};


pub struct Measure {
    id: String,
    target: u8,
}

impl Measure {
    pub fn new(id: &str, target: u8) -> Self {
        Self { id: id.to_string(), target }
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
}