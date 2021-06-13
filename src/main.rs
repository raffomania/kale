use anyhow::Result;
use calendar::Calendar;
use locale_config::Locale;

mod calendar;
mod tui;

fn main() -> Result<()> {
    let calendar = Calendar::new()?;
    let locale = Locale::current();
    tui::start(calendar, locale)?;
    Ok(())
}
