# Prism

> An explainable decision engine for model routing.

Prism separates routing from execution, allowing applications to choose the right model through explicit capabilities, policies, and transparent decisions.

---

## Why Prism?

Modern AI applications rarely rely on a single model.

As providers and models grow, routing logic gradually spreads throughout the application:

- provider-specific conditionals
- capability checks
- latency heuristics
- cost thresholds
- fallback chains

What begins as a few `if` statements eventually becomes infrastructure that is difficult to reason about, extend, and test.

Routing is a system of its own. Prism treats it that way.

---

## How It Works

Instead of asking *which provider should I call?*, applications describe **what they need**.

Prism then:

1. analyzes the request
2. identifies the required capabilities
3. evaluates routing policies
4. selects the best execution target
5. explains the decision

Execution is optional. Prism can recommend a model or execute the request through provider adapters.

---

## Crates

| Crate | Description |
|-------|-------------|
| `prism-core` | Domain types, traits, and contracts shared across all crates |
| `prism-engine` | 13-stage deterministic routing pipeline |
| `prism-registry` | Model profile storage and retrieval |
| `prism-tui` | Terminal interface for interactive decision exploration |

### prism-tui

Launch with `cargo run` to enter an interactive prompt evaluator.

Type or paste a request, press Enter, and explore the full routing pipeline:

- pipeline sidebar with stage-by-stage navigation
- structured workspace views per stage
- contextual details panel
- search, replay, and JSON export

---

## Core Ideas

### Routing is independent

Model selection should not be coupled to application logic.

### Decisions should be explainable

Every routing decision should be inspectable and reproducible.

### Policies drive routing

Cost, latency, privacy, governance, and organization-specific rules determine how routing decisions are made.

### Providers are interchangeable

Applications depend on capabilities rather than vendor-specific APIs.

---

## Pipeline

```
Prompt
  ↓
Normalization
  ↓
Intrinsic Extraction  →  Derived Analysis
  ↓
Requirement Inference  →  Capability Mapping  →  Prioritization
  ↓
Candidate Filtering
  ↓
Policy Evaluation
  ↓
Candidate Scoring
  ↓
Decision Selection
  ↓
Explanation Generation
  ↓
Decision Report
```

---

## Roadmap

- [x] Core routing engine
- [x] Capability taxonomy and extraction
- [x] Policy framework with typed rules
- [x] Candidate scoring with evidence
- [x] Interactive terminal UI
- [ ] Provider adapters
- [ ] HTTP API
- [ ] Configuration DSL
- [ ] Plugin system

---

## Getting Started

```sh
cargo build
cargo test
cargo run -p prism-tui
```

---

## Contributing

We welcome contributions from the community.

See **CONTRIBUTING.md** for setup instructions and contribution guidelines.

---


