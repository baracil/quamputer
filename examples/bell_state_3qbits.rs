use quamputer::computer::QuantumComputer;

use quamputer::common_gate::CommonGate::{CNot, Hadamard};

fn main() -> Result<(), String> {
    let computer = QuantumComputer::new(3);


    let circuit = computer.new_circuit_builder()
        .add_operation(Hadamard(0))
        .add_operation(CNot(1, [0]))
        .add_operation(CNot(2, [0]))
        .build()?;

    let initial_state = computer.zero_state();

    let result = circuit.execute(&initial_state);

    println!("input  : {:?}", initial_state);
    println!("output : {:?}", result.current_state());
    Ok(())
}

