use druid::{Env, PaintCtx, Point, Rect, RenderContext, Affine, Vec2};

use crate::_loop::Loop;
use crate::condition::StopCondition;
use crate::gui::circuit_drawer::{CIRCUIT_MARGIN, QBITS_SEPARATION, LOOP_BKG_COLOR, STROKE_COLOR};
use crate::gui::circuit_gui::{CircuitGui, GuiWidget};
use druid::tests::helpers::Record::Layout;


pub struct LoopGuiLayout {
    pub margin: f64,
    pub width: f64,
    pub rect:Rect,
}

pub struct LoopGui {
    pub layout: Option<LoopGuiLayout>,
    /// the circuit that makes the content of the loop
    pub circuit: CircuitGui,
    /// the condition used to stop the loop
    pub stop_condition: StopCondition,
}

impl LoopGuiLayout {
    fn full_width(&self) -> f64 {
        self.width + 2.*self.margin
    }

}

impl From<&Loop> for LoopGui {
    fn from(lp: &Loop) -> Self {
        LoopGui::new(&lp.circuit, &lp.stop_condition)
    }
}

impl From<&LoopGui> for Loop {
    fn from(lp: &LoopGui) -> Loop {
        Loop::new(&lp.circuit,&lp.stop_condition)
    }
}

impl LoopGui {

    pub fn new(circuit:impl Into<CircuitGui>, stop_condition:&StopCondition) -> Self {
        LoopGui{layout:None,circuit:circuit.into(),stop_condition:stop_condition.clone()}
    }

}

impl GuiWidget for LoopGui {
    fn update_layout(&mut self, env: &Env) {
        let qbit_sep = env.get(QBITS_SEPARATION);
        let margin = env.get(CIRCUIT_MARGIN);

        self.circuit.update_layout(env);
        let nb_qbits = self.circuit.nb_qbits;
        let width = self.circuit.full_width();
        let rect = Rect{x0:margin,x1:margin+width, y0: -qbit_sep*0.5, y1: (nb_qbits as f64 - 0.5)*qbit_sep};
        let layout = LoopGuiLayout{width,margin,rect};
        self.layout = Some(layout);
    }

    fn full_width(&self) -> f64 {
        self.layout.as_ref().map_or(0.0,|e| {e.full_width()})
    }

    fn paint(&self, ctx: &mut PaintCtx, env: &Env) {

        if let Some(layout) = &self.layout {
            ctx.with_save(|c| {
                let bkg_color = env.get(LOOP_BKG_COLOR);
                let stroke_color = env.get(STROKE_COLOR);

                c.fill(layout.rect, &bkg_color);
                c.stroke(layout.rect, &stroke_color, 1.0);

                c.transform(Affine::translate(Vec2{x:layout.margin,y:0.0}));
                self.circuit.paint(c, env)
            })
        }
    }
}
