use rand::Rng;

use crate::quiz::Question;

// Square area question implementation
#[derive(Clone, Debug, PartialEq)]
pub struct AreaQuestion {
    side_length: u32,
    unit: String,
    answer_text: String,
}

impl AreaQuestion {
    pub fn new(side: u32, unit: &str) -> Self {
        let area = side * side;
        Self {
            side_length: side,
            unit: unit.to_string(),
            answer_text: area.to_string(),
        }
    }

    pub fn random() -> Self {
        let mut rng = rand::rng();
        let side = rng.random_range(1..20);
        let units = ["centimeters", "meters", "inches"];
        let unit = units[rng.random_range(0..units.len())];
        Self::new(side, unit)
    }
}

impl Question for AreaQuestion {
    fn prompt(&self) -> String {
        format!(
            "What is the area of a square with sides of {} {}?",
            self.side_length, self.unit
        )
    }

    fn answer(&self) -> &str {
        &self.answer_text
    }
}
