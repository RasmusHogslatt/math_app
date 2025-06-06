use rand::Rng;

use crate::{quiz::Question, util::format_to_one_decimal};

const X_RANGE: (i32, i32) = (4, 6);
const Y_RANGE: (i32, i32) = (4, 6);

#[derive(Clone, Debug, PartialEq)]
pub struct ExpressionQuestion {
    x_factor: i32,
    y_factor: i32,
    x_value: i32,
    y_value: i32,
    sign: bool,
    answer: String,
}

impl ExpressionQuestion {
    pub fn new(x_factor: i32, y_factor: i32, x_value: i32, y_value: i32, sign: bool) -> Self {
        let answer: i32 = match sign {
            true => x_value * x_factor + y_value * y_factor,
            false => x_value * x_factor - y_value * y_factor,
        };

        Self {
            x_factor,
            y_factor,
            x_value,
            y_value,
            sign,
            answer: format_to_one_decimal(answer as f32),
        }
    }

    pub fn random() -> Self {
        let mut rng = rand::rng();
        let x_factor: i32 = rng.random_range(X_RANGE.0..X_RANGE.1);
        let y_factor: i32 = rng.random_range(Y_RANGE.0..Y_RANGE.1);
        let x_value: i32 = rng.random_range(0..20);
        let y_value: i32 = rng.random_range(0..20);
        let sign = rng.random_bool(0.5);
        Self::new(x_factor, y_factor, x_value, y_value, sign)
    }
}

impl Question for ExpressionQuestion {
    fn prompt(&self) -> String {
        let sign = if self.sign { "+" } else { "-" };
        format!(
            "Beräkna uttryckets värde: {}x {} {}y, where x={} and y={}?",
            self.x_factor, sign, self.y_factor, self.x_value, self.y_value
        )
    }

    fn answer(&self) -> &str {
        &self.answer
    }
}
