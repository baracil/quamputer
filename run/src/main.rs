
use quamputer::gate::gate::Gate::{Hadamard};


use quamputer::computer::QuantumComputer;



fn main() {
    let computer = QuantumComputer::new(2);


    let mut circuit = computer.new_circuit();

    circuit.push(Hadamard(1).single_control(0));


    let executable = computer.compile(&circuit);

    let initial_state = computer.same_amplitude(&[0,2]);
    let result = executable.launch(&initial_state);


    println!("input  : {:?}", initial_state);
    println!("result : {:?}", result);
}

