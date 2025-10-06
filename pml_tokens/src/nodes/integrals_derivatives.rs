use super::traits::OperationTrait;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Deserialize, Serialize)]
pub enum IntegralsDerivatives {
    Derivatives {
        var: String,
    },
    Integral {
        var: String,
        lower: Option<f64>,
        upper: Option<f64>,
    },
}

impl OperationTrait for IntegralsDerivatives {
    fn literal(&self) -> String {
        match self {
            Self::Derivatives { .. } => "d/dx".to_string(),
            Self::Integral { .. } => "∫".to_string(),
        }
    }

    fn description(&self) -> String {
        match self {
            Self::Derivatives { var } => {
                if var == "x" {
                    "Производная по x(d/dx)".to_string()
                } else {
                    format!("Производная по переменной {}", var)
                }
            }
            Self::Integral { lower, upper, var } => {
                if lower.is_some() && upper.is_some() {
                    format!("∫{} - Определённый интеграл (с пределами)", var)
                } else {
                    format!("Интеграл без пределов(эквивалент неопределённому)")
                }
            }
        }
    }
}
