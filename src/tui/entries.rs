use std::{convert::TryInto, io::Stdout};

use chrono::{Datelike, Duration, Local, NaiveDate, Timelike, Weekday};
use std::convert::TryFrom;
use tui::{
    backend::CrosstermBackend,
    layout::Rect,
    text::Span,
    widgets::{Block, Borders, List, ListItem},
    Frame,
};

use crate::calendar::Calendar;

pub fn draw(f: &mut Frame<CrosstermBackend<Stdout>>, entries: &Calendar) {
    let size = f.size();
    let width = size.width / 7;
    let cols: Vec<Rect> = (0..7)
        .map(|i| Rect::new(size.x + width * i, size.y, width, size.height))
        .collect();
    let start_date = NaiveDate::from_isoywd(
        Local::now().year(),
        Local::now().iso_week().week(),
        Weekday::Mon,
    );
    for i in 0usize..7 {
        let day = start_date + Duration::days(i.try_into().unwrap());

        let block = Block::default()
            .title(day.format("%a, %x").to_string())
            .borders(Borders::all());

        let day_list = build_list_for_day(size.height, entries, day).block(block);
        f.render_widget(day_list, cols[i]);
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
