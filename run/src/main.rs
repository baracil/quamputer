use quamputer::computer::QuantumComputer;
use quamputer::gate::Gate::{Hadamard, CNot};
use quamputer::measure::Measure;

fn main() -> Result<(),String> {
    let computer = QuantumComputer::new(3);

    let circuit = {

        let mut circuit_builder = computer.new_circuit_builder();
        circuit_builder
            .start_advanced_loop(|_,c| c.get_nb_zero("q0") >= 10 )
            .push(Hadamard(0))?
            .push(CNot(0,[0]))?
            .push(CNot(1,[2]))?
            .push(Measure::new("q0",0))?
            .end_loop()?
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

