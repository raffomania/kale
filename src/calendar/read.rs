use anyhow::{Context, Result};
use ical::parser::ical::component::IcalEvent;
use std::path::PathBuf;

const META_FILES: &[&str] = &["displayname", "color"];

pub fn read_events() -> Result<Vec<(PathBuf, IcalEvent)>> {
    let mut path = home::home_dir().context("no home dir")?;
    path.push(".local/share/vdirsyncer/calendars/personal");
    let files = std::fs::read_dir(path)?;

    let mut results: Vec<(PathBuf, IcalEvent)> = Vec::new();

    for file in files {
        let path = file?.path();
        if is_meta_file(&path) {
            continue;
        }

        let reader = std::io::BufReader::new(std::fs::File::open(path.clone())?);
        let parser = ical::IcalParser::new(reader);
        for comp in parser {
            for event in comp?.events {
                results.push((path.clone(), event));
            }
        }
    }

    Ok(results)
}

fn is_meta_file(path: &PathBuf) -> bool {
    META_FILES.iter().any(|f| f == &path.file_name().unwrap())
}
