use chrono::prelude::*;
use chrono::Duration;
use clap::{Parser, Subcommand};
use minijinja::{Environment, context, Source};

const DATE_STR_FORMAT: &str = "%b %e (%Y-%m-%d)";

#[derive(Parser)]
struct Cli {
    #[clap(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    Dates { period: String },
    Email { invoice_id: String },
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

    return (monday_start_date, sunday_start_date);
}

fn print_billing_period(period_name: String, start_date: Date<Local>, end_date: Date<Local>) {
    let start_date_str = start_date.format(DATE_STR_FORMAT);
    let end_date_str = end_date.format(DATE_STR_FORMAT);
    println!("{}: {} to {}", period_name, start_date_str, end_date_str);
}

fn main() {
    let cli = Cli::parse();

    match &cli.command {
        Commands::Dates { period } => {
            match period.as_str() {
                "last" => {
                    let (start_date, end_date) = get_billing_period_with_offset(-7);
                    print_billing_period("Last billing period".to_string(), start_date, end_date);
                }
                "current" => {
                    let (start_date, end_date) = get_billing_period(Local::today());
                    print_billing_period("Current billing period".to_string(), start_date, end_date);
                }
                "next" => {
                    let (start_date, end_date) = get_billing_period_with_offset(7);
                    print_billing_period("Next billing period".to_string(), start_date, end_date);
                }
                _ => {
                    println!("\"{}\" is not a recognized billing period, please use either \"last\", \"current\", or \"next\"", period)
                }
            }
        }
        Commands::Email { invoice_id } => {
            let mut env = Environment::new();
            let mut src = Source::new();
            src.load_from_path("templates", &["txt"]).unwrap();
            env.set_source(src);
            let template = env.get_template("email.txt").unwrap();
            println!("{}", template.render(context! { test_var => invoice_id }).unwrap());
        }
    }
}
