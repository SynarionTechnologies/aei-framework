# 🚀 AEI Framework – ROADMAP

_Autonomous & Evolutive Intelligence Framework (AEIF)_

---

## Phase 0 — Foundations & Project Specs

- [x] **Initialize the repository:**
  - Rust workspace
  - Modular crates structure
  - CI/CD for build & test
- [ ] **Documentation:**
  - [x] `README.md` (vision, goals, quickstart)
  - [ ] `GLOSSARY.md` or section

---

## Phase 1 — Core Architecture & Specifications

- [ ] **Define core traits and interfaces:**
  - `NeuralNetwork`: minimal interface for any network
  - `KnowledgeModule`: standard interface for attachable skills
  - `MemoryStore`: abstract storage backend
- [ ] **Module skeletons:** minimal code for each crate with clear responsibilities
- [ ] **Doc comments:** each interface documented (Rust style)

---

## Phase 2 — Minimal Neural Network Engine

- [ ] **`crates/core/nn` module:**
  - `Neuron`, `Layer`, `Network` structs
  - Forward propagation (sigmoid/ReLU)
  - Pluggable layer structure for extension
- [ ] **API design:**
  - Instantiate networks with arbitrary topology
  - Forward function: input → output
- [ ] **Examples:**
  - Simple usage in `examples/` (e.g., XOR, linear regression)

---

## Phase 3 — Persistence & Memory Abstraction

- [ ] **`crates/memory`:**
  - `MemoryStore` trait: put/get/save/load pattern
  - In-memory backend (HashMap)
  - File-based backend (JSON/YAML/TOML)
- [ ] **Serialization:**
  - Save and restore neural network weights/config
- [ ] **Demo:**
  - Store/load agent state and support checkpointing

---

## Phase 4 — Knowledge Modules & Semantic Memory

- [ ] **`crates/modules`:**
  - Define `KnowledgeModule` trait (versioned, attach/detach, identify)
  - Basic modules: e.g., "math", "echo", "counter"
- [ ] **Semantic memory:**
  - Structure knowledge as graphs or embeddings
  - Persistence hooks for long-term storage
- [ ] **API & documentation:**
  - How to build/extend modules and feed semantic memory

---

## Phase 5 — NLP Engine

- [ ] **Text processing pipeline:**
  - Tokenization, stemming, basic parsing
  - Embedding generation for integration with semantic memory
- [ ] **Language understanding modules:**
  - Intent recognition
  - Entity extraction linked to knowledge base
- [ ] **Examples:**
  - Simple chatbot demo showcasing NLP capabilities

---

## Phase 6 — Runtime & Agent Orchestration

- [ ] **`crates/runtime`:**
  - Agent scheduler (single & multi-agent)
  - Event loop or tick-based system
- [ ] **Interactions:**
  - Agent-to-agent communication (message/event bus)
  - Dynamic module orchestration
- [ ] **Examples:**
  - Multi-agent scenario in `examples/` (agents greeting, exchanging info)

---

## Phase 7 — Learning, Adaptation & Reasoning

- [ ] **Learning loop:**
  - Backpropagation for simple networks
  - Incremental training API
- [ ] **Reasoning abilities:**
  - Rule-based or heuristic reasoning over semantic memory
  - Simple planning/decision modules
- [ ] **Metrics & evaluation:**
  - Track performance and learning progress

---

## Phase 8 — Tests, Documentation, and Demos

- [ ] **Unit & integration tests:**
  - Coverage for core crates and modules
- [ ] **Comprehensive documentation:**
  - Guides, API docs, tutorials
- [ ] **Showcase demos:**
  - In `examples/`: create, train, persist, and interact with agents

---

## Phase 9 — Community, Extensibility & Release

- [ ] **Open source onboarding:**
  - [ ] Issues & milestones on GitHub
  - [x] Clear contributor guidelines
- [ ] **Extensibility:**
  - Plugin system and extension templates
  - FFI hooks for other languages
- [ ] **First release (v0.1):**
  - API stabilization
  - Publish on crates.io and announce to the community

---

## Stretch Goals

- [ ] Web or CLI interface for agent management
- [ ] Support for alternative neural architectures (RNN, CNN)
- [ ] Persistence in cloud backends
- [ ] Interfacing with real-world data (APIs, web scraping)
- [ ] Integration with other languages (Python bindings, etc.)

---

## How to contribute

- Discuss new features via GitHub issues
- Propose your own modules or skills
- Submit PRs with tests and documentation

---

*This roadmap is iterative and may evolve based on community feedback and project progress.*

