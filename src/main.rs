mod snake;
mod food;
mod game;
mod ui;

use game::Game;
use snake::Direction;
use crossterm::event::{self, Event, KeyCode};
use crossterm::{terminal, cursor, execute};
use std::io;


fn main() -> io::Result<()> {
    let mut game= Game::new(40, 20);
    
    //Enable raw mode for keyboard input
    terminal::enable_raw_mode()?;
    let mut stdout= io::stdout();
    execute!(stdout, cursor::Hide)?;

    let mut last_tick = std::time::Instant::now();
    let tick_rate = std::time::Duration::from_millis(100);

    loop{
        ui::render(&game);

        if game.game_over{
            //wait for restart or quit
            if let Event::Key(key)= event::read()?{
                match key.code{
                    KeyCode::Char('r')=> {
                        game.reset();
                        last_tick = std::time::Instant::now();
                    }
                    KeyCode::Char('q')=> break,
                    _=>{}
                }
            }
            continue;
        }

        //Process all input events during this tick
        let mut next_dir = None;
        let mut should_quit = false;

        while last_tick.elapsed() < tick_rate {
            let timeout = tick_rate.saturating_sub(last_tick.elapsed());
            if event::poll(timeout)? {
                if let Event::Key(key) = event::read()? {
                    match key.code {
                        KeyCode::Up => next_dir = Some(Direction::Up),
                        KeyCode::Down => next_dir = Some(Direction::Down),
                        KeyCode::Left => next_dir = Some(Direction::Left),
                        KeyCode::Right => next_dir = Some(Direction::Right),
                        KeyCode::Char('q') => {
                            should_quit = true;
                            break;
                        }
                        _ => {}
                    }
                }
            }
        }

        if should_quit {
            break;
        }

        if let Some(dir) = next_dir {
            game.snake.set_direction(dir);
        }

        game.update();
        game.check_collisions();
        last_tick = std::time::Instant::now();
    }

    //Disable raw mode and show cursor before exiting
    execute!(io::stdout(), cursor::Show)?;
    terminal::disable_raw_mode()?;
    Ok(())
}
