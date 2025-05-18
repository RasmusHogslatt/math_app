use super::NumberValue;
use crate::quiz::{Choice, MultipleChoiceQuestionProvider, Question};
use std::cmp::Ordering;

#[derive(Clone, Debug, PartialEq)]
pub struct FractionComparisonQuestion {
    first_value: NumberValue,
    second_value: NumberValue,
    correct_answer: usize, // 0 for first, 1 for second, 2 for equal
}

impl FractionComparisonQuestion {
    pub fn new(first: NumberValue, second: NumberValue) -> Self {
        let first_val = first.as_f64();
        let second_val = second.as_f64();

        let correct_answer = match first_val.partial_cmp(&second_val).unwrap() {
            Ordering::Greater => 0, // First is greater
            Ordering::Less => 1,    // Second is greater
            Ordering::Equal => 2,   // Equal
        };

        Self {
            first_value: first,
            second_value: second,
            correct_answer,
        }
    }

    pub fn random() -> Self {
        let mut rng = rand::rng();

        let (first, second) = (
            NumberValue::random_fraction(&mut rng),
            NumberValue::random_fraction(&mut rng),
        );

        // Ensure they're not equal
        if (first.as_f64() - second.as_f64()).abs() < 0.0001 {
            // If they're too close, just return a new random one
            return Self::random();
        }

        Self::new(first, second)
    }

    pub fn first_value(&self) -> &NumberValue {
        &self.first_value
    }

    pub fn second_value(&self) -> &NumberValue {
        &self.second_value
    }

    pub fn check_selected(&self, selected_index: usize) -> bool {
        selected_index == self.correct_answer
    }
}

impl Question for FractionComparisonQuestion {
    fn prompt(&self) -> String {
        "Vilket värde är störst".to_string()
    }

    fn answer(&self) -> &str {
        match self.correct_answer {
            0 => "vänster",
            1 => "höger",
            _ => "lika", // This shouldn't happen as we ensure they're not equal
        }
    }

    // Override check_answer to handle our custom click behavior
    fn check_answer(&self, answer: &str) -> bool {
        match answer {
            "0" => self.correct_answer == 0,
            "1" => self.correct_answer == 1,
            _ => false,
        }
    }

    // Override display to customize the output
    fn display(&self) -> String {
        "Vilket bråk är störst?".to_string()
    }
}

impl MultipleChoiceQuestionProvider for FractionComparisonQuestion {
    fn get_choices(&self) -> Vec<Choice> {
        vec![
            Choice {
                display_text: self.first_value.display(),
                value: "0".to_string(),
            },
            Choice {
                display_text: self.second_value.display(),
                value: "1".to_string(),
            },
        ]
    }
}
