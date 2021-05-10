use quamputer::computer::QuantumComputer;
use quamputer::gate::Gate::{Hadamard, CNot};
use quamputer::operation::QuantumOperation::Measure;
use quamputer::operation::MeasurePar;
use quamputer::operation::Condition::{MaxIteration, MaxZeroSampling};

fn main() -> Result<(),String> {
    let computer = QuantumComputer::new(3);

    let circuit = {
        let mut circuit_builder = computer.new_circuit_builder();
        circuit_builder
            .start_advanced_loop(MaxZeroSampling("q0".to_string(),10))
            .push(Hadamard(0))?
            .push(CNot(1,[0]))?
            .push(CNot(2,[1]))?
            .push(Measure(MeasurePar{id:"q0".to_string(),target:0}))?
            .end_loop()?
            .build()?
    };

    let executable = computer.compile(&circuit);
    let initial_state = computer.zero_state();
    let result = executable.execute(&initial_state);


    // println!("input     : {:?}", initial_state);
    // println!("result    : {:?}", result.current_state());
    println!("result q0 : {:?}", result.get_count("q0"));

    Ok(())
}

