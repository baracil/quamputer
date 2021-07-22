use quamputer::common_gate::CommonGate::{CNot, Hadamard};
use quamputer::computer::QuantumComputer;
use quamputer::condition::StopCondition;

fn main() -> Result<(), String> {
    let computer = QuantumComputer::new(3);

    let sub_circuit = computer.new_circuit_builder()
        .add_operation(Hadamard(0))
        .add_operation(CNot(1, [0]))
        .add_operation(CNot(2, [1]))
        .add_measure("q0", 1)
        .build()?;

    let circuit = computer.new_circuit_builder()
        .add_loop(sub_circuit,StopCondition::MaxZeroSampling {id:"q0".to_string(),nb:10})
        .build()?;

    for i in 1..5 {
        let initial_state = computer.zero_state();
        let result = circuit.execute(&initial_state);
        println!("-- Iteration {} --",i);
        println!("input       : {:?}", initial_state);
        println!("output      : {:?}", result.current_state());
        println!("q0 measures : {:?}", result.get_count("q0").unwrap());
    }
    Ok(())
}

