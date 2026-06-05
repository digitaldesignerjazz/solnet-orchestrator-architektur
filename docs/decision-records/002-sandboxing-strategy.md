# ADR-002: AI Agent Sandboxing Strategy

**Status:** Proposed  
**Date:** 2026-06-05  
**Deciders:** Sven Normen  
**Related:** ARCHITECTURE.md §4.2, Phase 2

## Context

The Orchestrator will spawn and manage potentially untrusted or semi-trusted AI agent code/processes (Grok-powered or custom). These agents need access to tools (blockchain, hardware HAL, external APIs) but must not be able to compromise the host node or other agents.

## Decision

Use a **layered sandboxing approach**:

1. **Process isolation** (first iteration): Run agents as separate processes or lightweight containers (Docker with seccomp, AppArmor, or gVisor).
2. **Capability-based tool access**: Agents receive only the minimal set of tools/capabilities they need for a specific task (e.g. read-only telemetry, specific actuator commands).
3. **Future hardening**: WebAssembly (Wasmtime) or Firecracker microVMs for stronger isolation with low overhead.
4. **Monitoring & kill switch**: All agent I/O and resource usage is observable; the orchestrator can terminate misbehaving agents instantly.

## Consequences

**Positive**
- Good balance between security and development velocity for early phases.
- Docker is already familiar from existing prototypes.
- Easy to integrate with existing CI and deployment pipelines.

**Trade-offs**
- Docker has some overhead and attack surface compared to pure WASM or microVMs.
- Requires careful seccomp/AppArmor profiles.

**Alternatives Considered**
- Pure WASM from day one — rejected for higher initial complexity and limited hardware access.
- Unrestricted host processes — rejected for obvious security reasons.

## Implementation Notes
- Start with Docker-based agent runner in Phase 2.
- Define clear capability manifest per agent type.
- Log all tool invocations for audit and self-improvement.
