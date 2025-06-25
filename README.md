# DisasterMesh

DisasterMesh is an **offline-first, resilient mesh-network communication platform** written in Rust.  It enables people to stay connected during disasters, network outages, or censorship events by forming ad-hoc wireless meshes (Bluetooth, Wi-Fi Direct, LoRa, etc.) with end-to-end encryption and store-and-forward routing.

---

## Features (MVP)

* 🗄  **Offline operation** – no internet or cellular required
* 📡  **Multi-transport** – Bluetooth LE & Wi-Fi Direct (LoRa/HAM planned)
* 🔀  **Mesh routing** – AODV with epidemic fallback
* 🔒  **Secure** – Ed25519 identity keys, AES-GCM encryption (placeholder in MVP)
* 🔁  **Store-and-forward** – deliver messages when paths appear
* 🖥️  **Cross-platform** – Linux, Windows, macOS (mobile planned)

---

## Repository layout

```
├── src/                # Core library crate
│   ├── message.rs      # Message structures
│   ├── message_manager.rs
│   ├── transport.rs
│   └── types.rs
├── tests/              # Example unit tests
├── Implementation Plan.md
├── Testing Plan.md
└── Cargo.toml
```

---

## Quick start

```bash
# Prerequisites: Rust 1.76+ (stable), cargo

git clone https://github.com/0xJosep/DisasterMesh.git
cd DisasterMesh

# Run the test-suite (should be green)
cargo test
```

---

## Roadmap

See **Implementation Plan.md** for the staged roadmap (Foundation → Transport → Routing → Security → Polish).

---

## Contributing

All welcome!  Please read **CONTRIBUTING.md** for guidelines and dev-container setup.

---

## License

Dual-licensed under **MIT** or **Apache-2.0** – choose at your option. 