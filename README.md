# Discord-PoiseSerenity (`theta_bot`)

> A high-performance, modular Discord bot built with Rust, Poise, and serenity. >:3

[![Rust](https://img.shields.io/badge/Rust-2024_Edition-orange?logo=rust)](https://www.rust-lang.org/)
[![Framework](https://img.shields.io/badge/Framework-Poise-blue)](https://github.com/serenity-rs/poise)
[![Status](https://img.shields.io/badge/Status-In_Active_Development-yellow)](#)

---

## Overview

**Discord-PoiseSerenity** or known as *ThetaBot* is a modern, cloud-native Discord bot designed with clean, modular architecture and type-safe command handling. Built to demonstrate high-concurrency bot patterns in Rust.

*Note: This project is currently a **Work in Progress (WIP)**.* :p

---

## Architecture & Tech Stack

- **Core Runtime & Framework**: Written in Rust (2024 Edition) using [`poise`](https://github.com/serenity-rs/poise) and [`serenity`](https://github.com/serenity-rs/serenity).
- **Asynchronous Engine**: Powered by `tokio` for non-blocking I/O operations.
- **Voice & Media**: Integrated with `songbird` for audio queuing and voice handling.
- **Data Persistence (Planned)**:
  - **PostgreSQL / Supabase**: For primary persistent user data, server configurations, and analytics.
  - **SQLite (`better-sqlite3` / `sqlx`)**: Local lightweight state caching and zero-latency session storage.
- **Development Environment**: Declarative and reproducible build environment via **NixOS**.

---

# Project Structure
```
src/
├── commands.rs         # Centralized command registration aggregator
├── commands/           # Modular command hierarchy
│   ├── debug/          # Developer & diagnostic utilities (e.g., /ping)
│   ├── user/           # User-facing commands
│   └── admin/          # Moderation tools
├── main.rs             # Application entrypoint & framework initialization
├── logger.rs           # Custom logging system
╰── error.rs            # Universal Error handler
```

And more modules will be added more soon! >:3c

---

## Features Roadmap
- **[x] Modular Command Systen**: Scalable directory structure for seamless command expansion
- **[x] Latency & Health diagnostic**: `/ping` and runtime metrics, could be added more
- **[ ] Dual-Database Integration**: Hybrid database setup (PostgreSQL + SQLite) for optimal query performance
- **[ ] Seyfert Parity**: Porting core moderation and utility features from the previous [Discord-SeyfertFramework](https://github.com/Aarchfel/Discord-SeyfertFramework/tree/development) Implementation
- **[ ] Voice Queueing System**: Audio player functionality powered by `songbird`
- **[ ] AI Moderation System**: Chat moderation powered with AI collecting messages each 40 seconds (Batching mechanism)
- **[ ] Ticket System**: Customizeable ticket system that creates dedicated channels for members
- **[ ] Verification System**: Guild member verification system with customizable options for administrators/moderators

---

## Getting Started (Local Development)

```
# Clone the repository
git clone [https://github.com/Aarchfel/theta_bot.git](https://github.com/Aarchfel/theta_bot.git)
cd theta_bot

# Run in development mode
cargo run
```

---

## License
Distributed under the MIT License. See [`LICENSE`](./LICENSE) for more information >:p

