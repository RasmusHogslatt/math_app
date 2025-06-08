use core::f64;
use rand::Rng;

use crate::{quiz::Question, quizzes::ShapeType};

#[derive(Clone, Debug, PartialEq)]
enum ShapeDimensions {
    Square {
        side: u32,
    },
    Rectangle {
        length: u32,
        width: u32,
    },
    Triangle {
        side_a: u32,
        side_b: u32,
        side_c: u32,
    },
    Circle {
        radius: u32,
    },
}

#[derive(Clone, Debug, PartialEq)]
pub struct CircumferenceQuestion {
    shape_type: ShapeType,
    dimensions: ShapeDimensions,
    unit: String,
    answer_text: String,
}

impl CircumferenceQuestion {
    pub fn new_square(side: u32, unit: &str) -> Self {
        let circumference = side * 4;
        Self {
            shape_type: ShapeType::Square,
            dimensions: ShapeDimensions::Square { side },
            unit: unit.to_string(),
            answer_text: circumference.to_string(),
        }
    }

    pub fn new_rectangle(length: u32, width: u32, unit: &str) -> Self {
        let circumference = 2 * (length + width);
        Self {
            shape_type: ShapeType::Rectangle,
            dimensions: ShapeDimensions::Rectangle { length, width },
            unit: unit.to_string(),
            answer_text: circumference.to_string(),
        }
    }

    pub fn new_triangle(side_a: u32, side_b: u32, side_c: u32, unit: &str) -> Self {
        // Triangle circumference = sum of all sides
        let circumference = side_a + side_b + side_c;
        Self {
            shape_type: ShapeType::Triangle,
            dimensions: ShapeDimensions::Triangle {
                side_a,
                side_b,
                side_c,
            },
            unit: unit.to_string(),
            answer_text: circumference.to_string(),
        }
    }

    pub fn new_circle(radius: u32, unit: &str) -> Self {
        let circumference = (2.0 * f64::consts::PI * radius as f64).round() as u32;
        Self {
            shape_type: ShapeType::Circle,
            dimensions: ShapeDimensions::Circle { radius },
            unit: unit.to_string(),
            answer_text: circumference.to_string(),
        }
    }

    pub fn random() -> Self {
        let mut rng = rand::rng();
        let units = ["cm", "m"];
        let unit = units[rng.random_range(0..units.len())];

        // Randomly select which shape to generate
        match rng.random_range(0..4) {
            0 => {
                // Square
                let side = rng.random_range(1..20);
                Self::new_square(side, unit)
            }
            1 => {
                // Rectangle
                let length = rng.random_range(2..20);
                let width = rng.random_range(1..length); // Ensure width <= length
                Self::new_rectangle(length, width, unit)
            }
            2 => {
                // Triangle
                let side_a = rng.random_range(3..15);
                let side_b = rng.random_range(3..15);
                let side_c = rng.random_range(3..15);
                Self::new_triangle(side_a, side_b, side_c, unit)
            }
            _ => {
                // Circle
                let radius = rng.random_range(1..15);
                Self::new_circle(radius, unit)
            }
        }
    }
}

impl Question for CircumferenceQuestion {
    fn prompt(&self) -> String {
        match &self.dimensions {
            ShapeDimensions::Square { side } => {
                format!(
                    "Beräkna omkrets av en kvadrat med sidan: {}{}?",
                    side, self.unit
                )
            }
            ShapeDimensions::Rectangle { length, width } => {
                format!(
                    "Beräkna omkrets av en rektangel sidorna {}{} och {}{}?",
                    length, self.unit, width, self.unit
                )
            }
            ShapeDimensions::Triangle {
                side_a,
                side_b,
                side_c,
            } => {
                format!(
                    "Beräkna omkrets av en triangel med sidorna {}{}, {}{} och {}{}?",
                    side_a, self.unit, side_b, self.unit, side_c, self.unit
                )
            }
            ShapeDimensions::Circle { radius } => {
                format!(
                    "Beräkna omkrets av en cirkel med radien {}{}? (Närmaste heltal)",
                    radius, self.unit
                )
            }
        }
    }

    fn answer(&self) -> &str {
        &self.answer_text
    }
}
