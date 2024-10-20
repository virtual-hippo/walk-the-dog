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
pub(super) struct Cell {
    pub(super) frame: SheetRect,
}

#[derive(Deserialize, Clone)]
pub(super) struct Sheet {
    pub(super) frames: HashMap<String, Cell>,
}
