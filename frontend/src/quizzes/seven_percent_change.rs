use crate::{quiz::Question, util::format_to_one_decimal};
use gloo::console::log;
use rand::Rng;

static ITEMS: [&str; 6] = [
    "Donald Duck comic book",
    "Nintendo Switch 2",
    "Tesla Cybertruck",
    "iPhone 16",
    "PlayStation 5",
    "Minecraft account",
];

#[derive(Clone, Debug, PartialEq)]
pub struct SevenPercentChangeQuestion {
    current_price: i32,
    percentage_change: i32,
    item: String,
    answer: String,
}

impl SevenPercentChangeQuestion {
    pub fn new(current_price: i32, percent_change: i32) -> Self {
        let change: f32 = current_price as f32 * percent_change as f32 / 100.0;
        let new_price: f32 = current_price as f32 + change;
        let answer = format_to_one_decimal(new_price as f32);

        Self {
            current_price,
            item: get_random_item(),
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
                "The price of {} is {}.  It increases by {}%, what is the new price?",
                self.item, self.current_price, self.percentage_change
            ),
            change if change < 0 => format!(
                "The price of {} is {}.  It decreases by {}%, what is the new price?",
                self.item, self.current_price, -self.percentage_change
            ),
            _ => format!("What is the current price?"),
        }
    }

    fn answer(&self) -> &str {
        &self.answer
    }
}

fn get_random_item() -> String {
    let mut rng = rand::rng();
    let index: usize = rng.random_range(0..ITEMS.len());
    ITEMS[index].to_string()
}
