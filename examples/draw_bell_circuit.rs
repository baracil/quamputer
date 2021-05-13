use raylib::prelude::*;
use quamputer::computer::QuantumComputer;
use quamputer::gate::Gate::{Hadamard, CNot, Fredkin, Toffoli};
use quamputer::gui::{Drawable, DrawingPar};
use rs_gui::font::FontInfo;
use quamputer::condition::StopCondition::MaxIteration;

fn main() -> Result<(),String>{
    let (mut rl, thread) = raylib::init()
        .size(640, 480)
        .title("3 Qbits Bell Circuit")
        .vsync()
        .resizable()
        .build();

    let computer = QuantumComputer::new(3);

    let circuit = computer.bell_state()
        .apply(Toffoli(2,[1,0]))
        .apply(Fredkin(0,1,[2]))
        .measure("q0",2)
        .build()?;


    let circuit = computer.new_circuit_builder()
        .apply_sub_circuit(circuit, MaxIteration(10))
        .build()?;

    let mut camera = Camera2D::default();
    camera.target = Vector2::zero();
    camera.zoom =1.0;
    init_camera(&mut camera, &rl);

    let font_info = {
        let font_size = 48;
        let font = rl.load_font_ex(&thread,"/home/Bastien Aracil/fonts/OpenSans-Regular.ttf",font_size, FontLoadEx::Default(200));
        FontInfo::new(font?,font_size)
    };

    let parameter = DrawingPar{
        font: font_info,
        nb_qbits:computer.nb_qbits(),
        register_spacing:100.0,
        register_thickness:2.,
        background_color:Color::BLACK,
        foreground_color:Color::WHITE,
        margin:20.0,
    };

    let offset = Vector2::zero();

    let mut screen_size = (rl.get_screen_width(), rl.get_screen_height());

    while !rl.window_should_close() {
        if rl.is_window_resized() {
            init_camera(&mut camera,&rl);
            screen_size.0 = rl.get_screen_width();
            screen_size.1 = rl.get_screen_height();
        }

        let mut d = rl.begin_drawing(&thread);
        d.clear_background(parameter.background_color);

        {
            let mut d = d.begin_mode2D(camera);
            circuit.draw(&mut d, offset,&parameter);
        }


    };

    Ok(())
}

fn init_camera(camera:&mut Camera2D, d:&RaylibHandle) {
    let height = d.get_screen_height();
    let width = d.get_screen_width();

    camera.offset.x = (width as f32)*0.5;
    camera.offset.y = (height as f32)*0.5;



}