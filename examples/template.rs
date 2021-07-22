use quamputer::common_gate::CommonGate::Fredkin;

fn main() -> Result<(), String> {
    let gate = Fredkin(0,1,[2]);
    Ok(())
}

