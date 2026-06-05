# Solnet Orchestrator - Detailed Architecture Specification

**Version:** 0.1.0  
**Status:** Draft / Living Document  
**Last Updated:** 2026-06-05  
**Owner:** Sven Normen / Esslinger & Co.

---

## 1. Executive Summary

The Solnet Orchestrator is the **control plane and intelligence layer** for the Solnet ecosystem. It provides unified coordination across a heterogeneous, globally distributed infrastructure consisting of:

- Mesh network nodes (Yggdrasil + NovaNet/xMesh/QNET)
- AI agent swarms (Grok-powered, self-evolving)
- Blockchain economic layer (QNET smart contracts, XCoin/QCoin)
- Real-world hardware & IoT devices (Soilnova environmental sensors, actuators, edge compute)

It enables **autonomous, goal-directed, economically aligned, and self-improving** operation of the entire system while maintaining strong resilience, privacy, and decentralization principles.

This document provides the detailed technical architecture, component specifications, interfaces, data models, non-functional requirements, and design decisions.

---

## 2. Architectural Principles & Constraints

### Core Principles
1. **Hybrid Intelligence** — Global visibility and complex planning in the orchestrator; execution, state, and most decision-making decentralized on the mesh.
2. **Self-Improvement First** — Every major component must support observability, simulation, and iterative improvement loops.
3. **Zero-Trust & Privacy by Design** — mTLS, capability-based authorization, data minimization, on-device processing where possible.
4. **Energy & Resource Awareness** — Scheduling and task placement consider power budget, carbon impact, and hardware capabilities.
5. **Immersive & Human-Centric** — Support for rich agent personalities, memory, and interaction styles aligned with user preferences (noble, roleplay, emotional AI).
6. **Economic Alignment** — Native, first-class integration with QNET for incentives, verification, governance, and value transfer.
7. **Resilience & Partition Tolerance** — Graceful degradation, local autonomy during disconnects, automatic recovery and reconciliation.

### Key Constraints
- Must run efficiently on edge hardware (Raspberry Pi class and better).
- Support for thousands of nodes and hundreds of concurrent agent swarms.
- GDPR/CCPA compliance for data flows involving personal or environmental data (Hannover base).
- Long-term maintainability and auditability (decision provenance on-chain where critical).

---

## 3. System Context & External Interfaces

### External Actors
- **Human Operators** (Sven Normen et al.): Strategic goal setting, oversight, immersive interaction via dashboards or agent chat.
- **Mesh Nodes**: Yggdrasil peers, NovaNet overlays, QNET participants.
- **AI Agents**: Spawned processes/containers with tool access (blockchain, hardware, external APIs).
- **Hardware Devices**: Soilnova sensors/actuators, custom prototypes, energy systems.
- **Blockchain Network**: QNET (primary), testnets, possibly bridges to other chains.
- **External Oracles/Services**: Weather, market data, identity (future).

### Integration Points (High-Level)
- **Grok Launcher (Rust + egui)**: Primary desktop/monitoring UI. Orchestrator exposes gRPC or REST + WebSocket for real-time updates. Dashboard can visualize node health, active swarms, task queues, and on-chain state.
- **Yggdrasil Admin API / Sockets**: Node discovery, peering info, bandwidth stats, admin commands.
- **QNET Smart Contracts**: Task escrow, result submission + verification, reward claims, reputation updates, governance proposals.
- **Hardware HAL**: Pluggable drivers (I2C, MQTT, custom protocols) with normalized telemetry schema.

---

## 4. Component Architecture (Detailed)

### 4.1 Orchestration Core
**Responsibilities**
- Node lifecycle (discover, register, heartbeat, decommission, quarantine)
- Task scheduling & workflow orchestration (DAG support, priorities, dependencies, deadlines)
- Resource allocation & placement (multi-objective: capability, proximity, load, energy, reputation, cost)
- Policy evaluation & enforcement (reactive rules engine)
- State reconciliation across partitions

**Internal Modules (proposed Rust crate structure)**
```
src/
├── core/
│   ├── mod.rs
│   ├── node_manager.rs          # Node registry, health, lifecycle
│   ├── task_scheduler.rs        # DAG executor, priority queue
│   ├── resource_allocator.rs    # Placement decisions
│   ├── policy_engine.rs         # Rule evaluation, triggers
│   ├── state_store.rs           # Hot state + CRDT interface
├── event_bus/
│   ├── mod.rs
│   ├── types.rs                 # Event enums, payloads
│   ├── bus.rs                   # Publish/subscribe abstraction
├── agents/
│   ├── mod.rs
│   ├── swarm_coordinator.rs
│   ├── agent_runtime.rs         # Spawning, sandboxing, tool registry
├── blockchain/
│   ├── mod.rs
│   ├── qnet_bridge.rs           # gRPC/HTTP client to QNET node or indexer
│   ├── contract_client.rs       # Smart contract interaction helpers
├── mesh/
│   ├── mod.rs
│   ├── yggdrasil_client.rs
│   ├── discovery.rs
├── hal/
│   ├── mod.rs
│   ├── traits.rs                # Hardware driver interface
│   ├── soilnova_driver.rs       # Example implementation
├── telemetry/
│   ├── mod.rs
│   ├── collector.rs
│   ├── exporter.rs              # Prometheus + custom
├── security/
│   ├── mod.rs
│   ├── auth.rs
│   ├── sandbox.rs
```

**Key Data Models (Rust structs, simplified)**
```rust
pub struct Node {
    pub id: NodeId,                    // Yggdrasil address or UUID
    pub capabilities: Vec<Capability>,
    pub location: Option<GeoLocation>,
    pub energy_profile: EnergyProfile,
    pub reputation: ReputationScore,
    pub last_heartbeat: Timestamp,
    pub status: NodeStatus,
}

pub struct Task {
    pub id: TaskId,
    pub workflow_id: Option<WorkflowId>,
    pub priority: Priority,
    pub required_capabilities: Vec<Capability>,
    pub deadline: Option<Timestamp>,
    pub payload: TaskPayload,          // JSON or protobuf
    pub status: TaskStatus,
    pub assigned_nodes: Vec<NodeId>,
    pub result_verification: VerificationMethod, // optimistic, zk, multi-party
}

pub enum Event {
    NodeJoined(Node),
    NodeHeartbeat(NodeId),
    TaskCompleted { task_id: TaskId, result: TaskResult },
    AgentSpawned { swarm_id: SwarmId, agent_id: AgentId },
    PolicyTriggered { policy_id: PolicyId, context: Context },
    OnChainEvent { tx_hash: TxHash, event_type: String },
    // ...
}
```

### 4.2 AI Agent Swarm Coordinator
- Dynamic swarm creation based on task requirements or policies.
- Support for multiple topologies (hierarchical supervisor, peer mesh, blackboard/stigmergy).
- Tool registry: blockchain calls, hardware HAL, external APIs, code execution, Grok inference.
- Sandboxing & capability scoping per agent.
- Lifecycle: spawn → assign tasks → monitor → aggregate results → terminate or evolve.
- Self-improvement hooks: trace logging, outcome scoring, prompt/weight evolution, A/B testing via simulation or on-chain.

**Immersive Extension (optional but recommended)**
Agents can maintain persistent memory, personality traits, and interaction style (e.g., "Sir Lancelot style", noble titles, emotional tone). This aligns with existing immersive roleplay sessions and can improve long-term human-AI collaboration quality.

### 4.3 Blockchain Bridge (QNET)
- Bidirectional: submit transactions (task escrow, result attestation, reward claims), read state (reputation, open tasks, governance).
- Verification strategies: optimistic (with challenge period), multi-agent consensus, future ZK proofs.
- Event listening (via indexer or direct node subscription) to trigger internal workflows.
- Gas/reward accounting and automatic claiming where authorized.

### 4.4 Mesh & Discovery Layer
- Primary: Yggdrasil for addressing, routing, and basic connectivity.
- Secondary: NovaNet/xMesh custom protocols for higher-level orchestration messages (task distribution, state sync).
- QNET as economic coordination overlay.
- Privacy: optional I2P/Tor tunneling for sensitive agent-to-agent or agent-to-orchestrator traffic.
- Local autonomy: every edge node can run a lightweight "local orchestrator" instance that continues critical tasks during global partition and reconciles later.

### 4.5 Hardware Abstraction Layer (HAL)
**Trait-based design (Rust)**
```rust
#[async_trait]
pub trait HardwareDriver: Send + Sync {
    async fn read_telemetry(&self) -> Result<TelemetryBatch>;
    async fn execute_command(&self, cmd: Command) -> Result<CommandResult>;
    fn capabilities(&self) -> Vec<Capability>;
    // calibration, OTA update hooks, etc.
}
```

Example drivers: Soilnova (soil moisture, temp, pH, EC), energy monitor, irrigation valve, camera/vision (future TinyML).

Normalized telemetry schema (example):
```json
{
  "node_id": "...",
  "timestamp": "...",
  "sensors": {
    "soil_moisture": 0.42,
    "temperature_c": 18.7,
    "ph": 6.8
  },
  "actuators": { "valve_1": "open" },
  "energy": { "battery_soc": 0.87, "solar_input_w": 12.4 }
}
```

### 4.6 Telemetry, Observability & Self-Improvement
- OpenTelemetry traces + metrics across all layers.
- Custom mesh-native exporter (lightweight, gossip-based aggregation).
- Anomaly detection models (edge + central).
- Full decision provenance: critical orchestration decisions hash-anchored on QNET.
- Simulation environment (future): replay traces, test new policies/agents safely before deployment.

---

## 5. Data Flow Examples

### 5.1 End-to-End: Soilnova Irrigation Workflow
1. Soilnova HAL reports telemetry (moisture low in zone A).
2. Telemetry collector → Policy engine evaluates rules → triggers "Irrigation Optimization" task.
3. Task scheduler creates workflow DAG.
4. Resource allocator selects suitable nodes + spawns specialized agent swarm.
5. Agents pull historical data + weather oracle → generate plan + estimated QCoin cost.
6. Plan submitted to operator (or auto-approved per policy) via Grok Launcher dashboard or agent chat.
7. Execution command sent via HAL; result + delta logged.
8. On-chain: task result attested, rewards distributed, reputation updated, decision hash recorded.
9. Learning loop: outcome scored → stored for future policy/agent improvement.

### 5.2 Partition Scenario
- Global orchestrator unreachable → local edge orchestrator takes over using last known policy snapshot + local CRDT state.
- Critical tasks continue (e.g. emergency irrigation, safety monitoring).
- Upon reconnection: state reconciliation (CRDT merge), backlog processing, audit log sync to chain.

---

## 6. Non-Functional Requirements

| Category          | Requirement                                      | Target (Initial)          | Notes |
|-------------------|--------------------------------------------------|---------------------------|-------|
| Scalability      | Concurrent nodes                                | 10,000+                   | Hierarchical orchestration planned |
| Scalability      | Concurrent agent swarms                         | 500+                      | Dynamic spawn/terminate           |
| Performance      | Task scheduling decision latency                | < 500ms (p95)             | Edge placement critical           |
| Resilience       | Recovery from 30% node failure                  | Automatic, < 60s          | Local fallback + reconciliation   |
| Energy           | Orchestrator overhead on edge node              | < 5% CPU / < 3W           | Rust + efficient eventing         |
| Security         | AuthN/Z                                         | mTLS + capabilities       | Zero-trust everywhere             |
| Privacy          | Data leaving node                               | Minimized                 | On-device inference preferred     |
| Auditability     | Critical decisions                              | On-chain hash + local log | Immutable provenance              |
| Maintainability  | New hardware driver integration                 | < 2 days                  | Trait + plugin model              |

---

## 7. Technology Choices & Rationale

- **Rust** for core: Memory safety, performance on edge, excellent async (tokio), strong typing for complex state machines.
- **gRPC + Protobuf** (or REST + JSON for simplicity first) for internal and external APIs.
- **Tokio + tracing** for async runtime and structured logging.
- **CRDT libraries** (automerge or custom) for distributed state.
- **NATS or custom Yggdrasil pub/sub** for event bus (low latency, resilient).
- **OpenTelemetry** for observability (future integration with Jaeger/Tempo).
- **Docker / OCI** for agent sandboxing and deployment uniformity.
- **QNET client libraries** (to be developed or adapted from existing blockchain work).

---

## 8. Open Questions & Future Work
- Exact QNET integration details (indexer vs direct node, custom pallet vs smart contracts).
- Optimal sandboxing technology for agents (Wasmtime, gVisor, Docker seccomp, or custom).
- How to handle long-term agent memory and personality persistence at scale.
- Formal verification opportunities for critical policy engine or resource allocator.
- Multi-orchestrator consensus / leader election for high availability.

---

## 9. Glossary
- **NodeId**: Yggdrasil IPv6-like address or stable UUID
- **SwarmId / AgentId**: ULID or UUID
- **Capability**: Enum or tagged struct describing what a node/agent can do (compute, sensor_soil, actuator_valve, llm_inference, etc.)
- **VerificationMethod**: optimistic | multi_agent_consensus | zk_proof

---

*This is a living document. Major changes should be recorded as Architecture Decision Records (ADRs) in `docs/decision-records/`.*
