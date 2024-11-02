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
            &&Rect::new_from_x_y(
                platform.frame.x,
                platform.frame.y,
                platform.frame.w * 3,
                platform.frame.h,
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

        Rect::new(self.position, platform.frame.w * 3, platform.frame.h)
    }

    pub(crate) fn bounding_boxes(&self) -> Vec<Rect> {
        const X_OFFSET: i16 = 60;
        const END_HEIGHT: i16 = 54;
        let destination_box = self.destination_box();
        vec![
            Rect::new_from_x_y(
                destination_box.x(),
                destination_box.y(),
                X_OFFSET,
                END_HEIGHT,
            ),
            Rect::new_from_x_y(
                destination_box.x() + X_OFFSET,
                destination_box.y(),
                destination_box.width - (X_OFFSET * 2),
                destination_box.height,
            ),
            Rect::new_from_x_y(
                destination_box.x() + destination_box.width - X_OFFSET,
                destination_box.y(),
                X_OFFSET,
                END_HEIGHT,
            ),
        ]
    }
}
