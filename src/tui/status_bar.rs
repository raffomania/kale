use chrono::prelude::*;
use chrono::{IsoWeek, Locale, NaiveDate};
use tui::{
    style::{Modifier, Style},
    text::Span,
    widgets::Widget,
};

pub struct StatusBar {
    week: IsoWeek,
    locale: Locale,
}

impl StatusBar {
    pub fn new(week: IsoWeek, locale: Locale) -> Self {
        StatusBar { week, locale }
    }
}

impl Widget for StatusBar {
    fn render(self, area: tui::layout::Rect, buf: &mut tui::buffer::Buffer) {
        let start_of_week =
            NaiveDate::from_isoywd(self.week.year(), self.week.week(), chrono::Weekday::Mon);
        let month = Local
            .from_local_date(&start_of_week)
            .unwrap()
            .format_localized("%B", self.locale);
        let date_text = format!("{} {}", month, self.week.year(),);
        let date_span = Span::styled(date_text, Style::default().add_modifier(Modifier::DIM));
        let kale = Span::from(format!("🥬"));
        buf.set_span(area.x, area.y, &kale, area.width);
        let x = area.x + area.width - (date_span.width() as u16);
        buf.set_span(x, area.y, &date_span, area.width);
    }
}
