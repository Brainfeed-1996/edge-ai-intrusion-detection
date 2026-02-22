# ADR 001: Backend Language Choice

## Status
Accepted

## Context
The Cyber Intel Aggregator requires high performance for scraping and NLP tasks, type safety, and efficient concurrency.

## Decision
We will use Rust for the backend services.

## Consequences
- High performance and memory safety.
- Steep learning curve for some team members.
- Excellent ecosystem for security tooling.

---

# ADR 002: Architectural Pattern

## Status
Accepted

## Context
Need to decouple the core business logic (scraping, processing) from external interfaces (DB, API, Web).

## Decision
We will use a tiered architecture:
- **Domain**: Core business entities and logic.
- **Services**: Orchestration of domain logic.
- **Adapters**: External interfaces (PostgreSQL, Scraping drivers, REST API).

## Consequences
- Better testability.
- Easier to swap out databases or scraping libraries.
