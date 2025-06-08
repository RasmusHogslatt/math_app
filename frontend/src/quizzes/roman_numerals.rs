use crate::quiz::{Choice, MultipleChoiceQuestionProvider, Question};
use rand::seq::SliceRandom;
use rand::Rng;

// Helper function to convert a number to a Roman numeral string.
fn to_roman(mut num: u16) -> String {
    let symbols = [
        (1000, "M"),
        (900, "CM"),
        (500, "D"),
        (400, "CD"),
        (100, "C"),
        (90, "XC"),
        (50, "L"),
        (40, "XL"),
        (10, "X"),
        (9, "IX"),
        (5, "V"),
        (4, "IV"),
        (1, "I"),
    ];

    let mut result = String::new();
    for &(value, symbol) in &symbols {
        while num >= value {
            result.push_str(symbol);
            num -= value;
        }
    }
    result
}

#[derive(Clone, Debug, PartialEq)]
pub struct RomanNumeralsQuestion {
    roman_numeral: String,
    choices: Vec<u16>,
    correct_answer: u16,
}

impl RomanNumeralsQuestion {
    pub fn random() -> Self {
        let mut rng = rand::thread_rng();
        // Generate a number in a range that produces interesting numerals
        let correct_answer = rng.gen_range(1..=100);
        let roman_numeral = to_roman(correct_answer);

        let mut choices = vec![correct_answer];
        // Generate 3 wrong answers for a total of 4 choices
        while choices.len() < 4 {
            // Generate wrong answers that are somewhat close to the correct one
            let offset = rng.gen_range(-10..=10);
            // Ensure offset is not zero and result is positive
            if offset == 0 {
                continue;
            }

            let potential_wrong_answer = if correct_answer as i16 + offset > 0 {
                (correct_answer as i16 + offset) as u16
            } else {
                continue; // Skip if it would result in zero or negative
            };

            if !choices.contains(&potential_wrong_answer) {
                choices.push(potential_wrong_answer);
            }
        }

        choices.shuffle(&mut rng);

        Self {
            roman_numeral,
            choices,
            correct_answer,
        }
    }
}

impl Question for RomanNumeralsQuestion {
    fn prompt(&self) -> String {
        format!("Vad blir den romerska siffran {}?", self.roman_numeral)
    }

    fn answer(&self) -> &str {
        // This is used to show the correct answer if the user is wrong.
        // We need to store it as a string somewhere or create it on the fly.
        // Box::leak creates a &'static str from a String, which is needed to match the trait signature.
        Box::leak(self.correct_answer.to_string().into_boxed_str())
    }

    fn check_answer(&self, answer: &str) -> bool {
        // The `answer` from the choice button will be the stringified number.
        answer
            .parse::<u16>()
            .map_or(false, |val| val == self.correct_answer)
    }
}

impl MultipleChoiceQuestionProvider for RomanNumeralsQuestion {
    fn get_choices(&self) -> Vec<Choice> {
        self.choices
            .iter()
            .map(|&val| Choice {
                display_text: val.to_string(),
                value: val.to_string(), // The value to check against the answer
            })
            .collect()
    }
}
