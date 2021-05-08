use quamputer::computer::QuantumComputer;
use quamputer::gate::Gate::{Hadamard, Not};
use quamputer::gate::cnot;
use quamputer::measure::Measure;

fn main() {
    let computer = QuantumComputer::new(2);
    let mut circuit_builder = computer.new_circuit_builder();

    circuit_builder.start_loop(2)
        .push(Hadamard(0))
        .push(Not(1).with_one_control(0))
        .end_loop();





    let executable = computer.compile(&circuit_builder.build().unwrap());

    let initial_state = computer.zero_state();

    let result = executable.execute(&initial_state);


    println!("input  : {:?}", initial_state);
    println!("result : {:?}", result.current_state);
    println!("result : {:?}", result.count.get("A"));
}

