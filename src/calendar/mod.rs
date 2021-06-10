use self::entry::Entry;

use anyhow::{Context, Result};
use chrono::NaiveDate;
use std::convert::TryFrom;

mod entry;
mod read;

pub struct Calendar {
    entries: Vec<Entry>,
}

impl Calendar {
    pub fn new() -> Result<Self> {
        let entries: Vec<Entry> = self::read::read_events()?
            .into_iter()
            .map(|(path, event)| {
                Entry::try_from(event.clone())
                    .context(format!(
                        "Could not conform event {:?}:\n {:#?}",
                        path, event.properties
                    ))
                    .unwrap()
            })
            .collect();

        Ok(Calendar { entries })
    }

    pub fn entries_for_day(&self, date: NaiveDate) -> Vec<&Entry> {
        self.entries
            .iter()
            .filter(|e| date == e.start_date.naive_local().date())
            .collect()
    }
}
