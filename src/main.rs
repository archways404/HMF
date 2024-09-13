use clap::Parser;

mod percentage;
mod time;

/// Simple CLI application to calculate time difference and percentage difference between two time values.
#[derive(Parser)]
#[command(name = "How Much Faster")]
#[command(version = "0.1")]
#[command(about = "Calculate time differences and percentage differences between two time values", long_about = None)]
struct Cli {
    /// First time value (e.g., "2s", "300ms")
    time1: String,

    /// Second time value (e.g., "888s", "500ms")
    time2: String,
}

fn main() {
    let args = Cli::parse();

    // Calculate the difference
    match time::calculate_time_difference(&args.time1, &args.time2) {
        Ok(diff) => {
            println!("Difference: {}", time::format_duration(diff));
        }
        Err(e) => {
            println!("Error: {}", e);
        }
    }

    // Calculate the percentage difference
    match percentage::calculate_percentage_difference(&args.time1, &args.time2) {
        Ok(message) => {
            println!("{}", message);
        }
        Err(e) => {
            println!("Error: {}", e);
        }
    }
}