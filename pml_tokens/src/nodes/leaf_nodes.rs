use super::sets::Sets;
use super::traits::OperationTrait;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Deserialize, Serialize)]
pub enum LeafNodes {
    Number(f64),
    Variable(String),
    Set(Sets),
}

impl OperationTrait for LeafNodes {
    fn description(&self) -> String {
        match self {
            Self::Number(num) => format!("{}", num),
            Self::Set(set) => set.description(),
            Self::Variable(var) => format!("{}", var),
        }
    }

    fn literal(&self) -> String {
        match self {
            Self::Number(num) => format!("Число {}", num),
            Self::Variable(var) => format!("Переменная {}", var),
            Self::Set(set) => set.literal(),
        }
    }
}
