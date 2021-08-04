mod druid_test;


use quamputer::circuit::Circuit;
use quamputer::common_gate::CommonGate::{CNot, Fredkin, Hadamard, Toffoli};
use quamputer::computer::QuantumComputer;
use quamputer::condition::StopCondition::MaxIteration;
use druid::{Widget, WindowDesc, AppLauncher, Point};
use quamputer::gui::circuit_drawer::{CircuitDrawer, CircuitDrawerState};
use druid::widget::EnvScope;

fn circuit1(computer: &QuantumComputer) -> Result<Circuit, String> {
    let circuit = computer.bell_state()
        .add_operation(Toffoli(2, [1, 0]))
        .add_operation(Fredkin(0, 1, [2]))
        .add_measure("q0", 2)
        .build()?;


    computer.new_circuit_builder()
        .add_operation(Toffoli(2, [1, 0]))
        .add_loop(circuit, MaxIteration(10))
        .build()
}

fn _circuit2(computer: &QuantumComputer) -> Result<Circuit, String> {
    computer.new_circuit_builder()
        .add_operation(Hadamard(0))
        .add_operation(CNot(1, [0]))
        .add_measure("q0", 0)
        .build()
}


fn build_root_widget() -> impl Widget<CircuitDrawerState> {
    CircuitDrawer::with_env_scope()
}

fn main() -> Result<(), String> {
    let main_window = WindowDesc::new(build_root_widget())
        .title("Hello World!")
        .window_size((400.0, 400.0));

    let computer = QuantumComputer::new(4);
    let circuit = circuit1(&computer)?;

    // create the initial app state
    let initial_state = CircuitDrawerState{pos:Point{x:200.,y:200.},circuit:circuit.into()};

    // start the application. Here we pass in the application state.
    AppLauncher::with_window(main_window)
        .log_to_console()
        .launch(initial_state)
        .expect("Failed to launch application");



    Ok(())
}
