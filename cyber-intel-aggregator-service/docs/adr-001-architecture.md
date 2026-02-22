# ADR 001: Project Architecture - Cyber Intel Aggregator Service

## Status
Proposed

## Context
We need a scalable, robust platform to scrape, process, and display threat intelligence from various sources (Dark Web, Clear Web).

## Decision
We will use a **Tiered Hexagonal Architecture** (Domain/Services/Adapters) to ensure separation of concerns and testability.

### Tech Stack:
- **Backend:** Rust (Axum/Tokio) for performance and safety.
- **Frontend:** Next.js (TypeScript) for a modern, responsive dashboard.
- **Database:** PostgreSQL for structured data; Redis for caching/rate-limiting.
- **NLP:** Integration with specialized libraries or APIs for entity extraction and sentiment analysis.
- **Observability:** Prometheus/Grafana for metrics, Jaeger for tracing.

## Consequences
- High development velocity for Rust might be slightly lower initially due to strictness, but long-term maintenance will be easier.
- Containerization will simplify deployment.
