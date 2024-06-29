use image::io::Reader;
use image::ImageError;
use image::DynamicImage::*;
use image::Rgba;
use crossterm::style::Color;
use colorsys;


const FILEPATH: &str = "cats/Cat-1/Cat-1-Run.png";
const FRAMES: u32 = 8;

const LIGHNTESS: &str = "$@B%8&WM#*oahkbdpqwmZO0QLCJUYXzcvunxrjft/\\|()1{}[]?-_+~<>i!lI;:,\"^`'.";

pub struct Square {
    pub height: u32,
    pub data: Vec <Option<(Color, char)>>,
}

pub struct Sprite {
    pub width: u32,
    pub height: u32,
    pub x: i32,
    pub y: i32,
    pub frames: Vec <Square>,

    current_frame: usize
}

impl Square {
    pub fn get(&self, x: u32, y: u32) -> Option<(Color, char)> {
        let ix = self.height * x + y;
        self.data[ix as usize]
    }
}

impl Sprite {
    pub fn get(&self, x: u32, y: u32) -> Option<(Color, char)> {
        self.frames[self.current_frame].get(x, y)
    }

    pub fn next_frame(&mut self) {
        self.current_frame = (self.current_frame + 1 ) % self.frames.len();
    }
}

fn pixel_to_data(pixel: &Rgba<u8>) -> Option<(Color, char)> {
        if pixel[3] < u8::MAX {
            return None;
        }

        let hsl_color: colorsys::Hsl = colorsys::Rgb::from(&(pixel[0], pixel[1], pixel[2])).into();
        let char_ix = LIGHNTESS.len() as f64 * (hsl_color.lightness() / 100.0);


        Some((Color::Rgb {
            r: pixel[0],
            g: pixel[1],
            b: pixel[2],
        }, LIGHNTESS.chars().nth(f64::floor(char_ix) as usize).unwrap()))
}

pub fn load_data() -> Result<Sprite, ImageError> {
    let dyn_img = Reader::open(FILEPATH)?
        .decode()?;

    if let ImageRgba8(img) = dyn_img {
        let frame_width = img.width() / FRAMES;
        let frame_height = img.height();

        let mut sprite = Sprite {
            x: -(frame_width as i32),
            y: 0,
            width: frame_width,
            height: frame_height,
            frames: Vec::new(),
            current_frame: 0,
        };

        for img_frame in 0..FRAMES {
            let mut result = Square {
                height: frame_height,
                data: Vec::new(),
            };

            for x in 0..frame_width {
                for y in 0..frame_height {
                    let pixel_color = img.get_pixel(img_frame * frame_width + x, y);
                    result.data.push(pixel_to_data(pixel_color));
                }
            }
            sprite.frames.push(result);
        }
        Ok(sprite)
    } else {
        Ok(Sprite  {
            x: 0,
            y: 0,
            width: 0,
            height: 0,
            frames: Vec::new(),
            current_frame: 0,
        })
    }
}
