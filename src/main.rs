use anyhow::{Context, Result};
use read::read_events;
use std::convert::TryFrom;

mod event;
mod read;

fn main() -> Result<()> {
    let events = read_events()?;
    for (path, event) in events {
        let event = event::Event::try_from(event.clone()).context(format!(
            "Could not conform event {:?}:\n {:#?}",
            path, event.properties
        ))?;
        // println!("{:?}", event);
    }
    // println!("{:?}", events.first().unwrap());
    Ok(())
}
