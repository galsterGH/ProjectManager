// Core module - contains the main data structures

pub mod graph;
pub mod node;
pub mod timeline;

// Re-export main types for convenience
pub use node::Node;
pub use node::NodeBuilder;
pub use timeline::Timeline;
//pub use graph::ProjectGraph;
