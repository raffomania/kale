use anyhow::Result;
use calendar::Calendar;

mod calendar;
mod tui;

fn main() -> Result<()> {
    let calendar = Calendar::new()?;
    tui::start(calendar)?;
    Ok(())
}
