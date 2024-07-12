pub mod sprite;
pub mod framevec;
pub mod texture;
pub mod state;
pub mod animation;
pub mod position;

use crate::{
    animation::Interpolator,
    debug::{Binding, ShowSprite},
    position::Position,
    sprite::Sprite,
    state::State,
};

use std::{
    io,
    time,
    time::Duration,
};

use crossterm::{
    ExecutableCommand,
    terminal, 
    event,
    event::{Event, KeyCode},
};

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


const CAT_YELLOW: &str = "cats/Cat-1/Cat-1-Run.png";
const CAT_YELLOW_FRAMES: u32 = 8;
const CAT_BLACK: &str = "cats/Cat-2/Cat-2-Licking-1.png";
const CAT_BLACK_FRAMES: u32 = 5;
const FPS: f64 = 60.0;
const TICK_RATE: f64 = 1000.0 / FPS;


fn main() -> io::Result<()> {
    let mut debug_binding = Binding {
        is_animation_running: true,
        sprite_show_layer: ShowSprite::Sobel,
    };

    let mut cats_state = State::new(
        vec![
        Sprite::new(CAT_YELLOW, CAT_YELLOW_FRAMES).set_pos(Position::new(-30.0, 0.0)),
        Sprite::new(CAT_BLACK, CAT_BLACK_FRAMES).set_pos(Position::new(30.0, 10.0)),
        ]
    );

    cats_state.get_sprite_mut(0).position.set_animation(
        Duration::from_millis(6000),
        Interpolator::linear(Position::zero(), Position::new(50.0, 0.0)),
    );

    let tick_rate = time::Duration::from_millis(f64::floor(TICK_RATE) as u64);
    let mut last_tick = time::Instant::now();

    let mut stdout = io::stdout();

    let _ = init_terminal(&mut stdout);

    let terminal_size = terminal::size().unwrap();

    loop {
        if read_events(&last_tick, &tick_rate, &mut debug_binding)? {
            break;
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

    exit_terminal(&mut stdout)
}

fn read_events(last_tick: &time::Instant, tick_rate: &time::Duration, debug_binding: &mut debug::Binding) -> io::Result<bool> {
    let timeout = tick_rate.saturating_sub(last_tick.elapsed());
    if event::poll(timeout)? {
        if let Event::Key(key) = event::read()? {
            match key.code {
                KeyCode::Char('q') => Ok(true),
                KeyCode::Char('s') => {
                    debug_binding.is_animation_running = !debug_binding.is_animation_running;
                    Ok(false)
                },
                KeyCode::Char('1') => {
                    debug_binding.sprite_show_layer = ShowSprite::Final;
                    Ok(false)
                },
                KeyCode::Char('2') => {
                    debug_binding.sprite_show_layer = ShowSprite::Sobel;
                    Ok(false)
                },
                _ => Ok(false)
            }
        } else { Ok(false) }
    } else { Ok(false) }
}

fn init_terminal(stdout: &mut io::Stdout) -> io::Result<()> {
    terminal::enable_raw_mode()?;
    stdout.execute(terminal::EnterAlternateScreen)?;
    Ok(())
}

fn exit_terminal(stdout: &mut io::Stdout) -> io::Result<()> {
    terminal::disable_raw_mode()?;
    stdout.execute(terminal::LeaveAlternateScreen)?;
    Ok(())
}

