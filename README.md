# Prism

> An explainable decision engine for model routing.

Prism separates routing from execution, allowing applications to choose the right model through explicit capabilities, policies, and transparent decisions.

<p align="center">
  <img src="docs/assets/architecture1.png" alt="Prism Architecture" width="900">
</p>

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

Instead of asking:

> Which provider should I call?

Applications describe **what they need**.

Prism then:

1. analyzes the request
2. identifies the required capabilities
3. evaluates routing policies
4. selects the best execution target
5. explains the decision

Execution is optional.

Prism can either recommend a model or execute the request through provider adapters.

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

## Roadmap

- Core routing engine
- Capability registry
- Policy framework
- Provider adapters
- SDKs
- Benchmark suite

---

## Contributing

We welcome contributions from the community.

See **CONTRIBUTING.md** for setup instructions and contribution guidelines.

---

## License

MIT
