use web_sys::HtmlImageElement;

use super::{Point, Rect, Renderer};

pub(crate) struct Image {
    element: HtmlImageElement,
    bounding_box: Rect,
}

impl Image {
    pub(crate) fn new(element: HtmlImageElement, position: Point) -> Self {
        let bounding_box = Rect::new(position, element.width() as i16, element.height() as i16);

        Self {
            element,
            bounding_box,
        }
    }

    pub(crate) fn draw(&self, renderer: &Renderer) {
        renderer.draw_entire_image(&self.element, &self.bounding_box().position);
    }

    pub(crate) fn bounding_box(&self) -> &Rect {
        &self.bounding_box
    }

    pub(crate) fn move_horizontally(&mut self, distance: i16) {
        self.set_x(self.bounding_box().x() + distance);
    }

    pub(crate) fn set_x(&mut self, x: i16) {
        self.bounding_box.set_x(x);
    }

    pub(crate) fn right(&self) -> i16 {
        self.bounding_box.right()
    }
}
