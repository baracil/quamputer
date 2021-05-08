use num_complex::{Complex64, Complex};
use crate::{power_of_two, QDimension};
use std::ops::{Deref, DerefMut, Add};
use num_traits::identities::One;
use num_traits::Zero;
use std::fmt::{Debug, Formatter, Result, Write};
use std::alloc::handle_alloc_error;

pub struct State {
    nb_qbits:u8,
    amplitudes:Vec<Complex64>,
}

impl Debug for State {

    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        let wavefunction = self.amplitudes.iter()
            .enumerate()
            .map(|(i, a)| { format!(" ({},{})x|{}>", a.re, a.im, i) })
            .reduce(|s1, s2| s1.add(&s2))
            .unwrap_or("".to_string());

        let text = format!("State {} : {} ", self.nb_qbits, &wavefunction);
        f.write_str(&text)
    }
}

impl State {
    pub fn mask(&self, qbit_idx:u8) -> usize {
        return power_of_two(self.nb_qbits-1-qbit_idx);
    }
}

impl Deref for State {
    type Target = Vec<Complex64>;

    fn deref(&self) -> &Self::Target {
        &self.amplitudes
    }
}

impl DerefMut for State {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.amplitudes
    }
}


impl State {

    pub(crate) fn same_amplitude(nb_qbits: u8, qbit_idx: &[usize]) -> State {
        let nb_amplitudes= power_of_two(nb_qbits);
        let mut amplitudes = Vec::with_capacity(nb_amplitudes);
        amplitudes.resize_with(nb_amplitudes, || Complex64::zero());

        let amplitude = Complex64::new((1.0/qbit_idx.len() as f64).sqrt(), 0.0);

        for qbit_idx in qbit_idx {
            amplitudes[*qbit_idx] = amplitude;
        }

        return Self{nb_qbits,amplitudes};

    }


    pub fn zero(nb_quits:u8) -> Self {
        State::same_amplitude(nb_quits,&[0])
    }

    pub fn nil(nb_quits:u8) -> Self {
        let nb_amplitudes= power_of_two(nb_quits);
        let mut amplitudes = Vec::with_capacity(nb_amplitudes);
        amplitudes.resize_with(nb_amplitudes, || Complex64::zero());
        Self{nb_qbits:nb_quits,amplitudes}
    }

    pub fn from(other:&State) -> Self {
        Self{nb_qbits:other.nb_qbits, amplitudes:other.amplitudes.clone()}
    }
}

impl QDimension for State {
    fn nb_qbits(&self) -> u8 {
        return self.nb_qbits
    }
}