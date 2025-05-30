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

pub fn validate_time_input(user_input: &str, expected_hour: u8, expected_minute: u8) -> bool {
    let trimmed = user_input.trim();

    // Path A: Input potentially contains a colon
    if let Some((h_str, m_str)) = trimmed.split_once(':') {
        // Check for empty parts (e.g., ":30", "10:")
        if h_str.is_empty() || m_str.is_empty() {
            return false; // Malformed input
        }

        match (h_str.parse::<u8>(), m_str.parse::<u8>()) {
            (Ok(h), Ok(m)) => {
                // Successfully parsed hour and minute parts
                if h < 24 && m < 60 {
                    // Check if parsed h, m are valid time components
                    // Compare with expected time
                    h == expected_hour && m == expected_minute
                } else {
                    // Parsed numbers are out of valid time range (e.g., "25:30", "10:70")
                    false
                }
            }
            _ => {
                // Failed to parse hour or minute string (e.g., "10:abc")
                false
            }
        }
    } else {
        // Path B: Input does NOT contain a colon; try to parse as HHMM or HMM
        // Check if the input is purely numeric and has a length of 3 or 4 digits
        if trimmed.chars().all(char::is_numeric) && (trimmed.len() == 3 || trimmed.len() == 4) {
            if let Ok(time_num) = trimmed.parse::<u32>() {
                let h_val: u32;
                let m_val: u32;

                if trimmed.len() == 3 {
                    // Handles "HMM", e.g., "715" -> hour=7, minute=15
                    // Also "705" -> hour=7, minute=5
                    h_val = time_num / 100;
                    m_val = time_num % 100;
                } else {
                    // trimmed.len() == 4
                    // Handles "HHMM", e.g., "1715" -> hour=17, minute=15
                    // Also "0715" -> hour=7, minute=15
                    h_val = time_num / 100;
                    m_val = time_num % 100;
                }

                // Check if derived h_val, m_val are valid time components
                if h_val < 24 && m_val < 60 {
                    // Compare with expected time
                    h_val == expected_hour as u32 && m_val == expected_minute as u32
                } else {
                    // Derived numbers are out of valid time range (e.g. input "1275" -> h=12, m=75)
                    false
                }
            } else {
                // This case should be rare if previous checks (is_numeric, length) passed,
                // but handles potential overflow if u32 parse failed for a 3-4 digit string.
                false
            }
        } else {
            // Not purely numeric, or not 3/4 digits long.
            false
        }
    }
}
