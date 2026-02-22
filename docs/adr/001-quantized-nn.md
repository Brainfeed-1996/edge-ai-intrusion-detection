# ADR 001: Selection of Quantized Neural Networks for Edge IDS

## Context
Running deep learning models on ARM/Jetson devices requires balancing accuracy with latency and power consumption.

## Decision
We chose Int8 Quantization for our inference engine.

## Status
Accepted

## Consequences
- **Pros**: Significant reduction in model size and latency; compatible with TensorRT and TFLite hardware acceleration.
- **Cons**: Minor drop in detection precision compared to FP32 models.
