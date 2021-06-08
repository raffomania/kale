use anyhow::{Context, Result};
use read::read_events;
use std::convert::TryFrom;

mod event;
mod read;
mod tui;

fn main() -> Result<()> {
    let events = read_events()?.into_iter().map(|(path, event)| {
        event::Event::try_from(event.clone())
            .context(format!(
                "Could not conform event {:?}:\n {:#?}",
                path, event.properties
            ))
            .unwrap()
    });
    tui::start()?;
    Ok(())
}
