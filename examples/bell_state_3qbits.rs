use quamputer::computer::QuantumComputer;
use quamputer::condition::StopCondition::{MaxZeroSampling, Or, MaxOneSample};

fn main() -> Result<(), String> {
    let computer = QuantumComputer::new(3);

    let circuit = {
        computer.new_circuit_builder()
            .add_loop(computer.bell_state().add_measure("q0", 0),
                      Or(Box::new(MaxZeroSampling { id: "q0".to_string(), nb: 10 }), Box::new(MaxOneSample { id: "q0".to_string(), nb: 10 })))
                          .build()?
    };

    let initial_state = computer.zero_state();

    let result = circuit.execute(&initial_state);

    println!("{:?}", result.get_count("q0"));
    Ok(())
}

