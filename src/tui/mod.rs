use anyhow::Result;
use crossterm::{
    event::{KeyCode, KeyEvent, KeyModifiers},
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
    ExecutableCommand,
};
use std::io;

mod entries;
mod events;

use tui::{backend::CrosstermBackend, Terminal};

use crate::calendar::Calendar;

use self::events::Event;

pub fn start(calendar: Calendar) -> Result<()> {
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    stdout.execute(EnterAlternateScreen)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;
    let rx = events::listen()?;

    loop {
        terminal.draw(|f| {
            entries::draw(f, &calendar);
        })?;

        match rx.recv()? {
            Event::Input(event) => {
                if let KeyEvent {
                    modifiers: KeyModifiers::CONTROL,
                    code: KeyCode::Char('c'),
                } = event
                {
                    break;
                }

                match event.code {
                    KeyCode::Char('q') => {
                        break;
                    }
                    _ => {}
                }
            }
            Event::Resize => {
                dbg!("resize");
            }
            Event::Quit => {
                break;
            }
        }
    }

    disable_raw_mode()?;
    terminal.backend_mut().execute(LeaveAlternateScreen)?;
    terminal.show_cursor()?;
    Ok(())
}
