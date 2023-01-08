use crate::backend::Stroke;
use crate::font::{load_font, StandardCache};
use crate::{Backend, DrawMode, Fill, FontEntry, TextSpan};
use font::Glyph;
use globalcache::sync::SyncCache;
use pathfinder_content::{fill::FillRule, outline::Outline};
use pathfinder_geometry::{rect::RectF, transform2d::Transform2F, vector::Vector2F};
use pdf::error::PdfError;
use pdf::font::Font as PdfFont;
use pdf::object::{ImageXObject, MaybeRef, Ref, Resolve, Resources, XObject};
use schemars::JsonSchema;
use serde::Serialize;
use serde_with::serde_as;
use std::path::PathBuf;
use std::sync::Arc;

pub struct Tracer<'a> {
    items: Vec<DrawItem>,
    view_box: RectF,
    cache: &'a TraceCache,
}
pub struct TraceCache {
    fonts: Arc<SyncCache<usize, Option<Arc<FontEntry>>>>,
    std: StandardCache,
}
impl TraceCache {
    pub fn new() -> Self {
        let standard_fonts =
            PathBuf::from(std::env::var_os("STANDARD_FONTS").expect("no STANDARD_FONTS"));

        TraceCache {
            fonts: SyncCache::new(),
            std: StandardCache::new(standard_fonts),
        }
    }

    pub fn new_embedded() -> Self {
        TraceCache {
            fonts: SyncCache::new(),
            std: StandardCache::new_embedded(),
        }
    }
}
impl<'a> Tracer<'a> {
    pub fn new(cache: &'a TraceCache) -> Self {
        Tracer {
            items: vec![],
            view_box: RectF::new(Vector2F::zero(), Vector2F::zero()),
            cache,
        }
    }
    pub fn view_box(&self) -> RectF {
        self.view_box
    }
    pub fn finish(self) -> Vec<DrawItem> {
        self.items
    }
}
impl<'a> Backend for Tracer<'a> {
    fn set_clip_path(&mut self, path: Option<&Outline>) {
        self.items.push(DrawItem::ClipPath(path.cloned()));
    }
    fn draw(
        &mut self,
        outline: &Outline,
        mode: &DrawMode,
        _fill_rule: FillRule,
        transform: Transform2F,
    ) {
        let stroke = match *mode {
            DrawMode::FillStroke(_, _, fill, alpha, ref style)
            | DrawMode::Stroke(fill, alpha, ref style) => Some((fill, alpha, style.clone())),
            DrawMode::Fill(_, _) => None,
        };
        self.items.push(DrawItem::Vector(VectorPath {
            outline: outline.clone(),
            fill: match *mode {
                DrawMode::Fill(fill, alpha) | DrawMode::FillStroke(fill, alpha, _, _, _) => {
                    Some((fill, alpha))
                }
                _ => None,
            },
            stroke,
            transform,
        }));
    }
    fn set_view_box(&mut self, r: RectF) {
        self.view_box = r;
    }
    fn draw_image(
        &mut self,
        xref: Ref<XObject>,
        _im: &ImageXObject,
        _resources: &Resources,
        transform: Transform2F,
        _resolve: &impl Resolve,
    ) {
        let rect = transform * RectF::new(Vector2F::new(0.0, 0.0), Vector2F::new(1.0, 1.0));
        self.items
            .push(DrawItem::Image(ImageObject { rect, id: xref }));
    }
    fn draw_inline_image(
        &mut self,
        im: &Arc<ImageXObject>,
        _resources: &Resources,
        transform: Transform2F,
        _resolve: &impl Resolve,
    ) {
        let rect = transform * RectF::new(Vector2F::new(0.0, 0.0), Vector2F::new(1.0, 1.0));

        self.items.push(DrawItem::InlineImage(InlineImageObject {
            rect,
            im: im.clone(),
        }));
    }
    fn draw_glyph(&mut self, _glyph: &Glyph, _mode: &DrawMode, _transform: Transform2F) {}
    fn get_font(
        &mut self,
        font_ref: &MaybeRef<PdfFont>,
        resolve: &impl Resolve,
    ) -> Result<Option<Arc<FontEntry>>, PdfError> {
        let mut error = None;
        let val = self
            .cache
            .fonts
            .get(&**font_ref as *const PdfFont as usize, || {
                match load_font(font_ref, resolve, &self.cache.std) {
                    Ok(Some(f)) => Some(Arc::new(f)),
                    Ok(None) => None,
                    Err(e) => {
                        error = Some(e);
                        None
                    }
                }
            });
        match error {
            None => Ok(val),
            Some(e) => Err(e),
        }
    }
    fn add_text(&mut self, span: TextSpan) {
        self.items.push(DrawItem::Text(span));
    }
}

#[derive(Debug)]
pub struct ImageObject {
    pub rect: RectF,
    pub id: Ref<XObject>,
}
#[derive(Debug)]
pub struct InlineImageObject {
    pub rect: RectF,
    pub im: Arc<ImageXObject>,
}

#[serde_as]
#[derive(Serialize, JsonSchema, Debug)]
#[serde(tag = "type", content = "data")]
pub enum DrawItem {
    Vector(
        #[schemars(with = "crate::serde_utils::SerializeAsDebug")]
        #[serde_as(as = "crate::serde_utils::SerializeAsDebug")]
        VectorPath,
    ),
    Image(
        #[schemars(with = "crate::serde_utils::SerializeAsDebug")]
        #[serde_as(as = "crate::serde_utils::SerializeAsDebug")]
        ImageObject,
    ),
    InlineImage(
        #[schemars(with = "crate::serde_utils::SerializeAsDebug")]
        #[serde_as(as = "crate::serde_utils::SerializeAsDebug")]
        InlineImageObject,
    ),
    Text(TextSpan),
    ClipPath(
        #[schemars(with = "crate::serde_utils::SerializeAsDebug")]
        #[serde_as(as = "crate::serde_utils::SerializeAsDebug")]
        Option<Outline>,
    ),
}

#[derive(Debug)]
pub struct VectorPath {
    pub outline: Outline,
    pub fill: Option<(Fill, f32)>,
    pub stroke: Option<(Fill, f32, Stroke)>,
    pub transform: Transform2F,
}
