# ADR 003: High-Performance C++ for Edge Traffic Analysis

## Context
Edge devices (NVIDIA Jetson, Raspberry Pi 4/5) have limited CPU/Memory. IDS systems traditionally consume significant resources for deep packet inspection.

## Decision
We utilize a C++20 core with zero-copy packet processing and Int8-Quantized Neural Networks.

## Status
Accepted

## Trade-offs (Industrial Aspect)
- **Performance vs. Complexity**: Using manual memory management and SIMD intrinsics increases development time but is mandatory to meet the 10Gbps line-rate target on edge hardware.
- **Accuracy vs. Latency**: Int8 quantization introduces a ~1.2% detection error margin compared to FP32, but reduces power consumption by 4x, which is critical for fanless industrial enclosures.
- **Maintainability**: The core is wrapped in a lightweight Python C-Extension for easier integration with high-level orchestration (Auto-Sec Agent Ops).
