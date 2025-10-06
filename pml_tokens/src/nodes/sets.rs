use super::traits::OperationTrait;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Deserialize, Serialize)]
pub enum StandardSet {
    Natural,     // ℕ
    NaturalZero, // ℕ₀
    Integer,     // ℤ
    Rational,    // ℚ
    Real,        // ℝ
    Complex,     // ℂ
    Prime,       // ℙ
    Empty,       // ∅
    Universal,   // 𝕌
    Boolean,     // 𝔹
    StringSet,   // 𝕊
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct CustomSet {
    pub range: Option<(f64, f64)>,
    pub elements: Option<Vec<f64>>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub enum Sets {
    Standart(StandardSet),
    Custom(CustomSet),
}

impl OperationTrait for StandardSet {
    fn literal(&self) -> String {
        match self {
            Self::Natural => "ℕ".to_string(),
            Self::NaturalZero => "ℕ₀".to_string(),
            Self::Integer => "ℤ".to_string(),
            Self::Rational => "ℚ".to_string(),
            Self::Real => "ℝ".to_string(),
            Self::Complex => "ℂ".to_string(),
            Self::Prime => "ℙ".to_string(),
            Self::Empty => "∅".to_string(),
            Self::Universal => "𝕌".to_string(),
            Self::Boolean => "𝔹".to_string(),
            Self::StringSet => "𝕊".to_string(),
        }
    }

    fn description(&self) -> String {
        match self {
            Self::Natural => "Natural numbers — натуральные числа (1, 2, 3, ...)".to_string(),
            Self::NaturalZero => {
                "Natural numbers with zero — натуральные с нулём (0, 1, 2, ...)".to_string()
            }
            Self::Integer => "Integers — целые числа (..., -2, -1, 0, 1, 2, ...)".to_string(),
            Self::Rational => "Rationals — рациональные числа (p/q)".to_string(),
            Self::Real => "Reals — вещественные числа".to_string(),
            Self::Complex => "Complex numbers — комплексные числа".to_string(),
            Self::Prime => "Prime numbers — простые числа".to_string(),
            Self::Empty => "Empty set — пустое множество".to_string(),
            Self::Universal => "Universal set — универсальное множество".to_string(),
            Self::Boolean => "Boolean set — булевы значения {true, false}".to_string(),
            Self::StringSet => "String set — множество строк".to_string(),
        }
    }
}

impl OperationTrait for Sets {
    fn literal(&self) -> String {
        match self {
            Self::Standart(set) => set.literal(),
            Self::Custom(CustomSet { elements, range }) => {
                if range.is_some() {
                    let range = range.unwrap();

                    if elements.is_some() {
                        return format!("[{} до {}], исключения: {:?}", range.0, range.1, elements);
                    }
                    format!("[{} до {}]", range.0, range.1)
                } else if elements.is_some() {
                    format!("{:?}", elements)
                } else {
                    format!("Что то не то")
                }
            }
        }
    }

    fn description(&self) -> String {
        match self {
            Self::Standart(set) => set.description(),
            Self::Custom(CustomSet { range, elements }) => {
                if range.is_some() && elements.is_none() {
                    let range = range.unwrap();
                    format!("Промежуток от {} до {}", range.0, range.1)
                } else if elements.is_some() && range.is_none() {
                    format!("Промежуток {:?}", elements)
                } else {
                    format!("Что-то не так")
                }
            }
        }
    }
}
// ForAll(x ∈ T and y ∈ [2,5], Integral(x, 0, π, sin(x^2) + y), T = {-100, 100}; [0])
