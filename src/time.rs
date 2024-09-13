use std::time::Duration;

pub fn parse_time(input: &str) -> Result<Duration, &'static str> {
    let input = input.trim(); // Trim leading/trailing spaces

    // Check if the input ends with 's' for seconds
    if input.ends_with("s") && !input.ends_with("ms") {
        let seconds: f64 = input.trim_end_matches("s").parse().map_err(|_| "Invalid number for seconds")?;
        Ok(Duration::from_secs_f64(seconds))

    // Check if the input ends with 'ms' for milliseconds
    } else if input.ends_with("ms") {
        let millis: f64 = input.trim_end_matches("ms").parse().map_err(|_| "Invalid number for milliseconds")?;
        Ok(Duration::from_millis(millis as u64))

    // If the input doesn't end with 's' or 'ms', return an error
    } else {
        Err("Invalid time format, must end with 's' for seconds or 'ms' for milliseconds")
    }
}

// Function to calculate the difference between two time durations
pub fn calculate_time_difference(time1: &str, time2: &str) -> Result<Duration, &'static str> {
    let duration1 = parse_time(time1)?;
    let duration2 = parse_time(time2)?;

    // Calculate the absolute difference between the two durations
    if duration1 > duration2 {
        Ok(duration1 - duration2)
    } else {
        Ok(duration2 - duration1)
    }
}

// Helper function to format Duration back into seconds or milliseconds
pub fn format_duration(duration: Duration) -> String {
    let millis = duration.as_millis();

    if millis >= 1000 {
        format!("{:.4}s", duration.as_secs_f64())
    } else {
        format!("{}ms", millis)
    }
}