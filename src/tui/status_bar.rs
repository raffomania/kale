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

    /// Get a string for the top right corner that describes the week we're currently showing.
    fn format_current_week(&self) -> String {
        let start_of_week =
            NaiveDate::from_isoywd(self.week.year(), self.week.week(), chrono::Weekday::Mon);
        let end_of_week =
            NaiveDate::from_isoywd(self.week.year(), self.week.week(), chrono::Weekday::Sun);

        // Only display the start month if it's a different one than the end of the week's month
        let start_of_week_format = if start_of_week.month() != end_of_week.month() {
            "%d. %B"
        } else {
            "%d."
        };

        let start_of_week = Local
            .from_local_date(&start_of_week)
            .unwrap()
            .format_localized(start_of_week_format, self.locale)
            .to_string();

        let end_of_week = Local
            .from_local_date(&end_of_week)
            .unwrap()
            .format_localized("%d. %B", self.locale);

        format!("{} - {} {}", start_of_week, end_of_week, self.week.year(),)
    }
}

impl Widget for StatusBar {
    fn render(self, area: tui::layout::Rect, buf: &mut tui::buffer::Buffer) {
        let date_span = Span::styled(
            self.format_current_week(),
            Style::default().add_modifier(Modifier::DIM),
        );
        let kale = Span::from(format!(" 🥬"));
        buf.set_span(area.x, area.y, &kale, area.width);
        let x = area.x + area.width - (date_span.width() as u16);
        buf.set_span(x, area.y, &date_span, area.width);
    }
}
