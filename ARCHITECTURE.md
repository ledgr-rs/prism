# Architecture

Prism is an explainable model routing engine.

Its responsibility is deciding **which model should handle a request** and **why**.

Execution is optional.

---

# Design Goals

The routing engine should be:

- explainable
- provider agnostic
- policy driven
- composable
- deterministic when required

Routing is treated as its own architectural layer rather than application logic.

---

# High-Level Architecture

> Replace with the latest architecture diagram.

![Prism Architecture](docs/assets/architecture.png)

---

# Routing Pipeline

```
Interfaces
      │
      ▼
Prism Core
      │
 ┌────┴────┐
 ▼         ▼
Prompt     Provider
Analysis   Registry
 ▼         ▼
Capability Provider
Profile    Catalog
      │
      ▼
    Router
      │
      ▼
Provider Adapters
```

---

# Components

## Interfaces

Entry points into Prism.

Examples:

- Rust SDK
- HTTP API
- CLI
- Agent Frameworks

Interfaces should never contain routing logic.

---

## Prism Core

Coordinates the routing pipeline.

Responsible for orchestrating components.

It does not implement provider-specific behavior.

---

## Prompt Analysis

Extracts routing requirements.

Produces a capability profile describing the request.

Examples:

- reasoning
- coding
- multimodal
- context size
- latency sensitivity

---

## Provider Registry

Maintains metadata describing available providers.

Examples:

- supported capabilities
- pricing
- context limits
- modalities
- latency
- availability

The registry should contain data, not logic.

---

## Router

Combines:

- capability profile
- provider catalog
- routing policies

Produces a recommendation.

---

## Policies

Policies modify routing decisions.

Examples:

- cost limits
- latency limits
- privacy
- regional restrictions
- preferred providers

Policies are configuration, not implementation.

---

## Provider Adapters

Responsible only for execution.

Adapters translate Prism requests into provider APIs.

They should never perform routing.

---

# Explainability

Every recommendation should be explainable.

Future routing reports may include:

- required capabilities
- candidate providers
- rejected providers
- applied policies
- final decision

---

# Future Extensions

The architecture intentionally leaves room for:

- plugin providers
- custom policies
- benchmarking
- local models
- policy DSL
- tracing
- telemetry

These should extend existing interfaces rather than changing them.

---

# Guiding Principle

Routing is a decision problem.

Execution is a separate concern.
