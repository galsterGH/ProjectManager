# Rust Learning Guide for Project Manager

This guide will teach you the Rust concepts you need to build the DAG-based project management tool. We'll learn by doing - each concept will be applied directly to your project.

## Phase 1: Rust Fundamentals

### 1.1 Enums with Data

**What are they?**
Enums in Rust can hold different data per variant. Unlike C-style enums that are just numbers, Rust enums are "algebraic data types" - each variant can be like a mini-struct.

**Your use case:** Node types
```rust
// Each variant holds different fields
enum Node {
    Spec {
        id: Uuid,
        name: String,
        description: String
    },
    Epic {
        id: Uuid,
        name: String,
        points: u32,          // Only Epics have points
        timeline: Timeline
    },
    UserStory {
        id: Uuid,
        name: String,
        points: u32,
        assignee: Option<String>  // Only Stories have assignees
    },
}
```

**Pattern matching** - How you work with enums:
```rust
match node {
    Node::Epic { points, .. } => println!("Epic has {} points", points),
    Node::UserStory { assignee, .. } => println!("Assigned to {:?}", assignee),
    Node::Spec { .. } => println!("This is a spec"),
}
```

**Exercise:** You'll create the Node enum with all 5 types: Spec, Project, SubProject, Epic, UserStory

---

### 1.2 Structs

**What are they?**
Structs are custom data types that group related data together.

**Your use case:** Timeline and shared attributes
```rust
struct Timeline {
    start: DateTime<Utc>,
    end: DateTime<Utc>,
    estimated_hours: u32,
}

struct DependencyInfo {
    depends_on: Vec<Uuid>,
    blocks: Vec<Uuid>,
}
```

**Exercise:** You'll create Timeline, DependencyInfo, and other supporting structs

---

### 1.3 Option<T> - Handling "may not exist"

**What is it?**
Rust has no `null`. Instead, we use `Option<T>`:
- `Some(value)` - there is a value
- `None` - there is no value

**Your use case:** Optional fields like assignee, points
```rust
struct UserStory {
    assignee: Option<String>,  // Might not be assigned yet
}

// Working with Option
match story.assignee {
    Some(name) => println!("Assigned to {}", name),
    None => println!("Unassigned"),
}

// Or use if let
if let Some(name) = story.assignee {
    println!("Assigned to {}", name);
}
```

---

### 1.4 Result<T, E> - Error Handling

**What is it?**
Functions that can fail return `Result<T, E>`:
- `Ok(value)` - success with value
- `Err(error)` - failure with error

**Your use case:** Graph operations that can fail
```rust
fn add_edge(from: Uuid, to: Uuid) -> Result<(), String> {
    if would_create_cycle(from, to) {
        return Err("Would create a cycle!".to_string());
    }
    Ok(())
}

// Using Result with ? operator
fn add_node_with_deps() -> Result<(), String> {
    add_edge(id1, id2)?;  // If Err, return early
    add_edge(id2, id3)?;  // Only runs if previous succeeded
    Ok(())
}
```

---

### 1.5 Ownership Basics

**The Rules:**
1. Each value has one owner
2. When owner goes out of scope, value is dropped
3. You can borrow references (&T) without taking ownership

**Why it matters:**
```rust
// This takes ownership - node is moved
fn consume(node: Node) { }

// This borrows - we can still use node after
fn inspect(node: &Node) { }

let my_node = Node::Spec { ... };
inspect(&my_node);  // Borrow it
inspect(&my_node);  // Can borrow again
consume(my_node);   // Moves ownership
// my_node is no longer accessible here
```

**Your use case:** Graph will own nodes, but we'll borrow references to read them

---

## Phase 2: Traits - Shared Behavior

### 2.1 What are Traits?

Traits define shared behavior across types. Like interfaces in other languages, but more powerful.

**Your use case:** Not all nodes have points, but Epic and UserStory do
```rust
trait Pointable {
    fn points(&self) -> Option<u32>;
    fn set_points(&mut self, points: u32);
}

// Only implement for Epic and UserStory
impl Pointable for Node {
    fn points(&self) -> Option<u32> {
        match self {
            Node::Epic { points, .. } => Some(*points),
            Node::UserStory { points, .. } => Some(*points),
            _ => None,  // Other types don't have points
        }
    }
}
```

**Exercise:** You'll create traits for:
- `Pointable` - Epic and UserStory have points
- `Schedulable` - All nodes have timelines
- `Identifiable` - All nodes have Uuid

---

## Phase 3: Working with Petgraph

### 3.1 What is Petgraph?

A graph data structure library. Provides:
- Graph storage
- DAG validation
- Topological sorting
- Graph algorithms

**Basic usage:**
```rust
use petgraph::Graph;

let mut graph = Graph::<Node, DependencyType>::new();

// Add nodes (returns NodeIndex)
let spec_idx = graph.add_node(Node::Spec { ... });
let epic_idx = graph.add_node(Node::Epic { ... });

// Add edges (dependencies)
graph.add_edge(spec_idx, epic_idx, DependencyType::BlockedBy);
```

**Your use case:** You'll wrap petgraph in a DAG struct that:
- Maintains Uuid → NodeIndex mapping
- Validates no cycles before adding edges
- Provides topological sorting

---

## Phase 4: Serialization with Serde

### 4.1 What is Serde?

Library for serializing/deserializing data. Turn Rust structs into JSON and back.

**Usage:**
```rust
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
struct Timeline {
    start: DateTime<Utc>,
    end: DateTime<Utc>,
}

// Automatic serialization
let json = serde_json::to_string(&timeline)?;

// Automatic deserialization
let timeline: Timeline = serde_json::from_str(&json)?;
```

**Your use case:** Save/load the entire graph to JSON

---

## Phase 5: CLI with Clap

### 5.1 What is Clap?

Command-line argument parser. Derive-based API makes it easy.

**Usage:**
```rust
use clap::{Parser, Subcommand};

#[derive(Parser)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    Add {
        node_type: String,
        name: String
    },
    List,
    View {
        view_type: String
    },
}
```

**Your use case:** Build the `pm` CLI with all subcommands

---

## Learning Path Summary

1. **Start with enums and structs** - Define Node and Timeline
2. **Add traits** - Pointable, Schedulable behaviors
3. **Integrate petgraph** - Build DAG wrapper
4. **Add persistence** - Serde JSON save/load
5. **Build CLI** - Clap subcommands
6. **Add views** - Swim lane and Gantt rendering

Each phase builds on the previous one. You'll write the code yourself, and I'll guide you through any concepts or syntax you need.

---

## Quick Reference

**Common Rust syntax you'll use:**
- `let x = 5;` - immutable binding
- `let mut x = 5;` - mutable binding
- `&x` - borrow reference
- `&mut x` - mutable reference
- `x?` - propagate error
- `x.unwrap()` - panic if error (use sparingly!)
- `x.expect("msg")` - panic with message
- `if let Some(v) = x` - pattern match Option
- `match x { ... }` - pattern matching
- `..` - ignore remaining fields in pattern
- `_` - ignore this value

**Ready to start?** Let me know when you want to begin, and we'll start with Phase 1!
