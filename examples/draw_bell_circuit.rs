use raylib::prelude::*;
use quamputer::computer::QuantumComputer;
use quamputer::gui::{ DrawingPar};
use rsgui::font::FontInfo;
use quamputer::condition::StopCondition::MaxIteration;
use quamputer::gui::gui_circuit::{GuiRoot, HoverData, GuiCircuitElement};
use quamputer::circuit::Circuit;
use quamputer::gui::camera_manager::CameraManager;

use quamputer::standard_gate::StandardGate::{Toffoli, Fredkin, Hadamard, CNot};

use quamputer::gui::mouse_information::MouseInformation;

fn circuit1(computer:&QuantumComputer) -> Result<Circuit,String> {

    let circuit = computer.bell_state()
        .add_operation(Toffoli(2, [1, 0]))
        .add_operation(Fredkin(0, 1, [2]))
        .add_measure("q0", 2)
        .build()?;


     computer.new_circuit_builder()
         .add_operation(Toffoli(2, [1,0]))
        .add_loop(circuit, MaxIteration(10))
        .build()
}

fn _circuit2(computer:&QuantumComputer) -> Result<Circuit,String> {
    computer.new_circuit_builder()
        .add_operation(Hadamard(0))
        .add_operation(CNot(1, [0]))
        .add_measure("q0", 0)
        .build()
}


fn main() -> Result<(), String> {



    let (mut rl, thread) = raylib::init()
        .size(640, 480)
        .title("3 Qbits Bell Circuit")
        .vsync()
        .msaa_4x()
        .resizable()
        .build();

    let font_info = {
        let font_size = 48;
        let font = rl.load_font_ex(&thread, "resources/fonts/OpenSans-Regular.ttf", font_size, FontLoadEx::Default(200));
        FontInfo::new(font?, font_size)
    };

    let reference = DrawingPar {
        font: font_info,
        hover_color: Color::VIOLET,
        register_spacing: 100.0,
        register_thickness: 2.,
        background_color: Color::BLACK,
        foreground_color: Color::WHITE,
        margin: 20.0,
    };


    let computer = QuantumComputer::new(6);

    let mut circuit  = GuiRoot::new(&circuit1(&computer)?, &reference);

    let mut camera_manager = CameraManager::default();

    let mut camera = Camera2D::default();
    camera.target.x = 200.0;
    camera.target.y = 200.0;
    camera.zoom = 1.0;
    camera.rotation = 0.0;
    init_camera(&mut camera, &rl);


    let mut screen_size = (rl.get_screen_width(), rl.get_screen_height());

    let mut need_layout = true;


    let mut mouse_info = MouseInformation::new();

    while !rl.window_should_close() {

        if rl.is_window_resized() {
            init_camera(&mut camera, &rl);
            screen_size.0 = rl.get_screen_width();
            screen_size.1 = rl.get_screen_height();
        }


        mouse_info.update(&rl,&camera);

        camera_manager.handle_camera(&rl,&mut camera,&mouse_info.middle_drag);


        {
            need_layout.then(|| {
                circuit.layout();
            });
            need_layout = false;
        }

        let mut d = rl.begin_drawing(&thread);


        d.clear_background(reference.background_color);
        {
            let mut d = d.begin_mode2D(camera);
            let hover = circuit.draw(&mut d, &mouse_info);

            if let Some(h) = &hover {

                if let HoverData::Gate(idx,_,Some(c)) = h {
                    if d.is_key_pressed(crate::consts::KeyboardKey::KEY_DELETE) {
                        if let Some(GuiCircuitElement::GuiGate(g)) = circuit.get_element_mut(*idx) {
                            g.control_bits.remove(*c);
                        }
                    }
                }

            }
        }


    };

    Ok(())
}

fn init_camera(camera: &mut Camera2D, d: &RaylibHandle) {
    let height = d.get_screen_height();
    let width = d.get_screen_width();

    camera.offset.x = (width as f32) * 0.5;
    camera.offset.y = (height as f32) * 0.5;
}