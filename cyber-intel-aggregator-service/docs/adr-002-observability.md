# ADR 002: Observability - Metrics and Tracing

## Status
Proposed

## Context
A distributed system requires deep visibility for debugging and performance monitoring.

## Decision
- **Metrics**: Prometheus for gathering time-series metrics.
- **Tracing**: OpenTelemetry (OTel) with Jaeger for distributed tracing.
- **Logging**: Structured JSON logging (Tracing crate in Rust).

## Consequences
- Requires OTel collector and backend (Jaeger/Prometheus) in Docker Compose.
- Performance overhead is minimal but present.
