use crate::framevec::FrameVec;
use crate::texture::TexturePixel;

pub struct Sprite {
    pub width: u32,
    pub height: u32,
    pub x: i32,
    pub y: i32,

    pub frame_vec: FrameVec,
}

impl Sprite {
    pub fn new(texture_path: &str, frame_count: u32) -> Self {
        let frame_vec = FrameVec::from_image(texture_path, frame_count).unwrap();


        Sprite {
            x: 0,
            y: 0,
            width: frame_vec.frame_width(),
            height: frame_vec.frame_height(),
            frame_vec,
        }
    }

    pub fn set_pos(mut self, x: i32, y: i32) -> Self {
        self.x = x;
        self.y = y;

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

}

