use quamputer::computer::QuantumComputer;
use quamputer::gate::Gate::{Hadamard, CNot};
use quamputer::condition::Condition::{MaxZeroSampling};


fn main() -> Result<(),String> {
    let computer = QuantumComputer::new(3);

    let circuit = {

        let bell_state = computer.new_circuit_builder()
            .apply(Hadamard(0))
            .apply(CNot(1, [0]))
            .apply(CNot(2, [1]))
            .measure("q0", 0)
            .build()?;



        let mut circuit_builder = computer.new_circuit_builder();
        circuit_builder
            .push_loop(MaxZeroSampling("q0".to_string(),10), bell_state)
            .build()?
    };

    let executable = computer.compile(&circuit);

    let initial_state = computer.zero_state();

    let result = executable.execute(&initial_state);

    println!("{:?}",result.get_count("q0"));
    Ok(())
}

