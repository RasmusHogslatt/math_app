use serde::{Deserialize, Serialize};

#[derive(Clone, PartialEq, Debug, Serialize, Deserialize)]
pub enum Course {
    Addition(AdditionSettings),
    Multiplication(MultiplicationSettings),
}

#[derive(Clone, PartialEq, Debug, Serialize, Deserialize)]
pub struct AdditionSettings {
    pub max: u8,
}

#[derive(Clone, PartialEq, Debug, Serialize, Deserialize)]
pub struct MultiplicationSettings {
    pub table: u8,
}
