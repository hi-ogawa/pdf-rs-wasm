use std::{fmt::Debug, sync::Arc};

use pathfinder_geometry::rect::RectF;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use serde_with::SerializeAs;

use crate::FontEntry;

// define serialization/schema for remote types
// - https://serde.rs/field-attrs.html
// - https://github.com/GREsau/schemars
// - https://github.com/jonasbb/serde_with

//
// RectF / LocalRectF
//

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

//
// Arc<FontEntry> / LocalFontEntry
//

#[derive(Serialize, Deserialize, JsonSchema)]
pub struct LocalArcFontEntry {
    name: String,
}

#[derive(Serialize, Deserialize, JsonSchema)]
pub struct LocalOptionArcFontEntry(Option<LocalArcFontEntry>);

serde_with::serde_conv!(
    pub AsLocalArcFontEntry,
    Arc<FontEntry>,
    |remote: &Arc<FontEntry>| LocalArcFontEntry { name: remote.name.clone() },
    |_: LocalArcFontEntry| -> Result<Arc<FontEntry>, _> { Err("RectF") }
);

//
// SerializeAsDebug
//

#[derive(Serialize, Deserialize, JsonSchema)]
pub struct SerializeAsDebug {
    debug: String,
}

impl<T: Debug> SerializeAs<T> for SerializeAsDebug {
    fn serialize_as<S>(source: &T, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        SerializeAsDebug::serialize(
            &SerializeAsDebug {
                debug: format!("{:?}", source),
            },
            serializer,
        )
    }
}
