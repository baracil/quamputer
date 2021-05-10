use quamputer::computer::QuantumComputer;
use quamputer::gate::Gate::{Hadamard, CNot};
use quamputer::measure::Measure;

fn main() -> Result<(),String> {
    let computer = QuantumComputer::new(3);

    let circuit = {

        let mut circuit_builder = computer.new_circuit_builder();
        circuit_builder
            .push(Hadamard(0))?
            .push(CNot(1,[0]))?
            .push(CNot(2,[1]))?
            .push(Measure::new("q0",0))?
            .build()?
    };


    let executable = computer.compile(&circuit);
    let initial_state = computer.zero_state();
    let result = executable.execute(&initial_state);


    println!("input     : {:?}", initial_state);
    println!("result    : {:?}", result.current_state());
    println!("result q0 : {:?}", result.get_count("q0"));

    Ok(())
}

