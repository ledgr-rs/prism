# Contributing to Prism

Thank you for your interest in contributing to Prism.

This document provides guidelines and expectations for contributors.

---

## Quick Links

- [Code of Conduct](CODE_OF_CONDUCT.md)
- [Project Context](context.md)
- [README](README.md)
- [Issue Tracker](https://github.com/anomalyco/opencode/issues)

---

## Philosophy

Before contributing, read [`context.md`](context.md) to understand the project's
design philosophy and current direction.

Prism follows an **architecture-first** approach. Interfaces and contracts are
designed before implementation. The architecture should remain understandable
without reading source code.

---

## Code of Conduct

All contributors must abide by the [Code of Conduct](CODE_OF_CONDUCT.md).

Be respectful, constructive, and professional.

---

## Getting Started

1. Fork the repository.
2. Clone your fork.
3. Create a branch for your work.
4. Make changes.
5. Open a pull request.

---

## Coding Guidelines

When contributing, follow these priorities in order:

- **Architecture before implementation** — Understand how your change fits into
  the pipeline before writing code.
- **Interfaces before concrete types** — Define contracts first, then implement
  them.
- **Explainability before optimization** — Routing decisions must be
  inspectable. Do not sacrifice clarity for performance.
- **Provider neutrality over vendor-specific features** — Prism must remain
  agnostic. Avoid assumptions about any specific provider.
- **Favor composition over inheritance.**
- **Keep modules small and focused.**

### Style

- Use clear, descriptive names.
- Write readable code. Comments should explain *why*, not *what*.
- Avoid unnecessary dependencies.
- Do not introduce hidden behavior or magic.

---

## Design Principles

Prism's design is guided by four principles:

1. **Explainable** — Every routing decision should be inspectable. A routing
   decision without justification is incomplete.
2. **Provider Agnostic** — Applications depend on capabilities, not vendor
   APIs. Providers are plugins.
3. **Policy Driven** — Business constraints (cost, latency, privacy,
   jurisdiction, reliability) belong in configurable policies, not hardcoded
   logic.
4. **Architecture First** — Interfaces and contracts come before
   implementation.

---

## Scope

### In Scope

- Architecture and interfaces
- Routing pipeline
- Provider abstraction and adapters
- Policy engine
- Documentation

### Not in Scope

Prism is **not**:

- an LLM framework
- an orchestration platform
- an agent framework
- a workflow engine
- a prompt engineering toolkit

If your contribution aligns with these non-goals, consider whether it belongs
in Prism or in a project that builds on Prism.

---

## Pull Request Process

1. Ensure your change is consistent with the project's architecture and
   philosophy.
2. Keep PRs focused. One change per PR.
3. Write a clear title and description explaining *what* and *why*.
4. Link any related issues.
5. Ensure your branch is up to date with the target branch.

---

## Reporting Issues

- Use the issue tracker to report bugs, request features, or ask questions.
- Search existing issues before opening a new one.
- Provide clear reproduction steps for bugs.

---

## License

By contributing, you agree that your contributions will be licensed under the
[MIT License](LICENSE).
