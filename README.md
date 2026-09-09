# Discord-PoiseSerenity (`theta_bot`)

> An all in one high-performance, modular Discord bot built with Rust, Poise, and Serenity built for moderation and utilities/funsies >:3

[![Rust](https://img.shields.io/badge/Rust-2024_Edition-orange?logo=rust)](https://www.rust-lang.org/)
[![Framework](https://img.shields.io/badge/Framework-Poise-blue)](https://github.com/serenity-rs/poise)
[![Status](https://img.shields.io/badge/Status-In_Active_Development-yellow)](#)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](LICENSE)

---

## Overview

**Discord-PoiseSerenity** or known as *ThetaBot* is a modern, cloud-native Discord bot designed with clean, modular architecture and type-safe command handling. Built to demonstrate high-concurrency bot patterns in Rust.

*Very Big Note: This project is currently a `Work in Progress (WIP)`* :3c

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

## Project Structure
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
- **[x] Modular Command System**: Scalable directory structure for seamless command expansion
- **[x] Latency & Health diagnostic**: `/ping` endpoint and runtime metrics with room for expansion
- **[ ] Seyfert Parity**: Porting core moderation and utility features from the previous [Discord-SeyfertFramework](https://github.com/Aarchfel/Discord-SeyfertFramework/tree/development) Implementation
- **[ ] Dual-Database Integration**: Hybrid database setup (PostgreSQL + SQLite) for optimal query performance
- **[ ] Verification System**: Guild member verification system with customizable options for administrators/moderators
- **[ ] AI Moderation System**: Chat moderation powered with AI collecting messages each 40 seconds (Batching mechanism)
- **[ ] Ticket System**: Customizeable ticket system that creates dedicated channels for members
- **[ ] Voice Queueing System**: Audio player functionality powered by `songbird`
- **[ ] Multi-Language Translation**: AI-powered translation support across multiple languages
- *And more to come...*

---

## Getting Started

### Prerequisites

* **Rust** (2024 Edition) installed via [`rustup`](https://rustup.rs/)
* A **Discord Bot Token** from the [Discord Developer Portal](https://discord.com/developers/applications)

### Environment Variables

Before running the bot, create a `.env` file in root directory:

```
cp .env.example .env
```

### Configure the following variables in your .env file:

| Variable | Required | Default | Description |
| :--- | :---: | :---: | :--- |
| `DISCORD_TOKEN` | Yes | - | Bot Token from Discord Developer Portal |


### Installation & Running

1. Clone the repository:

```
git clone https://github.com/Aarchfel/Discord-PoiseSerenity.git
cd Discord-PoiseSerenity
```

2. Build and run in development mode:
```
cargo run
```

3. Build for production:
```
cargo build --release
./target/release/theta_bot
```
---

## Special Thanks
- Thank you so much to myself for not giving up
- Thank you so much to my dear partner for all the love
- Thanks to my PC for not blowing up every time
- Thanks to my Father & Mother for my living

---

## License
Distributed under the MIT License. See [`LICENSE`](./LICENSE) for more information >:p

<br/>

<h1 align="center">
    Discord-PoiseSerenity
</h1>

<p align="center">
  <i>Programming is elegant. Only God knows how this code works.</i>
</p>

<p align="right">
  <sub><i>Officially started this project on September 8, 2026</i></sub>
</p>
