# ADR-001: Hybrid Orchestration Model (Central Intelligence + Decentralized Execution)

**Status:** Accepted  
**Date:** 2026-06-05  
**Deciders:** Sven Normen  
**Related:** ARCHITECTURE.md, Phase 1-2 Roadmap

## Context

The Solnet ecosystem spans globally distributed mesh nodes, AI agents, hardware devices, and blockchain state. A purely decentralized approach (every node fully autonomous) makes complex cross-cutting concerns difficult: global resource optimization, consistent policy enforcement, economic coordination via QNET, and high-level swarm intelligence.

A fully centralized model creates unacceptable single points of failure, latency in partitioned networks, and contradicts the decentralization ethos of Yggdrasil/NovaNet/QNET.

## Decision

Adopt a **hybrid model**:

- **Orchestrator instances** (one or more, with leader election or active-passive) provide the *control plane*: global visibility, complex planning, workflow orchestration, resource allocation decisions, and swarm coordination.
- **Execution and most state** remain fully decentralized on mesh nodes and agent runtimes.
- Use **CRDTs** (or conflict-free replicated data types) + eventual consistency for shared state (node registry subsets, task status, telemetry aggregates).
- Critical decisions and economic events are **anchored on QNET** (hashes, attestations, transactions).
- Edge nodes run lightweight **local orchestrator fallbacks** that can operate autonomously during partitions and reconcile upon reconnection.

## Consequences

**Positive**
- Combines global optimization intelligence with local resilience and low latency.
- Scales to thousands of nodes while keeping the orchestrator relatively lightweight.
- Enables sophisticated agent swarms and economic mechanisms that are hard in pure P2P.
- Graceful degradation: system continues critical functions even if global orchestrator is unreachable.
- Aligns with self-improvement vision (orchestrator can simulate and improve strategies centrally while execution stays distributed).

**Negative / Trade-offs**
- Adds complexity (reconciliation logic, CRDT design, partition detection).
- Requires careful design of what state is centralized vs. decentralized to avoid consistency pitfalls.
- Orchestrator becomes a high-value target (mitigated by zero-trust, replication, and capability-based access).

**Neutral**
- Future evolution toward multi-orchestrator consensus or fully decentralized planning (via swarm intelligence or on-chain governance) remains possible as an upgrade path.

## Alternatives Considered

1. **Pure P2P / fully decentralized** — Rejected for complexity of global optimization and economic coordination.
2. **Strongly centralized with edge caching** — Rejected due to partition intolerance and single point of failure risk in global mesh.
3. **Hierarchical (regional orchestrators + global meta-orchestrator)** — Accepted as future evolution (Phase 4).

## References
- ARCHITECTURE.md §4 (Component Architecture)
- Non-Functional Requirements table (resilience, partition tolerance)
- Yggdrasil partition behavior and NovaNet overlay design
