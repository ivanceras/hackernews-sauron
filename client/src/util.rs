use chrono::{DateTime, Utc};

/// Return the time ago for a date
pub fn time_ago(date: DateTime<Utc>) -> String {
    let now = Utc::now();

    const SECONDS_IN_MINUTE: f32 = 60.0;
    const SECONDS_IN_HOUR: f32 = SECONDS_IN_MINUTE * 60.0;
    const SECONDS_IN_DAY: f32 = SECONDS_IN_HOUR * 24.0;
    const SECONDS_IN_YEAR: f32 = SECONDS_IN_DAY * 365.0; // Ignore leap years for now

    let seconds = (now - date).num_seconds() as f32;
    if seconds < SECONDS_IN_MINUTE {
        let seconds = seconds.floor() as i32;
        if seconds < 2 {
            format!("{} second", seconds)
        } else {
            format!("{} seconds", seconds)
        }
    } else if seconds < SECONDS_IN_HOUR {
        let minutes = (seconds / SECONDS_IN_MINUTE).floor() as i32;
        if minutes < 2 {
            format!("{} minute", minutes)
        } else {
            format!("{} minutes", minutes)
        }
    } else if seconds < SECONDS_IN_DAY {
        let hours = (seconds / SECONDS_IN_HOUR).floor() as i32;
        if hours < 2 {
            format!("{} hour", hours)
        } else {
            format!("{} hours", hours)
        }
    } else if seconds < SECONDS_IN_YEAR {
        let days = (seconds / SECONDS_IN_DAY).floor() as i32;
        if days < 2 {
            format!("{} day", days)
        } else {
            format!("{} days", days)
        }
    } else {
        let years = (seconds / SECONDS_IN_YEAR).floor() as i32;
        if years < 2 {
            format!("{} year", years)
        } else {
            format!("{} years", years)
        }
    }
}
