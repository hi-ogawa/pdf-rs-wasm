use super::{Fill, FontEntry, TextSpan};
use font::Glyph;
use pathfinder_content::{fill::FillRule, outline::Outline, stroke::StrokeStyle};
use pathfinder_geometry::{rect::RectF, transform2d::Transform2F};
use pdf::error::PdfError;
use pdf::font::Font as PdfFont;
use pdf::object::{ImageXObject, MaybeRef, Ref, Resolve, Resources, XObject};
use std::sync::Arc;

pub trait Backend {
    fn set_clip_path(&mut self, path: Option<&Outline>);
    fn draw(
        &mut self,
        outline: &Outline,
        mode: &DrawMode,
        fill_rule: FillRule,
        transform: Transform2F,
    );
    fn set_view_box(&mut self, r: RectF);
    fn draw_image(
        &mut self,
        xref: Ref<XObject>,
        im: &ImageXObject,
        resources: &Resources,
        transform: Transform2F,
        resolve: &impl Resolve,
    );
    fn draw_inline_image(
        &mut self,
        im: &Arc<ImageXObject>,
        resources: &Resources,
        transform: Transform2F,
        resolve: &impl Resolve,
    );
    fn draw_glyph(&mut self, glyph: &Glyph, mode: &DrawMode, transform: Transform2F) {
        self.draw(&glyph.path, mode, FillRule::Winding, transform);
    }
    fn get_font(
        &mut self,
        font_ref: &MaybeRef<PdfFont>,
        resolve: &impl Resolve,
    ) -> Result<Option<Arc<FontEntry>>, PdfError>;
    fn add_text(&mut self, span: TextSpan);
}
#[derive(Clone)]
pub enum DrawMode {
    Fill(Fill, f32),
    Stroke(Fill, f32, Stroke),
    FillStroke(Fill, f32, Fill, f32, Stroke),
}
#[derive(Clone, Debug)]
pub struct Stroke {
    pub dash_pattern: Option<(Vec<f32>, f32)>,
    pub style: StrokeStyle,
}
