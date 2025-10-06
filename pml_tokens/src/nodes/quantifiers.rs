use crate::nodes::prelude::LeafNodes;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct QuantifiersNode {
    pub var: LeafNodes,
    pub set: LeafNodes,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub enum Quantifiers {
    ForAll(Vec<QuantifiersNode>),
    Exists(Vec<QuantifiersNode>),
}

impl QuantifiersNode {
    pub fn new(var_name: impl Into<String>, set: LeafNodes) -> Self {
        QuantifiersNode {
            var: LeafNodes::Variable(var_name.into()),
            set,
        }
    }
}
