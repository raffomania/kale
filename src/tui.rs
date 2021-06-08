use anyhow::Result;
use crossterm::{
    event::{self, KeyCode},
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
    ExecutableCommand,
};
use std::{io, sync::mpsc, thread};

enum Event<I> {
    Input(I),
    Resize,
}

use tui::{
    backend::CrosstermBackend,
    layout::{Constraint, Direction, Layout},
    widgets::{Block, Borders},
    Terminal,
};

pub fn start() -> Result<()> {
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    stdout.execute(EnterAlternateScreen)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    let (tx, rx) = mpsc::channel();
    thread::spawn(move || loop {
        match event::read().unwrap() {
            event::Event::Key(key) => tx.send(Event::Input(key)).unwrap(),
            event::Event::Resize(_, _) => tx.send(Event::Resize).unwrap(),
            _ => {}
        }
    });

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
            Event::Input(event) => match event.code {
                KeyCode::Char('q') => {
                    break;
                }
                _ => {}
            },
            Event::Resize => {}
        }
    }

    disable_raw_mode()?;
    terminal.backend_mut().execute(LeaveAlternateScreen)?;
    terminal.show_cursor()?;
    Ok(())
}
