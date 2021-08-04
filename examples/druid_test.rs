use druid::widget::prelude::*;
use druid::widget::{Flex, Label, TextBox};
use druid::{AppLauncher, Data, Lens, UnitPoint, WidgetExt, WindowDesc, Point, Color};
use druid::kurbo::{Line, Circle};

const VERTICAL_WIDGET_SPACING: f64 = 20.0;
const TEXT_BOX_WIDTH: f64 = 200.0;

pub fn main() {
    // describe the main window
    let main_window = WindowDesc::new(build_root_widget())
        .title("Hello World!")
        .window_size((400.0, 400.0));

    // create the initial app state
    let initial_state = CanvasData::new();

    // start the application. Here we pass in the application state.
    AppLauncher::with_window(main_window)
        .log_to_console()
        .launch(initial_state)
        .expect("Failed to launch application");
}

fn build_root_widget() -> impl Widget<CanvasData> {
    let canvas = Canvas{};
    return canvas;
    // // a label that will determine its text based on the current app data.
    // let label = Label::new(|data: &HelloState, _env: &Env| {
    //     if data.name.is_empty() {
    //         "Hello anybody!?".to_string()
    //     } else {
    //         format!("Hello {}!", data.name)
    //     }
    // })
    //     .with_text_size(32.0);
    //
    //
    //
    //
    // // a textbox that modifies `name`.
    // let textbox = TextBox::new()
    //     .with_placeholder("Who are we greeting?")
    //     .with_text_size(18.0)
    //     .fix_width(TEXT_BOX_WIDTH)
    //     .lens(HelloState::name);
    //
    // // arrange the two widgets vertically, with some padding
    // Flex::column()
    //     .with_child(label)
    //     .with_spacer(VERTICAL_WIDGET_SPACING)
    //     .with_child(textbox)
    //     .align_vertical(UnitPoint::CENTER)
}

pub struct Canvas {
}

#[derive(Data,Clone)]
pub struct CanvasData {
    mouse_position : Option<Point>,
    stroke_color : Color
}

impl CanvasData {

    pub fn new() -> Self {
        return CanvasData{mouse_position:None,stroke_color:Color::rgb8(0x00, 0x80, 0x00)}
    }
}

impl Widget<CanvasData> for Canvas {

    fn event(&mut self, ctx: &mut EventCtx, event: &Event, data: &mut CanvasData, env: &Env) {
        match event {
            Event::MouseMove(m) => {
                data.mouse_position = Some(m.pos);
                ctx.request_paint();
            }
            _ => {}
        }
    }

    fn lifecycle(&mut self, ctx: &mut LifeCycleCtx, event: &LifeCycle, data: &CanvasData, env: &Env) {
    }

    fn update(&mut self, ctx: &mut UpdateCtx, old_data: &CanvasData, data: &CanvasData, env: &Env) {
    }

    fn layout(&mut self, ctx: &mut LayoutCtx, bc: &BoxConstraints, data: &CanvasData, env: &Env) -> Size {
        let size = bc.max();
        println!("{}",size);
        bc.max()
    }

    fn paint(&mut self, ctx: &mut PaintCtx, data: &CanvasData, env: &Env) {
        let canvas_size = ctx.size();

        let line = Line::new(Point{x:0.0,y:0.0},Point{x:canvas_size.width, y:canvas_size.height});
        let color = Color::rgb8(0xFF, 0xFF, 0xFF);

        ctx.render_ctx.stroke(line,&color,10.0 );


        if let Some(point) = data.mouse_position {
            let circle = Circle::new(point,10.0);

            ctx.render_ctx.stroke(circle,&data.stroke_color,2.0)
        }
    }

}