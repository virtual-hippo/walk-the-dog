use web_sys::{CanvasRenderingContext2d, HtmlImageElement};

use super::{Point, Rect};

pub struct Renderer {
    context: CanvasRenderingContext2d,
}

impl Renderer {
    pub fn new(context: CanvasRenderingContext2d) -> Self {
        Renderer { context }
    }

    pub fn clear(&self, rect: &Rect) {
        self.context.clear_rect(
            rect.x().into(),
            rect.y().into(),
            rect.width.into(),
            rect.height.into(),
        );
    }

    pub fn draw_image(&self, image: &HtmlImageElement, frame: &Rect, destination: &Rect) {
        self.context
            .draw_image_with_html_image_element_and_sw_and_sh_and_dx_and_dy_and_dw_and_dh(
                &image,
                frame.x().into(),
                frame.y().into(),
                frame.width.into(),
                frame.height.into(),
                destination.x().into(),
                destination.y().into(),
                destination.width.into(),
                destination.height.into(),
            )
            .expect("Drawing is throwing exceptions! Unrecoverable error.");
    }

    pub fn draw_entire_image(&self, image: &HtmlImageElement, position: &Point) {
        self.context
            .draw_image_with_html_image_element(image, position.x.into(), position.y.into())
            .expect("Drawing is throwing exceptions! Unrecoverable error.");
        self.draw_rect(&Rect::new(
            *position,
            image.width() as i16,
            image.height() as i16,
        ));
    }

    pub fn draw_rect(&self, bounding_box: &Rect) {
        self.context.set_stroke_style_str("FF0000");
        self.context.begin_path();
        self.context.rect(
            bounding_box.x().into(),
            bounding_box.y().into(),
            bounding_box.width.into(),
            bounding_box.height.into(),
        );
        self.context.stroke();
    }
}
