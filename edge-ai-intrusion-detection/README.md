# Edge AI Intrusion Detection System (IDS)

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
