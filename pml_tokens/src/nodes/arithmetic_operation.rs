use super::traits::OperationTrait;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ArithmeticOperation {
    Add,
    Substract,
    Multiply,
    Divide,
    Power,
    Negate,
}

impl OperationTrait for ArithmeticOperation {
    fn literal(&self) -> String {
        match self {
            Self::Add => "+".to_string(),
            Self::Substract => "-".to_string(),
            Self::Multiply => "*".to_string(),
            Self::Divide => "/".to_string(),
            Self::Power => "^".to_string(),
            Self::Negate => "-".to_string(),
        }
    }

    fn description(&self) -> String {
        match self {
            Self::Add => "+ - Сложение".to_string(),
            Self::Substract => "- - Вычитание".to_string(),
            Self::Multiply => "* - Умножение".to_string(),
            Self::Divide => "/ - Деление".to_string(),
            Self::Power => "^ - Возведение в степень".to_string(),
            Self::Negate => "- - Унарный минус".to_string(),
        }
    }
}
