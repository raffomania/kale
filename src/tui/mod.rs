use anyhow::Result;
use crossterm::{
    event::{KeyCode, KeyModifiers},
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
    ExecutableCommand,
};
use std::io;

mod events;

use tui::{
    backend::CrosstermBackend,
    layout::{Constraint, Direction, Layout},
    widgets::{Block, Borders},
    Terminal,
};

use self::events::Event;

pub fn start() -> Result<()> {
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    stdout.execute(EnterAlternateScreen)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;
    let rx = events::listen()?;

    loop {
        terminal.draw(|f| {
            let size = f.size();
            let constraints: Vec<Constraint> =
                (0..7).map(|_| Constraint::Percentage(100 / 7)).collect();
            let layout = Layout::default()
                .direction(Direction::Horizontal)
                .constraints(constraints)
                .split(size);
            for i in 0..7 {
                let block = Block::default()
                    .title(format!("Day {}", i))
                    .borders(Borders::all());
                f.render_widget(block, layout[i]);
            }
        })?;

        match rx.recv()? {
            Event::Input(event) => {
                if event.modifiers.contains(KeyModifiers::CONTROL)
                    && event.code == KeyCode::Char('c')
                {
                    break;
                }

                match event.code {
                    KeyCode::Char('q') => {
                        break;
                    }
                    a => {
                        dbg!(a);
                    }
                }
            }
            Event::Resize => {}
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
