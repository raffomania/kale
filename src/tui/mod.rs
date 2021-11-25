use anyhow::Result;
use chrono::{DateTime, Datelike, Duration, Local};
use crossterm::{
    event::{KeyCode, KeyEvent, KeyModifiers},
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
    ExecutableCommand,
};
use std::{convert::TryFrom, io};

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

struct State {
    date: DateTime<Local>,
}

pub fn start(calendar: Calendar, locale: locale_config::Locale) -> Result<()> {
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    stdout.execute(EnterAlternateScreen)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;
    let rx = events::listen()?;

    let date = Local::now();
    let mut state = State { date };
    let date_locale = locale
        .tags_for("time")
        .next()
        .map(|range| locale_config::Locale::from(range))
        .unwrap_or(locale_config::Locale::user_default());
    let date_locale = date_locale.to_string().replace("-", "_");
    let date_locale =
        chrono::Locale::try_from(date_locale.as_str()).unwrap_or(chrono::Locale::en_US);

    loop {
        terminal.draw(|f| {
            let size = f.size();
            let layout = Layout::default()
                .constraints(vec![Constraint::Length(2), Constraint::Min(0)])
                .split(size);
            let status_bar = StatusBar::new(state.date.iso_week(), date_locale);
            f.render_widget(status_bar, layout[0]);
            let table = EntryTable::new(&calendar, state.date.iso_week(), date_locale);
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
                    KeyCode::Char('l') => {
                        state.date = state.date + Duration::weeks(1);
                    }
                    KeyCode::Char('h') => {
                        state.date = state.date - Duration::weeks(1);
                    }
                    _ => {}
                }
            }
            Event::Resize => {
                // dbg!("resize");
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
