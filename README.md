# Harmonic Inference

An AI-powered music analysis and generation system combining Rust for high-performance signal processing with Python for machine learning inference.

## Project Overview

Harmonic Inference is an engineering portfolio project demonstrating:
- Real-time audio signal processing in Rust
- Deep learning model inference with PyTorch
- REST API for music analysis
- Distributed training pipeline for music generation models
- Production-grade testing and benchmarking

## Features

- Audio feature extraction and analysis
- Neural network based music style classification
- Harmonic progression prediction
- Real-time MIDI processing
- Model serving with low-latency inference
- Comprehensive test suite and benchmarks

## Tech Stack

### Rust
- High-performance audio DSP
- FFT and spectral analysis
- Real-time signal processing
- Binary serialization and caching

### Python
- PyTorch for deep learning
- TensorFlow for model training
- Librosa for audio analysis
- FastAPI for REST endpoints
- Pytest for testing

## Project Structure

```
harmonic-inference/
├── rust/
│   ├── audio-processor/     # Core audio processing library
│   ├── inference-engine/    # Model inference runtime
│   └── Cargo.toml
├── python/
│   ├── training/            # Model training scripts
│   ├── api/                 # FastAPI server
│   ├── models/              # Pre-trained model weights
│   └── requirements.txt
├── tests/                   # Integration tests
├── benchmarks/              # Performance benchmarks
└── docs/                    # Architecture documentation
```

## Getting Started

### Prerequisites
- Rust 1.70+
- Python 3.10+
- CUDA 11.8 (optional, for GPU training)

### Installation

```bash
# Clone repository
git clone https://github.com/legato-sonata/harmonic-inference.git
cd harmonic-inference

# Setup Rust environment
cd rust
cargo build --release

# Setup Python environment
cd ../python
python -m venv venv
source venv/bin/activate
pip install -r requirements.txt
```

## Usage

### Audio Analysis

```python
from harmonic_inference import AudioAnalyzer

analyzer = AudioAnalyzer()
features = analyzer.extract_features('music.wav')
genre = analyzer.classify_genre(features)
harmonics = analyzer.predict_progressions(features)
```

### Running the API

```bash
cd python
uvicorn api.server:app --host 0.0.0.0 --port 8000
```

Then make requests to:
- `POST /api/analyze` - Analyze audio file
- `POST /api/generate` - Generate music based on parameters
- `POST /api/classify` - Classify music genre/style

## Performance Benchmarks

See `benchmarks/` directory for detailed performance metrics on various operations.

## Contributing

Contributions welcome. Please see CONTRIBUTING.md for guidelines.

## License

MIT License - see LICENSE file
