pub mod gate;
pub mod operations;


use crate::state::State;

pub trait GateOp {

    fn max_qbit_idx(&self) -> u8;

    ///
    /// Apply the current gate to the provided state
    /// and return the result.
    ///
    fn apply(&self, input:&State) -> State;

}