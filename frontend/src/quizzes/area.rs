use core::f64;
use std::fmt;

use rand::Rng;

use crate::quiz::Question;

#[derive(Clone, Debug, PartialEq)]
pub enum ShapeType {
    Square,
    Rectangle,
    Triangle,
    Circle,
}

impl fmt::Display for ShapeType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ShapeType::Square => write!(f, "kvadrat"),
            ShapeType::Rectangle => write!(f, "rektangel"),
            ShapeType::Triangle => write!(f, "triangel"),
            ShapeType::Circle => write!(f, "cirkel"),
        }
    }
}

#[derive(Clone, Debug, PartialEq)]
enum ShapeDimensions {
    Square { side: u32 },
    Rectangle { length: u32, width: u32 },
    Triangle { base: u32, height: u32 },
    Circle { radius: u32 },
}

#[derive(Clone, Debug, PartialEq)]
pub struct AreaQuestion {
    shape_type: ShapeType,
    dimensions: ShapeDimensions,
    unit: String,
    answer_text: String,
}

impl AreaQuestion {
    pub fn new_square(side: u32, unit: &str) -> Self {
        let area = side * side;
        Self {
            shape_type: ShapeType::Square,
            dimensions: ShapeDimensions::Square { side },
            unit: unit.to_string(),
            answer_text: area.to_string(),
        }
    }

    pub fn new_rectangle(length: u32, width: u32, unit: &str) -> Self {
        let area = length * width;
        Self {
            shape_type: ShapeType::Rectangle,
            dimensions: ShapeDimensions::Rectangle { length, width },
            unit: unit.to_string(),
            answer_text: area.to_string(),
        }
    }

    pub fn new_triangle(base: u32, height: u32, unit: &str) -> Self {
        // Triangle area = 0.5 * base * height
        let area = (base * height) / 2;
        Self {
            shape_type: ShapeType::Triangle,
            dimensions: ShapeDimensions::Triangle { base, height },
            unit: unit.to_string(),
            answer_text: area.to_string(),
        }
    }

    pub fn new_circle(radius: u32, unit: &str) -> Self {
        let area = (f64::consts::PI * (radius * radius) as f64).round() as u32;
        Self {
            shape_type: ShapeType::Circle,
            dimensions: ShapeDimensions::Circle { radius },
            unit: unit.to_string(),
            answer_text: area.to_string(),
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
                let base = rng.random_range(2..20);
                let height = rng.random_range(2..20);
                Self::new_triangle(base, height, unit)
            }
            _ => {
                // Circle
                let radius = rng.random_range(1..15);
                Self::new_circle(radius, unit)
            }
        }
    }
}

impl Question for AreaQuestion {
    fn prompt(&self) -> String {
        match &self.dimensions {
            ShapeDimensions::Square { side } => {
                format!(
                    "Beräkna area av en kvadrat med sidan: {}{}?",
                    side, self.unit
                )
            }
            ShapeDimensions::Rectangle { length, width } => {
                format!(
                    "Beräkna area av en rektangel sidorna {}{} och {}{}?",
                    length, self.unit, width, self.unit
                )
            }
            ShapeDimensions::Triangle { base, height } => {
                format!(
                    "Beräkna area av en triangel med bas {}{} och höjd {}{}?",
                    base, self.unit, height, self.unit
                )
            }
            ShapeDimensions::Circle { radius } => {
                format!(
                    "Beräkna area av en cirkel med radien {}{}? (Närmaste heltal)",
                    radius, self.unit
                )
            }
        }
    }

    fn answer(&self) -> &str {
        &self.answer_text
    }
}
