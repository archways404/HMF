use std::time::Duration;

// Parse time strings into Durations
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

// Calculate the percentage difference between two time durations and return a descriptive message
pub fn calculate_percentage_difference(time1: &str, time2: &str) -> Result<String, &'static str> {
    let duration1 = parse_time(time1)?;
    let duration2 = parse_time(time2)?;

    // Convert both durations to f64 in seconds for calculation
    let duration1_secs = duration1.as_secs_f64();
    let duration2_secs = duration2.as_secs_f64();

    // Avoid division by zero
    if duration1_secs == 0.0 || duration2_secs == 0.0 {
        return Err("One of the durations is zero, cannot calculate percentage.");
    }

    // Determine which case is faster and return the appropriate message
    if duration1_secs > duration2_secs {
        let percentage_diff = ((duration1_secs - duration2_secs) / duration2_secs) * 100.0;
        Ok(format!("{} is [ {:.2}% ] faster than {}", time2, percentage_diff, time1))
    } else if duration2_secs > duration1_secs {
        let percentage_diff = ((duration2_secs - duration1_secs) / duration1_secs) * 100.0;
        Ok(format!("{} is [ {:.2}% ] faster than {}", time1, percentage_diff, time2))
    } else {
        Ok(String::from("Both cases have the same duration"))
    }
}
