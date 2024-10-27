use crate::{
    engine::{Point, Rect, Renderer},
    walk_the_dog::*,
};

use web_sys::HtmlImageElement;

pub(crate) struct Platform {
    sheet: Sheet,
    image: HtmlImageElement,
    pub(crate) position: Point,
}

impl Platform {
    pub(crate) fn new(sheet: Sheet, image: HtmlImageElement, position: Point) -> Self {
        Platform {
            sheet,
            image,
            position,
        }
    }

    pub(crate) fn draw(&self, renderer: &Renderer) {
        let platform = self
            .sheet
            .frames
            .get("13.png")
            .expect("13.png does not exist");

        renderer.draw_image(
            &self.image,
            &Rect::new(
                platform.frame.x.into(),
                platform.frame.y.into(),
                (platform.frame.w * 3).into(),
                platform.frame.h.into(),
            ),
            &self.bounding_box(),
        );
    }

    pub(crate) fn bounding_box(&self) -> Rect {
        let platform = self
            .sheet
            .frames
            .get("13.png")
            .expect("13.png does not exist");

        Rect::new(
            self.position.x.into(),
            self.position.y.into(),
            (platform.frame.w * 3).into(),
            platform.frame.h.into(),
        )
    }
}
