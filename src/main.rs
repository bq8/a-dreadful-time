use std::fs::File;
use std::io::Read;

use chrono::prelude::*;
use chrono::Duration;
use clap::{Parser, Subcommand};

const DATE_STR_FORMAT: &str = "%b %e (%Y-%m-%d)";

#[derive(Parser)]
struct Cli {
    #[clap(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    Last {},
    Current {},
    Next {},
}

fn get_billing_period_with_offset(offset: i64) -> (Date<Local>, Date<Local>) {
    let date = Local::today() + Duration::days(offset);
    let billing_period = get_billing_period(date);

    return billing_period;
}

fn get_billing_period(date: Date<Local>) -> (Date<Local>, Date<Local>) {
    let offset = date.weekday().num_days_from_monday();
    let monday_start_date = date - Duration::days(offset.into());
    let sunday_start_date = monday_start_date + Duration::days(6);
    let start_date_str = monday_start_date.to_string();
    let end_date_str = sunday_start_date.to_string();

    return (monday_start_date, sunday_start_date);
}

fn main() {
    let cli = Cli::parse();

    match &cli.command {
        Commands::Last {} => {
            let (start_date, end_date) = get_billing_period_with_offset(-7);
            let start_date_str = start_date.format(DATE_STR_FORMAT);
            let end_date_str = end_date.format(DATE_STR_FORMAT);
            println!("Last billing period is from {start_date_str} to {end_date_str}");
        }
        Commands::Current {} => {
            let (start_date, end_date) = get_billing_period(Local::today());
            let start_date_str = start_date.format(DATE_STR_FORMAT);
            let end_date_str = end_date.format(DATE_STR_FORMAT);
            println!("Current billing period is from {start_date_str} to {end_date_str}");
        }
        Commands::Next {} => {
            let (start_date, end_date) = get_billing_period_with_offset(7);
            let start_date_str = start_date.format(DATE_STR_FORMAT);
            let end_date_str = end_date.format(DATE_STR_FORMAT);
            println!("Next billing period is from {start_date_str} to {end_date_str}");
        }
    }
}
