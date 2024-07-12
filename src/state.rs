use crate::{
    sprite::Sprite,
    position::Position,
};
use crate::texture::TexturePixel;

use std::{
    io::{self, Write, Stdout},
    time::Duration,
};
use crossterm::{
    ExecutableCommand, QueueableCommand,
    terminal, cursor, style::{self},
};

const CAT_YELLOW: &str = "cats/Cat-1/Cat-1-Run.png";
const CAT_YELLOW_FRAMES: u32 = 8;
const CAT_BLACK: &str = "cats/Cat-2/Cat-2-Licking-1.png";
const CAT_BLACK_FRAMES: u32 = 5;

pub struct State {
    pub sprites: Vec <Sprite>,
}

impl State  {
    pub fn new() -> Self {
        State {
            sprites: vec![
                Sprite::new(CAT_YELLOW, CAT_YELLOW_FRAMES).set_pos(Position::new(-30.0, 0.0)),
                Sprite::new(CAT_BLACK, CAT_BLACK_FRAMES).set_pos(Position::new(30.0, 10.0)),
            ],
        }
    }

    pub fn update(&mut self, dt: Duration) {
        for sprite in &mut self.sprites {
            sprite.update(dt);
        }
    }
    

    pub fn draw_frame(&self, stdout: &mut Stdout, terminal_size: &(u16, u16), sprite_show_layer: crate::debug::ShowSprite) -> io::Result<()> {
        stdout.execute(terminal::Clear(terminal::ClearType::All))?;


        self.draw_sprite(stdout, terminal_size, 0, sprite_show_layer)?;
        self.draw_sprite(stdout, terminal_size, 1, sprite_show_layer)?;

        stdout.flush()?;
        Ok(())
    }

    fn draw_sprite(&self, stdout: &mut Stdout, terminal_size: &(u16, u16), sprite_id: usize, sprite_show_layer: crate::debug::ShowSprite) -> io::Result<()> {
        let sprite = &self.sprites[sprite_id];

        for w in 0..sprite.width {
            for h in 0..sprite.height {
                let coord = (sprite.position.property.x_i32() + w as i32, (sprite.position.property.y_i32() + h as i32));
                if coord.0 < 0 || coord.0 >= terminal_size.0 as i32 || coord.1 < 0 || coord.1 >= terminal_size.1 as i32 {
                    continue;
                }


                if let TexturePixel(Some((color, c))) = sprite.pixel_at(w, h, sprite_show_layer) { 
                    stdout
                        .queue(cursor::MoveTo(coord.0 as u16, coord.1 as u16))?
                        .queue(style::SetForegroundColor(color))?
                        .queue(style::Print(c))?;
                }
            }
        }
        Ok(())
    }

    pub fn get_sprite_mut(&mut self, id: usize) -> &mut Sprite {
        &mut self.sprites[id]
    }
}
