use rand::Rng;

use crate::quiz::Question;

#[derive(Clone, Debug, PartialEq)]
pub struct RoundingQuestion {
    number: f32,
    answer: String,
}

impl RoundingQuestion {
    pub fn new(number: f32) -> Self {
        Self {
            number: number,
            answer: (number.round() as i32).to_string(),
        }
    }

    pub fn random() -> Self {
        let mut rng = rand::rng();
        let number: f32 = rng.random_range(-20.0..20.0);
        Self::new(number)
    }
}

impl Question for RoundingQuestion {
    fn prompt(&self) -> String {
        format!("Avrunda till närmaste heltal: {}", self.number)
    }

    fn answer(&self) -> &str {
        &self.answer
    }
}
