use chrono::{DateTime, Datelike, Timelike};

pub fn time(t: &DateTime<chrono::Local>) -> String {
    format!("{:02}:{:02}:{:02}", t.hour(), t.minute(), t.second())
}

pub fn date(t: &DateTime<chrono::Local>) -> String {
    format!("{:04}-{:02}-{:02}", t.year(), t.month(), t.day())
}
