use std::convert::TryInto;

use chrono::{prelude::*, Duration};
use chrono::{IsoWeek, NaiveDate, Timelike, Weekday};
use std::convert::TryFrom;
use tui::{
    layout::Rect,
    text::Span,
    widgets::{Block, Borders, List, ListItem, Widget},
};

use crate::calendar::Calendar;

pub struct EntryTable<'a> {
    entries: &'a Calendar,
    week: IsoWeek,
    locale: chrono::Locale,
}

impl<'a> EntryTable<'a> {
    pub fn new(entries: &'a Calendar, week: IsoWeek, locale: chrono::Locale) -> Self {
        EntryTable {
            entries,
            week,
            locale,
        }
    }
}

impl<'a> Widget for EntryTable<'a> {
    fn render(self, size: Rect, buf: &mut tui::buffer::Buffer) {
        let width = size.width / 7;
        let cols: Vec<Rect> = (0..7)
            .map(|i| Rect::new(size.x + width * i, size.y, width, size.height))
            .collect();
        let start_date = NaiveDate::from_isoywd(self.week.year(), self.week.week(), Weekday::Mon);
        for i in 0usize..7 {
            let day = start_date + Duration::days(i.try_into().unwrap());

            let timezone_day = Local.from_local_date(&day).unwrap();
            let block = Block::default()
                .title(
                    timezone_day
                        .format_localized("%a, %x", self.locale)
                        .to_string(),
                )
                .borders(Borders::all());

            let day_list = build_list_for_day(size.height, self.entries, day).block(block);
            day_list.render(cols[i], buf);
        }
    }
}

fn build_list_for_day(lines: u16, entries: &Calendar, day: NaiveDate) -> List {
    let entries = entries.entries_for_day(day);

    let mut items: Vec<ListItem> = (0..lines).map(|_| ListItem::new(Span::raw(""))).collect();

    for entry in entries {
        let time = entry.start_date.time().format("%H:%M");
        let label = format!("{} {}", time, entry.name.clone());

        let lines_per_slot = lines / 24;
        let index: u16 = lines_per_slot * u16::try_from(entry.start_date.time().hour()).unwrap();

        items[usize::try_from(index).unwrap()] = ListItem::new(Span::raw(label));
    }

    List::new(items)
}
