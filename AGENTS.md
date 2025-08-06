# AGENTS.md

## Purpose
This file defines mandatory guidelines for Codex and any future AI assistant contributing to the Autonomous & Evolutive Intelligence Framework (AEIF). It ensures that all code, documentation, tests, and architectural decisions remain consistent with project standards.

---

## Architectural Principles
1. **Domain-Driven Design (DDD)**
   - Model core domains explicitly.
   - Isolate bounded contexts and express ubiquitous language.
2. **Command Query Responsibility Segregation (CQRS)**
   - Separate command handling (state changes) from queries (read-only operations).
3. **Event Sourcing**
   - Persist domain events as the single source of truth.
   - Rebuild state from event streams.

**Any new feature or change MUST use DDD, CQRS, and Event Sourcing.**

---

## Language and Style
- All code, comments, commit messages, and documentation are in **English** unless explicitly instructed otherwise.
- Rust code must be clear, idiomatic, and modular.
- Maintain a formal, precise tone suitable for open‑source collaboration.

---

## Documentation Requirements
1. **Rustdoc**
   - Provide `///` comments for public items.
2. **Usage Guides**
   - Add or update documentation in `/docs/en/`.
   - Translate and maintain parity in `/docs/fr/` (and other languages if present).
3. **README / ROADMAP**
   - Update when changes impact the overall project.
4. **Consistency**
   - Documentation across languages must remain synchronized.

---

## Testing Requirements
- Every feature or bug fix MUST include:
  - Unit tests.
  - Integration tests when appropriate.
  - Event sourcing tests validating event emission and replay.
- Provide usage examples demonstrating API or feature behavior.

---

## Non‑Negotiable Rules
- Do **not** break the existing architecture.
- Do **not** output in French by default.
- Do **not** omit tests or documentation.
- Ensure extensibility, clean code, and refactoring aligned with the project’s evolutive nature.

---

## Examples

### Agent Design
```rust
pub struct Agent {
    id: AgentId,
    state: AgentState,
}

impl Agent {
    pub fn handle(&mut self, cmd: Command) -> Result<Vec<Event>, Error> {
        match cmd {
            Command::Activate => Ok(vec![Event::Activated]),
            Command::Deactivate => Ok(vec![Event::Deactivated]),
        }
    }
}
```

### Command and Event Structure
```rust
/// Commands mutate state
pub enum Command {
    Activate,
    Deactivate,
}

/// Events represent state changes
pub enum Event {
    Activated,
    Deactivated,
}
```

### Prompt Template for Feature Requests
```
Feature: <concise description>
Bounded Context: <context name>
Commands:
  - <CommandName>: <purpose>
Events:
  - <EventName>: <result>

Provide:
1. Rust code using DDD, CQRS, Event Sourcing.
2. Usage examples.
3. Tests (unit/integration/event sourcing).
4. Documentation updates in /docs/en/ and /docs/fr/.
```

---

## Final Notes
By following these guidelines, Codex and future AI assistants will ensure the AEIF remains robust, consistent, and evolutive. Always verify that every change adheres to this file before submitting.

