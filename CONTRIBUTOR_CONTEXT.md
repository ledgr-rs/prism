# Contributor Context

This document provides architectural context for contributors.

Read this before opening a pull request.

---

# Philosophy

Prism values architecture over features.

A small, well-designed interface is preferable to a large implementation.

The repository intentionally evolves from contracts toward implementations.

---

# Current Status

Prism is in its early stages.

The priority is building a stable architecture before implementing routing algorithms.

Documentation, interfaces, and design discussions are considered valuable contributions.

---

# Repository Goals

We are building:

- explainable routing
- provider abstraction
- configurable policies
- reusable interfaces

We are **not** building another agent framework.

---

# Repository Structure

```
docs/
    specifications
    diagrams

src/
    core
    routing
    policies
    providers
    registry
    sdk
```

Module names may evolve as the project grows.

---

# Design Rules

## Explainability First

Every routing decision should eventually be explainable.

If a feature makes explanations harder, reconsider it.

---

## Provider Neutrality

Avoid provider-specific assumptions.

The architecture should support any current or future provider.

---

## Composition

Prefer composition.

Avoid deep inheritance.

---

## Interfaces

Define interfaces before implementations.

Avoid coupling modules together unnecessarily.

---

## Keep Components Small

Each component should have one responsibility.

Examples:

Prompt Analysis

↓

Capability Profile

↓

Router

↓

Provider Adapter

Avoid combining these responsibilities.

---

# Pull Requests

Good pull requests:

- improve architecture
- simplify interfaces
- reduce coupling
- improve documentation
- improve tests

Large feature PRs should be discussed before implementation.

---

# Issues

Before opening an issue, consider:

- Is this architectural?
- Is this provider-specific?
- Does this belong in a plugin?
- Does this introduce hidden behavior?

---

# Code Style

Prefer:

- explicit names
- small modules
- clear ownership
- immutable data where practical
- descriptive documentation

Avoid:

- magic values
- hidden globals
- unnecessary abstractions
- premature optimization

---

# Documentation

Every major architectural change should update:

- README
- ARCHITECTURE.md
- Context documents
- Diagrams (if applicable)

Documentation is part of the implementation.

---

# Long-Term Vision

Prism should become a reusable routing engine that can be embedded into:

- editors
- coding agents
- AI applications
- orchestration frameworks
- inference gateways

without requiring changes to its architecture.

---

# Final Principle

If a contribution makes Prism easier to understand, it is probably moving in the right direction.

If it makes Prism harder to understand, reconsider the design before adding more code.
