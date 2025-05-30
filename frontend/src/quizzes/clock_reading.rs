use crate::{quiz::Question, util::validate_time_input};
use rand::Rng;

#[derive(Clone, Debug, PartialEq)]
pub struct ClockReadingQuestion {
    hour: u8,            // 0-23 (24-hour format)
    minute: u8,          // 0-59
    is_afternoon: bool,  // true for eftermiddag (PM), false for förmiddag (AM)
    answer_text: String, // Expected answer in "HH:MM" or "HHMM" format (24-hour)
}

impl ClockReadingQuestion {
    pub fn new(hour: u8, minute: u8, is_afternoon: bool) -> Self {
        // Convert 12-hour display to 24-hour format
        let hour_24 = if is_afternoon && hour != 12 {
            hour + 12
        } else if !is_afternoon && hour == 12 {
            // 12 AM (midnight) is 00 hours
            0
        } else {
            hour
        };

        Self {
            hour: hour_24,
            minute,
            is_afternoon,
            answer_text: format!("{:02}:{:02}", hour_24, minute),
        }
    }

    pub fn random() -> Self {
        let mut rng = rand::rng();

        // Generate random time
        let display_hour = rng.random_range(1..=12);

        let minute_slot = rng.random_range(0..12);
        let minute = minute_slot * 5;

        // Randomly choose AM or PM
        let is_afternoon = rng.random_range(0..2) == 1;

        Self::new(display_hour, minute, is_afternoon)
    }

    pub fn random_any_minute() -> Self {
        let mut rng = rand::rng();

        // Generate completely random time
        let display_hour = rng.random_range(1..=12);
        let minute = rng.random_range(0..60); // Generates 0..59

        // Randomly choose AM or PM
        let is_afternoon = rng.random_range(0..2) == 1;

        Self::new(display_hour, minute, is_afternoon)
    }

    // Getters for the clock component (returns 12-hour format for display)
    pub fn display_hour(&self) -> u8 {
        let hour_12 = if self.hour == 0 {
            // Midnight (00:xx) should display as 12
            12
        } else if self.hour > 12 {
            self.hour - 12
        } else {
            self.hour
        };
        hour_12
    }

    pub fn minute(&self) -> u8 {
        self.minute
    }

    pub fn is_afternoon(&self) -> bool {
        self.is_afternoon
    }

    pub fn hour_angle(&self) -> f64 {
        let display_hour = self.display_hour();
        let hour_degrees = (display_hour as f64) * 30.0;

        let minute_offset = (self.minute as f64) * 0.5;

        hour_degrees + minute_offset
    }

    pub fn minute_angle(&self) -> f64 {
        // Each minute is 6 degrees (360/60)
        (self.minute as f64) * 6.0
    }
}

impl Question for ClockReadingQuestion {
    fn prompt(&self) -> String {
        format!("Vad visar klockan?")
    }

    fn answer(&self) -> &str {
        &self.answer_text
    }

    fn check_answer(&self, answer: &str) -> bool {
        validate_time_input(answer, self.hour, self.minute)
    }
}
