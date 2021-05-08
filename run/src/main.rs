use quamputer::computer::QuantumComputer;
use quamputer::gate::Gate::{Hadamard, Not};
use quamputer::gate::cnot;


fn main() {
    let computer = QuantumComputer::new(2);
    let mut circuit = computer.new_circuit();

    circuit.push(Hadamard(0))
        .push(Not(1).with_one_control(0));
        // there are shortcut for Cnot and Toffoli like cnot(0,1);

    let executable = computer.compile(&circuit);

    let initial_state = computer.zero_state();

    let result = executable.execute(&initial_state);


    println!("input  : {:?}", initial_state);
    println!("result : {:?}", result);
}

