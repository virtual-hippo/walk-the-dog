use web_sys::HtmlImageElement;

use super::{Point, Rect, Renderer};

pub(crate) struct Image {
    element: HtmlImageElement,
    position: Point,
    bounding_box: Rect,
}

impl Image {
    pub(crate) fn new(element: HtmlImageElement, position: Point) -> Self {
        let bounding_box = Rect::new(
            position.x.into(),
            position.y.into(),
            element.width() as f32,
            element.height() as f32,
        );
        Self {
            element,
            position,
            bounding_box,
        }
    }

    pub(crate) fn draw(&self, renderer: &Renderer) {
        renderer.draw_entire_image(&self.element, &self.position);
    }

    pub(crate) fn bounding_box(&self) -> &Rect {
        &self.bounding_box
    }

    pub(crate) fn move_horizontally(&mut self, distance: i16) {
        self.set_x(self.position.x + distance);
    }

    pub(crate) fn set_x(&mut self, x: i16) {
        self.bounding_box.x = x as f32;
        self.position.x = x;
    }

    pub(crate) fn right(&self) -> i16 {
        (self.bounding_box.x + self.bounding_box.width) as i16
    }
}
