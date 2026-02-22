# ADR 004: C++/Python Hybridization for Edge IDS

## Status
Accepted

## Context
High-performance packet inspection requires C++ for efficiency, but flexible AI reporting and integration are easier to implement in Python.

## Decision
Use `pybind11` to create a Python wrapper for the core C++ IDS engine.
- C++: Handles low-level packet processing and quantized neural network inference.
- Python: Orchestrates the workflow, provides a lightweight AI classification layer for reporting, and handles network communication with central services.

## Consequences
- Requires a build step (CMake/pybind11) to generate the `.so` or `.pyd` module.
- Development speed increases for the reporting layer while maintaining performance for the hot path.
