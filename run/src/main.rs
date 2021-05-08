use quamputer::computer::QuantumComputer;
use quamputer::gate::Gate::{Hadamard, Not, Swap};
use quamputer::gate::{cnot, cswap};
use quamputer::measure::Measure;

fn main() {
    let computer = QuantumComputer::new(3);
    let mut circuit_builder = computer.new_circuit_builder();

    circuit_builder
        .push(Hadamard(0))
        .start_loop(2)
        .push(cnot(0,1))
        .push(cswap(0,1,2))
        .end_loop();





    let executable = computer.compile(&circuit_builder.build().unwrap());

    let initial_state = computer.zero_state();

    let result = executable.execute(&initial_state);


    println!("input  : {:?}", initial_state);
    println!("result : {:?}", result.current_state);
    println!("result : {:?}", result.count.get("A"));
}

