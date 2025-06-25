# DisasterMesh Project Brief for LLM Assistant

## Project Overview

**Project Name**: DisasterMesh  
**Primary Language**: Rust  
**Project Type**: Ad-hoc communication application for disaster/emergency scenarios  
**Key Constraint**: Must work without internet connectivity  

### Core Mission
Build a resilient, offline-first communication system that enables people to communicate during disasters, emergencies, or network outages using mesh networking, local wireless protocols, and radio communication.

## Technical Requirements

### Must-Have Features
- **Offline Operation**: Zero dependency on internet or cellular networks
- **Multi-Transport**: Support multiple communication methods (Bluetooth, WiFi Direct, LoRa, HAM radio)
- **Mesh Networking**: Self-healing network topology with automatic peer discovery
- **Cross-Platform**: Desktop (Linux, Windows, macOS) and mobile (Android, future iOS)
- **Security**: End-to-end encryption with forward secrecy
- **Store-and-Forward**: Messages can be relayed when recipients are offline
- **Low Power**: Battery-efficient for mobile/emergency use

### Technical Constraints
- **No Internet**: Cannot rely on external servers, DNS, or cloud services
- **Resource Limited**: Must work on low-power devices and older hardware
- **Regulatory**: Must comply with radio transmission regulations
- **Reliability**: System must be fault-tolerant and self-recovering

## Architecture Overview

### Core Components
1. **Message Manager**: Handles message creation, encryption, storage, and validation
2. **Routing Engine**: Implements mesh routing algorithms (AODV + epidemic routing)
3. **Transport Layer**: Abstracted interface for different communication methods
4. **User Interface**: Cross-platform UI using Tauri + web technologies
5. **Platform Layer**: OS-specific implementations for radio/network access

### Key Design Patterns
- **Transport Abstraction**: All communication methods implement a common `Transport` trait
- **Async-First**: Built on Tokio for efficient I/O handling
- **Event-Driven**: Components communicate via channels and events
- **Modular**: Each transport and feature can be enabled/disabled at compile time

## Key Technical Decisions

### Language Choice: Rust
- **Memory Safety**: Critical for security and reliability in emergency scenarios
- **Performance**: Efficient resource usage for battery-powered devices
- **Cross-Platform**: Excellent cross-compilation support
- **Ecosystem**: Strong networking, crypto, and embedded libraries

### UI Framework: Tauri
- **Cross-Platform**: Single codebase for desktop and mobile
- **Small Bundle**: Minimal resource footprint
- **Security**: Built-in security features and sandboxing
- **Flexibility**: Web technologies for rapid UI development

### Networking Approach: Mesh + Multiple Transports
- **Redundancy**: Multiple communication paths increase reliability
- **Range**: Different transports have different range/power characteristics
- **Adaptability**: System can adapt to available hardware and conditions

## Core Data Structures

```rust
// Primary message structure
#[derive(Serialize, Deserialize, Clone)]
pub struct Message {
    pub id: MessageId,           // Unique identifier
    pub sender: UserId,          // Sender's public key hash
    pub recipient: Option<UserId>, // None for broadcast
    pub content: MessageContent, // Actual message data
    pub timestamp: SystemTime,   // Creation time
    pub ttl: Duration,          // Time to live
    pub hop_count: u8,          // Routing hop counter
    pub signature: Vec<u8>,     // Ed25519 signature
}

// Transport abstraction
#[async_trait]
pub trait Transport: Send + Sync {
    async fn start(&mut self) -> Result<()>;
    async fn send(&self, peer: PeerId, data: Vec<u8>) -> Result<()>;
    async fn broadcast(&self, data: Vec<u8>) -> Result<()>;
    fn get_peers(&self) -> Vec<PeerInfo>;
}

// Routing information
#[derive(Clone)]
pub struct RouteInfo {
    pub destination: UserId,
    pub next_hop: PeerId,
    pub hop_count: u8,
    pub last_updated: SystemTime,
    pub link_quality: f32,
}
```

## Critical Libraries and Dependencies

### Core Dependencies
```toml
tokio = "1.37.0"           # Async runtime (pinned)
serde = "1.0.188"          # Serialization (pinned)
bincode = "1.3.3"          # Binary serialization (pinned)
ring = "0.16.20"           # Cryptography (pinned)
ed25519-dalek = "2.1.0"    # Digital signatures (pinned)
aes-gcm = "0.10.3"         # Symmetric encryption (pinned)
sled = "0.34.7"            # Embedded database (pinned)
tauri = "1.6.0"            # UI framework (pinned)
btleplug = "0.11.5"        # Bluetooth LE (pinned)
mdns = "3.2.0"             # Local service discovery (pinned)
anyhow = "1.0.75"          # Error handling (pinned)
tracing = "0.1.40"         # Logging (pinned)

# Dev tooling
cargo-edit = { version = "0.11.8", optional = true } # Adds `cargo add`, `cargo rm`, etc.
```

### Transport-Specific Libraries
- **Bluetooth**: `btleplug` for cross-platform BLE
- **WiFi Direct**: Platform-specific APIs (no universal Rust crate)
- **LoRa**: `radio-sx127x` or similar for hardware modules
- **HAM Radio**: Audio processing crates for digital modes

## Security Architecture

### Cryptographic Approach
- **Identity Keys**: Ed25519 keypairs for user identity
- **Message Encryption**: AES-256-GCM with ephemeral keys
- **Key Exchange**: X25519 ECDH for forward secrecy
- **Message Authentication**: Ed25519 signatures on all messages
- **Trust Model**: Web of trust with manual key verification

### Security Principles
- **Zero Trust**: Verify all messages and peers
- **Forward Secrecy**: Past messages remain secure if keys are compromised
- **Minimal Exposure**: Encrypt everything, authenticate everything
- **Key Rotation**: Support for updating keys without losing message history

## Routing Algorithm Details

### Primary: AODV (Ad-hoc On-Demand Distance Vector)
- **Route Discovery**: RREQ/RREP messages for finding paths
- **Route Maintenance**: RERR messages for broken links
- **Link Quality**: Factor in signal strength and reliability
- **Loop Prevention**: Sequence numbers prevent routing loops

### Fallback: Epidemic Routing
- **Sparse Networks**: When AODV can't find routes
- **Store-and-Forward**: Messages are carried until delivery opportunity
- **Controlled Flooding**: TTL and hop limits prevent infinite propagation
- **Buffer Management**: LRU eviction when storage is full

## Platform-Specific Considerations

### Linux
- **WiFi Direct**: Use `wpa_supplicant` P2P functionality
- **Bluetooth**: BlueZ D-Bus interface via `btleplug`
- **LoRa**: Direct SPI access to radio modules
- **Permissions**: May require elevated privileges for some operations

### Windows
- **WiFi Direct**: Windows.Devices.WiFiDirect APIs
- **Bluetooth**: Windows Runtime APIs
- **LoRa**: Serial/USB communication with modules
- **Packaging**: NSIS installer with driver considerations

### macOS
- **WiFi Direct**: Limited API access, may need workarounds
- **Bluetooth**: Core Bluetooth framework
- **LoRa**: Serial/USB access similar to other platforms
- **Sandboxing**: App Store requirements may limit functionality

### Android (via Tauri Mobile)
- **WiFi Direct**: Android WiFi Direct APIs
- **Bluetooth**: Android Bluetooth APIs
- **Permissions**: Complex permission model for radio access
- **Background**: Service restrictions for background operation

## Development Guidelines

### Code Style
- Follow standard Rust conventions (rustfmt, clippy)
- Use `anyhow` for application errors, `thiserror` for library errors
- Prefer composition over inheritance
- Use async/await throughout for I/O operations
- Extensive use of `Result<T, E>` for error handling

### Testing Strategy
- **Unit Tests**: Test individual components in isolation
- **Integration Tests**: Test component interactions
- **Property Tests**: Use `quickcheck` for protocol testing
- **Fuzzing**: Continuous randomized testing with `cargo-fuzz`
- **Simulation**: Mock transports for large-scale testing
- **SimNet CI**: Deterministic 5–10 node docker-compose mesh run in CI
- **Hardware Tests**: Real device testing for transport layers

### Error Handling Patterns
```rust
// Application errors
use anyhow::{Context, Result};

async fn send_message(msg: Message) -> Result<()> {
    transport.send(msg)
        .await
        .context("Failed to send message")?;
    Ok(())
}

// Library errors
use thiserror::Error;

#[derive(Error, Debug)]
pub enum TransportError {
    #[error("Connection failed: {0}")]
    ConnectionFailed(String),
    #[error("Invalid message format")]
    InvalidMessage,
}
```

### Async Patterns
```rust
// Prefer structured concurrency
use tokio::select;

async fn transport_loop() -> Result<()> {
    loop {
        select! {
            msg = incoming_rx.recv() => {
                handle_incoming_message(msg?).await?;
            }
            _ = heartbeat_interval.tick() => {
                send_heartbeat().await?;
            }
            _ = shutdown_rx.recv() => {
                break;
            }
        }
    }
    Ok(())
}
```

## Message Flow Architecture

### Outgoing Messages
1. User creates message in UI
2. Message Manager validates and encrypts
3. Routing Engine determines next hop(s)
4. Transport Layer sends to peer(s)
5. Store locally for potential retransmission

### Incoming Messages
1. Transport Layer receives raw data
2. Message Manager deserializes and validates
3. Check for duplicates and TTL
4. If for us: decrypt and display
5. If for others: route according to algorithm
6. Store for potential forwarding

### Store-and-Forward
1. Messages for offline users are queued
2. Periodic attempts to find new routes
3. When target comes online, deliver queued messages
4. Implement priority queuing for emergency messages

## Common Implementation Patterns

### Transport Implementation Template
```rust
pub struct BluetoothTransport {
    manager: Manager,
    peers: Arc<RwLock<HashMap<PeerId, Peripheral>>>,
    event_tx: broadcast::Sender<TransportEvent>,
}

#[async_trait]
impl Transport for BluetoothTransport {
    async fn start(&mut self) -> Result<()> {
        // Initialize hardware
        // Start scanning for peers
        // Set up connection handlers
    }
    
    async fn send(&self, peer: PeerId, data: Vec<u8>) -> Result<()> {
        // Find peer connection
        // Fragment message if needed
        // Send with retries
    }
}
```

### Event Handling Pattern
```rust
#[derive(Debug, Clone)]
pub enum AppEvent {
    MessageReceived(Message),
    PeerConnected(PeerInfo),
    PeerDisconnected(PeerId),
    RouteDiscovered(RouteInfo),
    TransportError(TransportError),
}

// Components communicate via channels
let (event_tx, event_rx) = broadcast::channel(1000);
```

## Performance Targets

### Scalability
- **Network Size**: 50-200 active nodes
- **Message Rate**: 100 messages/minute per node
- **Latency**: <5s local, <30s multi-hop
- **Storage**: 1GB message history

### Resource Usage
- **Memory**: <100MB baseline, <500MB with full message cache
- **CPU**: <5% average on mobile devices
- **Battery**: 24+ hours active use on smartphone
- **Bandwidth**: Efficient use of limited radio spectrum
- **LoRa Link Budget**: 24-byte packet ≤ 5 s per km

## Emergency-Specific Features

### Message Priorities
```rust
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub enum MessagePriority {
    Emergency = 0,    // Life threatening
    Urgent = 1,       // Time sensitive
    Normal = 2,       // Regular communication
    Background = 3,   // System messages
}
```

### Location Sharing
- GPS coordinates when available
- Manual location entry
- Location-based message routing
- Emergency beacon functionality

### Resource Sharing
- Share available resources (food, water, shelter)
- Medical assistance requests
- Skill/capability sharing
- Coordination for rescue efforts

## Testing Scenarios

### Network Scenarios
- **Dense Network**: Many nodes in small area
- **Sparse Network**: Few nodes, large distances
- **Mobile Network**: Nodes moving constantly
- **Partitioned Network**: Disconnected groups that merge/split

### Failure Scenarios
- **Node Failures**: Random device shutdowns
- **Transport Failures**: Bluetooth/WiFi hardware issues
- **Message Loss**: Interference and signal degradation
- **Attack Scenarios**: Malicious nodes, message flooding

### Emergency Scenarios
- **Power Grid Failure**: Devices running on battery
- **Natural Disaster**: Physical destruction and movement
- **Communication Blackout**: All traditional networks down
- **Mass Evacuation**: Large-scale population movement

## Security Considerations for LLM Assistance

### What to Be Careful About
- Never hardcode cryptographic keys or secrets
- Avoid creating backdoors or weak crypto implementations
- Be cautious with serialization of untrusted data
- Validate all inputs from network sources
- Consider timing attacks in crypto code

### Security Review Points
- All cryptographic implementations should use established libraries
- Key generation must use secure random number generators
- Message validation must be performed before processing
- Rate limiting should be implemented to prevent DoS attacks
- Audit trails should be maintained for security events

## Development Phases for LLM Context

When working on this project, understand that development follows this progression:

1. **Foundation Phase**: Basic message structures, local storage, simple UI
2. **Transport Phase**: Implement individual transport methods
3. **Routing Phase**: Add mesh networking and routing algorithms  
4. **Security Phase**: Harden cryptography and add security features
5. **Polish Phase**: Optimization, testing, and user experience improvements

Always consider the offline-first constraint and the emergency use case when making implementation decisions. Prioritize reliability and simplicity over advanced features.

---

## Project Roadmap & Milestones

| Phase | Calendar Weeks | Definition of Done | Key KPIs |
|-------|---------------|--------------------|----------|
| Foundation | 0–4 | Message structs, local storage, minimal Tauri UI | RTT single-node < 50 ms; memory < 40 MB |
| Transport | 5–10 | BLE + Wi-Fi Direct transports, peer discovery, 3-node demo | 3-hop BLE latency < 1.5 s; battery draw < 3 %/h; memory < 60 MB |
| Routing | 11–16 | AODV route discovery, store-and-forward queue, simulation tests | 95 % route success (50 nodes); 5-hop latency < 5 s |
| Security | 17–19 | Noise_XX handshake, AES-GCM encryption, signature verification | Handshake < 500 ms; 100 % replay-attack prevention |
| Polish | 20–24 | UX polish, performance optimisation, regulatory documentation | 24 h battery life; memory < 100 MB; 200-node sim passes |

> Review milestones at end of each phase and update estimates as necessary.

## Wire Format & Versioning

The on-air envelope is a length-prefixed protobuf message (`proto3`).

```text
uint32 version   // defaults to 1, increment on breaking change
bytes  payload   // serialized Message
bytes  signature // Ed25519 over (version || payload)
```

Compatibility strategy:

1. Minor compatible additions use higher field numbers, receivers ignore unknown fields.  
2. Breaking changes bump `version`; nodes advertise supported range during peer handshake.  
3. Reserve field numbers 1000-1999 for experimental extensions.

## Transport Abstraction Enhancements

- Provide `fn mtu(&self) -> usize` and `fn link_quality(&self) -> f32` in the `Transport` trait.  
- Add optional `libp2p` backend (feature flag `libp2p`).  
- Each transport publishes sleep/duty-cycle hints so the router can schedule bursts.

## Security Additions

- Handshake: `Noise_XX_25519_AESGCM_SHA256` (libraries: `snow`, `ring`).  
- Key rotation: publish `KeyUpdate` control message; peers replace public key after signature verification.  
- Key revocation: CRL gossip; expired keys rejected at validation step.  
- DoS guard: hard-limit 20 user messages/s per peer, exponential backoff thereafter.  
- Mobile platforms should store private key in OS secure storage (Android Keystore, Secure Enclave, TPM).

## Data Storage & Sync

- Sled database runs in a single column-family `messages`.  
- Outgoing queue persisted with priority index (`MessagePriority` + timestamp).  
- Provide encrypted export/import (`.dmbackup`) for data recovery and device migration.  
- Old messages (> TTL) are GC-eligible unless user has "pinned" them in UI.

## Testing & CI/CD

- Minimum toolchain: Rust 1.76, edition 2021 (`rust-toolchain.toml`).  
- CI: GitHub Actions matrix (ubuntu-latest, windows-latest, macos-latest).  
- Linters: `cargo clippy --all-targets --all-features -D warnings`, `cargo fmt -- --check`.  
- Security: `cargo audit`, `cargo deny`.  
- Fuzzing jobs nightly via `cargo-fuzz` target `deserialize_envelope`.  
- `simnet` docker-compose job: 10 container nodes with TC latency 100–500 ms for regression tests.

## Platform Gaps & Risks

- **iOS** is currently *out of scope* until Tauri Mobile gains support.  
- **macOS** lacks official Wi-Fi Direct APIs; fallback via Ethernet or designate gateway node.  
- Maintain region-by-region table of permitted LoRa frequencies & duty-cycle.

## UX & Accessibility

- Initial flows: Send text, SOS beacon, contact list, settings import/export keys.  
- First-run wizard creates/backs-up identity keys entirely offline (QR + mnemonic).  
- Provide language packs: English, Spanish (baseline), others via community.  
- WCAG 2.1 AA contrast & keyboard navigation compliance.

## Documentation & Contributor Experience

- `CONTRIBUTING.md`: setup, cross-compilation (`cargo ndk`, `zig build-std`), code-style.  
- Maintain `docs/adr` folder—new ADR per major decision (transport abstraction, routing algorithm, wire format, etc.).  
- Auto-generated Mermaid sequence diagrams (`cargo mermaid`) for message flow & transport handshake.