use crate::state::QuantumState;
use crate::gate::ExecutionContext;
use crate::QDimension;

pub fn apply_controlled_swap(control_qbits: &[u8], target1: u8, target2: u8, context: &mut ExecutionContext) {
    let control_mask = context.control_mask(control_qbits);
    let mask1 = context.mask(target1);
    let mask2 = context.mask(target2);
    let not_mask = !(mask1 | mask2);

    let mut result = QuantumState::nil(context.nb_qbits());
    let len = context.nb_amplitudes();
    for src in 0..len {
        let control_set = (src & control_mask) == control_mask;

        let target = if control_set {
            let target1_mask = if (src & mask1) != 0 { mask2 } else { 0 };
            let target2_mask = if (src & mask2) != 0 { mask1 } else { 0 };
            (src & not_mask) | target2_mask | target1_mask
        } else {
            src
        };
        result[target] = context.current_amplitude_at(src);
    }
    context.set_current_state(result);
}


#[cfg(test)]
mod tests_not {
    use std::ops::{Sub};

    use num_complex::{Complex64};
    use num_traits::identities::One;
    use num_traits::Zero;

    use crate::state::QuantumState;
    use crate::gate::ExecutionContext;
    
    use super::*;

    #[test]
    fn swap_test_on_00() {
        let mut context = ExecutionContext::initialize(&QuantumState::same_amplitude(2, &[0]));
        apply_controlled_swap(&[], 0, 1, &mut context);

        assert!(context.norm_of_diff(0, Complex64::one()) < 1e-6);
        assert!(context.norm_of_diff(1, Complex64::zero()) < 1e-6);
        assert!(context.norm_of_diff(2, Complex64::zero()) < 1e-6);
        assert!(context.norm_of_diff(3, Complex64::zero()) < 1e-6);
    }

    #[test]
    fn swap_test_on_01() {
        let mut context = ExecutionContext::initialize(&QuantumState::same_amplitude(2, &[1]));
        apply_controlled_swap(&[], 0, 1, &mut context);

        assert!(context.norm_of_diff(0,Complex64::zero()) < 1e-6);
        assert!(context.norm_of_diff(1,Complex64::zero()) < 1e-6);
        assert!(context.norm_of_diff(2,Complex64::one()) < 1e-6);
        assert!(context.norm_of_diff(3,Complex64::zero()) < 1e-6);
    }

    #[test]
    fn swap_test_on_10() {
        let mut context = ExecutionContext::initialize(&QuantumState::same_amplitude(2, &[2]));
        apply_controlled_swap(&[], 0, 1, &mut context);

        assert!(context.norm_of_diff(0,Complex64::zero()) < 1e-6);
        assert!(context.norm_of_diff(1,Complex64::one()) < 1e-6);
        assert!(context.norm_of_diff(2,Complex64::zero()) < 1e-6);
        assert!(context.norm_of_diff(3,Complex64::zero()) < 1e-6);
    }

    #[test]
    fn swap_test_on_11() {
        let mut context = ExecutionContext::initialize(&QuantumState::same_amplitude(2, &[3]));
        apply_controlled_swap(&[], 0, 1, &mut context);

        assert!(context.norm_of_diff(0, Complex64::zero()) < 1e-6);
        assert!(context.norm_of_diff(1, Complex64::zero()) < 1e-6);
        assert!(context.norm_of_diff(2, Complex64::zero()) < 1e-6);
        assert!(context.norm_of_diff(3, Complex64::one()) < 1e-6);
    }

}
