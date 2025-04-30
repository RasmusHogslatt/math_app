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

    // Parse both as f32 first to see if they're numerically equivalent
    if let (Ok(expected_value), Ok(user_value)) =
        (expected_answer.parse::<f32>(), user_answer.parse::<f32>())
    {
        // Check if they're essentially the same number
        // This handles cases where one is "7" and the other is "7.0"
        if (expected_value - user_value).abs() < 0.0001 {
            return true;
        }
    }

    // Handle the specific case of whole numbers
    if expected_answer.find('.').is_none() {
        // Expected is a whole number like "7"
        // Check if user input is that number with ".0" appended
        if let Some(decimal_idx) = user_answer.find('.') {
            let before_decimal = &user_answer[0..decimal_idx];
            let after_decimal = &user_answer[decimal_idx + 1..];

            // If user entered something like "7.0" and expected is "7"
            if before_decimal == expected_answer && after_decimal.chars().all(|c| c == '0') {
                return true;
            }
        }
    } else {
        // Expected has a decimal point
        // Check if user didn't include decimal point for numbers like "7.0"
        if let Some(decimal_idx) = expected_answer.find('.') {
            let before_decimal = &expected_answer[0..decimal_idx];
            let after_decimal = &expected_answer[decimal_idx + 1..];

            // If expected is like "7.0" and user entered "7"
            if after_decimal.chars().all(|c| c == '0') && before_decimal == user_answer {
                return true;
            }
        }
    }

    false
}
