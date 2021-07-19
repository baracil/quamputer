use crate::gate::ExecutionContext;
use serde::{Serialize,Deserialize};


/// Variants used to define the stop condition in a loop.
#[derive(Clone, Serialize, Deserialize)]
pub enum StopCondition {
    /// Condition that stops the loop after the first iteration
    Once(),
    /// Condition that stops the loop after the given number of iterations
    MaxIteration(u32),
    /// Condition that stops when the measurement gate with the id 'id' measures at least 'zero' 'nb' times
    MaxZeroSampling{id:String,nb:u32},
    /// Condition that stops when the measurement gate with the id 'id' measures at least 'one' 'nb' times
    MaxOneSample{id:String,nb:u32},
    Or(Box<StopCondition>,Box<StopCondition>),
    And(Box<StopCondition>, Box<StopCondition>),
}


impl StopCondition {

    /// Check if the current context match this condition
    ///
    /// # Arguments
    ///
    /// - nb_iterations : the current number of iterations of the loop
    /// - context : the execution context of the quantum program
    pub fn is_end_of_loop(&self, nb_iterations: u32, context: &ExecutionContext) -> bool {
        match self {
            StopCondition::Once() => nb_iterations>=1,
            StopCondition::MaxIteration(nb) => nb_iterations>=*nb,
            StopCondition::MaxZeroSampling{id, nb} => context.get_nb_zero(id)>=*nb,
            StopCondition::MaxOneSample{id, nb} => context.get_nb_one(id)>=*nb,
            StopCondition::Or(lhs,rhs) => {
                lhs.is_end_of_loop(nb_iterations,context) || rhs.is_end_of_loop(nb_iterations,context)
            }
            StopCondition::And(lhs,rhs) => {
                lhs.is_end_of_loop(nb_iterations,context) && rhs.is_end_of_loop(nb_iterations,context)
            }
        }
    }
}
