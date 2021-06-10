use anyhow::{anyhow, Context, Result};
use chrono::{
    format::{Fixed, Item, Numeric, Pad},
    DateTime, Datelike, Local, TimeZone,
};
use chrono_tz::Tz;
use ical::{parser::ical::component::IcalEvent, property::Property};
use std::convert::TryFrom;

#[derive(Debug)]
pub struct Entry {
    pub name: String,
    pub start_date: DateTime<Local>,
    pub is_whole_day: bool,
}

impl TryFrom<IcalEvent> for Entry {
    type Error = anyhow::Error;

    fn try_from(event: IcalEvent) -> Result<Self> {
        let name = find_value(event.clone(), "SUMMARY")?;

        let start_date = find_property(event, "DTSTART")?;

        let (start_date, is_whole_day) = match parse_date(&start_date) {
            Ok(date) => (date, true),
            Err(_) => (parse_datetime(&start_date)?, false),
        };
        Ok(Entry {
            name,
            start_date,
            is_whole_day,
        })
    }
}

fn find_property(event: IcalEvent, name: &str) -> Result<Property> {
    event
        .properties
        .into_iter()
        .find(|p| p.name == name)
        .context(format!("property not found: {}", name))
}

fn find_value(event: IcalEvent, name: &str) -> Result<String> {
    find_property(event, name)?
        .value
        .context(format!("Property without value: {}", name))
}

fn parse_datetime(prop: &Property) -> Result<DateTime<Local>> {
    let value = prop.value.as_ref().context("no value found")?;
    parse_utc_datetime(&value)
        .map_err(|e| e.context("datetime parsing failed, trying simple date"))
        .or_else(|e| parse_naive_datetime(&value, &prop.params).context(e))
}

fn parse_naive_datetime(
    value: &str,
    params: &Option<Vec<(String, Vec<String>)>>,
) -> Result<DateTime<Local>> {
    let params = params.as_ref().context("Need params here")?;
    let zone = params
        .iter()
        .find(|(k, _)| k == "TZID")
        .context("Did not find timezone param")?
        .1
        .first()
        .context("Timezone param contained no value")?;
    let zone: Tz = zone.parse().map_err(|e: String| anyhow!(e))?;
    const ITEMS: &'static [Item<'static>] = &[
        Item::Numeric(Numeric::Year, Pad::Zero),
        Item::Numeric(Numeric::Month, Pad::Zero),
        Item::Numeric(Numeric::Day, Pad::Zero),
        Item::Literal("T"),
        Item::Numeric(Numeric::Hour, Pad::Zero),
        Item::Numeric(Numeric::Minute, Pad::Zero),
        Item::Numeric(Numeric::Second, Pad::Zero),
        Item::Space(""),
    ];

    let mut parsed = chrono::format::Parsed::new();
    parsed.nanosecond = Some(0);
    chrono::format::parse(&mut parsed, value, ITEMS.iter().cloned())
        .context(format!("Could not parse datetime: {}", value))?;
    let local = parsed
        .to_naive_datetime_with_offset(0)
        .context("Could not parse datetime")?;

    Ok(zone
        .from_local_datetime(&local)
        .single()
        .context("Found an ambiguous date")?
        .with_timezone(&Local))
}

fn parse_utc_datetime(s: &str) -> Result<DateTime<Local>> {
    const ITEMS: &'static [Item<'static>] = &[
        Item::Numeric(Numeric::Year, Pad::Zero),
        Item::Numeric(Numeric::Month, Pad::Zero),
        Item::Numeric(Numeric::Day, Pad::Zero),
        Item::Literal("T"),
        Item::Numeric(Numeric::Hour, Pad::Zero),
        Item::Numeric(Numeric::Minute, Pad::Zero),
        Item::Numeric(Numeric::Second, Pad::Zero),
        Item::Fixed(Fixed::TimezoneOffsetZ),
        Item::Space(""),
    ];

    let mut parsed = chrono::format::Parsed::new();
    parsed.nanosecond = Some(0);
    parsed.offset = Some(0);
    chrono::format::parse(&mut parsed, s, ITEMS.iter().cloned())
        .context(format!("Could not parse datetime: {}", s))?;
    parsed
        .to_datetime()
        .context("Could not parse datetime")
        .map(|d| d.with_timezone(&Local))
}

fn parse_date(prop: &Property) -> Result<DateTime<Local>> {
    let value = prop.value.as_ref().context("no value found")?;
    const ITEMS: &'static [Item<'static>] = &[
        Item::Numeric(Numeric::Year, Pad::Zero),
        Item::Numeric(Numeric::Month, Pad::Zero),
        Item::Numeric(Numeric::Day, Pad::Zero),
    ];

    let mut parsed = chrono::format::Parsed::new();
    chrono::format::parse(&mut parsed, &value, ITEMS.iter().cloned())
        .context(format!("Could not parse date: '{}'", value))?;
    let date = parsed
        .to_naive_date()
        .context(format!("Invalid date: {}", value))?;

    let local_date = Local
        .ymd(date.year(), date.month(), date.day())
        .and_hms(0, 0, 0);

    Ok(local_date)
}
