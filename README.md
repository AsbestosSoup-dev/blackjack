# 🃏 Blackjack
[![Rust](https://img.shields.io/badge/Rust-2024-orange?logo=rust)](https://www.rust-lang.org)
[![License: MIT](https://img.shields.io/badge/License-MIT-blue.svg)](LICENSE)
[![CI](https://github.com/AsbestosSoup-dev/blackjack/actions/workflows/ci.yml/badge.svg?branch=main)](https://github.com/AsbestosSoup-dev/blackjack/actions/workflows/ci.yml)
[![OS Support](https://img.shields.io/badge/platform-Windows%20%7C%20macOS%20%7C%20Linux-success)](#)

A cross-platform, terminal-based multiplayer blackjack game written in **Rust**.  
Play solo, with bots, or over LAN/WAN — all through a clean, responsive text UI.

---

## ✨ Features

### 🎮 Gameplay
- **1–8 player table** (default 1–6)
- **Standard blackjack rules**: 6-deck shoe, dealer stands on soft 17
- **Actions**: Hit, Stand, Double (Splits, Insurance, Side Bets coming soon)
- **Deterministic RNG**: Seeded rounds for fair play
- **Bot players**: Fill empty seats with configurable AI (basic strategy to card counting)

### 💻 Terminal Interface
- **Clean, adaptive TUI** via Ratatui + Crossterm
- **Dealer + player grid layout** with card visualization
- **Action panel**, message log, and audit view
- **Themes**: Default / Dark / High-Contrast
- **Spectator mode** for observers
- **Chat system**: Players and spectators communicate in-game

### 🌐 Networking
- **Always-on server architecture**: Even singleplayer runs a local server
- **Seamless multiplayer**: "Open to LAN/WAN" from any game in progress
- **Auto-discovery**: UDP broadcast finds local games automatically
- **Manual connection**: Direct IP:Port entry for WAN or discovery failures
- **Late joins & reconnections**: Players can join mid-session or recover from disconnects
- **TLS encryption**: Built-in support for secure WAN play

### 🧑‍💼 Admin / Debug Mode
- **Host-only console** accessible in-game
- **Transparent actions**: All admin commands logged and broadcast to players
- **Abilities**:
    - View or shuffle deck
    - Adjust player credits
    - Inject specific cards (testing)
    - Skip phases
    - Simulate network conditions (latency, packet loss)
- **Audit trail**: Every action marked `[ADMIN]` for transparency

### 💾 Persistence
- **Session save/restore**: Host can save game state and resume later
- **Player credits**: Optional persistent balances across sessions
- **Configuration**: Auto-save/load from `~/.config/blackjack/config.toml`
- **Per-player stats**: Track hands won, total wagered (opt-in)

### ⚙️ Cross-Platform
- **Runs on Windows, macOS, and Linux**
- **Portable single binary**
- **Clean terminal restoration** on exit

---

## 🚀 Quick Start

### Prerequisites
- **Rust 1.85+** (Edition 2024)
- Terminal with Unicode support

### Installation

```bash
# Clone the repository
git clone https://github.com/AsbestosSoup-dev/blackjack.git
cd blackjack

# Build release binary
cargo build --release

# Run the game
./target/release/blackjack
```

### First Launch

1. **Splash screen**: Press `SPACE` to continue
2. **Main menu**:
    - **Host Game**: Start a new session (singleplayer or multiplayer)
    - **Join Game**: Connect to an existing game
    - **Settings**: Configure rules, network, display, credits

---

## 🎯 How to Play

### Singleplayer
1. Select **Host Game** from main menu
2. Configure game rules (or use defaults)
3. Game starts with local server on `127.0.0.1` (private)
4. Add bots via settings to fill table

### Multiplayer (LAN)
**Host:**
1. Start game as singleplayer
2. Press `ESC` → **Open to LAN**
3. Server rebinds to `0.0.0.0:7777` and broadcasts discovery
4. Share game name with friends

**Join:**
1. Select **Join Game** from main menu
2. Wait 3 seconds for auto-discovery
3. Select game from list OR enter IP manually
4. Connect and play

### Multiplayer (WAN)
**Host:**
1. Open to LAN, then configure port forwarding (default port `7777`)
2. Share public IP:Port with friends
3. Optionally set password in Settings → Network

**Join:**
1. Select **Join Game** → **Manual IP**
2. Enter `host_ip:7777` and optional password
3. Connect via TLS-encrypted session

---

## 🏗️ Architecture

### Technology Stack

| Component | Technology | Notes |
|-----------|-----------|-------|
| **Core Logic** | `bevy_ecs 0.17` | Phase-based systems, entity component architecture |
| **Networking** | `tokio 1.x`, `bincode 2.0` | Async TCP, minimal runtime features |
| **UI** | `ratatui 0.29`, `crossterm` | Cross-platform terminal rendering |
| **Logging** | `tracing 0.1` | Async-native structured logging with spans |
| **Serialization** | `serde`, `bincode`, `toml` | Binary protocol + human-readable config |
| **Security** | `rustls 0.23`, `argon2 0.5`, `hmac` | TLS 1.3, password hashing, session tokens |

### Network Architecture

```mermaid
flowchart LR
    subgraph Client["Client (TUI)"]
        UI["Ratatui UI\n(render TableSnapshot)"]
        Input["Keyboard Input\n(send Actions)"]
        NetClient["TCP Client"]
    end

    subgraph Server["Server (Always Running)"]
        Listener["TCP Listener\n(0.0.0.0:7777 or 127.0.0.1)"]
        GameLoop["ECS Game Loop\n(bevy systems)"]
        Broadcaster["Snapshot Broadcaster"]
        Discovery["UDP Discovery\n(multicast)"]
    end

    Input -->|Action| NetClient
    NetClient -->|TCP| Listener
    Listener --> GameLoop
    GameLoop --> Broadcaster
    Broadcaster -->|TableSnapshot| NetClient
    NetClient --> UI
    Discovery -.->|Beacon| NetClient
```

**Key Design Decisions:**
- **Server-authoritative**: All game logic runs on host, clients are presentation layer
- **Snapshot-based sync**: Full state broadcast each tick (simple, resilient)
- **Always-server model**: Even singleplayer runs local server for uniform architecture
- **UDP discovery**: LAN games auto-detected via multicast beacons
- **TLS by default**: WAN-ready from day one

### ECS Tick Cycle

```
1. Receive Network Events (client actions queued)
2. Run Phase Systems (Lobby → Betting → Turns → Dealer → Payout)
3. Update Resources (RNG, timers, round counter)
4. Build TableSnapshot DTO
5. Broadcast to all connected clients
6. Sleep until next tick (16ms target)
```

### Round Lifecycle

```mermaid
sequenceDiagram
    participant Host
    participant Player
    participant Server

    Player->>Host: UDP Discovery Request
    Host-->>Player: Game Info Beacon
    Player->>Server: TCP Connect + Auth
    Server-->>Player: TableSnapshot (Lobby)
    
    Note over Server: Phase: Betting
    Player->>Server: Action::Bet(amount)
    Server-->>Player: Updated Snapshot
    
    Note over Server: Phase: Dealing
    Server-->>Player: Cards dealt
    
    Note over Server: Phase: Player Turns
    loop Player Actions
        Player->>Server: Action::Hit/Stand/Double
        Server-->>Player: Updated Snapshot
    end
    
    Note over Server: Phase: Dealer Turn
    Server-->>Player: Dealer plays (automated)
    
    Note over Server: Phase: Payout
    Server-->>Player: Winnings + Round Summary
    
    Note over Server: Next round begins
```

---

## 📦 Dependencies

### Core (Edition 2024)
- **bevy_ecs 0.17**: Entity component system for game state
- **tokio 1.x**: Async runtime (minimal features: `rt-multi-thread`, `net`, `sync`, `time`, `macros`)
- **tracing 0.1 + tracing-subscriber 0.3**: Structured logging for async debugging

### Serialization
- **bincode 2.0**: Binary network protocol (explicit configuration)
- **serde 1.x**: Serialization framework
- **toml 0.9**: Human-readable configuration files
- **uuid 1.x**: Player IDs (v7 time-ordered UUIDs)

### UI
- **ratatui 0.29**: Terminal UI framework (pre-1.0 but stable)
- **crossterm 0.29**: Cross-platform terminal manipulation (re-exported by ratatui)

### Security
- **rustls 0.23**: Modern TLS 1.3 library
- **tokio-rustls 0.26**: Async TLS integration
- **argon2 0.5**: Password hashing (Argon2id variant)
- **hmac 0.12 + sha3 0.10**: Session token authentication

### Utilities
- **rand 0.9**: RNG for card shuffling
- **dirs 6**: Platform-specific config directories
- **thiserror 2**: Structured error types
- **anyhow 1**: Error propagation and context

---

## ⚙️ Configuration

### Game Rules (`~/.config/blackjack/config.toml`)

```toml
[game]
min_bet = 10
max_bet = 500
starting_credits = 1000
num_decks = 6
dealer_hits_soft_17 = false

[side_bets]
insurance_enabled = false
perfect_pairs_enabled = false
twentyone_plus_three_enabled = false

[network]
mode = "Solo"  # Solo | LAN | WAN
port = 7777
password = ""  # Optional for WAN

[display]
theme = "default"  # default | dark | high-contrast

[persistence]
save_credits = false
save_stats = false
```

---

## 🛡️ Security

### LAN Play
- **No encryption**: Plaintext TCP within trusted local network
- **No authentication**: All local clients trusted

### WAN Play
- **TLS 1.3 encryption**: Mandatory via rustls
- **Optional password**: Argon2id hashing (19 MiB memory, 2 iterations)
- **Session tokens**: HMAC-SHA256 for stateless authentication
- **Rate limiting**: Prevents action spam and DoS

### Admin Transparency
- All admin actions logged and broadcast to players
- Audit trail prevents hidden manipulation
- Players can see exact admin commands executed

**Note**: For internet play, recommend SSH tunnel or VPN for additional security layer.

---

## 🧠 Roadmap

| Version | Features |
|---------|----------|
| **v0.1 (Current)** | Core gameplay, singleplayer, basic multiplayer |
| **v0.2** | Splits, insurance, perfect pairs side bet |
| **v0.3** | Advanced bots (card counting), replay system |
| **v0.4** | Session statistics, leaderboards |
| **v1.0** | Full feature set, production polish |

---

## 👨‍💻 Development

### Project Structure

```
src/
├── lib.rs              # Library entry for testing
├── main.rs             # Binary entry point
├── app.rs              # Application orchestration
├── config.rs           # Configuration management
├── error.rs            # Centralized error types
├── logging.rs          # Tracing initialization
├── core/               # Game logic
│   ├── card.rs
│   ├── hand.rs
│   ├── shoe.rs
│   ├── rules.rs
│   └── payout.rs
├── engine/             # ECS systems
│   ├── game.rs
│   ├── systems.rs
│   ├── bot.rs
│   └── admin.rs
├── net/                # Networking
│   ├── protocol.rs
│   ├── server.rs
│   ├── client.rs
│   ├── session.rs
│   └── discovery.rs
├── tui/                # Terminal UI
│   ├── render.rs
│   ├── input.rs
│   ├── layout.rs
│   └── theme.rs
├── types/              # Shared domain types
│   ├── player.rs
│   ├── phase.rs
│   └── action.rs
└── persist/            # Save/load
    ├── session.rs
    └── stats.rs
```

### Building

```bash
# Development build (optimized for iteration)
cargo build

# Release build (optimized for performance)
cargo build --release

# Run tests
cargo test

# Run with logging
RUST_LOG=blackjack=debug cargo run
```

### Testing

```bash
# Unit tests (inline in source files)
cargo test --lib

# Integration tests
cargo test --test '*'

# Specific module
cargo test core::hand
```

---

## 🐛 Troubleshooting

**Port already in use**
- Change port in Settings → Network
- Default fallback: OS assigns random available port

**Can't discover LAN games**
- Check firewall allows UDP multicast
- Verify same subnet (192.168.x.x or 10.x.x.x)
- Use Manual IP as fallback

**Connection drops during game**
- Server attempts 3 reconnections automatically
- Host can resume session if crash via persistence
- Check network stability (ping host)

**Performance issues**
- Reduce terminal size (smaller render area)
- Disable animations in settings
- Check CPU usage (game should be <5%)

---

## 🧾 License

MIT License - see [LICENSE](LICENSE) file

---

## ✍️ Credits

**Game Design & Development**: AsbestosSoup  
**Architecture & Documentation**: Collaborative design with Claude (Anthropic)

**Special Thanks**:
- Rust community for excellent crates ecosystem
- Bevy project for accessible ECS architecture
- Ratatui maintainers for terminal UI framework

---

## 📞 Contact

- **GitHub Issues**: Bug reports and feature requests
- **Discussions**: Design feedback and gameplay suggestions

**Note**: This is a learning project focused on Rust best practices, multiplayer networking, and production-quality architecture. Contributions welcome!
