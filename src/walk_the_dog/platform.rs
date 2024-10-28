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
            &self.destination_box(),
        );

        for bounding_box in &self.bounding_boxes() {
            renderer.draw_rect(bounding_box);
        }
    }

    pub(crate) fn destination_box(&self) -> Rect {
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

    pub(crate) fn bounding_boxes(&self) -> Vec<Rect> {
        const X_OFFSET: f32 = 60.0;
        const END_HEIGHT: f32 = 54.0;
        let destination_box = self.destination_box();
        vec![
            Rect::new(destination_box.x, destination_box.y, X_OFFSET, END_HEIGHT),
            Rect::new(
                destination_box.x + X_OFFSET,
                destination_box.y,
                destination_box.width - (X_OFFSET * 2.0),
                destination_box.height,
            ),
            Rect::new(
                destination_box.x + destination_box.width - X_OFFSET,
                destination_box.y,
                X_OFFSET,
                END_HEIGHT,
            ),
        ]
    }
}
