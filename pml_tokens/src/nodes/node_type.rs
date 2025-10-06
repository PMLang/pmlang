use super::arithmetic_operation;
use super::functions;
use super::integrals_derivatives;
use super::leaf_nodes;
use super::logics;
use super::quantifiers;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Deserialize, Serialize)]
pub enum NodeType {
    Arithmetic(arithmetic_operation::ArithmeticOperation),
    Function(functions::Functions),
    IntegralDerivative(integrals_derivatives::IntegralsDerivatives),
    LeafNode(leaf_nodes::LeafNodes),
    Logic(logics::Logics),
    Quantify(quantifiers::Quantifiers),
    Empty,
}
