


use crate::{QDimension};
use crate::state::State;

pub fn apply_controlled_hadamard(control_qbits: &[u8], target: u8, state: &State) -> State {
    let control_mask = state.control_nask(control_qbits);
    let mask = state.mask(target);
    let not_mask = !mask;

    let mut result = State::nil(state.nb_qbits());

    let sqrt_of_one_over_two = 0.5_f64.sqrt();

    let len = state.len();
    for i in 0..len {
        let amplitude = state[i];
        let control_set = (i & control_mask) == control_mask;
        if !control_set {
            result[i] += amplitude;
        } else {
            let amplitude = amplitude*sqrt_of_one_over_two;
            let without_bit = i & not_mask; //|n0m>
            let with_bit = i | mask; //|n0m>

            let bit_set = without_bit != i; // cas |1> if set, |0> otherwise

            if bit_set {
                result[without_bit] += amplitude;
                result[with_bit] -= amplitude;
            } else {
                result[without_bit] += amplitude;
                result[with_bit] += amplitude;
            }
        }
    };

    result
}


pub fn apply_controlled_not(control_qbits: &[u8], target: u8, state: &State) -> State {
    let control_mask = state.control_nask(control_qbits);
    let target_mask = state.mask(target);

    let mut result = State::nil(state.nb_qbits());

    let len = state.len();
    for i in 0..len {
        let control_set = (i & control_mask) == control_mask;
        let source = if control_set { i ^ target_mask } else { i };
        result[i] = state[source]
    }
    result
}


#[cfg(test)]
mod tests_not {
    use std::ops::Sub;

    use num_complex::{Complex, Complex64};
    use num_traits::identities::One;
    use num_traits::Zero;

    use crate::gate::operations::apply_not_gate;
    use crate::state::State;

    #[test]
    fn not_test_on_zero() {
        let state = State::zero(3);
        let result = apply_not_gate(2, &state);

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
        let state = State::zero(3);
        let result = apply_not_gate(2, &state);
        let result = apply_not_gate(1, &result);

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
        let mut state = State::zero(3);
        state[0] = Complex::zero();
        state[1] = Complex::one();
        state[6] = Complex::one();
        let result = apply_not_gate(1, &state);

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

    use crate::gate::operations::{apply_not_gate, apply_toffoli_gate};
    use crate::state::State;

    #[test]
    fn toffoli_test_on_zero() {
        let state = State::zero(3);
        let result = apply_toffoli_gate(0, 1, 2, &state);

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
        let mut state = State::zero(3);
        state[0] = Complex::zero();
        state[2] = Complex::one();
        state[6] = Complex::new(2.0, 0.0);
        state[7] = Complex::new(3.0, 0.0);
        let result = apply_toffoli_gate(0, 1, 2, &state);

        println!("input {:?}", state);
        println!("input {:?}", result);

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
