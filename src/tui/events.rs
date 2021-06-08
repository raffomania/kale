use anyhow::Result;
use std::{sync::mpsc, thread};

use crossterm::event;
use signal_hook::{
    consts::{SIGINT, SIGTERM},
    iterator::Signals,
};

pub enum Event {
    Input(event::KeyEvent),
    Resize,
    Quit,
}

pub fn listen() -> Result<mpsc::Receiver<Event>> {
    let (tx, rx) = mpsc::channel();
    let terminal_events = tx.clone();
    thread::spawn(move || loop {
        match event::read().unwrap() {
            event::Event::Key(key) => terminal_events.send(Event::Input(key)).unwrap(),
            event::Event::Resize(_, _) => terminal_events.send(Event::Resize).unwrap(),
            _ => {}
        }
    });

    let mut signals = Signals::new(&[SIGTERM, SIGINT])?;

    let signal_events = tx;
    thread::spawn(move || {
        for _ in signals.forever() {
            signal_events.send(Event::Quit).unwrap();
        }
    });
    Ok(rx)
}
