use web_sys::{CanvasRenderingContext2d, HtmlImageElement};

use super::Point;

pub struct Rect {
    x: f32,
    y: f32,
    width: f32,
    height: f32,
}

impl Rect {
    pub fn new(x: f32, y: f32, width: f32, height: f32) -> Self {
        Rect {
            x,
            y,
            width,
            height,
        }
    }

    pub fn intersects(&self, other: &Rect) -> bool {
        self.x < (other.x + other.width)
            && (self.x + self.width) > other.x
            && self.y < (other.y + other.height)
            && (self.y + self.height) > other.y
    }
}

pub struct Renderer {
    context: CanvasRenderingContext2d,
}

impl Renderer {
    pub fn new(context: CanvasRenderingContext2d) -> Self {
        Renderer { context }
    }

    pub fn clear(&self, rect: &Rect) {
        self.context.clear_rect(
            rect.x.into(),
            rect.y.into(),
            rect.width.into(),
            rect.height.into(),
        );
    }

    pub fn draw_image(&self, image: &HtmlImageElement, frame: &Rect, destination: &Rect) {
        self.context
            .draw_image_with_html_image_element_and_sw_and_sh_and_dx_and_dy_and_dw_and_dh(
                &image,
                frame.x.into(),
                frame.y.into(),
                frame.width.into(),
                frame.height.into(),
                destination.x.into(),
                destination.y.into(),
                destination.width.into(),
                destination.height.into(),
            )
            .expect("Drawing is throwing exceptions! Unrecoverable error.");
        self.draw_rect(destination);
    }

    pub fn draw_entire_image(&self, image: &HtmlImageElement, position: &Point) {
        self.context
            .draw_image_with_html_image_element(image, position.x.into(), position.y.into())
            .expect("Drawing is throwing exceptions! Unrecoverable error.");
        self.draw_rect(&Rect::new(
            position.x.into(),
            position.y.into(),
            image.width() as f32,
            image.height() as f32,
        ));
    }

    fn draw_rect(&self, destination: &Rect) {
        self.context.set_stroke_style_str("blue");
        self.context.stroke_rect(
            destination.x.into(),
            destination.y.into(),
            destination.width.into(),
            destination.height.into(),
        );
    }
}
