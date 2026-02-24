# AXON

Local-First Autonomous Development Infrastructure  
Written in Rust

Event-Driven · Self-Hosted · Transparent · AGPLv3

---

## Overview

AXON is a modular, event-driven development runtime designed to operate primarily on your local machine.

It treats your development environment as coordinated infrastructure rather than isolated tools.

AXON integrates:

- Local LLM runtimes (via Ollama)
- Retrieval-Augmented Generation (RAG)
- Multi-worker orchestration
- Persistent session memory
- Controlled shell execution
- Optional Telegram control plane
- WebSocket dashboard bridge

AXON is infrastructure.  
It is not a SaaS product.

---

## System Model

AXON operates at the system layer of development.

It does not replace your editor.
It does not inject suggestions into your cursor.

Instead, it observes, coordinates, and maintains your development runtime.

Your workspace becomes an active system rather than a passive directory of files.

---

## Architecture Principles

AXON is built around:

- Event-driven core
- Explicit worker registry
- Async orchestration layer
- Persistent runtime state
- Deterministic execution flow
- Transparent command handling

Design goals:

- Local-first by default
- No hidden telemetry
- No background data collection
- No forced cloud dependency
- Explicit over implicit behavior

Fully written in Rust.

---

## Independence

AXON is an independent project.

It is not backed by a startup.  
It is not funded by investors.  
It is not affiliated with any corporation.  

It is built and maintained by a single developer.

The objective is architectural exploration and long-term technical autonomy.

---

## License

AXON is licensed under the **GNU Affero General Public License v3 (AGPLv3)**.

This means:

- You may use, modify, and redistribute the software.
- If you modify AXON and deploy it as a network-accessible service, you must provide the modified source code under AGPLv3.

See the `LICENSE` file for full legal terms.

---

## Contributor Agreement

AXON uses a Contributor License Agreement (CLA).

By submitting a Pull Request, you agree to the terms defined in `CLA.md`.

In summary:

- Contributions must be original.
- Contributions are accepted under AGPLv3.
- The maintainer retains the right to relicense contributions in future versions (including commercial or dual-licensed distributions).

If you do not agree with these terms, do not submit contributions.

---

## Security Notice

AXON may execute shell commands on your local machine depending on configuration.

You are solely responsible for reviewing and approving execution behavior.

AXON:

- Runs locally
- Has no telemetry
- Has no auto-update mechanism
- Does not escalate privileges implicitly
- Does not provide OS-level sandboxing

Review configuration carefully before enabling automation.

---

## Legal Disclaimer

This software is provided **“AS IS”**, without warranty of any kind, express or implied.

Use responsibly.

---

## Quick Start

### Requirements

- Rust (stable, 2021 edition)
- Ollama installed locally
- Supported OS: Linux / macOS (Windows experimental)

### Build

    cargo build --release

### Run

    cargo run

Configuration can be adjusted via the configuration schema under `src/config/`.

---

## Module Structure

AXON is organized into modular runtime domains:

- `core/`        → runtime lifecycle & system state
- `event/`       → event bus & dispatch system
- `worker/`      → worker abstraction layer
- `workers/`     → concrete worker implementations
- `orchestrator/`→ routing & coordination logic
- `ai/`          → LLM routing & provider integrations
- `rag/`         → vector memory & search engine
- `memory/`      → persistent and vector state layers
- `shell/`       → controlled command execution
- `session/`     → identity & persistence
- `telegram/`    → optional control interface
- `util/`        → logging & runtime utilities

This modular structure is designed for extensibility and isolation of concerns.

---

## Project Status

AXON is an evolving infrastructure project.

Core runtime components are implemented, but the system remains under active development.

Breaking changes may occur during early releases.

---

## Roadmap

A structured roadmap is maintained in `ROADMAP.md`.

Current focus areas:

- Runtime hardening
- Worker isolation refinement
- Backpressure management
- Security review
- Extensibility layer

---

## Governance

AXON is maintained by a single developer.

Architectural decisions prioritize:

- Runtime determinism
- Explicit system behavior
- Local-first guarantees
- Long-term maintainability

Future structural changes will be documented transparently.

---

## Support

AXON does not provide commercial support.

Community discussion and contributions are welcome via GitHub.

Security issues must follow the reporting guidelines defined in `SECURITY.md`.