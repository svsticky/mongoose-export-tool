mod args;
mod dates;

use crate::args::Args;
use crate::dates::{get_monday, get_monday_ago};
use color_eyre::Result;
use time::{format_description, Date, Duration};

fn main() -> Result<()> {
    color_eyre::install()?;

    let args = Args::try_parse()?;

    if args.help {
        Args::print_help();
        return Ok(());
    }

    let monday = get_monday();

    // Period is inclusive on the start and end date
    let period_end = monday - Duration::weeks(args.n) - Duration::days(1);
    let period_start = get_monday_ago(&get_monday(), args.n + 1);

    let url = format_url(period_start, period_end);

    println!("{url}");

    Ok(())
}

/// Format the Mongoose URL with the provided start and end dates
fn format_url(period_start: Date, period_end: Date) -> String {
    let format = format_description::parse("[year]-[month]-[day]").unwrap();
    let start = period_start.format(&format).unwrap();
    let end = period_end.format(&format).unwrap();

    format!(
        "https://mongoose.svsticky.nl/transactions/export?type=mollie&start_date={start}&end_date={end}&response_type=csv"
    )
}
