# Cyber Intel Aggregator Service

[![Deployment Status](https://img.shields.io/badge/Status-Deploying-yellow.svg)](https://intel.brainfeed.tech)
[![Stack: Rust](https://img.shields.io/badge/Stack-Rust-orange.svg)](https://www.rust-lang.org/)
[![Architecture: Clean](https://img.shields.io/badge/Architecture-Clean/Modular-blue.svg)]()

Advanced threat intelligence platform for Dark Web and Clear Web monitoring.

## 🏗 Modular Architecture
Built with a tiered, decoupled architecture:
- **Domain**: Pure business logic (Scraping rules, NLP entities, Threat models).
- **Services**: Use-case orchestration (Intel aggregation, Alerting workflows).
- **Adapters**: Infrastructure implementations (PostgreSQL, Redis, Scraper Drivers, REST/gRPC API).

## 🚀 Quick Start (Docker)
```bash
# Clone the repository
git clone https://github.com/Brainfeed-1996/cyber-intel-aggregator-service.git

# Start the full stack (Rust Backend, Next.js Frontend, PG, Redis, Prometheus)
docker-compose up -d
```

## 📈 Tech Stack & Observability
- **Backend**: Rust (Axum / Tokio)
- **Frontend**: Next.js (Tailwind / Shadcn UI)
- **Storage**: PostgreSQL (TimescaleDB for intel history), Redis (Queue/Cache)
- **Metrics**: Prometheus exported on `:8080/metrics`
- **Dashboard**: [https://intel.brainfeed.tech](https://intel.brainfeed.tech)

## 🗺 Roadmap
- [x] Initial Modular Scaffolding.
- [x] Docker Compose Orchestration.
- [ ] Tor-based Scraper Adapter.
- [ ] Real-time NLP Analysis Layer.
