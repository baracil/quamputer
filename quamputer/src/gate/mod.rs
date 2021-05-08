pub mod not;
pub mod gate;
pub mod operations;

use crate::QDimension;
use crate::state::State;

pub trait GateOp {

    ///
    /// Apply the current gate to the provided state
    /// and return the result.
    ///
    fn apply(&self, input:&State) -> State;

}