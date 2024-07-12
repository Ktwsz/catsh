pub mod sprite;
pub mod framevec;
pub mod texture;
pub mod state;
pub mod animation;
pub mod position;

use crate::animation::Interpolator;
use state::State;
use position::Position;
use std::time::Duration;

use std::io;
use crossterm::{
    ExecutableCommand,
    terminal, 
    event,
};
use crossterm::event::{Event, KeyCode};
use std::time;

pub mod debug {
    #[derive(Clone, Copy)]
    pub enum ShowSprite {
        Final,
        Sobel,
    }

    pub struct Binding {
        pub is_animation_running: bool,
        pub sprite_show_layer: ShowSprite,
    }
}

use crate::debug::{Binding, ShowSprite};


fn main() -> io::Result<()>{
    let mut debug_binding = Binding {
        is_animation_running: true,
        sprite_show_layer: ShowSprite::Sobel,
    };

    let mut cats_state = State::new();

    cats_state.get_sprite_mut(0).position.set_animation(
        Duration::from_millis(6000),
        Interpolator::linear(Position::zero(), Position::new(50.0, 0.0)),
    );

    let fps = 60.0;
    let tick_rate = time::Duration::from_millis(f64::floor(1000.0 / fps) as u64);
    let mut last_tick = time::Instant::now();

    let mut stdout = io::stdout();

    terminal::enable_raw_mode()?;
    stdout.execute(terminal::EnterAlternateScreen)?;

    let terminal_size = terminal::size().unwrap();

    loop {
        let timeout = tick_rate.saturating_sub(last_tick.elapsed());
        if event::poll(timeout)? {
            if let Event::Key(key) = event::read()? {
                match key.code {
                    KeyCode::Char('q') => break,
                    KeyCode::Char('s') => {
                        debug_binding.is_animation_running = !debug_binding.is_animation_running;
                    },
                    KeyCode::Char('1') => {
                        debug_binding.sprite_show_layer = ShowSprite::Final;
                    },
                    KeyCode::Char('2') => {
                        debug_binding.sprite_show_layer = ShowSprite::Sobel;
                    },
                    _ => {}
                }
            }
        }
        let dt = last_tick.elapsed();
        if dt >= tick_rate {
            if debug_binding.is_animation_running {
                cats_state.update(dt);
            }
            let _ = cats_state.draw_frame(&mut stdout, &terminal_size, debug_binding.sprite_show_layer);
            last_tick = time::Instant::now();
        }
    }

    terminal::disable_raw_mode()?;
    stdout.execute(terminal::LeaveAlternateScreen)?;

    Ok(())
}

