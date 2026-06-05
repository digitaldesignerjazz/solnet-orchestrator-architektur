# WireGuard Phase A – Implementierungsplan & Code-Struktur

**Status:** Draft  
**Ziel:** Erste funktionsfähige WireGuard-Unterstützung im Solnet Orchestrator
**Scope:** Key-Management + einfache Tunnel-Konfiguration + grundlegende API

## 1. Ziele von Phase A

- Nodes können WireGuard-Schlüsselpaare generieren lassen
- Orchestrator kann WireGuard-Konfigurationen für Tunnel zwischen Nodes erzeugen
- Einfache REST + WebSocket API zum Anfordern und Abrufen von Tunnel-Konfigurationen
- Grundlegende Interface-Verwaltung auf Node-Seite (über System-Commands)
- Erste Integration in bestehende Architektur (NodeManager, EventBus, API)

## 2. Empfohlener Technischer Ansatz

| Komponente              | Technologie                          | Begründung                              |
|-------------------------|--------------------------------------|-----------------------------------------|
| Key Generation          | `x25519-dalek` + `rand`              | Rein Rust, sicher, leichtgewichtig     |
| Interface Management    | `std::process::Command` + `wg` / `ip` | Schnell, stabil, weit verbreitet       |
| Config Generation       | Rust String-Templates                | Volle Kontrolle                        |
| API                     | Bestehendes Axum + WebSocket         | Konsistent mit aktueller Architektur   |

## 3. Modul-Struktur (Vorschlag)

```
src/
├── wireguard/
│   ├── mod.rs                 # Public API des Moduls
│   ├── keys.rs                # Key-Generierung & Encoding
│   ├── config.rs              # WireGuard Config Generator
│   ├── manager.rs             # Interface-Management (wg/ip Commands)
│   ├── types.rs               # Data Models (WireGuardPeer, TunnelRequest, etc.)
├── core/
│   ├── node_manager.rs        # Erweiterung um WireGuard-Felder
├── api/
│   ├── server.rs              # Neue Endpoints + WebSocket Commands
```

## 4. Wichtige Data Models

```rust
pub struct WireGuardKeyPair {
    pub private_key: String,   // Base64
    pub public_key: String,    // Base64
}

pub struct WireGuardPeerConfig {
    pub public_key: String,
    pub allowed_ips: Vec<String>,
    pub endpoint: Option<String>,
    pub persistent_keepalive: Option<u16>,
}

pub struct WireGuardInterfaceConfig {
    pub private_key: String,
    pub address: String,           // z.B. "10.200.0.2/24"
    pub listen_port: Option<u16>,
    pub peers: Vec<WireGuardPeerConfig>,
}

pub struct TunnelRequest {
    pub source_node_id: String,
    pub target_node_id: String,
    pub allowed_ips: Option<Vec<String>>,
}
```

## 5. API-Endpunkte (Phase A)

### REST

| Methode | Endpoint                        | Beschreibung                                      |
|---------|----------------------------------|---------------------------------------------------|
| POST    | `/wireguard/keys/generate`      | Neues Schlüsselpaar generieren                    |
| POST    | `/wireguard/tunnel/request`     | Tunnel-Konfiguration zwischen zwei Nodes anfordern |
| GET     | `/wireguard/config/{node_id}`   | Aktuelle WireGuard-Konfig für einen Node abrufen  |

### WebSocket Commands (über `/ws/events`)

```json
{ "action": "create_wireguard_tunnel", "target_node_id": "..." }
```

## 6. Implementierungsreihenfolge (empfohlen)

1. **Grundlegende Modelle & Key-Generierung** (`wireguard/keys.rs` + `types.rs`)
2. **Config Generator** (`wireguard/config.rs`)
3. **Einfacher Manager** für Interface-Commands (`wireguard/manager.rs`)
4. **Erweiterung von NodeManager** (WireGuard-Felder speichern)
5. **Neue REST-Endpunkte** im API-Layer
6. **WebSocket Command Handler** erweitern
7. **Integration in main.rs** + Heartbeat-Loop (optional)
8. **Dokumentation & Tests**

## 7. Offene Fragen / Entscheidungen

- Soll der Orchestrator die privaten Schlüssel der Nodes speichern? (Sicherheitsrisiko!)
  → **Empfehlung:** Nur Public Keys + generierte Configs. Private Keys bleiben auf dem Node.
- Wie werden erlaubte IPs (`AllowedIPs`) bestimmt?
- Soll es eine dedizierte WireGuard-HAL geben oder über bestehende HAL laufen?
- Naming: `WireGuardManager` vs. `TunnelManager`?

## 8. Nächste Schritte nach Phase A

- Intelligente Tunnel-Auswahl durch den Orchestrator (Phase B)
- Health-Monitoring von WireGuard-Interfaces
- Userspace-Fallback mit `boringtun`
- Bessere Integration ins Dashboard (Tunnel-Status anzeigen)
