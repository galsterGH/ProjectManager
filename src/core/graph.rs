// ProjectGraph - wrapper around petgraph for DAG operations
//
// This file is for Phase 3 - you'll work on this after mastering enums and structs
// For now, it's just a placeholder

use super::Node;
use petgraph::{adj::NodeIndex, Graph};
use uuid::Uuid;
use std::collections::HashMap;
use serde::{Serialize,Deserialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum DependencyType{
    Blocks,
    ResourcesRequiredFor,
    Contains,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProjectGraph{
    graph: Graph<Node,DependencyType>,
    uid_to_index : HashMap<Uuid,NodeIndex>,
}

impl ProjectGraph{
    
    pub fn new() -> Self{
        ProjectGraph{
            graph: Graph::new(),
            uid_to_index: HashMap::new()
        }
    }

    
}