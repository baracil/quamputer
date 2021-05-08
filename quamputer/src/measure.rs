use crate::gate::QuantumOperation;
use crate::state::QuantumState;
use crate::QDimension;

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

    fn apply(&self, input: &QuantumState) -> QuantumState {
        let select_state = Measure::pick_on_state(input);
        let output = QuantumState::same_amplitude(input.nb_qbits(), &[select_state]);
    }
}