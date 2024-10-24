use serde::Deserialize;
use std::collections::HashMap;

#[derive(Deserialize, Clone)]
pub(super) struct SheetRect {
    pub(super) x: i16,
    pub(super) y: i16,
    pub(super) w: i16,
    pub(super) h: i16,
}

#[derive(Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub(super) struct Cell {
    pub(super) frame: SheetRect,
    pub(super) sprite_source_size: SheetRect,
}

#[derive(Deserialize, Clone)]
pub(super) struct Sheet {
    pub(super) frames: HashMap<String, Cell>,
}
