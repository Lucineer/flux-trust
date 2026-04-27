# flux-trust

Bayesian trust engine for autonomous agents. Models trust as a probability distribution updated through direct interactions, reputation signals, and decay over time.

## Core Concept

Trust is not a scalar — it's a belief state. Each agent maintains a trust distribution over every other agent it knows, updated via Bayesian inference on observed interactions.

```
Interaction → Evidence → Belief Update → New Trust Distribution
    ↓                              ↓
Action quality              P(trustworthy | evidence)
Communication honesty       Bayesian posterior
Cooperation history         Decay with time
```

## Key Operations

- **Trust.init(agent)** — Initialize trust distribution (uniform prior)
- **Trust.update(agent, evidence)** — Bayesian belief update on interaction
- **Trust.score(agent)** — Point estimate from distribution (expected value)
- **Trust.decay(agent, time)** — Time-based decay (forgetting curve)
- **Trust.revoke(agent)** — Hard reset to adversarial prior
- **Trust.transfer(source, target)** — Inherited trust (friend-of-friend)

## Quick Start

```bash
git clone https://github.com/Lucineer/flux-trust.git
cd flux-trust
cargo test    # run tests
```

## Variants

- [flux-trust-c](https://github.com/Lucineer/flux-trust-c) — C11 implementation
- [fluxtrust-go](https://github.com/Lucineer/fluxtrust-go) — Go implementation

## Design

- **Bayesian priors** — new agents start with conservative trust
- **Multi-signal** — combines action quality, communication, and cooperation
- **Temporal decay** — trust fades without recent interaction
- **Transfer** — agents can inherit trust through delegation chains

---

## Fleet Context

Part of the Lucineer/Cocapn fleet. See [fleet-onboarding](https://github.com/Lucineer/fleet-onboarding) for boarding protocol.

- **Vessel:** JetsonClaw1 (Jetson Orin Nano 8GB)
- **Domain:** Low-level systems, CUDA, edge computing
- **Comms:** Bottles via Forgemaster/Oracle1, Matrix #fleet-ops
