use super::traits::OperationTrait;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Deserialize, Serialize)]
pub enum Functions {
    Sin,
    Cos,
    Tan,
    Exp,
    Ln,
    Log,
    Sqrt,
    Abs,
}

impl OperationTrait for Functions {
    fn literal(&self) -> String {
        match self {
            Self::Sin => "sin".to_string(),
            Self::Cos => "cos".to_string(),
            Self::Tan => "tn".to_string(),
            Self::Exp => "exp".to_string(),
            Self::Ln => "ln".to_string(),
            Self::Log => "log".to_string(),
            Self::Sqrt => "sqrt".to_string(),
            Self::Abs => "abs".to_string(),
        }
    }
    fn description(&self) -> String {
        match self {
            Self::Sin => "sin - Синус угла".to_string(),
            Self::Cos => "cos - Косинус угла".to_string(),
            Self::Tan => "tn - Тангенс угла".to_string(),
            Self::Exp => "exp - Экспонента (e^x)".to_string(),
            Self::Ln => "ln - Натуральный логарифм".to_string(),
            Self::Log => "log - Логарифм".to_string(),
            Self::Sqrt => "sqrt - Квадратный корень".to_string(),
            Self::Abs => "abs - Модуль числа".to_string(),
        }
    }
}
