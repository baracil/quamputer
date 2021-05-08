


use crate::{QDimension};
use crate::state::QuantumState;
use std::f64::consts::FRAC_1_SQRT_2;
use crate::gate::ExecutionContext;

pub fn apply_controlled_swap(control_qbits: &[u8], target1: u8, target2:u8, context: &mut ExecutionContext) {
    let control_mask = context.control_mask(control_qbits);
    let mask1 = context.mask(target1);
    let mask2 = context.mask(target2) ;
    let not_mask = !(mask1|mask2);

    let mut result = QuantumState::nil(context.nb_qbits());
    let len = context.nb_amplitudes();
    for i in 0..len {
        let amplitude = context.current_state[i];
        let control_set = (i&control_mask) == control_mask;
        if !control_set {
            result[i] = amplitude
        } else {
            let target1_mask = if (i&mask1) != 0 {mask2} else {0};
            let target2_mask = if (i&mask2) != 0 {mask1} else {0};

            let j = (i&not_mask) | target2_mask | target1_mask;

            result[i] = context.current_state[j];
        }
    }
    context.current_state = result
}

pub fn apply_controlled_hadamard(control_qbits: &[u8], target: u8, context: &mut ExecutionContext)  {
    let control_mask = context.control_mask(control_qbits);
    let mask = context.mask(target);
    let not_mask = !mask;

    let mut result = QuantumState::nil(context.nb_qbits());

    let len = context.nb_amplitudes();
    for i in 0..len {
        let amplitude = context.current_state[i];
        let control_set = (i & control_mask) == control_mask;
        if !control_set {
            result[i] += amplitude;
        } else {
            let amplitude = amplitude*FRAC_1_SQRT_2;
            let without_bit = i & not_mask; //|n0m>
            let with_bit = i | mask; //|n1m>

            let bit_set = without_bit != i; // case |1> if set, |0> otherwise

            if bit_set {
                result[without_bit] += amplitude;
                result[with_bit] -= amplitude;
            } else {
                result[without_bit] += amplitude;
                result[with_bit] += amplitude;
            }
        }
    };

    context.current_state = result
}


pub fn apply_controlled_not(control_qbits: &[u8], target: u8, context: &mut ExecutionContext) {
    let control_mask = context.control_mask(control_qbits);
    let target_mask = context.mask(target);

    let mut result = QuantumState::nil(context.nb_qbits());

    let len = context.nb_amplitudes();
    for i in 0..len {
        let control_set = (i & control_mask) == control_mask;
        let source = if control_set { i ^ target_mask } else { i };
        result[i] = context.current_state[source]
    }
    context.current_state = result;
}


#[cfg(test)]
mod tests_not {
    use std::ops::Sub;

    use num_complex::{Complex, Complex64};
    use num_traits::identities::One;
    use num_traits::Zero;

    use crate::state::QuantumState;
    use crate::operations::{apply_controlled_not, apply_controlled_swap};
    use crate::gate::ExecutionContext;

    #[test]
    fn swap_test_on_00() {
        let mut context = ExecutionContext::initialize(&QuantumState::same_amplitude(2,&[0]));
        apply_controlled_swap(&[], 0, 1,&mut context);
        let result = context.current_state;

        assert!((result[0].sub(Complex64::one()).norm()) < 1e-6);
        assert!((result[1].sub(Complex64::zero()).norm()) < 1e-6);
        assert!((result[2].sub(Complex64::zero()).norm()) < 1e-6);
        assert!((result[3].sub(Complex64::zero()).norm()) < 1e-6);
    }

    #[test]
    fn swap_test_on_01() {
        let mut context = ExecutionContext::initialize(&QuantumState::same_amplitude(2,&[1]));
        apply_controlled_swap(&[], 0, 1,&mut context);
        let result = context.current_state;

        assert!((result[0].sub(Complex64::zero()).norm()) < 1e-6);
        assert!((result[1].sub(Complex64::zero()).norm()) < 1e-6);
        assert!((result[2].sub(Complex64::one()).norm()) < 1e-6);
        assert!((result[3].sub(Complex64::zero()).norm()) < 1e-6);
    }

    #[test]
    fn swap_test_on_10() {
        let mut context = ExecutionContext::initialize(&QuantumState::same_amplitude(2,&[2]));
        apply_controlled_swap(&[], 0, 1,&mut context);
        let result = context.current_state;

        assert!((result[0].sub(Complex64::zero()).norm()) < 1e-6);
        assert!((result[1].sub(Complex64::one()).norm()) < 1e-6);
        assert!((result[2].sub(Complex64::zero()).norm()) < 1e-6);
        assert!((result[3].sub(Complex64::zero()).norm()) < 1e-6);
    }

    #[test]
    fn swap_test_on_11() {
        let mut context = ExecutionContext::initialize(&QuantumState::same_amplitude(2,&[3]));
        apply_controlled_swap(&[], 0, 1,&mut context);
        let result = context.current_state;

        assert!((result[0].sub(Complex64::zero()).norm()) < 1e-6);
        assert!((result[1].sub(Complex64::zero()).norm()) < 1e-6);
        assert!((result[2].sub(Complex64::zero()).norm()) < 1e-6);
        assert!((result[3].sub(Complex64::one()).norm()) < 1e-6);
    }




    #[test]
    fn not_test_on_zero() {
        let mut context = ExecutionContext::initialize(&QuantumState::zero(3));
        apply_controlled_not(&[], 2, &mut context);
        let result = context.current_state;

        assert!((result[0].sub(Complex64::zero()).norm()) < 1e-6);
        assert!((result[1].sub(Complex64::one()).norm()) < 1e-6);
        assert!((result[2].sub(Complex64::zero()).norm()) < 1e-6);
        assert!((result[3].sub(Complex64::zero()).norm()) < 1e-6);
        assert!((result[4].sub(Complex64::zero()).norm()) < 1e-6);
        assert!((result[5].sub(Complex64::zero()).norm()) < 1e-6);
        assert!((result[6].sub(Complex64::zero()).norm()) < 1e-6);
        assert!((result[7].sub(Complex64::zero()).norm()) < 1e-6);
    }

    #[test]
    fn not_test_on_one() {
        let mut context = ExecutionContext::initialize(&QuantumState::zero(3));
        apply_controlled_not(&[], 2, &mut context);
        apply_controlled_not(&[], 1, &mut context);
        let result = context.current_state;

        assert!((result[0].sub(Complex64::zero()).norm()) < 1e-6);
        assert!((result[1].sub(Complex64::zero()).norm()) < 1e-6);
        assert!((result[2].sub(Complex64::zero()).norm()) < 1e-6);
        assert!((result[3].sub(Complex64::one()).norm()) < 1e-6);
        assert!((result[4].sub(Complex64::zero()).norm()) < 1e-6);
        assert!((result[5].sub(Complex64::zero()).norm()) < 1e-6);
        assert!((result[6].sub(Complex64::zero()).norm()) < 1e-6);
        assert!((result[7].sub(Complex64::zero()).norm()) < 1e-6);
    }

    #[test]
    fn not_test_superpos() {
        let mut state = QuantumState::zero(3);
        state[0] = Complex::zero();
        state[1] = Complex::one();
        state[6] = Complex::one();

        let mut context = ExecutionContext::initialize(&state);
        apply_controlled_not(&[], 1, &mut context);
        let result = context.current_state;

        assert!((result[0].sub(Complex64::zero()).norm()) < 1e-6);
        assert!((result[1].sub(Complex64::zero()).norm()) < 1e-6);
        assert!((result[2].sub(Complex64::zero()).norm()) < 1e-6);
        assert!((result[3].sub(Complex64::one()).norm()) < 1e-6);
        assert!((result[4].sub(Complex64::one()).norm()) < 1e-6);
        assert!((result[5].sub(Complex64::zero()).norm()) < 1e-6);
        assert!((result[6].sub(Complex64::zero()).norm()) < 1e-6);
        assert!((result[7].sub(Complex64::zero()).norm()) < 1e-6);
    }
}

#[cfg(test)]
mod tests_toffoli {
    use std::ops::Sub;

    use num_complex::{Complex, Complex64};
    use num_traits::identities::One;
    use num_traits::Zero;

    use crate::state::QuantumState;
    use crate::operations::apply_controlled_not;
    use crate::gate::ExecutionContext;

    #[test]
    fn toffoli_test_on_zero() {
        let state = QuantumState::zero(3);
        let mut context = ExecutionContext::initialize(&state);
        apply_controlled_not(&[0, 1], 2, &mut context);
        let result = context.current_state;

        assert!((result[0].sub(Complex64::one()).norm()) < 1e-6);
        assert!((result[1].sub(Complex64::zero()).norm()) < 1e-6);
        assert!((result[2].sub(Complex64::zero()).norm()) < 1e-6);
        assert!((result[3].sub(Complex64::zero()).norm()) < 1e-6);
        assert!((result[4].sub(Complex64::zero()).norm()) < 1e-6);
        assert!((result[5].sub(Complex64::zero()).norm()) < 1e-6);
        assert!((result[6].sub(Complex64::zero()).norm()) < 1e-6);
        assert!((result[7].sub(Complex64::zero()).norm()) < 1e-6);
    }

    #[test]
    fn not_test_on_superposition() {
        let mut state = QuantumState::zero(3);
        state[0] = Complex::zero();
        state[2] = Complex::one();
        state[6] = Complex::new(2.0, 0.0);
        state[7] = Complex::new(3.0, 0.0);

        let mut context = ExecutionContext::initialize(&state);
        apply_controlled_not(&[0, 1], 2, &mut context);
        let result = context.current_state;
        assert!((result[0].sub(Complex64::zero()).norm()) < 1e-6);
        assert!((result[1].sub(Complex64::zero()).norm()) < 1e-6);
        assert!((result[2].sub(Complex64::one()).norm()) < 1e-6);
        assert!((result[3].sub(Complex64::zero()).norm()) < 1e-6);
        assert!((result[4].sub(Complex64::zero()).norm()) < 1e-6);
        assert!((result[5].sub(Complex64::zero()).norm()) < 1e-6);
        assert!((result[6].sub(Complex64::new(3.0, 0.0)).norm()) < 1e-6);
        assert!((result[7].sub(Complex64::new(2.0, 0.0)).norm()) < 1e-6);
    }
}
