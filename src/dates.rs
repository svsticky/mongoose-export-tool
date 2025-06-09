use time::{Date, Duration, OffsetDateTime, Weekday};

/// Calculate the monday `n` weeks before the provided monday
pub fn get_monday_ago(current_monday: &Date, n: i64) -> Date {
    *current_monday - Duration::weeks(n)
}

/// Get the monday in the current week
pub fn get_monday() -> Date {
    let today = OffsetDateTime::now_utc();
    match today.weekday() {
        Weekday::Monday => today,
        Weekday::Tuesday => today - Duration::days(1),
        Weekday::Wednesday => today - Duration::days(2),
        Weekday::Thursday => today - Duration::days(3),
        Weekday::Friday => today - Duration::days(4),
        Weekday::Saturday => today - Duration::days(5),
        Weekday::Sunday => today - Duration::days(6),
    }
    .date()
}
