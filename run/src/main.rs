use quamputer::circuit::QuantumCircuit;
use quamputer::gate::gate::Gate::{Not, CNot, Hadamard};
use quamputer::gate::gate::{Gate, cnot, toffoli};
use quamputer::QDimension;
use quamputer::computer::QuantumComputer;
use std::process::exit;
use quamputer::state::State;

fn main() {
    let computer = QuantumComputer::new(2);


    let mut circuit = computer.new_circuit();

    circuit.push(Hadamard(0))
        .push(cnot(0, 1))
        .push(Hadamard(0));
    ;

    let executable = computer.compile(&circuit);

    let initial_state = computer.zero_state();
    let result = executable.launch(&initial_state);


    println!("input  : {:?}", initial_state);
    println!("result : {:?}", result);
}

