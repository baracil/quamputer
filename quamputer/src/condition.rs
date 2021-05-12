use crate::gate::ExecutionContext;
use serde::{Serialize,Deserialize};

pub trait EndOfLoopPredicate where Self : Serialize {
    fn is_end_of_loop(&self, nb_iterations:u32, context: &ExecutionContext) -> bool ;
}

#[derive(Clone, Serialize, Deserialize)]
pub enum Condition {
    MaxIteration(u32),
    MaxZeroSampling(String,u32),
    MaxOneSample(String,u32),
}


impl EndOfLoopPredicate for Condition {
    fn is_end_of_loop(&self, nb_iterations: u32, context: &ExecutionContext) -> bool {
        match self {
            Condition::MaxIteration(nb) => nb_iterations>=*nb,
            Condition::MaxZeroSampling(id, nb) => context.get_nb_zero(id)>=*nb,
            Condition::MaxOneSample(id, nb) => context.get_nb_one(id)>=*nb
        }
    }
}
