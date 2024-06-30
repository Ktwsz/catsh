pub mod sprite;
pub mod framevec;
pub mod texture;
pub mod state;

use state::State;

use std::io;
use crossterm::{
    ExecutableCommand,
    terminal, 
    event,
};
use crossterm::event::{Event, KeyCode};
use std::time;


fn main() -> io::Result<()>{
    let mut cats_state = State::new();

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
                    _ => {}
                }
            }
        }
        let dt = last_tick.elapsed();
        if dt >= tick_rate {
            cats_state.update(dt);
            let _ = cats_state.draw_frame(&mut stdout, &terminal_size);
            last_tick = time::Instant::now();
        }
    }

    terminal::disable_raw_mode()?;
    stdout.execute(terminal::LeaveAlternateScreen)?;

    Ok(())
}

