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
            ShapeType::Square => write!(f, "square"),
            ShapeType::Rectangle => write!(f, "rectangle"),
            ShapeType::Triangle => write!(f, "triangle"),
            ShapeType::Circle => write!(f, "circle"),
        }
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum ShapeDimensions {
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
        // Circle area = π * r²
        // For simplicity, we'll use 3.14159 as π and round to the nearest integer
        let pi = 3.14159;
        let area = (pi * (radius * radius) as f64).round() as u32;
        Self {
            shape_type: ShapeType::Circle,
            dimensions: ShapeDimensions::Circle { radius },
            unit: unit.to_string(),
            answer_text: area.to_string(),
        }
    }

    pub fn random() -> Self {
        let mut rng = rand::rng();
        let units = ["centimeters", "meters", "inches"];
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
                    "What is the area of a square with sides of {} {}?",
                    side, self.unit
                )
            }
            ShapeDimensions::Rectangle { length, width } => {
                format!(
                    "What is the area of a rectangle with length {} {} and width {} {}?",
                    length, self.unit, width, self.unit
                )
            }
            ShapeDimensions::Triangle { base, height } => {
                format!(
                    "What is the area of a triangle with base {} {} and height {} {}?",
                    base, self.unit, height, self.unit
                )
            }
            ShapeDimensions::Circle { radius } => {
                format!(
                    "What is the area of a circle with radius {} {}? (Round to the nearest whole number)",
                    radius, self.unit
                )
            }
        }
    }

    fn answer(&self) -> &str {
        &self.answer_text
    }
}
