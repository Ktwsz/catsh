use crate::sprite::Sprite;
use crate::texture::TexturePixel;

use std::io::{self, Write, Stdout};
use crossterm::{
    ExecutableCommand, QueueableCommand,
    terminal, cursor, style::{self},
};

use std::time::Duration;

const CAT_YELLOW: &str = "cats/Cat-1/Cat-1-Run.png";
const CAT_YELLOW_FRAMES: u32 = 8;
const CAT_BLACK: &str = "cats/Cat-2/Cat-2-Licking-1.png";
const CAT_BLACK_FRAMES: u32 = 5;

pub struct State {
    sprites: Vec <Sprite>,
    animation_ctr: i32,
    animation_ctr_2: i32,
    animation_delay: i32,
    delay_done: bool,
}

impl State {
    pub fn new(sprite_show: crate::ShowSprite) -> Self {
        State {
            sprites: vec![
                Sprite::new(CAT_YELLOW, CAT_YELLOW_FRAMES, sprite_show).set_pos(-30, 0),
                Sprite::new(CAT_BLACK, CAT_BLACK_FRAMES, sprite_show).set_pos(30, 10),
            ],

            animation_ctr: 0,
            animation_ctr_2: 0,
            animation_delay: 100,
            delay_done: false,

        }
    }

    pub fn update(&mut self, _dt: Duration) {
        self.animation_ctr += 1;
        self.animation_ctr_2 += 1;

        if (self.delay_done && self.animation_ctr == 5) || (!self.delay_done && self.animation_ctr == self.animation_delay) {
            self.animation_ctr = 0;
            self.delay_done = true;

            self.sprites[0].frame_vec.next_frame();
            self.sprites[0].x += 10;
        }

        if self.animation_ctr_2 == 10 {
            self.sprites[1].frame_vec.next_frame();
            self.animation_ctr_2 = 0;
        }
    }
    

    pub fn draw_frame(&self, stdout: &mut Stdout, terminal_size: &(u16, u16)) -> io::Result<()> {
        stdout.execute(terminal::Clear(terminal::ClearType::All))?;


        self.draw_sprite(stdout, terminal_size, 0)?;
        self.draw_sprite(stdout, terminal_size, 1)?;

        stdout.flush()?;
        Ok(())
    }

    fn draw_sprite(&self, stdout: &mut Stdout, terminal_size: &(u16, u16), sprite_id: usize) -> io::Result<()> {
        let sprite = &self.sprites[sprite_id];

        for w in 0..sprite.width {
            for h in 0..sprite.height {
                let coord = (sprite.x + w as i32, (sprite.y + h as i32));
                if coord.0 < 0 || coord.0 >= terminal_size.0 as i32 || coord.1 < 0 || coord.1 >= terminal_size.1 as i32 {
                    continue;
                }


                if let &TexturePixel(Some((color, c))) = sprite.pixel_at(w, h) { 
                    stdout
                        .queue(cursor::MoveTo(coord.0 as u16, coord.1 as u16))?
                        .queue(style::SetForegroundColor(color))?
                        .queue(style::Print(c))?;
                }
            }
        }
        Ok(())
    }
}
