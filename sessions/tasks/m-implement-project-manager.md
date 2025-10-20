---
task: m-implement-project-manager
branch: feature/implement-project-manager
status: pending
created: 2025-10-18
modules: [core, tasks, cli]
---

# Project Management Tool in Rust

## Problem/Goal
Build a DAG-based project management tool in Rust that models hierarchical work items with dependencies, time tracking, and resource allocation. Users can define work items as nodes in a directed acyclic graph (Spec → Project → Sub-Project → Epic → User Story) and visualize them through multiple views (swim lanes, Gantt charts). The tool uses topological sorting for intelligent resource allocation and scheduling.

## Success Criteria
- [ ] **Graph Data Model**
  - [ ] DAG structure with cycle detection
  - [ ] Node types: Spec, Project, Sub-Project, Epic, User Story
  - [ ] Dependency relationships between nodes
  - [ ] Node-specific attributes (points for Epic/Story, timelines, etc.)
- [ ] **Core Operations**
  - [ ] Add nodes with type-specific attributes
  - [ ] Define dependencies between nodes
  - [ ] Validate DAG (no cycles)
  - [ ] Topological sort for scheduling
- [ ] **Time & Resource Management**
  - [ ] Timeline tracking per node
  - [ ] Points estimation for Epics/Stories
  - [ ] Resource allocation based on topological order
- [ ] **Visualization & Views**
  - [ ] Swim lane view
  - [ ] Gantt chart view
  - [ ] Dependency graph visualization
- [ ] **Persistence**
  - [ ] Save/load graph structures
  - [ ] Preserve relationships and attributes
- [ ] **CLI Interface**
  - [ ] Add nodes with dependencies
  - [ ] Query and filter nodes
  - [ ] Generate views
  - [ ] Resource allocation commands

## Context Manifest

### Current Project State
- **Build Status**: Compiles cleanly (fixed)
- **Module Structure**: Created proper Rust module hierarchy
  - [src/lib.rs](src/lib.rs) - Library root
  - [src/core/](src/core/) - Core data structures module
  - [src/storage/](src/storage/) - Persistence module
  - [src/cli/](src/cli/) - CLI interface module
- **Rust Version**: Edition 2021, rustc 1.89.0
- **Dependencies Configured**:
  - ✅ chrono (0.4.38) - Timestamps and timelines
  - ✅ uuid (1.10.0 + v6, serde) - Node identifiers
  - ✅ once_cell (1.19.0) - Global state
  - ✅ serde + serde_json (1.0) - Serialization
  - ✅ clap (4.5 + derive) - CLI parsing
  - ✅ anyhow (1.0) - Error handling
  - ⚠️ petgraph (NEEDS TO BE ADDED) - DAG and graph algorithms

### Architecture Overview

**Module Structure**:
```
src/
├── main.rs                    # CLI entry point
├── lib.rs                     # Library root (exports modules)
├── core/
│   ├── mod.rs                # Core module exports
│   ├── node.rs               # Node enum with all types
│   ├── graph.rs              # DAG wrapper around petgraph
│   ├── timeline.rs           # Timeline struct
│   └── attributes.rs         # Type-specific attributes
├── storage/
│   ├── mod.rs                # Storage module exports
│   └── json.rs               # JSON persistence
├── cli/
│   ├── mod.rs                # CLI module exports
│   └── commands.rs           # Subcommands (add, list, view, etc.)
└── views/
    ├── mod.rs                # View module exports
    ├── swimlane.rs           # Swim lane rendering
    └── gantt.rs              # Gantt chart rendering
```

**Data Model Design**:

1. **Node Types** (enum with variants):
   - `Spec { id, name, description, timeline }`
   - `Project { id, name, spec_id, timeline, sub_projects }`
   - `SubProject { id, name, project_id, timeline, epics }`
   - `Epic { id, name, points, timeline, stories }`
   - `UserStory { id, name, points, timeline, assignee }`

2. **Graph Structure**:
   - Uses `petgraph::Graph<Node, DependencyType>`
   - Node indices tracked by Uuid → NodeIndex mapping
   - DAG validation on every edge addition
   - Topological sort for ordering

3. **Supporting Types**:
   - `Timeline { start: DateTime, end: DateTime, estimated_hours: u32 }`
   - `DependencyType` enum (BlockedBy, DependsOn, PartOf)
   - `Points` type alias for story points (u32)

**CLI Architecture** (using clap subcommands):
- `pm add <type> <name>` - Add node (spec, project, epic, story, etc.)
- `pm link <from-id> <to-id>` - Create dependency between nodes
- `pm list [--type <type>]` - List nodes, optionally filtered
- `pm show <id>` - Show node details with dependencies
- `pm timeline <id>` - Set/update timeline for node
- `pm points <id> <points>` - Set story points (Epic/Story only)
- `pm view swimlane` - Render swim lane view
- `pm view gantt` - Render Gantt chart
- `pm schedule` - Show topological order for resource allocation
- `pm validate` - Check DAG for cycles

**Persistence Strategy**:
- Store graph in `~/.project-manager/graph.json`
- Serialize both nodes and edges with attributes
- Preserve Uuid → NodeIndex mappings
- Rebuild graph structure on load

### Implementation Steps (Learning Path)

**Phase 1: Rust Fundamentals**
1. Learn enums with data (Node types)
2. Learn structs (Timeline, attributes)
3. Learn Option and Result types
4. Understand ownership basics

**Phase 2: Core Data Model**
1. Define Node enum with all variants
2. Create Timeline and supporting structs
3. Add traits for shared behavior (Schedulable, Pointable)
4. Implement basic serialization

**Phase 3: Graph Implementation**
1. Integrate petgraph
2. Build DAG wrapper with validation
3. Implement topological sorting
4. Add dependency management

**Phase 4: Persistence**
1. JSON serialization of graph
2. Save/load operations
3. Handle errors gracefully

**Phase 5: CLI & Views**
1. Set up clap subcommands
2. Implement node operations
3. Build swim lane renderer
4. Build Gantt chart renderer

**Phase 6: Resource Allocation**
1. Timeline conflict detection
2. Resource-aware scheduling
3. Critical path analysis

## Context Files
- [Cargo.toml](Cargo.toml) - Dependencies configuration
- [src/lib.rs](src/lib.rs) - Library root (empty, needs module declarations)
- [src/main.rs](src/main.rs) - CLI entry point
- [src/core/mod.rs](src/core/mod.rs) - Core module (needs implementation)
- [src/storage/mod.rs](src/storage/mod.rs) - Storage module (needs implementation)
- [src/cli/mod.rs](src/cli/mod.rs) - CLI module (needs implementation)

## User Notes
**Learning Approach**: This is a learning project for Rust. Guy will write the code himself with guided instruction. Focus on teaching Rust fundamentals (enums, structs, ownership, traits) through practical implementation of a DAG-based project management system.

**Key Teaching Moments**:
- Enums with data variants for node types
- Trait-based polymorphism for shared behavior
- Graph algorithms with petgraph
- Serialization with serde
- CLI design with clap

## Work Log
- [2025-10-18] Task created
- [2025-10-18] Updated scope to DAG-based PM tool with learning path
