use quamputer::computer::QuantumComputer;
use quamputer::gate::Gate::{Hadamard, Not, Swap, Z};
use quamputer::gate::{cnot, cswap};
use quamputer::measure::Measure;

fn main() {
    let computer = QuantumComputer::new(3);
    let mut circuit_builder = computer.new_circuit_builder();

    circuit_builder
        .start_advanced_loop(|_,c| c.get_nb_zero("q0") >= 10 )
        .push(Hadamard(0))
        .push(cnot(0,1))
        .push(cnot(1,2))
        .push(Measure::new("q0",0))
        .end_loop()
    ;





    let executable = computer.compile(&circuit_builder.build().unwrap());

    let initial_state = computer.zero_state();

    let result = executable.execute(&initial_state);


    println!("input     : {:?}", initial_state);
    println!("result    : {:?}", result.current_state);
    println!("result q0 : {:?}", result.count.get("q0"));
    println!("result q1 : {:?}", result.count.get("q1"));
    println!("result q2 : {:?}", result.count.get("q2"));
}

