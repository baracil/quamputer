use crate::gate::{QuantumOperation, ExecutionContext, MeasureCount};
use crate::state::QuantumState;
use crate::QDimension;
use crate::gate::State::MEASURED;

pub struct Measure {
    id:String,
    target:u8,
}

impl Measure {
    pub fn new(id:&str, target:u8) -> Self {
        Self{id:id.to_string(), target}
    }

}


impl Measure {

    fn pick_on_state(input: &QuantumState) -> usize {
        let mut target = 1.0-(rand::random::<f64>());
        for (index, amplitude) in input.iter().enumerate() {
            target -= amplitude.norm_sqr();
            if target<=0.0 {
                return index;
            }
        };
        input.len()-1
    }
}

impl QuantumOperation for Measure {
    fn max_qbit_idx(&self) -> u8 {
        self.target
    }

    fn apply(&self, context: &mut ExecutionContext)  {

        let mask = context.mask(self.target);

        let select_state = Measure::pick_on_state(&context.current_state);
        let output = QuantumState::same_amplitude(context.nb_qbits(), &[select_state]);

        context.current_state = output;
        context.state = MEASURED(select_state);

        let measured_one = (select_state&mask) == mask;

        match (measured_one,context.count.get_mut(&self.id)) {
            (true,Some(mc)) => mc.nb_one+=1,
            (false,Some(mc)) => mc.nb_zero+=1,
            (true, None) => {
                context.count.insert(self.id.clone(),MeasureCount{nb_zero:0,nb_one:1});
            },
            (false, None) => {
                context.count.insert(self.id.clone(),MeasureCount{nb_zero:1,nb_one:0});
            },
        }

    }
}