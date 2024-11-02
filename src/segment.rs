use std::rc::Rc;

use web_sys::HtmlImageElement;

use crate::engine::{Image, Point, Rect, SpriteSheet};
use crate::walk_the_dog::{Barrier, Obstacle, Platform};

const FIRST_PLATFORM: i16 = 370;
const LOW_PLATFORM: i16 = 420;

const STONE_ON_GROUND: i16 = 546;

pub(crate) fn stone_and_platform(
    stone: HtmlImageElement,
    sprite_sheet: Rc<SpriteSheet>,
    offset_x: i16,
) -> Vec<Box<dyn Obstacle>> {
    const INITIAL_STONE_OFFSET: i16 = 150;

    const FLOATING_PLATFORM_SPRITES: [&str; 3] = ["13.png", "14.png", "15.png"];
    const FLOATING_PLATFORM_BOUNDING_BOXES: [Rect; 3] = [
        Rect::new_from_x_y(0, 0, 60, 54),
        Rect::new_from_x_y(60, 0, 384 - (60 * 2), 93),
        Rect::new_from_x_y(384 - 60, 0, 60, 54),
    ];

    vec![
        Box::new(Barrier::new(Image::new(
            stone,
            Point {
                x: offset_x + INITIAL_STONE_OFFSET,
                y: STONE_ON_GROUND,
            },
        ))),
        Box::new(create_floating_platform(
            sprite_sheet,
            Point {
                x: offset_x + FIRST_PLATFORM,
                y: LOW_PLATFORM,
            },
            &FLOATING_PLATFORM_SPRITES,
            &FLOATING_PLATFORM_BOUNDING_BOXES,
        )),
    ]
}

pub(crate) fn platform_and_stone(
    stone: HtmlImageElement,
    sprite_sheet: Rc<SpriteSheet>,
    offset_x: i16,
) -> Vec<Box<dyn Obstacle>> {
    const INITIAL_STONE_OFFSET: i16 = 450;

    const FLOATING_PLATFORM_SPRITES: [&str; 1] = ["14.png"];
    const FLOATING_PLATFORM_BOUNDING_BOXES: [Rect; 1] = [Rect::new_from_x_y(0, 0, 128, 93)];

    vec![
        Box::new(Barrier::new(Image::new(
            stone,
            Point {
                x: offset_x + INITIAL_STONE_OFFSET,
                y: STONE_ON_GROUND,
            },
        ))),
        Box::new(create_floating_platform(
            sprite_sheet,
            Point {
                x: offset_x + FIRST_PLATFORM,
                y: LOW_PLATFORM,
            },
            &FLOATING_PLATFORM_SPRITES,
            &FLOATING_PLATFORM_BOUNDING_BOXES,
        )),
    ]
}

fn create_floating_platform(
    sprite_sheet: Rc<SpriteSheet>,
    position: Point,
    sprite_names: &[&str],
    bounding_boxes: &[Rect],
) -> Platform {
    Platform::new(sprite_sheet, position, sprite_names, bounding_boxes)
}
