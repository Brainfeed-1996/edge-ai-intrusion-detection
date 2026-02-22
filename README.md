# 🕵️ Edge AI Intrusion Detection System (IDS)

[![C++](https://img.shields.io/badge/Language-C%2B%2B-00599C.svg)](https://isocpp.org)
[![Python](https://img.shields.io/badge/Language-Python-3776AB.svg)](https://www.python.org)
[![Edge AI](https://img.shields.io/badge/Performance-Optimized-orange.svg)]()

A lightweight, high-performance IDS designed for Edge devices (ARM/Jetson).

## Features
- Real-time network traffic analysis using C++/Python.
- Quantized Neural Network for low-latency inference on the edge.
- Designed for industrial IoT and edge computing environments.

## Architecture
1. **Data Capture**: High-speed packet capture using Libpcap/PF_RING.
2. **Feature Extraction**: Efficient C++ processing of flow-based features.
3. **AI Inference**: Quantized TensorFlow Lite / TensorRT model for classification.
4. **Alerting**: Low-overhead signaling mechanism.

## SRE/Monitoring
- Performance monitoring using built-in telemetry.
- Latency and throughput tracking for AI inference.

## ADR
- [ADR-001: Quantized Neural Network for Edge](docs/adr/001-quantized-nn.md)
- [ADR-004: C++/Python Hybridization via pybind11](docs/adr/004-cpp-python-hybridization.md)
