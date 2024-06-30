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
    pub fn new(x: i32, y:i32, width: u32, height: u32, texture_path: &str, frame_count: u32) -> Self {
        let frame_vec = FrameVec::from_image(texture_path, frame_count).unwrap();


        Sprite {
            x, y,
            //width: frame_vec.frame_width(),
            //height: frame_vec.frame_height(),
            width,
            height,
            frame_vec,
        }
    }

    pub fn pixel_at(&self, x: u32, y: u32) -> &TexturePixel {
        let w = x as f64 / self.width as f64;
        let h = y as f64 / self.height as f64;

        self.frame_vec.get_frame().get(w, h) 
    }

}

