use image::{RgbaImage, Rgba};
use crossterm::style::Color;

//const LIGHNTESS: &str = "$@B%8&WM#*oahkbdpqwmZO0QLCJUYXzcvunxrjft/\\|()1{}[]?-_+~<>i!lI;:,\"^`'.";
const LIGHNTESS: &str = "`.-':_,^=;><+!rc*/z?sLTv)J7(|Fi{C}fI31tlu[neoZ5Yxjya]2ESwqkP6h9d4VpOGbUAKXHm8RD#$Bg0MNWQ%&@";

type GrayScale = f64;

#[derive(Clone, Copy)]
pub struct TexturePixel<T = Color>(pub Option<(T, char)>);

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

pub struct Texture <T = Color> {
    pub width: u32,
    pub height: u32,
    data: Vec <TexturePixel<T>>,
}

impl Texture {
    pub fn from_image(image: &RgbaImage, current_frame: u32, frame_count: u32, sprite_show: crate::ShowSprite) -> Self {
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

        if let crate::ShowSprite::Sobel = sprite_show {
            apply_sobel(&mut texture);
        }

        texture
    }
}    

impl <T> Texture<T> {
    pub fn get(&self, x: i32, y: i32) -> &TexturePixel<T> {
        //let scale_x = f64::floor(self.width as f64 * x) as u32;
        //let scale_y = f64::floor(self.height as f64 * y) as u32;
        //
        //let ix = self.height * scale_x + scale_y;
        //&self.data[ix as usize]

        &self.data[(self.height as i32 * y + x) as usize]
    }

    pub fn position(&self, ix: usize) -> (u32, u32) {
        (ix as u32 / self.height, ix as u32 % self.height)
    }
}


fn apply_sobel(texture: &mut Texture) {
    let kernel_x = [
        [1.0, 0.0, -1.0],
        [2.0, 0.0, -2.0],
        [1.0, 0.0, -1.0],
    ];

    let G_x = convolute(kernel_x, texture);

    let kernel_y = [
        [1.0, 2.0, 1.0],
        [0.0, 0.0, 0.0],
        [-1.0, -2.0, -1.0],
    ];

    let G_y = convolute(kernel_y, texture);

    texture.data = (0..G_x.data.len())
        .map(|i| {
            let x2 = if let TexturePixel(Some((val, _c))) = G_x.data[i] { val * val } else { 0.0 };
            let y2 = if let TexturePixel(Some((val, _c))) = G_y.data[i] { val * val } else { 0.0 };

            let gs = f64::sqrt(x2 + y2);

            TexturePixel(Some((Color::Rgb {
                r: gs as u8,
                g: gs as u8,
                b: gs as u8,
            },
            '*')))
        })
        .collect()
}

fn convolute(kernel: [[f64; 3]; 3], texture: &Texture) -> Texture<GrayScale> {
    let grayscale_texture = grayscale(texture);

    Texture {
        width: texture.width,
        height: texture.height,
        data: (0..grayscale_texture.data.len())
            .map(|ix| {
                let (p_x, p_y) = texture.position(ix);

                let mut conv_pixel = 0.0;


                for x in -1..=1 {
                    for y in -1..=1 {
                        if p_x as i32 + x < 0 
                            || p_x as i32 + x >= texture.width as i32
                            || p_y as i32 + y < 0
                            || p_y as i32 + y >= texture.height as i32 {
                            continue;
                        }

                        let pixel_color = if let &TexturePixel::<GrayScale>(Some((color, _))) = grayscale_texture.get(x + p_x as i32, y + p_y as i32) { color } else { 0.0 };

                        conv_pixel += kernel[(1 + x) as usize][(1 + y) as usize] * pixel_color;
                    }
                }

                TexturePixel(Some((conv_pixel, '*')))
            })
            .collect()
    }
}

fn grayscale(texture: &Texture) -> Texture<GrayScale> {
    let grayscale_transform = 
        |color| if let Color::Rgb { r, g, b } = color {
            0.299 * r as f64 + 0.587 * g as f64 + 0.114 + b as f64
        } else {
            0.0
        };

    Texture {
        width: texture.width,
        height: texture.height,
        data: texture.data.iter()
            .map(|&ref pixel| {
                if let &TexturePixel(Some((color, c))) = pixel {
                    TexturePixel(Some((grayscale_transform(color), c)))
                } else  {
                    TexturePixel(None)
                }
            })
            .collect()
    }

}
