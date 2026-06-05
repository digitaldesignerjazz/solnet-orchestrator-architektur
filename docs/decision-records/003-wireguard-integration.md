# ADR-003: WireGuard Integration for Secure Node Connectivity

**Status:** Proposed  
**Date:** 2026-06-05  
**Deciders:** Sven Normen  
**Related:** ARCHITECTURE.md, Yggdrasil integration, NovaNet/xMesh

## Context

The Solnet ecosystem currently relies heavily on **Yggdrasil** for mesh networking. While Yggdrasil is excellent for global, addressable, end-to-end encrypted connectivity without central coordination, it has limitations in certain environments:

- Performance on high-latency or lossy links
- NAT traversal can be unreliable in some corporate/ISP environments
- Limited control over routing and encryption parameters
- Desire for a well-known, audited, high-performance VPN protocol as complement or fallback

**WireGuard** offers:
- Extremely simple and auditable codebase
- Excellent performance (kernel module or userspace)
- Strong modern cryptography (Noise protocol)
- Easy peer configuration
- Good NAT traversal with UDP hole punching

Integrating WireGuard would allow Solnet nodes to establish secure, high-performance tunnels on demand, either as a transport under Yggdrasil/NovaNet or as a standalone mesh overlay.

## Decision

Adopt a **hybrid approach**:

- **Primary mesh**: Continue using Yggdrasil + NovaNet/xMesh for global addressing and discovery.
- **WireGuard as optional secure transport layer**: Nodes can establish WireGuard tunnels to selected peers for improved performance, reliability, or when Yggdrasil connectivity is poor.
- The **Orchestrator** becomes responsible for:
  - Generating and distributing WireGuard keypairs and peer configurations
  - Deciding which peers should establish direct WireGuard tunnels (based on latency, geography, policy, or manual request)
  - Monitoring WireGuard interface health

This keeps the system flexible: nodes can operate with pure Yggdrasil, pure WireGuard, or a combination.

## Consequences

**Positive**
- Significantly better performance for high-bandwidth or latency-sensitive traffic
- More reliable connectivity behind difficult NATs
- Strong, well-audited encryption as additional layer
- Easier integration with existing infrastructure (many routers, VPS providers, and tools already support WireGuard)
- Clean separation: Yggdrasil for global reachability, WireGuard for optimized paths

**Trade-offs & Risks**
- Additional complexity in key and configuration management
- Need for careful coordination to avoid routing loops or conflicts with Yggdrasil
- WireGuard requires kernel support (or userspace implementation like `boringtun`)
- Key distribution must be secure (orchestrator becomes a trust anchor for WireGuard keys)

**Implementation Considerations**
- Use `wireguard-rs` or shell out to `wg` / `wg-quick` for interface management (initially simpler to use system tools)
- Store WireGuard private keys securely on nodes (never send private keys over network)
- Orchestrator only distributes public keys + allowed IPs + endpoints
- Support for dynamic endpoint updates (roaming nodes)
- Optional: Use WireGuard as underlay for selected high-priority flows while keeping Yggdrasil as control plane

## Phased Rollout Proposal

**Phase A – Foundation (Short term)**
- Add WireGuard key generation and basic peer configuration distribution in the Orchestrator
- Simple REST/WebSocket API to request a WireGuard tunnel to another node
- Basic interface bring-up on nodes (via HAL or dedicated agent)

**Phase B – Intelligent Tunnel Management**
- Orchestrator decides which peers should have direct WireGuard tunnels (based on observed latency, bandwidth, or policy)
- Automatic configuration push when beneficial
- Health monitoring and automatic fallback to Yggdrasil

**Phase C – Advanced**
- WireGuard as primary transport for selected traffic classes
- Integration with QNET for economic incentives around providing good tunnels
- Support for WireGuard over IPv6 + Yggdrasil dual-stack

## Open Questions
- Should WireGuard tunnels be established peer-to-peer (nodes talk directly) or always mediated/coordinated by the Orchestrator?
- How do we handle key rotation and revocation?
- Should we support userspace WireGuard (`boringtun`) for environments without kernel module support?
- How tightly should WireGuard be coupled with the existing Yggdrasil/NovaNet stack?

## References
- WireGuard whitepaper and protocol
- Yggdrasil + WireGuard hybrid setups in the community
- Existing Rust crates: `wireguard-rs`, `boringtun`
