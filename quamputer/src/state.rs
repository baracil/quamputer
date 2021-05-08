use num_complex::{Complex64};
use crate::{power_of_two, QDimension};
use std::ops::{Deref, DerefMut, Add};

use num_traits::Zero;
use std::fmt::{Debug, Formatter, Result};

/// Quantum state
///
pub struct QuantumState {
    nb_qbits: u8,
    amplitudes: Vec<Complex64>,
}

impl Deref for QuantumState {
    type Target = Vec<Complex64>;
    fn deref(&self) -> &Self::Target {
        &self.amplitudes
    }
}

impl DerefMut for QuantumState {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.amplitudes
    }
}


impl Debug for QuantumState {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        let wave_function = self.amplitudes.iter()
            .enumerate()
            .map(|(i, a)| { format!(" ({0:.6},{1:.6})x|{2:0>3$b}>", a.re, a.im, i,self.nb_qbits as usize) })
            .reduce(|s1, s2| s1.add(&s2))
            .unwrap_or("".to_string());

        let text = format!("State {} qbits : {} ", self.nb_qbits, &wave_function);
        f.write_str(&text)
    }
}

impl QDimension for QuantumState {
    fn nb_qbits(&self) -> u8 {
        return self.nb_qbits;
    }
}

impl QuantumState {

    pub (crate) fn mask(&self, qbit_idx: u8) -> usize {
        return power_of_two(self.nb_qbits - 1 - qbit_idx);
    }

    pub (crate) fn control_nask(&self, control_qbits: &[u8]) -> usize {
        control_qbits.iter()
            .map(|i| self.mask(*i))
            .reduce(|m1, m2| m1 + m2)
            .unwrap_or(0)
    }
}


impl QuantumState {

    pub (crate) fn same_amplitude(nb_qbits: u8, qbit_idx: &[usize]) -> QuantumState {
        let nb_amplitudes = power_of_two(nb_qbits);
        let mut amplitudes = Vec::with_capacity(nb_amplitudes);
        amplitudes.resize_with(nb_amplitudes, || Complex64::zero());

        let amplitude = Complex64::new((1.0 / qbit_idx.len() as f64).sqrt(), 0.0);

        for qbit_idx in qbit_idx {
            amplitudes[*qbit_idx] = amplitude;
        }

        return Self { nb_qbits, amplitudes };
    }


    pub (crate) fn zero(nb_quits: u8) -> Self {
        QuantumState::same_amplitude(nb_quits, &[0])
    }

    pub (crate) fn nil(nb_quits: u8) -> Self {
        let nb_amplitudes = power_of_two(nb_quits);
        let mut amplitudes = Vec::with_capacity(nb_amplitudes);
        amplitudes.resize_with(nb_amplitudes, || Complex64::zero());
        Self { nb_qbits: nb_quits, amplitudes }
    }

    pub (crate) fn from(other: &QuantumState) -> Self {
        Self { nb_qbits: other.nb_qbits, amplitudes: other.amplitudes.clone() }
    }
}

