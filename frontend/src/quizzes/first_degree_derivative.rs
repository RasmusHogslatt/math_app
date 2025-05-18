use rand::Rng;

use crate::quiz::Question;

#[derive(Clone, Debug, PartialEq)]
pub struct FirstDegreeDerivativeQuestion {
    // Represent a polynomial function like ax^n + bx^m + c
    terms: Vec<(i32, u32)>, // Each term is (coefficient, exponent)
    constant: i32,          // Constant term
    answer_text: String,    // The derivative expression
}

impl FirstDegreeDerivativeQuestion {
    // Create a new derivative question with specific terms
    pub fn new(terms: Vec<(i32, u32)>, constant: i32) -> Self {
        // Calculate the derivative
        let derivative_terms: Vec<(i32, u32)> = terms
            .iter()
            .filter(|(_, exponent)| *exponent > 0) // Filter out constants (x^0)
            .map(|(coef, exp)| (coef * (*exp as i32), exp - 1)) // Apply power rule: d/dx(ax^n) = a*n*x^(n-1)
            .collect();

        // Format the answer as a string
        let answer = FirstDegreeDerivativeQuestion::format_polynomial(&derivative_terms, 0);

        Self {
            terms,
            constant,
            answer_text: answer,
        }
    }

    // Generate a random first-degree derivative question
    pub fn random() -> Self {
        let mut rng = rand::rng();

        // Generate between 1 and 3 terms
        let num_terms = rng.random_range(1..4);
        let mut terms = Vec::with_capacity(num_terms);

        // Generate terms with coefficients and exponents
        for _ in 0..num_terms {
            let coefficient = rng.random_range(-9..10);
            if coefficient == 0 {
                continue; // Skip zero terms
            }

            let exponent = rng.random_range(1..4); // Power from 1 to 3
            terms.push((coefficient, exponent));
        }

        // Add a constant term sometimes
        let constant = if rng.random_bool(0.7) {
            rng.random_range(-10..11)
        } else {
            0
        };

        Self::new(terms, constant)
    }

    // Helper method to format a polynomial expression
    // Updated format_polynomial method to handle zero constants correctly
    fn format_polynomial(terms: &[(i32, u32)], constant: i32) -> String {
        // If we have no terms and constant is zero, return "0"
        if terms.is_empty() && constant == 0 {
            return "0".to_string();
        }

        // If we have no terms but constant is non-zero, just return the constant
        if terms.is_empty() {
            return constant.to_string();
        }

        let mut result = String::new();
        let mut first_term = true;

        // Add each term to the result
        for (coef, exp) in terms {
            if *coef == 0 {
                continue; // Skip zero terms
            }

            // Handle the sign
            if first_term {
                if *coef < 0 {
                    result.push('-');
                }
            } else if *coef < 0 {
                result.push_str(" - ");
            } else {
                result.push_str(" + ");
            }

            // Add coefficient (unless it's 1 or -1 and not a constant)
            let abs_coef = coef.abs();
            if abs_coef != 1 || *exp == 0 {
                result.push_str(&abs_coef.to_string());
            }

            // Add variable and exponent
            match exp {
                0 => {}                                      // No variable for constant terms
                1 => result.push('x'),                       // Just 'x' for first degree
                _ => result.push_str(&format!("x^{}", exp)), // 'x^n' for higher degrees
            }

            first_term = false;
        }

        // Add constant if non-zero
        if constant != 0 {
            if !first_term {
                if constant < 0 {
                    result.push_str(" - ");
                    result.push_str(&constant.abs().to_string());
                } else {
                    result.push_str(" + ");
                    result.push_str(&constant.to_string());
                }
            } else {
                result.push_str(&constant.to_string());
            }
        }

        result
    }

    // Format the original polynomial
    fn format_original(&self) -> String {
        Self::format_polynomial(&self.terms, self.constant)
    }
}

impl Question for FirstDegreeDerivativeQuestion {
    fn prompt(&self) -> String {
        format!("Beräkna f'(x): f(x) = {}", self.format_original())
    }

    fn answer(&self) -> &str {
        &self.answer_text
    }

    // Override to handle different equivalent forms of the answer
    fn check_answer(&self, answer: &str) -> bool {
        // This is a simple implementation that checks exact match
        // For a more robust solution, you'd want to parse and compare expressions algebraically

        // Normalize the answer by removing spaces and converting to lowercase
        let normalized_answer = answer.trim().to_lowercase().replace(" ", "");
        let normalized_correct = self.answer_text.to_lowercase().replace(" ", "");

        normalized_answer == normalized_correct
    }
}
