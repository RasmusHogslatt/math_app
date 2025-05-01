pub fn format_to_one_decimal(value: f32) -> String {
    // Multiply by 10, round to nearest integer, then divide by 10
    let rounded = (value * 10.0).round() / 10.0;

    // Format with exactly one decimal place
    let formatted = format!("{:.1}", rounded);

    // Trim the ".0" if present
    if formatted.ends_with(".0") {
        formatted.trim_end_matches(".0").to_string()
    } else {
        formatted
    }
}

pub fn validate_input(expected_answer: &str, user_answer: &str) -> bool {
    // Trim whitespace from user input
    let user_answer = user_answer.trim();

    // Direct string comparison (case where formats match exactly)
    if expected_answer == user_answer {
        return true;
    }

    // Try to parse as f32 to handle numeric equivalence
    if let (Ok(expected_value), Ok(user_value)) =
        (expected_answer.parse::<f32>(), user_answer.parse::<f32>())
    {
        // Check if they're essentially the same number
        if (expected_value - user_value).abs() < 0.0001 {
            return true;
        }
    }

    // Handle cases where there might be different decimal representations
    let normalized_expected = normalize_number_format(expected_answer);
    let normalized_user = normalize_number_format(user_answer);

    if normalized_expected == normalized_user {
        return true;
    }

    false
}

fn normalize_number_format(number_str: &str) -> String {
    if let Ok(value) = number_str.parse::<f32>() {
        // Format all numbers consistently
        if value.fract() < 0.0001 {
            // It's essentially a whole number
            return format!("{}", value.round() as i32);
        } else {
            // It has a fractional part - format with 1 decimal place
            return format!("{:.1}", value);
        }
    }

    // If parsing fails, return the original string
    number_str.to_string()
}
