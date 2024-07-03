use image::{RgbaImage, Rgba};
use crossterm::style::Color;

const LIGHNTESS: &str = ".;coPO?#%@";

type GrayScale = f64;

#[derive(Clone, Copy)]
pub struct TexturePixel<T = Color>(pub Option<(T, char)>);

#[derive(Clone, Copy)]
enum EdgePiece {
    Vertical,
    Horizontal,
    DashRight,
    DashLeft
}

const HORIZONTAL_CHAR: char = '-';
const VERTICAL_CHAR: char = '|';
const DASHRIGHT_CHAR: char = '/';
const DASHLEFT_CHAR: char = '\\';

impl TexturePixel {
    fn new(pixel: &Rgba<u8>) -> Self {
        if pixel[3] < u8::MAX {
            return TexturePixel(None);
        }

        let hsl_color: colorsys::Hsl = colorsys::Rgb::from(&(pixel[0], pixel[1], pixel[2])).into();
        let char_ix = LIGHNTESS.len() as f64 * (hsl_color.lightness() / 100.0);


        let default = LIGHNTESS.chars().nth(LIGHNTESS.len() - 1).unwrap();
        TexturePixel::new_tuple(Color::Rgb {
            r: pixel[0],
            g: pixel[1],
            b: pixel[2],
        }, LIGHNTESS.chars().nth(f64::floor(char_ix) as usize).unwrap_or(default))
    }
}

impl <T> TexturePixel<T> {
    fn new_tuple(color: T, c: char) -> TexturePixel<T> {
        TexturePixel(Some((
            color, c
        )))
    }

    fn empty() -> TexturePixel<T> {
        TexturePixel(None)
    }

    fn map<U, F>(&self, f: F) -> TexturePixel<U>
        where F: FnOnce((T, char)) -> (U, char),
              T: Copy {
        let &TexturePixel(opt) = self;
        
        TexturePixel(opt.map(f))
    }

    fn color_or(&self, default: T) -> T 
    where T: Copy {
        if let &TexturePixel::<T>(Some((color, _))) = self {
            color
        } else {
            default
        }
    }

    fn char_or(&self, default: char) -> char
    where T: Copy {
        if let &TexturePixel::<T>(Some((_, c))) = self {
            c
        } else {
            default
        }
    }
}

pub struct Texture <T = Color> {
    pub width: u32,
    pub height: u32,
    data: Vec <TexturePixel<T>>,
    edge_texture: Vec<TexturePixel<EdgePiece>>,
}

impl Texture {
    pub fn from_image(image: &RgbaImage, current_frame: u32, frame_count: u32, sprite_show: crate::ShowSprite) -> Self {
        let frame_width = image.width() / frame_count;
        let frame_height = image.height();

        let mut texture = Texture {
            width: frame_width,
            height: frame_height,
            data: Vec::new(),
            edge_texture: Vec::new(),
        };

        for x in 0..frame_width {
            for y in 0..frame_height {
                let pixel_color = image.get_pixel(current_frame * frame_width + x, y);
                texture.data.push(TexturePixel::new(pixel_color));
            }
        }

        texture.edge_texture = sobel(&texture);

        texture
    }
}    

impl <T> Texture<T> {
    pub fn get(&self, x: i32, y: i32) -> &TexturePixel<T> {
        &self.data[(self.height as i32 * y + x) as usize]
    }

    pub fn position(&self, ix: usize) -> (u32, u32) {
        (ix as u32 % self.height, ix as u32 / self.height)
    }
}


fn sobel_transform(grayscale_texture: &Texture<GrayScale>, kernel_x: &[[f64; 3]; 3], kernel_y: &[[f64; 3]; 3], i: usize) -> TexturePixel <EdgePiece> {
    let x = convolute_transform(kernel_x, grayscale_texture, i).color_or(0.0);
    let y = convolute_transform(kernel_y, grayscale_texture, i).color_or(0.0);

    let threshold = 0.1;
    if f64::sqrt(x*x + y*y) < threshold {
        return TexturePixel::empty();
    }

    let theta = f64::atan2(y, x);
    let abs_theta = f64::abs(theta) / std::f64::consts::PI;

    use EdgePiece::*;
    match abs_theta {
        v if 0.0 <= v && v < 0.05 => TexturePixel::new_tuple(Horizontal, HORIZONTAL_CHAR),
        v if 0.9 < v && v <= 1.0 => TexturePixel::new_tuple(Horizontal, HORIZONTAL_CHAR),
        v if 0.45 < v && v < 0.55 => TexturePixel::new_tuple(Vertical, VERTICAL_CHAR),
        v if 0.05 < v && v < 0.45 => if theta > 0.0 {
                TexturePixel::new_tuple(DashRight, DASHRIGHT_CHAR)
            } else {
                TexturePixel::new_tuple(DashLeft, DASHLEFT_CHAR)
            },
        v if 0.55 < v && v < 0.9 => if theta < 0.0 {
                TexturePixel::new_tuple(DashRight, DASHRIGHT_CHAR)
            } else {
                TexturePixel::new_tuple(DashLeft, DASHLEFT_CHAR)
            },
        _ => TexturePixel::empty(),
    }

    
}

fn sobel(texture: &Texture) -> Vec <TexturePixel<EdgePiece>> {
    let kernel_x = [
        [1.0, 0.0, -1.0],
        [2.0, 0.0, -2.0],
        [1.0, 0.0, -1.0],
    ];

    let kernel_y = [
        [1.0, 2.0, 1.0],
        [0.0, 0.0, 0.0],
        [-1.0, -2.0, -1.0],
    ];

    let grayscale_texture = grayscale(texture);

    (0..texture.data.len())
        .map(|i| sobel_transform(&grayscale_texture, &kernel_x, &kernel_y, i))
        .collect()
}

fn convolute_transform(kernel: &[[f64; 3]; 3], texture: &Texture<GrayScale>, ix: usize) -> TexturePixel<GrayScale> {
    let (p_x, p_y) = texture.position(ix);

    let mut conv_pixel = 0.0;

    let pixel_char = texture.data[ix].char_or(LIGHNTESS.chars().nth(0).unwrap());

    for x in -1..=1 {
        for y in -1..=1 {
            if p_x as i32 + x < 0 
                || p_x as i32 + x >= texture.width as i32
                    || p_y as i32 + y < 0
                    || p_y as i32 + y >= texture.height as i32 {
                        continue;
            }

            let pixel_color = texture.get(x + p_x as i32, y + p_y as i32).color_or(0.0);

            conv_pixel += kernel[(1 + x) as usize][(1 + y) as usize] * pixel_color;
        }
    }

    TexturePixel::new_tuple(conv_pixel, pixel_char)
}

fn grayscale_transform(pixel: &TexturePixel) -> TexturePixel<GrayScale> {
    pixel.map(|pixel_data| {
        let pixel_color = pixel_data.0;
        let pixel_char = pixel_data.1;

        if let Color::Rgb {r, g, b } = pixel_color {
            (0.299 * r as f64 + 0.587 * g as f64 + 0.114 + b as f64, pixel_char)
        } else {
            (0.0, pixel_char)
        }
    })
}

fn grayscale(texture: &Texture) -> Texture<GrayScale> {
    Texture {
        width: texture.width,
        height: texture.height,
        data: texture.data.iter()
            .map(grayscale_transform)
            .collect(),

        edge_texture: Vec::new(),
    }
}
