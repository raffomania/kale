use chrono::{Datelike, IsoWeek, NaiveDate};
use tui::{
    layout::{Constraint, Direction, Layout},
    style::{Modifier, Style},
    text::{Span, Spans},
    widgets::Widget,
};

pub struct StatusBar {
    week: IsoWeek,
}

impl StatusBar {
    pub fn new(week: IsoWeek) -> Self {
        StatusBar { week }
    }
}

impl Widget for StatusBar {
    fn render(self, area: tui::layout::Rect, buf: &mut tui::buffer::Buffer) {
        let start_of_week =
            NaiveDate::from_isoywd(self.week.year(), self.week.week(), chrono::Weekday::Mon);
        let start_of_month = start_of_week.with_day(1).unwrap();
        let week_in_month = start_of_week.iso_week().week() - start_of_month.iso_week().week() + 1;
        let month = start_of_week.format("%B");
        let date_text = format!("Week {}, {} {}", week_in_month, month, start_of_week.year());
        let date_span = Span::styled(date_text, Style::default().add_modifier(Modifier::DIM));
        let kale = Span::from(format!("🥬"));
        buf.set_span(area.x, area.y, &kale, area.width);
        let x = area.x + area.width - (date_span.width() as u16);
        buf.set_span(x, area.y, &date_span, area.width);
    }
}
