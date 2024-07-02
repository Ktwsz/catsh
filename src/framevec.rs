use crate::texture::Texture;
use image::{
    ImageError,
    DynamicImage::*,
    io::Reader,
};

pub struct FrameVec {
    frames: Vec <Texture>,
    current_frame: usize
}

impl FrameVec {
    pub fn from_image(img_path: &str, frame_count: u32, sprite_show: crate::ShowSprite) -> Result<Self, ImageError> {
        let mut frame_vec = FrameVec {
            frames: Vec::new(),
            current_frame: 0,
        };

        let dyn_img = Reader::open(img_path)?
            .decode()?;

        if let ImageRgba8(img) = dyn_img {
            for img_frame in 0..frame_count {
                frame_vec.frames.push(Texture::from_image(&img, img_frame, frame_count, sprite_show));
            }
        }

        Ok(frame_vec)
    }

    pub fn frame_width(&self) -> u32 {
        self.frames[0].width
    }

    pub fn frame_height(&self) -> u32 {
        self.frames[0].height
    }


    pub fn next_frame(&mut self) {
        self.current_frame = (self.current_frame + 1 ) % self.frames.len();
    }

    pub fn get_frame(&self) -> &Texture {
        &self.frames[self.current_frame]
    }
}

