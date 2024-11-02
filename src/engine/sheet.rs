use serde::Deserialize;
use std::collections::HashMap;

#[derive(Deserialize, Clone)]
pub(crate) struct SheetRect {
    pub(crate) x: i16,
    pub(crate) y: i16,
    pub(crate) w: i16,
    pub(crate) h: i16,
}

#[derive(Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub(crate) struct Cell {
    pub(crate) frame: SheetRect,
    pub(crate) sprite_source_size: SheetRect,
}

#[derive(Deserialize, Clone)]
pub(crate) struct Sheet {
    pub(crate) frames: HashMap<String, Cell>,
}
