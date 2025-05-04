use crate::quiz::Question;
use rand::Rng;
use std::cmp::Ordering;

// Represents a value that can be either a fraction or decimal
#[derive(Clone, Debug, PartialEq)]
pub enum NumberValue {
    Fraction { numerator: i32, denominator: i32 },
    Decimal(f64),
}

impl NumberValue {
    // Convert to f64 for comparison
    pub fn as_f64(&self) -> f64 {
        match self {
            NumberValue::Fraction {
                numerator,
                denominator,
            } => *numerator as f64 / *denominator as f64,
            NumberValue::Decimal(val) => *val,
        }
    }

    // Format for display
    pub fn display(&self) -> String {
        match self {
            NumberValue::Fraction {
                numerator,
                denominator,
            } => format!("{}/{}", numerator, denominator),
            NumberValue::Decimal(val) => format!("{:.2}", val),
        }
    }

    // Generate a random fraction
    pub fn random_fraction(rng: &mut impl Rng) -> Self {
        let denominator = rng.random_range(2..=10);
        let numerator = rng.random_range(1..denominator);

        // Try to keep the fraction in simplest form
        let gcd = gcd(numerator, denominator);
        let numerator = numerator / gcd;
        let denominator = denominator / gcd;

        NumberValue::Fraction {
            numerator,
            denominator,
        }
    }

    // Generate a random decimal
    pub fn random_decimal(rng: &mut impl Rng) -> Self {
        // Generate a number between 0.1 and 0.9 with one decimal place
        let decimal = (rng.random_range(1..10) as f64) / 10.0;
        NumberValue::Decimal(decimal)
    }
}

// Helper function to find greatest common divisor (for simplifying fractions)
fn gcd(a: i32, b: i32) -> i32 {
    if b == 0 { a } else { gcd(b, a % b) }
}

#[derive(Clone, Debug, PartialEq)]
pub struct NumberComparisonQuestion {
    first_value: NumberValue,
    second_value: NumberValue,
    correct_answer: usize, // 0 for first, 1 for second, 2 for equal
}

impl NumberComparisonQuestion {
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
        let use_different_types = rng.random_bool(0.7); // 70% chance to mix fraction and decimal

        let (first, second) = if use_different_types {
            // Mix fraction and decimal
            if rng.random_bool(0.5) {
                (
                    NumberValue::random_fraction(&mut rng),
                    NumberValue::random_decimal(&mut rng),
                )
            } else {
                (
                    NumberValue::random_decimal(&mut rng),
                    NumberValue::random_fraction(&mut rng),
                )
            }
        } else {
            // Both same type
            if rng.random_bool(0.5) {
                (
                    NumberValue::random_fraction(&mut rng),
                    NumberValue::random_fraction(&mut rng),
                )
            } else {
                (
                    NumberValue::random_decimal(&mut rng),
                    NumberValue::random_decimal(&mut rng),
                )
            }
        };

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

impl Question for NumberComparisonQuestion {
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
        format!(
            "Vilket värde är störst: {} eller {}?",
            self.first_value.display(),
            self.second_value.display()
        )
    }
}
