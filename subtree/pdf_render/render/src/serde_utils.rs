use pathfinder_geometry::rect::RectF;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

// define serialization/schema for remote types
// - https://serde.rs/field-attrs.html
// - https://github.com/GREsau/schemars
// - https://github.com/jonasbb/serde_with

#[derive(Serialize, Deserialize, JsonSchema)]
pub struct LocalRectF {
    x: f32,
    y: f32,
    w: f32,
    h: f32,
}

#[derive(Serialize, Deserialize, JsonSchema)]
pub struct LocalOptionRectF(Option<LocalRectF>);

serde_with::serde_conv!(
    pub AsLocalRectF,
    RectF,
    |remote: &RectF| LocalRectF { x: remote.origin_x(), y: remote.origin_y(), w: remote.width(), h: remote.height() },
    |_: LocalRectF| -> Result<RectF, _> { Err("RectF") }
);
