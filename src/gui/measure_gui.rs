use druid::{ArcStr, Env, FontFamily, PaintCtx, Point, Rect, RenderContext, TextLayout, FontDescriptor, Size};
use druid::kurbo::Shape;
use serde::__private::de::IdentifierDeserializer;

use crate::gui::circuit_drawer::{CIRCUIT_MARGIN, DEFAULT_MEASURE_WIDTH, GATE_BKG_COLOR, MEASURE_WIDTH, QBITS_SEPARATION, STROKE_COLOR};
use crate::gui::circuit_gui::GuiWidget;
use crate::measure::Measure;
use std::ops::{Sub, MulAssign, Mul};

pub struct MeasureGuiLayout {
    pub margin: f64,
    pub width: f64,
    pub center:Point,
    pub size:Size,
    pub rect:Rect,
}

pub struct MeasureGui {
    pub layout:Option<MeasureGuiLayout>,
    /// Uniq value used to identify the measurement
    pub id: String,
    /// the qbit to measure
    pub qbit_target: u8,
}

impl MeasureGuiLayout {
    fn full_width(&self) -> f64 {
        self.margin*2.+self.width
    }
}

impl From<&Measure> for MeasureGui {
    fn from(m: &Measure) -> Self {
        MeasureGui { layout:None, id: m.id.clone(), qbit_target: m.qbit_target }
    }
}

impl From<&MeasureGui> for Measure {
    fn from(measure: &MeasureGui) -> Self {
        Measure { qbit_target: measure.qbit_target, id: measure.id.clone() }
    }
}

impl GuiWidget for MeasureGui {
    fn update_layout(&mut self, env: &Env) {
        let qbit_sep = env.get(QBITS_SEPARATION);
        let margin = env.get(CIRCUIT_MARGIN);
        let width = env.get(MEASURE_WIDTH);
        let measure_y = (self.qbit_target as f64) * qbit_sep;

        let size = Size::new(width,qbit_sep);
        let center = Point{x:margin+width*0.5,y:measure_y};

        let rect = Rect::from_center_size(center,size);

        let layout = MeasureGuiLayout{width,margin,center,size,rect};

        self.layout = Some(layout);
    }

    fn full_width(&self) -> f64 {
        self.layout.as_ref().map_or(0.0,|l| {l.full_width()})
    }

    fn paint(&self, ctx: &mut PaintCtx, env: &Env) {
        let bkg_color = env.get(GATE_BKG_COLOR);
        let stroke_color = env.get(STROKE_COLOR);

        if let Some(layout) = &self.layout {
            ctx.fill(layout.rect, &bkg_color);
            ctx.stroke(layout.rect, &stroke_color, 1.0);

            let mut id_layout = TextLayout::<String>::from_text(&self.id);
            id_layout.set_font(FontDescriptor::new(FontFamily::SERIF).with_size(18.0));
            id_layout.set_text_color(stroke_color);
            id_layout.rebuild_if_needed(ctx.text(), env);
            let size = id_layout.size();

            let point = layout.center.sub(size.to_vec2().mul(0.5));
            id_layout.draw(ctx,point)

        }


    }
}
