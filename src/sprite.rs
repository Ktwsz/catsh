use crate::framevec::FrameVec;
use crate::texture::TexturePixel;
use crate::position::Position;
use crate::animation::AnimateProperty;

use std::time::Duration;

pub struct Sprite {
    pub width: u32,
    pub height: u32,
    pub position: AnimateProperty<Position>,

    pub frame_vec: FrameVec,
}

impl Sprite {
    pub fn new(texture_path: &str, frame_count: u32) -> Self {
        let frame_vec = FrameVec::from_image(texture_path, frame_count).unwrap();


        Sprite {
            position: AnimateProperty::new(Position::zero()),
            width: frame_vec.frame_width(),
            height: frame_vec.frame_height(),
            frame_vec,
        }
    }

    pub fn set_pos(mut self, pos: Position) -> Self {
       self.position.property = pos;
       self
    }

    pub fn resize(self, width: u32, height: u32) -> Self {
        self
    }

    pub fn pixel_at(&self, x: u32, y: u32, sprite_show_layer: crate::debug::ShowSprite) -> TexturePixel {
        let w = x;
        let h = y;

        self.frame_vec.get_frame().get_draw_pixel(w as i32, h as i32, sprite_show_layer) 
    }

    pub fn update(&mut self, dt: Duration) {
        self.position.update(dt);
    }

}

