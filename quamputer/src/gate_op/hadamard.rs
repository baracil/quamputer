use crate::{QDimension};
use crate::state::QuantumState;
use std::f64::consts::FRAC_1_SQRT_2;


use crate::gate::ExecutionContext;


pub fn apply_controlled_hadamard(control_qbits: &[u8], target: u8, context: &mut ExecutionContext) {
    let control_mask = context.control_mask(control_qbits);
    let mask = context.mask(target);
    let not_mask = !mask;

    let mut result = QuantumState::nil(context.nb_qbits());

    let len = context.nb_amplitudes();
    for src in 0..len {
        let amplitude = context.current_amplitude_at(src);
        let control_set = (src & control_mask) == control_mask;
        if control_set {
            let amplitude = amplitude * FRAC_1_SQRT_2;
            let without_bit = src & not_mask; //|n0m>
            let with_bit = src | mask; //|n1m>

            let bit_set = without_bit != src; // case |1> if set, |0> otherwise

            if bit_set {
                result[without_bit] += amplitude;
                result[with_bit] -= amplitude;
            } else {
                result[without_bit] += amplitude;
                result[with_bit] += amplitude;
            }
        } else {
            result[src] += amplitude;
        }
    };

    context.set_current_state(result)
}
