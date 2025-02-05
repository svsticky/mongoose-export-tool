use color_eyre::Result;
use time::{format_description, Date, Duration, OffsetDateTime, Weekday};

fn main() -> Result<()> {
    color_eyre::install()?;

    let period_end = get_monday();
    let period_start = get_last_monday(&period_end);

    let url = format_url(period_start, period_end)?;

    println!("{url}");

    Ok(())
}

/// Calculate the monday before the provided monday
fn get_last_monday(current_monday: &Date) -> Date {
    *current_monday - Duration::weeks(1)
}

/// Get the monday in the current week
fn get_monday() -> Date {
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

/// Format the Mongoose URL with the provided start and end dates
fn format_url(period_start: Date, period_end: Date) -> Result<String> {
    let format = format_description::parse("[year]-[month]-[day]")?;
    let start = period_start.format(&format)?;
    let end = period_end.format(&format)?;

    Ok(format!(
        "https://mongoose.svsticky.nl/transactions/export?type=mollie&start_date={start}&end_date={end}&response_type=csv"
    ))
}
