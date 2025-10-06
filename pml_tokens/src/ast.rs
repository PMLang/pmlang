use serde::{Deserialize, Serialize};

use super::nodes::prelude::*;

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct ASTNode {
    pub node: NodeType,
    pub children: Vec<ASTNode>,
    // pub line: usize,
    // pub column: usize,
}
