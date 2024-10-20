use web_sys::HtmlImageElement;

use super::{Point, Renderer};

pub(crate) struct Image {
    element: HtmlImageElement,
    position: Point,
}

impl Image {
    pub(crate) fn new(element: HtmlImageElement, position: Point) -> Self {
        Self { element, position }
    }

    pub(crate) fn draw(&self, renderer: &Renderer) {
        renderer.draw_entire_image(&self.element, &self.position);
    }
}
