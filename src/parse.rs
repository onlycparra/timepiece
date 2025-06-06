use chrono::{DateTime, Duration, Local, NaiveDateTime, NaiveTime, TimeZone};

pub fn time(given_str: &str) -> Option<DateTime<Local>> {
    let now = Local::now().naive_local();
    let now_time = now.time();
    let now_day = now.date();

    // if the string has only two elements, assume it is hours and minutes
    let time_elems: Vec<&str> = given_str.split(':').collect();

    let std_given_str = match time_elems.len() {
        2 => format!("{}:{}:00", time_elems[0], time_elems[1]),
        3 => format!("{}:{}:{}", time_elems[0], time_elems[1], time_elems[2]),
        _ => return None,
    };

    // parse time
    let target_time = NaiveTime::parse_from_str(&std_given_str, "%H:%M:%S").expect(&format!(
        "Error: \"{given_str}\" does not look like a timestamp."
    ));

    // determine whether it is today or tomorrow
    let target_date = if target_time < now_time {
        now_day + Duration::days(1)
    } else {
        now_day
    };

    let target_datetime = NaiveDateTime::new(target_date, target_time);
    Local.from_local_datetime(&target_datetime).single()
}

pub fn dur(t: &str) -> Option<Duration> {
    let increments = t.split(':').collect::<Vec<&str>>();

    let hours;
    let minutes;
    let seconds;
    match increments.len() {
        0 => {
            hours = "0";
            minutes = "0";
            seconds = "0";
        }

        1 => {
            hours = "0";
            minutes = "0";
            seconds = increments[0];
        }

        2 => {
            hours = "0";
            minutes = increments[0];
            seconds = increments[1];
        }

        3 => {
            hours = increments[0];
            minutes = increments[1];
            seconds = increments[2];
        }

        _ => return None,
    }

    let hours: i64 = hours.parse().ok()?;
    let minutes: i64 = minutes.parse().ok()?;
    let seconds: i64 = seconds.parse().ok()?;

    Some(Duration::seconds((hours * 3600) + (minutes * 60) + seconds))
}
