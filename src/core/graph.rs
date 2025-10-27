// ProjectGraph - wrapper around petgraph for DAG operations
//
// This file is for Phase 3 - you'll work on this after mastering enums and structs
// For now, it's just a placeholder

use super::Node;
use petgraph::visit::{EdgeRef, Visitable};
use petgraph::{Graph, Directed};
use petgraph::graph::NodeIndex;
use petgraph::algo::is_cyclic_directed;
use uuid::Uuid;
use std::collections::HashMap;
use serde::{Serialize,Deserialize};

#[derive(Debug, Clone,Copy, Serialize, Deserialize)]
pub enum DependencyType{
    Blocks,
    ResourcesRequiredFor,
    Contains,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProjectGraph{
    graph: Graph<Node,DependencyType,Directed>,
    uid_to_index : HashMap<Uuid,NodeIndex>,
}

impl ProjectGraph{
    
    pub fn new() -> Self{
        ProjectGraph{
            graph: Graph::new(),
            uid_to_index: HashMap::new()
        }
    }

    fn is_valid_connection(from: &Node, to: &Node, dep_type: &DependencyType)-> bool{
        use Node::*;
        use DependencyType::*;

        match (from, to, dep_type) {
            (Spec{..}, Spec{..},Contains) => true,
            (Spec{..}, Project{..}, Contains) => true,
            (Project{..}, Project{..},Contains) => true,
            (Project{..}, Epic{..}, Contains) => true,
            (Project{..}, UserStory{..}, Contains) => true,
            (Epic{..}, UserStory{..}, Contains) => true,
            (UserStory{..}, Tasks{..}, Contains) => true,

            // Blocks relationships (same or compatible levels)
            (Project{..},Project{..},Blocks) => true,
            (Epic{..}, Epic{..}, Blocks) => true,
            (UserStory{..}, UserStory{..}, Blocks) => true,
            (Tasks{..}, Tasks{..}, Blocks) => true,
            (UserStory{..}, Epic{..}, Blocks) => true,  // Story can block epic

            // ResourcesRequiredFor (flexible, mostly same level)
            (Project{..}, Project{..},ResourcesRequiredFor) => true,
            (Epic{..}, Epic{..}, ResourcesRequiredFor) => true,
            (UserStory{..}, UserStory{..}, ResourcesRequiredFor) => true,
            (Tasks{..}, Tasks{..}, ResourcesRequiredFor) => true,
            (Tasks{..}, UserStory{..}, ResourcesRequiredFor) => true,

            //everything else is invalid
            _ => false
        }

    }

    fn try_connect(self :&mut Self, node1: &Node, node2: &Node, dep_type :DependencyType)-> Result<(),&'static str>{
        let u1 = node1.get_id();
        let u2 = node2.get_id();
        let from_idx = *self.uid_to_index.get(&u1).expect("Bug: node existence was already verified");
        let to_idx = *self.uid_to_index.get(&u2).expect("Bug: node existence was already verified");

        self.graph.add_edge(from_idx,to_idx,dep_type);

        if is_cyclic_directed(&self.graph){
            if let Some(edge_idx) = self.graph.find_edge(from_idx,to_idx){
                self.graph.remove_edge(edge_idx);
            }

            return Err("Connection would create a cycle");
        }

        Ok(())
    }

    pub fn add_node(self: &mut Self, node: &Node)->Result<(),&'static str>{
        let node_id = node.get_id();
        
        // check that the node_id is not already associated with another node_idx
        if self.uid_to_index.keys().any(|k| *k == node_id){
            return Err("The node has already been inserted into the graph");
        }

        let node_idx: NodeIndex = self.graph.add_node(node.clone());
        self.uid_to_index.insert(node_id,node_idx);
        Ok(())
    }

    pub fn connect_nodes(self: &mut Self, node1: &Node, node2: &Node, dep_type: DependencyType)->Result<(),&'static str>{
        let u1: Uuid = node1.get_id();
        let u2: Uuid = node2.get_id();

        if !Self::is_valid_connection(node1,node2,&dep_type){
            return Err("Invalid connection between the two nodes");
        }

        if !self.uid_to_index.contains_key(&u1) || !self.uid_to_index.contains_key(&u2){
            return Err("One or more of the nodes does not exist in the graph");
        }

        self.try_connect(node1,node2,dep_type)
    }

    pub fn get_node(self : &Self, id : Uuid)->Option<&Node>{
        self.uid_to_index.get(&id).and_then(|idx| self.graph.node_weight(*idx))
    }

    pub fn get_dependencies(self : &Self, uuid: Uuid) -> Option<Vec<(Uuid,DependencyType)>>{
            self.uid_to_index.get(&uuid).map(|idx|{
                self.graph.edges(*idx)
                    .filter_map(|e|{
                        self.graph.node_weight(e.target())
                            .map(|target| (target.get_id(),*e.weight()))
                    })
                    .collect()
            })
    }


}