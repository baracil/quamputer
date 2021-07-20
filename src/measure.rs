use std::collections::HashMap;
use std::ops::Sub;



pub enum State {
    Measured(usize),
    NotMeasured,
}

#[derive(Debug, Copy, Clone)]
pub struct MeasureCount {
    pub nb_zero: u32,
    pub nb_one: u32,
}

