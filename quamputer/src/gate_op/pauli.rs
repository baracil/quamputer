use crate::state::QuantumState;
use num_complex::Complex64;
use std::ops::Mul;
use crate::gate::ExecutionContext;
use crate::QDimension;

pub fn apply_controlled_pauli_z(control_qbits: &[u8], target: u8, context: &mut ExecutionContext) {
    let control_mask = context.control_mask(control_qbits);
    let target_mask = context.mask(target);

    let mut result = QuantumState::nil(context.nb_qbits());

    let len = context.nb_amplitudes();
    for i in 0..len {
        let amplitude = context.current_state[i];

        let control_set = (i & control_mask) == control_mask;
        let bit_set = (i& target_mask) != 0;

        let amplitude = match (control_set, bit_set) {
            (true,true) => -amplitude,
            (_,_) => amplitude
        };

        result[i] = amplitude;
    }

    context.current_state = result
}

pub fn apply_controlled_pauli_y(control_qbits: &[u8], target: u8, context: &mut ExecutionContext) {
    let control_mask = context.control_mask(control_qbits);
    let mask = context.mask(target);

    let mut result = QuantumState::nil(context.nb_qbits());

    let i = Complex64::i();
    let minus_i = Complex64::new(0.0, -1.0);

    let len = context.nb_amplitudes();
    for src in 0..len {
        let amplitude = context.current_state[src];

        let control_set = (src & control_mask) == control_mask;
        let bit_set = (src & mask) != 0;


        let (amplitude,dst) = match (control_set, bit_set) {
            (true,true) => (minus_i.mul(amplitude), src^mask),
            (true,false) => (i.mul(amplitude), src^mask),
            (false,_) => (amplitude,src)
        };

        result[dst] = amplitude;
    }

    context.current_state = result
}

pub fn apply_controlled_pauli_x(control_qbits: &[u8], target: u8, context: &mut ExecutionContext) {
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
mod tests_pauli {
    use super::*;
    use std::ops::{Sub, Neg};
    use num_traits::Zero;
    use num_traits::One;

    #[test]
    fn pauli_y_test_on_0() {
        let mut context = ExecutionContext::initialize(&QuantumState::same_amplitude(1, &[0]));
        apply_controlled_pauli_y(&[], 0, &mut context);
        let result = context.current_state;

        assert!((result[0].sub(Complex64::zero()).norm()) < 1e-6);
        assert!((result[1].sub(Complex64::i()).norm()) < 1e-6);
    }

    #[test]
    fn pauli_y_test_on_1() {
        let mut context = ExecutionContext::initialize(&QuantumState::same_amplitude(1, &[1]));
        apply_controlled_pauli_y(&[], 0, &mut context);
        let result = context.current_state;

        assert!((result[0].sub(Complex64::new(0.0, -1.0)).norm()) < 1e-6);
        assert!((result[1].sub(Complex64::zero()).norm()) < 1e-6);
    }

    #[test]
    fn pauli_y_test_on_mix01() {
        let mut context = ExecutionContext::initialize(&QuantumState::zero(1));
        let c1 = Complex64::new(1.0, 2.0);
        let c2 = Complex64::new(3.0, 4.0);
        context.current_state[0] = c1;
        context.current_state[1] = c2;


        apply_controlled_pauli_y(&[], 0, &mut context);
        let result = context.current_state;

        assert!((result[0].sub(c2.mul(Complex64::i().neg())).norm()) < 1e-6);
        assert!((result[1].sub(c1.mul(Complex64::i())).norm()) < 1e-6);
    }

    #[test]
    fn pauli_z_test_on_0() {
        let mut context = ExecutionContext::initialize(&QuantumState::same_amplitude(1, &[0]));
        apply_controlled_pauli_z(&[], 0, &mut context);
        let result = context.current_state;

        assert!((result[0].sub(Complex64::one()).norm()) < 1e-6);
        assert!((result[1].sub(Complex64::zero()).norm()) < 1e-6);
    }

    #[test]
    fn pauli_z_test_on_1() {
        let mut context = ExecutionContext::initialize(&QuantumState::same_amplitude(1, &[1]));
        apply_controlled_pauli_z(&[], 0, &mut context);
        let result = context.current_state;

        assert!((result[0].sub(Complex64::zero()).norm()) < 1e-6);
        assert!((result[1].sub(Complex64::new(-1.0, 0.0)).norm()) < 1e-6);
    }

    #[test]
    fn pauli_z_test_on_mix01() {
        let mut context = ExecutionContext::initialize(&QuantumState::zero(1));
        let c1 = Complex64::new(1.0, 2.0);
        let c2 = Complex64::new(3.0, 4.0);
        context.current_state[0] = c1;
        context.current_state[1] = c2;


        apply_controlled_pauli_z(&[], 0, &mut context);
        let result = context.current_state;

        assert!((result[0].sub(c1).norm()) < 1e-6);
        assert!((result[1].sub(c2.neg()).norm()) < 1e-6);
    }

}

#[cfg(test)]
mod tests_not {
    use std::ops::{Sub};

    use num_complex::{Complex, Complex64};
    use num_traits::identities::One;
    use num_traits::Zero;

    use crate::state::QuantumState;
    use crate::gate::ExecutionContext;
    use crate::gate_op::pauli::{apply_controlled_pauli_x};

    #[test]
    fn not_test_on_zero() {
        let mut context = ExecutionContext::initialize(&QuantumState::zero(3));
        apply_controlled_pauli_x(&[], 2, &mut context);
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
        apply_controlled_pauli_x(&[], 2, &mut context);
        apply_controlled_pauli_x(&[], 1, &mut context);
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
        apply_controlled_pauli_x(&[], 1, &mut context);
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
    use crate::gate::ExecutionContext;
    use crate::gate_op::pauli::apply_controlled_pauli_x;

    #[test]
    fn toffoli_test_on_zero() {
        let state = QuantumState::zero(3);
        let mut context = ExecutionContext::initialize(&state);
        apply_controlled_pauli_x(&[0, 1], 2, &mut context);
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
        apply_controlled_pauli_x(&[0, 1], 2, &mut context);
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
