# Cyber Intel Aggregator Service

Advanced threat intelligence platform for Dark Web and Clear Web monitoring.

## Architecture
- **Language**: Rust (Backend), Next.js (Frontend)
- **Database**: PostgreSQL (Primary), Redis (Caching/Queue)
- **Architecture**: Tiered (Domain/Services/Adapters)
- **Observability**: Prometheus & Grafana

## Deployment
This service is production-ready via Docker Compose.
- Live Dashboard: [https://intel.brainfeed.tech](https://intel.brainfeed.tech) (Placeholder)

## Tech Stack
- **NLP**: Custom Rust-based text analysis.
- **Scraping**: Headless browser integration for TOR and Clear Web.
- **Metrics**: Prometheus metrics exported on `/metrics`.
