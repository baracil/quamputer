use druid::{BoxConstraints, Color, Data, Env, Event, EventCtx, LayoutCtx, LifeCycle, LifeCycleCtx, PaintCtx, Point, Size, UpdateCtx, Widget, RenderContext, Affine, Vec2};
use druid::env::Key;
use druid::widget::EnvScope;

use crate::circuit::Circuit;
use crate::gui::circuit_gui::{CircuitGui, GuiWidget, CircuitElementGui};
use crate::operation::CircuitElement;
use druid::kurbo::PathSeg::Line;
use druid::kurbo::{Shape, BezPath};

pub(crate) const LOOP_BKG_COLOR: Key<Color> = Key::new("quamputer.gui.loop.bkg.color");
pub(crate) const STROKE_COLOR: Key<Color> = Key::new("quamputer.gui.stoke.color");
pub(crate) const GATE_BKG_COLOR: Key<Color> = Key::new("quamputer.gui.gate.bkg.color");
pub(crate) const CIRCUIT_MARGIN: Key<f64> = Key::new("quamputer.gui.circuit.margin");
pub(crate) const GATE_WIDTH: Key<f64> = Key::new("quamputer.gui.gate.width");
pub(crate) const MEASURE_WIDTH: Key<f64> = Key::new("quamputer.gui.measure.width");
pub(crate) const QBITS_SEPARATION: Key<f64> = Key::new("quamputer.gui.qbits.separation");

pub(crate) const DEFAULT_LOOP_BKG_COLOR: Color = Color::rgba8(0x80, 0x80, 0x80, 0x80);
pub(crate) const DEFAULT_STROKE_COLOR: Color = Color::rgb8(0xff, 0xff, 0xff);
pub(crate) const DEFAULT_GATE_BKG_COLOR: Color = Color::rgb8(0x00, 0x00, 0x00);
pub(crate) const DEFAULT_CIRCUIT_MARGIN: f64 = 5.;
pub(crate) const DEFAULT_GATE_WIDTH: f64 = 50.;
pub(crate) const DEFAULT_MEASURE_WIDTH: f64 = 50.;
pub(crate) const DEFAULT_QBITS_SEPARATION: f64 = 50.;

pub struct CircuitDrawer {}

#[derive(Data, Clone)]
pub struct CircuitDrawerState {
    pub pos: Point,
    pub circuit: CircuitGui,
}

impl CircuitDrawer {
    pub fn with_env_scope() -> impl Widget<CircuitDrawerState> {
        EnvScope::new(
            |env, data| {
                env.set(LOOP_BKG_COLOR, DEFAULT_LOOP_BKG_COLOR);
                env.set(STROKE_COLOR, DEFAULT_STROKE_COLOR);
                env.set(CIRCUIT_MARGIN, DEFAULT_CIRCUIT_MARGIN);
                env.set(GATE_WIDTH, DEFAULT_GATE_WIDTH);
                env.set(MEASURE_WIDTH, DEFAULT_MEASURE_WIDTH);
                env.set(QBITS_SEPARATION, DEFAULT_QBITS_SEPARATION);
                env.set(GATE_BKG_COLOR, DEFAULT_GATE_BKG_COLOR);
            },
            CircuitDrawer {},
        )
    }
}

impl Widget<CircuitDrawerState> for CircuitDrawer {
    fn event(&mut self, ctx: &mut EventCtx, event: &Event, data: &mut CircuitDrawerState, env: &Env) {
        if let _WindowConnected = event {
            data.circuit.update_layout(env);
        }
    }

    fn lifecycle(&mut self, ctx: &mut LifeCycleCtx, event: &LifeCycle, data: &CircuitDrawerState, env: &Env) {}

    fn update(&mut self, ctx: &mut UpdateCtx, old_data: &CircuitDrawerState, data: &CircuitDrawerState, env: &Env) {}

    fn layout(&mut self, ctx: &mut LayoutCtx, bc: &BoxConstraints, data: &CircuitDrawerState, env: &Env) -> Size {
        bc.max()
    }

    fn paint(&mut self, ctx: &mut PaintCtx, data: &CircuitDrawerState, env: &Env) {
        let circuit = &data.circuit;
        let nb_qbits = circuit.nb_qbits;

        ctx.with_save(|c| {
            c.transform(Affine::translate(data.pos.to_vec2()));
            self.paint_qbits(nb_qbits, circuit.width, c, env);

            for element in circuit.elements.lock().unwrap().iter() {
                element.paint(c, env);
                c.transform(Affine::translate(Vec2{x:element.full_width(),y:0.0}));
            }
        });
    }
}

impl CircuitDrawer {
    fn paint_qbits(&self, nb_qbits:u8, width:f64, ctx: &mut PaintCtx, env: &Env) {
        let color = env.get(STROKE_COLOR);
        let qbits_sep = env.get(QBITS_SEPARATION);

        let mut path = BezPath::new();
        let mut start = Point{x:0.0,y:0.0};
        let mut end = Point{x:width,y:0.0};
        for i in 0..nb_qbits {
            start.y = (i as f64)*qbits_sep;
            end.y = start.y;
            path.move_to(start);
            path.line_to(end)
        }

        ctx.render_ctx.stroke(path,&color,3.0);
    }
}