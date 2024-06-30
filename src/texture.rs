use image::{RgbaImage, Rgba};
use crossterm::style::Color;

const LIGHNTESS: &str = "$@B%8&WM#*oahkbdpqwmZO0QLCJUYXzcvunxrjft/\\|()1{}[]?-_+~<>i!lI;:,\"^`'.";

pub struct TexturePixel(pub Option<(Color, char)>);

impl TexturePixel {
    fn new(pixel: &Rgba<u8>) -> Self {
        if pixel[3] < u8::MAX {
            return TexturePixel(None);
        }

        let hsl_color: colorsys::Hsl = colorsys::Rgb::from(&(pixel[0], pixel[1], pixel[2])).into();
        let char_ix = LIGHNTESS.len() as f64 * (hsl_color.lightness() / 100.0);


        TexturePixel(Some((Color::Rgb {
            r: pixel[0],
            g: pixel[1],
            b: pixel[2],
        }, LIGHNTESS.chars().nth(f64::floor(char_ix) as usize).unwrap())))
    }

}

pub struct Texture {
    pub width: u32,
    pub height: u32,
    data: Vec <TexturePixel>,
}

impl Texture {
    pub fn from_image(image: &RgbaImage, current_frame: u32, frame_count: u32) -> Self {
        let frame_width = image.width() / frame_count;
        let frame_height = image.height();

        let mut texture = Texture {
            width: frame_width,
            height: frame_height,
            data: Vec::new(),
        };

        for x in 0..frame_width {
            for y in 0..frame_height {
                let pixel_color = image.get_pixel(current_frame * frame_width + x, y);
                texture.data.push(TexturePixel::new(pixel_color));
            }
        }

        texture
    }
    

    pub fn get(&self, x: f64, y: f64) -> &TexturePixel {
        let scale_x = f64::floor(self.width as f64 * x) as u32;
        let scale_y = f64::floor(self.height as f64 * y) as u32;

        let ix = self.height * scale_x + scale_y;
        &self.data[ix as usize]
    }
}

