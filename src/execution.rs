use crate::state::QuantumState;
use std::collections::HashMap;
use num_complex::Complex64;
use crate::measure::{MeasureCount, State};
use num_traits::One;
use crate::measure::State::{Measured, NotMeasured};
use std::ops::Sub;

/// Contains information about the execution
/// of the quantum circuit
pub struct ExecutionContext {
    /// Current quantum state
    current_state: QuantumState,
    /// Indicates if the quantum state has just been measured
    state: State,
    /// Measurement results by measurement's id
    count: HashMap<String, MeasureCount>,
}

impl ExecutionContext {

    pub(crate) fn increase_zero(&mut self, id: &String) {
        self.increase_count(id, |c| c.nb_zero += 1)
    }

    pub(crate) fn increase_one(&mut self, id: &String) {
        self.increase_count(id, |c| c.nb_one += 1)
    }

    fn increase_count(&mut self, id: &String, action: fn(&mut MeasureCount) -> ()) {
        match self.count.get_mut(id) {
            Some(c) => (action)(c),
            None => {
                let mut count = MeasureCount { nb_one: 0, nb_zero: 0 };
                (action)(&mut count);
                self.count.insert(id.clone(), count);
            }
        }
    }

    pub(crate) fn set_measurement(&mut self, select_state: usize) {
        let mut output = QuantumState::nil(self.nb_qbits());
        output[select_state] = Complex64::one();

        self.current_state = output;
        self.state = Measured(select_state);
    }

    pub(crate) fn pick_on_state(&self) -> usize {
        let mut target = 1.0 - (rand::random::<f64>());
        for (index, amplitude) in self.current_state.iter().enumerate() {
            target -= amplitude.norm_sqr();
            if target <= 0.0 {
                return index;
            }
        };
        self.current_state.len() - 1
    }

    pub fn current_state(&self) -> &QuantumState {
        &self.current_state
    }

    pub(crate) fn current_amplitude_at(&self, idx: usize) -> Complex64 {
        self.current_state[idx]
    }

    pub(crate) fn _norm_of_diff(&self, idx: usize, reference: Complex64) -> f64 {
        self.current_state[idx].sub(reference).norm()
    }

    pub(crate) fn set_current_state(&mut self, new_state: QuantumState) {
        self.current_state = new_state;
        self.state = NotMeasured;
    }

    pub(crate) fn initialize(initial_state: &QuantumState) -> Self {
        Self { current_state: QuantumState::from(initial_state), state: NotMeasured, count: HashMap::new() }
    }

    pub(crate) fn mask(&self, qbit_idx: u8) -> usize {
        self.current_state.mask(qbit_idx)
    }

    pub(crate) fn control_mask(&self, control_qbits: &[u8]) -> usize {
        self.current_state.control_mask(control_qbits)
    }
    pub(crate) fn nb_amplitudes(&self) -> usize {
        self.current_state.len()
    }


    pub fn get_count(&self, variable: &str) -> Option<&MeasureCount> {
        self.count.get(variable)
    }

    pub fn get_nb_zero(&self, variable: &str) -> u32 {
        match self.count.get(variable) {
            Some(c) => c.nb_zero,
            None => 0
        }
    }

    pub fn get_nb_one(&self, variable: &str) -> u32 {
        match self.count.get(variable) {
            Some(c) => c.nb_one,
            None => 0
        }
    }
}

impl ExecutionContext {
    pub fn nb_qbits(&self) -> u8 {
        self.current_state.nb_qbits()
    }
}

