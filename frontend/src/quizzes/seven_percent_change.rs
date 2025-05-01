use crate::{quiz::Question, util::format_to_one_decimal};
use gloo::console::log;
use rand::Rng;

#[derive(Clone, Debug, PartialEq)]
pub struct SevenPercentChangeQuestion {
    current_price: i32,
    percentage_change: i32,
    answer: String,
}

impl SevenPercentChangeQuestion {
    pub fn new(current_price: i32, percent_change: i32) -> Self {
        let change: f32 = current_price as f32 * percent_change as f32 / 100.0;
        let new_price: f32 = current_price as f32 + change;
        let answer = format_to_one_decimal(new_price as f32);
        log!(&format!(
            "Current Price: {}, Percentage Change: {}, Change: {}, New Price: {}",
            current_price, percent_change, change, answer
        ));
        Self {
            current_price,
            percentage_change: percent_change,
            answer,
        }
    }

    pub fn random() -> Self {
        let mut rng = rand::rng();
        let current_price: i32 = rng.random_range(1..1000);
        let percentage_change: i32 = rng.random_range(-50..250);
        Self::new(current_price, percentage_change)
    }
}

impl Question for SevenPercentChangeQuestion {
    fn prompt(&self) -> String {
        match self.percentage_change {
            change if change > 0 => format!(
                "If the current price is {} and it increases by {}%, what will be the new price?",
                self.current_price, self.percentage_change
            ),
            change if change < 0 => format!(
                "If the current price is {} and it decreases by {}%, what will be the new price?",
                self.current_price, -self.percentage_change
            ),
            _ => format!("What is the current price?"),
        }
    }

    fn answer(&self) -> &str {
        &self.answer
    }
}
