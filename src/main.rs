pub mod sprite;
pub mod framevec;
pub mod texture;

use sprite::Sprite;
use texture::TexturePixel;

use std::io::{self, Write, Stdout};
use crossterm::{
    ExecutableCommand, QueueableCommand,
    terminal, cursor, style::{self},
    event,
};
use crossterm::event::{Event, KeyCode};
use std::time;

const FILEPATH: &str = "cats/Cat-1/Cat-1-Run.png";
const FRAMES: u32 = 8;


fn main() -> io::Result<()>{
    let mut sprite = Sprite::new(0, 0, 30, 30, FILEPATH, FRAMES);

    let fps = 60.0;
    let tick_rate = time::Duration::from_millis(f64::floor(1000.0 / fps) as u64);
    let mut last_tick = time::Instant::now();

    let mut animation_ctr = 0;
    let animation_delay = 100;
    let mut delay_done = false;

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
        if last_tick.elapsed() >= tick_rate {
            let _ = draw_frame(&mut stdout, &sprite, &terminal_size);
            last_tick = time::Instant::now();

            animation_ctr += 1;
            if (delay_done && animation_ctr == 5) || (!delay_done && animation_ctr == animation_delay) {
                animation_ctr = 0;
                delay_done = true;
                sprite.frame_vec.next_frame();
                sprite.x += 10;
            }
        }
    }

    terminal::disable_raw_mode()?;
    stdout.execute(terminal::LeaveAlternateScreen)?;

    Ok(())
}

fn draw_frame(stdout: &mut Stdout, sprite: &Sprite, terminal_size: &(u16, u16)) -> io::Result<()> {
    stdout.execute(terminal::Clear(terminal::ClearType::All))?;


    draw_square(stdout, &sprite, terminal_size)?;

    stdout.flush()?;
    Ok(())
}

fn draw_square(stdout: &mut Stdout, sprite: &Sprite, terminal_size: &(u16, u16)) -> io::Result<()> {
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
