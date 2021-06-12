use anyhow::Result;
use chrono::{Datelike, Local};
use crossterm::{
    event::{KeyCode, KeyEvent, KeyModifiers},
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
    ExecutableCommand,
};
use std::io;

mod entry_table;
mod events;
mod status_bar;

use tui::{
    backend::CrosstermBackend,
    layout::{Constraint, Layout},
    Terminal,
};

use crate::{calendar::Calendar, tui::status_bar::StatusBar};

use self::{entry_table::EntryTable, events::Event};

pub fn start(calendar: Calendar) -> Result<()> {
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    stdout.execute(EnterAlternateScreen)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;
    let rx = events::listen()?;

    loop {
        terminal.draw(|f| {
            let size = f.size();
            let date = Local::now();
            let week = date.iso_week();
            let layout = Layout::default()
                .constraints(vec![Constraint::Length(2), Constraint::Min(0)])
                .split(size);
            let status_bar = StatusBar::new(week);
            f.render_widget(status_bar, layout[0]);
            let table = EntryTable::new(&calendar, week);
            f.render_widget(table, layout[1]);
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
