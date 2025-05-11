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

pub fn validate_input(expected_answer_str: &str, user_answer_str: &str) -> bool {
    let trimmed_user_answer = user_answer_str.trim();

    //1. Direct comparison
    if expected_answer_str == trimmed_user_answer {
        return true;
    }

    // Attempt to parse both expected and user answers as f32.
    let expected_parse_result = expected_answer_str.parse::<f32>();
    let user_parse_result = trimmed_user_answer.parse::<f32>();

    if let (Ok(expected_val), Ok(user_val)) = (expected_parse_result, user_parse_result) {
        if (expected_val - user_val).abs() < 0.0001 {
            return true;
        }

        let normalized_expected_str = format_to_one_decimal(expected_val);
        let normalized_user_str = format_to_one_decimal(user_val);

        if normalized_expected_str == normalized_user_str {
            return true;
        }
    }

    false
}
